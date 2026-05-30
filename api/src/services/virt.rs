use std::sync::Arc;
use serde_json::json;
use crate::client::TrueNasClient;
use crate::methods::Virt;
use crate::models::virt::{
    ContainerResponse, ContainerUpdate, Device, ImageChoice, StopArgs,
};
use crate::result::ApiResult;

pub struct VirtService {
    client: Arc<TrueNasClient>,
}

impl VirtService {
    pub fn new(client: Arc<TrueNasClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_instances(&self) -> ApiResult<Vec<ContainerResponse>> {
        self.client
            .call::<Vec<ContainerResponse>>(Virt::GET_ALL_INSTANCES, vec![])
            .await
    }

    pub async fn get_instance(&self, id: &str) -> ApiResult<ContainerResponse> {
        self.client
            .call::<ContainerResponse>(Virt::GET_ALL_INSTANCES, vec![json!(id)])
            .await
    }

    pub async fn start_instance(&self, id: &str) -> ApiResult<f64> {
        self.client
            .call::<f64>(Virt::START_INSTANCE, vec![json!(id)])
            .await
    }

    pub async fn stop_instance(
        &self,
        id: &str,
        timeout: Option<i32>,
        force: Option<bool>,
    ) -> ApiResult<f64> {
        let args = StopArgs { timeout, force };
        self.client
            .call::<f64>(Virt::STOP_INSTANCE, vec![json!(id), json!(args)])
            .await
    }

    pub async fn restart_instance(
        &self,
        id: &str,
        timeout: Option<i32>,
        force: Option<bool>,
    ) -> ApiResult<i32> {
        let args = StopArgs { timeout, force };
        self.client
            .call::<i32>(Virt::RESTART_INSTANCE, vec![json!(id), json!(args)])
            .await
    }

    pub async fn delete_instance(&self, id: &str) -> ApiResult<i32> {
        self.client
            .call::<i32>(Virt::DELETE_INSTANCE, vec![json!(id)])
            .await
    }

    pub async fn update_instance(
        &self,
        id: &str,
        update: ContainerUpdate,
    ) -> ApiResult<ContainerResponse> {
        self.client
            .call::<ContainerResponse>(Virt::UPDATE_INSTANCE, vec![json!(id), json!(update)])
            .await
    }

    pub async fn get_instance_devices(&self, id: &str) -> ApiResult<Vec<Device>> {
        // note: the original Kotlin used GET_ALL_INSTANCES with an id to get devices
        // but the proper endpoint for devices might be different.
        // Here we keep the same logic.
        self.client
            .call::<Vec<Device>>(Virt::GET_ALL_INSTANCES, vec![json!(id)])
            .await
    }

    pub async fn delete_instance_device(&self, id: &str, device_name: &str) -> ApiResult<bool> {
        self.client
            .call::<bool>(Virt::DELETE_INSTANCE_DEVICE, vec![json!(id), json!(device_name)])
            .await
    }

    pub async fn get_image_choices(&self) -> ApiResult<Vec<ImageChoice>> {
        self.client
            .call::<Vec<ImageChoice>>(Virt::GET_IMAGE_CHOICES, vec![])
            .await
    }
}