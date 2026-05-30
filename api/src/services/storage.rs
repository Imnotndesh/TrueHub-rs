use std::sync::Arc;
use serde_json::json;
use crate::client::TrueNasClient;
use crate::methods::Storage;
use crate::models::storage::{
    DatasetCreationResponse, DatasetDetailsResponse, DatasetOptions,
    DeleteSnapshotTaskArgs, DeleteWillChangeRetentionForArgs, DestroySnapshotsArgs,
    ExecuteSnapshotTaskArgs, GetSnapshotTaskInstanceArgs, PoolScrubQueryResponse,
    PoolScrubQuerySingleArgs, RunPoolScrubArgs, SnapshotCreationResponse,
    SnapshotTaskCreateArgs, TakeActionOnPoolScrubArgs, UpdatePoolScrubArgs,
    UpdateSnapshotTaskArgs, UpdateWillChangeRetentionForArgs, ZfsDataset,
};
use crate::result::ApiResult;

pub struct StorageService {
    client: Arc<TrueNasClient>,
}

impl StorageService {
    pub fn new(client: Arc<TrueNasClient>) -> Self {
        Self { client }
    }

    // Scrub tasks
    pub async fn get_scrub_tasks(&self) -> ApiResult<Vec<PoolScrubQueryResponse>> {
        self.client
            .call::<Vec<PoolScrubQueryResponse>>(Storage::POOL_SCRUB_QUERY, vec![])
            .await
    }

    pub async fn get_scrub_task_instance(
        &self,
        args: PoolScrubQuerySingleArgs,
    ) -> ApiResult<PoolScrubQueryResponse> {
        self.client
            .call::<PoolScrubQueryResponse>(Storage::POOL_SCRUB_GET_INSTANCE, vec![json!(args)])
            .await
    }

    pub async fn create_scrub_task(
        &self,
        data: crate::models::storage::UpdatePoolScrubDetails,
    ) -> ApiResult<PoolScrubQueryResponse> {
        self.client
            .call::<PoolScrubQueryResponse>(Storage::POOL_SCRUB_CREATE, vec![json!(data)])
            .await
    }

    pub async fn delete_scrub_task(&self, id: i32) -> ApiResult<bool> {
        self.client
            .call::<bool>(Storage::POOL_SCRUB_DELETE, vec![json!(id)])
            .await
    }

    pub async fn update_scrub_task(
        &self,
        args: UpdatePoolScrubArgs,
    ) -> ApiResult<PoolScrubQueryResponse> {
        self.client
            .call::<PoolScrubQueryResponse>(Storage::POOL_SCRUB_UPDATE, vec![json!(args.id_), json!(args.data)])
            .await
    }

    pub async fn set_scrub_state(&self, args: TakeActionOnPoolScrubArgs) -> ApiResult<i32> {
        self.client
            .call::<i32>(Storage::POOL_SCRUB_ACTION, vec![json!(args.name), json!(args.action)])
            .await
    }

    pub async fn run_scrub_task(&self, args: RunPoolScrubArgs) -> ApiResult<bool> {
        self.client
            .call::<bool>(Storage::POOL_SCRUB_RUN, vec![json!(args.name), json!(args.threshold)])
            .await
    }

    // Snapshot tasks
    pub async fn create_snapshot_task(
        &self,
        args: SnapshotTaskCreateArgs,
    ) -> ApiResult<SnapshotCreationResponse> {
        self.client
            .call::<SnapshotCreationResponse>(Storage::SNAPSHOT_TASK_CREATE, vec![json!(args)])
            .await
    }

    pub async fn delete_snapshot_task(&self, args: DeleteSnapshotTaskArgs) -> ApiResult<i32> {
        self.client
            .call::<i32>(Storage::SNAPSHOT_TASK_DELETE, vec![json!(args)])
            .await
    }

    pub async fn check_snapshot_task_affected_by_deletion(
        &self,
        args: DeleteWillChangeRetentionForArgs,
    ) -> ApiResult<Vec<serde_json::Value>> {
        self.client
            .call::<Vec<serde_json::Value>>(
                Storage::SNAPSHOT_TASK_DELETE_WILL_CHANGE_RETENTION,
                vec![json!(args)],
            )
            .await
    }

    pub async fn execute_snapshot_task(&self, args: ExecuteSnapshotTaskArgs) -> ApiResult<serde_json::Value> {
        self.client
            .call::<serde_json::Value>(Storage::SNAPSHOT_TASK_RUN, vec![json!(args)])
            .await
    }

    pub async fn query_all_snapshot_tasks(&self) -> ApiResult<Vec<SnapshotCreationResponse>> {
        self.client
            .call::<Vec<SnapshotCreationResponse>>(Storage::SNAPSHOT_TASK_CREATE, vec![])
            .await
    }

    pub async fn get_snapshot_task_instance(
        &self,
        args: GetSnapshotTaskInstanceArgs,
    ) -> ApiResult<SnapshotCreationResponse> {
        self.client
            .call::<SnapshotCreationResponse>(Storage::SNAPSHOT_TASK_GET_INSTANCE, vec![json!(args)])
            .await
    }

    pub async fn update_snapshot_task(
        &self,
        args: UpdateSnapshotTaskArgs,
    ) -> ApiResult<SnapshotCreationResponse> {
        self.client
            .call::<SnapshotCreationResponse>(Storage::SNAPSHOT_TASK_UPDATE, vec![json!(args)])
            .await
    }

    pub async fn check_affected_tasks_if_updated(
        &self,
        args: UpdateWillChangeRetentionForArgs,
    ) -> ApiResult<Vec<serde_json::Value>> {
        self.client
            .call::<Vec<serde_json::Value>>(
                Storage::SNAPSHOT_TASK_UPDATE_WILL_CHANGE_RETENTION,
                vec![json!(args)],
            )
            .await
    }

    // Dataset operations
    pub async fn get_all_datasets(&self) -> ApiResult<Vec<ZfsDataset>> {
        self.client
            .call::<Vec<ZfsDataset>>(Storage::DATASET_QUERY, vec![])
            .await
    }

    pub async fn get_dataset_details(&self) -> ApiResult<Vec<DatasetDetailsResponse>> {
        self.client
            .call::<Vec<DatasetDetailsResponse>>(Storage::DATASET_DETAILS, vec![])
            .await
    }

    pub async fn destroy_dataset_snapshots(&self, args: DestroySnapshotsArgs) -> ApiResult<i32> {
        self.client
            .call::<i32>(Storage::DATASET_DESTROY_SNAPSHOTS, vec![json!(args)])
            .await
    }

    pub async fn delete_dataset(&self, dataset_name: &str, recursive: bool) -> ApiResult<bool> {
        let options = json!({ "recursive": recursive });
        self.client
            .call::<bool>(Storage::DATASET_DELETE, vec![json!(dataset_name), options])
            .await
    }

    pub async fn create_dataset(
        &self,
        name: &str,
        dataset_type: DatasetOptions,
    ) -> ApiResult<DatasetCreationResponse> {
        let preset = match dataset_type {
            DatasetOptions::Share => crate::models::storage::Presets::share_dataset(),
            DatasetOptions::Generic => crate::models::storage::Presets::generic_dataset(),
        };
        let mut payload = serde_json::to_value(preset).unwrap();
        payload["name"] = json!(name);
        self.client
            .call::<DatasetCreationResponse>(Storage::DATASET_CREATE, vec![payload])
            .await
    }
}