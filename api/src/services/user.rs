use std::sync::Arc;
use serde_json::json;
use crate::client::TrueNasClient;
use crate::methods::User;
use crate::models::user::{
    ChangeUserPasswordRequest, GetUserObjRequest, UserObjResponse,
    UserUpdate, UserUpdateResponse,
};
use crate::result::ApiResult;

pub struct UserService {
    client: Arc<TrueNasClient>,
}

impl UserService {
    pub fn new(client: Arc<TrueNasClient>) -> Self {
        Self { client }
    }

    pub async fn change_password(
        &self,
        username: &str,
        old_password: &str,
        new_password: &str,
    ) -> ApiResult<serde_json::Value> {
        let req = ChangeUserPasswordRequest {
            username: username.to_string(),
            old_password: old_password.to_string(),
            new_password: new_password.to_string(),
        };
        self.client
            .call::<serde_json::Value>(User::CHANGE_PASSWORD, vec![json!(req)])
            .await
    }

    pub async fn update_user(
        &self,
        user_id: i32,
        update: UserUpdate,
    ) -> ApiResult<UserUpdateResponse> {
        self.client
            .call::<UserUpdateResponse>(User::USER_UPDATE, vec![json!(user_id), json!(update)])
            .await
    }

    pub async fn get_user_obj(&self, req: GetUserObjRequest) -> ApiResult<UserObjResponse> {
        self.client
            .call::<UserObjResponse>(User::GET_USER_OBJ, vec![json!(req)])
            .await
    }
}