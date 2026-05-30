use std::sync::Arc;
use serde_json::json;
use crate::client::TrueNasClient;
use crate::methods::Vm;
use crate::models::vm::{
    DeleteOptions, StartOptions, StopOptions, VmDisplayUriQueryResponse,
    VmQueryResponse, VmStatus,
};
use crate::result::ApiResult;

pub struct VmService {
    client: Arc<TrueNasClient>,
}

impl VmService {
    pub fn new(client: Arc<TrueNasClient>) -> Self {
        Self { client }
    }

    pub async fn query_all_vms(&self) -> ApiResult<Vec<VmQueryResponse>> {
        self.client
            .call::<Vec<VmQueryResponse>>(Vm::GET_ALL_VM_INSTANCES, vec![])
            .await
    }

    pub async fn get_vm_instance(&self, id: i32) -> ApiResult<VmQueryResponse> {
        self.client
            .call::<VmQueryResponse>(Vm::GET_INSTANCE, vec![json!(id)])
            .await
    }

    pub async fn start_vm(&self, id: i32, overcommit: bool) -> ApiResult<i32> {
        let options = StartOptions { overcommit };
        self.client
            .call::<i32>(Vm::START_VM_INSTANCE, vec![json!(id), json!(options)])
            .await
    }

    pub async fn restart_vm(&self, id: i32) -> ApiResult<i32> {
        self.client
            .call::<i32>(Vm::RESTART_INSTANCE, vec![json!(id)])
            .await
    }

    pub async fn stop_vm(
        &self,
        id: i32,
        force: Option<bool>,
        force_after_timeout: Option<bool>,
    ) -> ApiResult<i32> {
        let options = StopOptions {
            force: force.unwrap_or(false),
            force_after_timeout: force_after_timeout.unwrap_or(false),
        };
        self.client
            .call::<i32>(Vm::STOP_INSTANCE, vec![json!(id), json!(options)])
            .await
    }

    pub async fn suspend_vm(&self, id: i32) -> ApiResult<serde_json::Value> {
        self.client
            .call::<serde_json::Value>(Vm::SUSPEND_VM, vec![json!(id)])
            .await
    }

    pub async fn power_off_vm(&self, id: i32) -> ApiResult<serde_json::Value> {
        self.client
            .call::<serde_json::Value>(Vm::POWER_OFF_VM, vec![json!(id)])
            .await
    }

    pub async fn resume_vm(&self, id: i32) -> ApiResult<serde_json::Value> {
        self.client
            .call::<serde_json::Value>(Vm::RESUME_VM, vec![json!(id)])
            .await
    }

    pub async fn clone_vm(&self, id: i32, clone_name: Option<&str>) -> ApiResult<bool> {
        self.client
            .call::<bool>(Vm::CLONE_VM, vec![json!(id), json!(clone_name)])
            .await
    }

    pub async fn get_memory_usage(&self, id: i32) -> ApiResult<i32> {
        self.client
            .call::<i32>(Vm::GET_VM_MEMORY_USAGE, vec![json!(id)])
            .await
    }

    pub async fn delete_vm(
        &self,
        id: i32,
        delete_zvols: Option<bool>,
        force: Option<bool>,
    ) -> ApiResult<bool> {
        let options = DeleteOptions {
            zvols: delete_zvols.unwrap_or(false),
            force: force.unwrap_or(false),
        };
        self.client
            .call::<bool>(Vm::DELETE_INSTANCE, vec![json!(id), json!(options)])
            .await
    }

    pub async fn get_display_uri(&self, id: i32) -> ApiResult<VmDisplayUriQueryResponse> {
        self.client
            .call::<VmDisplayUriQueryResponse>(Vm::GET_DISPLAY_URL, vec![json!(id)])
            .await
    }

    pub async fn get_vm_status(&self, id: i32) -> ApiResult<VmStatus> {
        self.client
            .call::<VmStatus>(Vm::GET_VM_STATUS, vec![json!(id)])
            .await
    }
}