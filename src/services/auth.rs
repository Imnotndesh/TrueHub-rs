use std::sync::Arc;
use serde_json::json;
use crate::client::TrueNasClient;
use crate::methods::Auth as AuthMethods;
use crate::models::auth::{
    AuthResponse, AuthSessionResultItem, AuthenticatorLevel,
    LoginExResult, LoginMechanism, TokenRequest,
};
use crate::result::ApiResult;

pub struct AuthService {
    client: Arc<TrueNasClient>,
}

impl AuthService {
    pub fn new(client: Arc<TrueNasClient>) -> Self {
        Self { client }
    }

    pub async fn login(&self, username: &str, password: &str) -> ApiResult<bool> {
        self.client.call::<bool>(
            AuthMethods::LOGIN,
            vec![json!(username), json!(password)],
        ).await
    }

    pub async fn login_with_api_key(&self, api_key: &str) -> ApiResult<bool> {
        self.client.call::<bool>(
            AuthMethods::API_LOGIN,
            vec![json!(api_key)],
        ).await
    }

    pub async fn login_with_token(&self, token: &str) -> ApiResult<bool> {
        self.client.call::<bool>(
            AuthMethods::TOKEN_LOGIN,
            vec![json!(token)],
        ).await
    }

    pub async fn logout(&self) -> ApiResult<bool> {
        self.client.call::<bool>(AuthMethods::LOGOUT, vec![]).await
    }

    pub async fn me(&self) -> ApiResult<AuthResponse> {
        self.client.call::<AuthResponse>(AuthMethods::ME, vec![]).await
    }

    pub async fn generate_token(&self, request: TokenRequest) -> ApiResult<String> {
        self.client.call::<String>(
            AuthMethods::GEN_TOKEN,
            vec![
                json!(request.ttl),
                json!(request.attrs),
                json!(request.match_origin),
                json!(request.single_use),
            ],
        ).await
    }

    pub async fn generate_onetime_password(&self, username: &str) -> ApiResult<String> {
        self.client.call::<String>(
            AuthMethods::GEN_ONETIME_PASSWORD,
            vec![json!({ "username": username })],
        ).await
    }

    pub async fn mechanism_choices(&self) -> ApiResult<Vec<String>> {
        self.client.call::<Vec<String>>(AuthMethods::MECHANISM_CHOICES, vec![]).await
    }

    pub async fn login_ex(
        &self,
        mechanism: LoginMechanism,
        include_user_info: bool,
        include_reconnect_token: bool,
    ) -> ApiResult<LoginExResult> {
        let mut params = serde_json::to_value(&mechanism).unwrap_or(json!({}));

        let mut login_options = serde_json::Map::new();
        if include_user_info { login_options.insert("user_info".into(), json!(true)); }
        if include_reconnect_token { login_options.insert("reconnect_token".into(), json!(true)); }
        if !login_options.is_empty() {
            params["login_options"] = serde_json::Value::Object(login_options);
        }

        let raw = self.client.call::<serde_json::Value>(
            AuthMethods::LOGIN_EX,
            vec![params],
        ).await;

        match raw {
            ApiResult::Success(val) => {
                let response_type = val["response_type"].as_str().unwrap_or("AUTH_ERR");
                let result = match response_type {
                    "SUCCESS" => {
                        let level = match val["authenticator"].as_str().unwrap_or("LEVEL_1") {
                            "LEVEL_2" => AuthenticatorLevel::Level2,
                            _ => AuthenticatorLevel::Level1,
                        };
                        let user_info = val.get("user_info")
                            .and_then(|u| serde_json::from_value(u.clone()).ok());
                        LoginExResult::Success { user_info, authenticator: level }
                    }
                    "OTP_REQUIRED" => LoginExResult::OtpRequired {
                        username: val["username"].as_str().unwrap_or("").to_string(),
                    },
                    "EXPIRED" => LoginExResult::Expired,
                    "REDIRECT" => {
                        let urls = val["urls"].as_array()
                            .map(|a| a.iter().filter_map(|u| u.as_str().map(String::from)).collect())
                            .unwrap_or_default();
                        LoginExResult::Redirect { urls }
                    }
                    _ => LoginExResult::AuthErr,
                };
                ApiResult::Success(result)
            }
            ApiResult::Error { message } => ApiResult::Error { message },
            ApiResult::Loading => ApiResult::Loading,
        }
    }

    pub async fn get_sessions(&self) -> ApiResult<Vec<AuthSessionResultItem>> {
        self.client.call::<Vec<AuthSessionResultItem>>(AuthMethods::GET_SESSIONS, vec![]).await
    }

    pub async fn terminate_other_sessions(&self) -> ApiResult<bool> {
        self.client.call::<bool>(AuthMethods::TERMINATE_OTHER_SESSION, vec![]).await
    }

    pub async fn terminate_session(&self, id: &str) -> ApiResult<bool> {
        self.client.call::<bool>(AuthMethods::TERMINATE_SESSION, vec![json!(id)]).await
    }
}