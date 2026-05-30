use std::sync::{Arc, Mutex};
use api::manager::TrueNasApiManager;

#[derive(Clone)]
pub struct AppState {
    pub manager: Arc<Mutex<Option<TrueNasApiManager>>>,
    pub token: Arc<Mutex<Option<String>>>,
    pub username: Arc<Mutex<Option<String>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            manager: Arc::new(Mutex::new(None)),
            token: Arc::new(Mutex::new(None)),
            username: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_manager(&self, m: TrueNasApiManager) {
        *self.manager.lock().unwrap() = Some(m);
    }

    pub fn set_token(&self, t: String) {
        *self.token.lock().unwrap() = Some(t);
    }

    pub fn set_username(&self, u: String) {
        *self.username.lock().unwrap() = Some(u);
    }

    pub fn get_username(&self) -> String {
        self.username.lock().unwrap()
            .clone()
            .unwrap_or_else(|| "User".to_string())
    }
}