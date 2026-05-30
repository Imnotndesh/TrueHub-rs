use std::sync::Arc;
use serde_json::json;
use crate::client::TrueNasClient;
use crate::methods::System as SystemMethods;
use crate::models::system::{
    AlertCategoriesResponse, AlertResponse, DiskDetails,
    GraphResult, Job, Pool, ReportingGraphQuery,
    ReportingGraphRequest, ReportingGraphResponse, SystemInfo,
};
use crate::result::ApiResult;

pub struct SystemService {
    client: Arc<TrueNasClient>,
}

impl SystemService {
    pub fn new(client: Arc<TrueNasClient>) -> Self {
        Self { client }
    }

    pub async fn get_system_info(&self) -> ApiResult<SystemInfo> {
        self.client.call(SystemMethods::INFO, vec![]).await
    }

    pub async fn shutdown(&self, reason: &str) -> ApiResult<serde_json::Value> {
        self.client.call(SystemMethods::SHUTDOWN, vec![json!(reason)]).await
    }

    pub async fn get_pools(&self) -> ApiResult<Vec<Pool>> {
        self.client.call(SystemMethods::GET_POOLS, vec![]).await
    }

    pub async fn get_disks(&self) -> ApiResult<Vec<DiskDetails>> {
        self.client.call(SystemMethods::GET_DISKS, vec![]).await
    }

    pub async fn get_graphs(&self) -> ApiResult<Vec<GraphResult>> {
        self.client.call(SystemMethods::GET_GRAPHS, vec![]).await
    }

    pub async fn get_reporting_data(
        &self,
        graphs: Vec<ReportingGraphRequest>,
        query: Option<ReportingGraphQuery>,
    ) -> ApiResult<Vec<ReportingGraphResponse>> {
        self.client.call(
            SystemMethods::GET_GRAPH_DATA,
            vec![json!(graphs), json!(query)],
        ).await
    }

    pub async fn get_job(&self, job_id: i32) -> ApiResult<Job> {
        let filters = json!([[["id", "=", job_id]]]);
        let result: ApiResult<Vec<Job>> = self.client.call(
            SystemMethods::GET_JOBS,
            vec![filters],
        ).await;

        match result {
            ApiResult::Success(mut jobs) => {
                if let Some(job) = jobs.pop() {
                    ApiResult::Success(job)
                } else {
                    ApiResult::Error { message: format!("Job {} not found", job_id) }
                }
            }
            ApiResult::Error { message } => ApiResult::Error { message },
            ApiResult::Loading => ApiResult::Loading,
        }
    }

    pub async fn get_active_jobs(&self, state: &str) -> ApiResult<Vec<Job>> {
        let filters = json!([[["state", "=", state]]]);
        self.client.call(SystemMethods::GET_JOBS, vec![filters]).await
    }

    pub async fn dismiss_alert(&self, uuid: &str) -> ApiResult<serde_json::Value> {
        self.client.call(SystemMethods::DISMISS_ALERT, vec![json!(uuid)]).await
    }

    pub async fn list_alerts(&self) -> ApiResult<Vec<AlertResponse>> {
        self.client.call(SystemMethods::LIST_ALERTS, vec![]).await
    }

    pub async fn list_alert_categories(&self) -> ApiResult<Vec<AlertCategoriesResponse>> {
        self.client.call(SystemMethods::LIST_CATEGORIES, vec![]).await
    }

    pub async fn list_alert_policies(&self) -> ApiResult<Vec<String>> {
        self.client.call(SystemMethods::LIST_POLICIES, vec![]).await
    }

    pub async fn restore_alert(&self, uuid: &str) -> ApiResult<serde_json::Value> {
        self.client.call(SystemMethods::RESTORE_ALERT, vec![json!(uuid)]).await
    }
}