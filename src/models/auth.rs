use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub pw_name: Option<String>,
    #[serde(rename = "pw_gecos")]
    pub pw_gecos: Option<String>,
    #[serde(rename = "pw_dir")]
    pub pw_dir: Option<String>,
    #[serde(rename = "pw_shell")]
    pub pw_shell: Option<String>,
    #[serde(rename = "pw_uid")]
    pub pw_uid: Option<u64>,
    #[serde(rename = "pw_gid")]
    pub pw_gid: Option<u64>,
    #[serde(rename = "grouplist")]
    pub group_list: Option<Vec<String>>,
    pub sid: Option<String>,
    pub source: Option<String>,
    pub local: bool,
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    pub two_factor_config: Option<HashMap<String, serde_json::Value>>,
    pub privilege: Option<HashMap<String, serde_json::Value>>,
    pub account_attributes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequest {
    pub ttl: i32,
    pub attrs: HashMap<String, serde_json::Value>,
    pub match_origin: bool,
    pub single_use: bool,
}

impl Default for TokenRequest {
    fn default() -> Self {
        Self {
            ttl: 6000,
            attrs: HashMap::new(),
            match_origin: true,
            single_use: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSessionResultItem {
    pub id: Option<String>,
    pub current: bool,
    pub internal: bool,
    pub origin: String,
    pub credentials: Credentials,
    pub credentials_data: Option<HashMap<String, serde_json::Value>>,
    pub secure_transport: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Credentials {
    UnixSocket,
    LoginPassword,
    LoginTwofactor,
    LoginOnetimePassword,
    ApiKey,
    Token,
    TruenasNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOneTimePasswordRequest {
    pub username: String,
}

// LoginMechanisms
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mechanism", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoginMechanism {
    PasswordPlain {
        username: String,
        password: String,
    },
    ApiKeyPlain {
        username: String,
        api_key: String,
    },
    AuthTokenPlain {
        token: String,
    },
    OtpToken {
        otp_token: String,
    },
}

// LoginExResult
#[derive(Debug, Clone)]
pub enum LoginExResult {
    Success {
        user_info: Option<AuthResponse>,
        authenticator: AuthenticatorLevel,
    },
    OtpRequired {
        username: String,
    },
    AuthErr,
    Expired,
    Redirect {
        urls: Vec<String>,
    },
}

#[derive(Debug, Clone)]
pub enum AuthenticatorLevel {
    Level1,
    Level2,
}