mod client;
mod manager;
pub mod result;
pub mod methods;
pub mod services;
pub mod models;
mod logger;
mod store;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use manager::TrueNasApiManager;
    use models::auth::TokenRequest;
    use result::ApiResult;

    #[tokio::test]
    async fn test_login_and_get_token() {
        let manager = TrueNasApiManager::new("wss://192.168.1.70/api/current", true);
        let connected = manager.connect().await;
        if !connected {
            let state = manager.client.state_receiver().borrow().clone();
            println!("Connection state: {:?}", state);
        }
        assert!(connected, "Failed to connect");

        let login = manager.auth.login("admin", "s3rv3rhp").await;
        match &login {
            ApiResult::Success(ok) => println!("Login success: {ok}"),
            ApiResult::Error { message } => panic!("Login failed: {message}"),
            _ => panic!("Unexpected state"),
        }
        assert!(matches!(login, ApiResult::Success(true)));

        let token = manager.auth.generate_token(TokenRequest::default()).await;
        match &token {
            ApiResult::Success(t) => println!("Token: {t}"),
            ApiResult::Error { message } => panic!("Token generation failed: {message}"),
            _ => panic!("Unexpected state"),
        }
        assert!(matches!(token, ApiResult::Success(_)));

        manager.disconnect().await;
    }
}
