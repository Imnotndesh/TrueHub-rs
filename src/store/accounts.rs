use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use keyring::Entry;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoginMethod {
    Password,
    ApiKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedServer {
    pub id: String,
    pub server_url: String,
    pub insecure: bool,
    pub nickname: Option<String>,
    pub last_used: u64,
}

impl SavedServer {
    pub fn new(server_url: impl Into<String>, insecure: bool, nickname: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            server_url: server_url.into(),
            insecure,
            nickname,
            last_used: now_millis(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedAccount {
    pub id: String,
    pub server_id: String,
    pub username: String,
    pub login_method: LoginMethod,
    pub last_used: u64,
    pub auto_login_enabled: bool,
}

impl SavedAccount {
    pub fn new(server_id: impl Into<String>, username: impl Into<String>, login_method: LoginMethod) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            server_id: server_id.into(),
            username: username.into(),
            login_method,
            last_used: now_millis(),
            auto_login_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct AccountStore {
    servers: Vec<SavedServer>,
    accounts: Vec<SavedAccount>,
    last_used_profile: Option<String>,
    current_session: Option<String>,
}

fn store_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("truehub")
        .join("accounts.json")
}

fn now_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn keyring_entry(account_id: &str, key_type: &str) -> Result<Entry, String> {
    Entry::new(&format!("truehub_{account_id}"), key_type).map_err(|e| e.to_string())
}

pub fn save_credential(account_id: &str, key_type: &str, value: &str) -> Result<(), String> {
    keyring_entry(account_id, key_type)?.set_password(value).map_err(|e| e.to_string())
}

pub fn get_credential(account_id: &str, key_type: &str) -> Option<String> {
    keyring_entry(account_id, key_type).ok()?.get_password().ok()
}

pub fn delete_credential(account_id: &str, key_type: &str) {
    if let Ok(entry) = keyring_entry(account_id, key_type) {
        let _ = entry.delete_credential();
    }
}

pub fn save_account_credentials(
    account_id: &str,
    login_method: &LoginMethod,
    api_key: Option<&str>,
    username: Option<&str>,
    password: Option<&str>,
) -> Result<(), String> {
    match login_method {
        LoginMethod::ApiKey => {
            if let Some(key) = api_key {
                save_credential(account_id, "api_key", key)?;
            }
        }
        LoginMethod::Password => {
            if let Some(u) = username { save_credential(account_id, "username", u)?; }
            if let Some(p) = password { save_credential(account_id, "password", p)?; }
        }
    }
    Ok(())
}

pub fn get_account_credentials(account_id: &str, login_method: &LoginMethod) -> (Option<String>, Option<String>) {
    match login_method {
        LoginMethod::ApiKey => (get_credential(account_id, "api_key"), None),
        LoginMethod::Password => (
            get_credential(account_id, "username"),
            get_credential(account_id, "password"),
        ),
    }
}

pub fn clear_account_credentials(account_id: &str) {
    delete_credential(account_id, "api_key");
    delete_credential(account_id, "username");
    delete_credential(account_id, "password");
}

async fn load_store() -> AccountStore {
    match fs::read_to_string(store_path()).await {
        Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
        Err(_) => AccountStore::default(),
    }
}

async fn save_store(store: &AccountStore) -> Result<(), String> {
    let path = store_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(store).map_err(|e| e.to_string())?;
    fs::write(path, json).await.map_err(|e| e.to_string())
}

pub async fn save_server(server: SavedServer) -> Result<(), String> {
    let mut store = load_store().await;
    match store.servers.iter().position(|s| s.id == server.id) {
        Some(i) => store.servers[i] = server,
        None => store.servers.push(server),
    }
    save_store(&store).await
}

pub async fn get_servers() -> Vec<SavedServer> {
    load_store().await.servers
}

pub async fn get_server(server_id: &str) -> Option<SavedServer> {
    get_servers().await.into_iter().find(|s| s.id == server_id)
}

pub async fn delete_server(server_id: &str) -> Result<(), String> {
    let mut store = load_store().await;
    let removed_accounts: Vec<_> = store.accounts.iter()
        .filter(|a| a.server_id == server_id)
        .map(|a| a.id.clone())
        .collect();
    for id in &removed_accounts {
        clear_account_credentials(id);
    }
    store.servers.retain(|s| s.id != server_id);
    store.accounts.retain(|a| a.server_id != server_id);
    save_store(&store).await
}

pub async fn save_account(account: SavedAccount) -> Result<(), String> {
    let mut store = load_store().await;
    match store.accounts.iter().position(|a| a.id == account.id) {
        Some(i) => store.accounts[i] = account,
        None => store.accounts.push(account),
    }
    save_store(&store).await
}

pub async fn get_accounts() -> Vec<SavedAccount> {
    load_store().await.accounts
}

pub async fn get_account(account_id: &str) -> Option<SavedAccount> {
    get_accounts().await.into_iter().find(|a| a.id == account_id)
}

pub async fn get_accounts_for_server(server_id: &str) -> Vec<SavedAccount> {
    get_accounts().await.into_iter().filter(|a| a.server_id == server_id).collect()
}

pub async fn delete_account(account_id: &str) -> Result<(), String> {
    clear_account_credentials(account_id);
    let mut store = load_store().await;
    store.accounts.retain(|a| a.id != account_id);
    save_store(&store).await
}

pub async fn save_last_used_profile(server_id: &str, account_id: &str) -> Result<(), String> {
    let mut store = load_store().await;
    store.last_used_profile = Some(format!("{server_id}:{account_id}"));
    if let Some(s) = store.servers.iter_mut().find(|s| s.id == server_id) {
        s.last_used = now_millis();
    }
    if let Some(a) = store.accounts.iter_mut().find(|a| a.id == account_id) {
        a.last_used = now_millis();
    }
    save_store(&store).await
}

pub async fn get_last_used_profile() -> Option<(String, String)> {
    let store = load_store().await;
    let value = store.last_used_profile?;
    let parts: Vec<_> = value.splitn(2, ':').collect();
    if parts.len() != 2 { return None; }
    Some((parts[0].to_string(), parts[1].to_string()))
}

pub async fn save_current_session(server_id: &str, account_id: &str, token: &str) -> Result<(), String> {
    save_credential("session", "token", token)?;
    let mut store = load_store().await;
    store.current_session = Some(format!("{server_id}:{account_id}"));
    save_store(&store).await
}

pub async fn get_current_session() -> Option<(String, String, String)> {
    let store = load_store().await;
    let session = store.current_session?;
    let token = get_credential("session", "token")?;
    let parts: Vec<_> = session.splitn(2, ':').collect();
    if parts.len() != 2 { return None; }
    Some((parts[0].to_string(), parts[1].to_string(), token))
}

pub async fn clear_current_session() -> Result<(), String> {
    delete_credential("session", "token");
    let mut store = load_store().await;
    store.current_session = None;
    save_store(&store).await
}

pub async fn clear_all() -> Result<(), String> {
    let store = load_store().await;
    for account in &store.accounts {
        clear_account_credentials(&account.id);
    }
    clear_account_credentials("session");
    fs::remove_file(store_path()).await.map_err(|e| e.to_string())
}