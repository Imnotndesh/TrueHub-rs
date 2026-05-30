// user.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdateRequest {
    pub id: i32,
    pub user_update: UserUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserUpdate {
    pub username: Option<String>,
    #[serde(rename = "full_name")]
    pub full_name: Option<String>,
    pub shell: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub password_disabled: Option<bool>,
    pub locked: Option<bool>,
    pub sudo_commands: Option<Vec<String>>,
    pub sudo_commands_nopasswd: Option<Vec<String>>,
    pub sshpubkey: Option<String>,
    pub group: Option<i32>,
    pub groups: Option<Vec<i32>>,
    pub home: Option<String>,
    pub home_create: Option<bool>,
    pub home_mode: Option<String>,
    pub uid: Option<i32>,
    pub smb: Option<bool>,
    #[serde(rename = "subuid")]
    pub subuid: Option<serde_json::Value>, // can be "DIRECT", i32, or null
    pub group_create: Option<bool>,
    pub generate_random_password: Option<bool>,
    pub roles: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdateResponse {
    pub id: i32,
    pub uid: i32,
    pub username: String,
    pub unixhash: Option<String>,
    pub smbhash: Option<String>,
    #[serde(default = "default_home")]
    pub home: String,
    #[serde(default = "default_shell")]
    pub shell: String,
    pub full_name: String,
    pub builtin: bool,
    #[serde(default = "default_true")]
    pub smb: bool,
    pub subuid: Option<serde_json::Value>,
    #[serde(default)]
    pub password_disabled: bool,
    #[serde(default)]
    pub locked: bool,
    pub sudo_commands: Option<Vec<String>>,
    pub sudo_commands_nopasswd: Option<Vec<String>>,
    pub email: Option<String>,
    pub group: UserGroup,
    pub groups: Vec<i32>,
    pub sshpubkey: Option<String>,
    pub immutable: bool,
    pub twofactor_auth_configured: bool,
    pub local: bool,
    pub id_type_both: bool,
    pub nt_name: Option<String>,
    pub sid: Option<String>,
    pub roles: Vec<String>,
    pub password_change_required: bool,
    pub password_last_change: Option<String>,
    pub password_age_days: Option<i32>,
    pub password_history: Option<Vec<PasswordHistoryEntry>>,
    pub password: Option<String>,
}

fn default_home() -> String { "/var/empty".to_string() }
fn default_shell() -> String { "/usr/bin/zsh".to_string() }
fn default_true() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroup {
    pub id: i32,
    pub bsdgrp_gid: i32,
    pub bsdgrp_group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHistoryEntry {
    pub hash: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserObjRequest {
    pub username: Option<String>,
    pub uid: Option<i32>,
    pub get_groups: Option<bool>,
    pub sid_info: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserObjResponse {
    pub pw_name: String,
    pub pw_gecos: String,
    pub pw_dir: String,
    pub pw_shell: String,
    pub pw_uid: i32,
    pub pw_gid: i32,
    pub grouplist: Option<Vec<i32>>,
    pub sid: Option<String>,
    pub source: UserSource,
    pub local: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserSource {
    Local,
    ActiveDirectory,
    Ldap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellChoice {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeUserPasswordRequest {
    pub username: String,
    pub old_password: String,
    pub new_password: String,
}