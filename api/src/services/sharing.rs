use std::sync::Arc;
use crate::client::TrueNasClient;
use crate::methods::Shares;
use crate::models::shares::{NfsShare, SmbShare};
use crate::result::ApiResult;

pub struct SharingService {
    client: Arc<TrueNasClient>,
}

impl SharingService {
    pub fn new(client: Arc<TrueNasClient>) -> Self {
        Self { client }
    }

    pub async fn get_smb_shares(&self) -> ApiResult<Vec<SmbShare>> {
        self.client
            .call::<Vec<SmbShare>>(Shares::GET_SMB_SHARES, vec![])
            .await
    }

    pub async fn get_nfs_shares(&self) -> ApiResult<Vec<NfsShare>> {
        self.client
            .call::<Vec<NfsShare>>(Shares::GET_NFS_SHARES, vec![])
            .await
    }
}