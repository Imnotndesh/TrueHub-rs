use std::sync::Arc;
use crate::client::TrueNasClient;
use crate::result::ApiResult;
use crate::services::auth::AuthService;
use crate::services::system::SystemService;

pub struct TrueNasApiManager {
    pub(crate) client: Arc<TrueNasClient>,
    pub auth: AuthService,
    pub system: SystemService,
}

impl TrueNasApiManager {
    pub fn new(server_url: impl Into<String>, insecure: bool) -> Self {
        let client = Arc::new(TrueNasClient::new(server_url, insecure));
        let auth = AuthService::new(Arc::clone(&client));
        let system = SystemService::new(Arc::clone(&client));
        Self { client, auth,system }
    }

    pub async fn connect(&self) -> bool {
        self.client.connect().await
    }

    pub async fn disconnect(&self) {
        self.client.disconnect().await
    }

    pub fn is_connected(&self) -> bool {
        matches!(
            *self.client.state_receiver().borrow(),
            crate::client::ConnectionState::Connected
        )
    }

    pub async fn call<T>(&self, method: &str, params: Vec<serde_json::Value>) -> ApiResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let result = self.client.call::<T>(method, params.clone()).await;
        if self.is_auth_error(&result) {
            return self.attempt_recovery(method, params).await;
        }
        result
    }

    fn is_auth_error<T>(&self, result: &ApiResult<T>) -> bool {
        match result {
            ApiResult::Error { message } => {
                let msg = message.to_lowercase();
                msg.contains("enotauthenticated") || msg.contains("invalid session")
            }
            _ => false,
        }
    }

    async fn attempt_recovery<T>(&self, method: &str, params: Vec<serde_json::Value>) -> ApiResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        if self.client.connect().await {
            return self.client.call::<T>(method, params).await;
        }
        ApiResult::Error { message: "Session expired. Please login again.".to_string() }
    }
}