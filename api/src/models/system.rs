use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub version: String,
    pub buildtime: Option<HashMap<String, i64>>,
    pub hostname: String,
    pub physmem: Option<i64>,
    pub model: String,
    pub cores: f64,
    pub physical_cores: Option<i32>,
    pub loadavg: Vec<f64>,
    pub uptime: String,
    pub system_serial: Option<String>,
    pub system_product: Option<String>,
    pub system_product_version: Option<String>,
    pub license: Option<String>,
    pub boottime: Option<HashMap<String, i64>>,
    pub datetime: Option<HashMap<String, i64>>,
    pub timezone: String,
    pub system_manufacturer: Option<String>,
    pub ecc_memory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: i32,
    pub method: String,
    pub arguments: Vec<serde_json::Value>,
    pub logs_path: Option<String>,
    pub logs_excerpt: Option<String>,
    pub result_encoding_error: Option<String>,
    pub progress: Option<JobProgress>,
    pub result: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub exception: Option<String>,
    pub exc_info: Option<ExcInfo>,
    pub state: String,
    pub time_started: Option<HashMap<String, i64>>,
    pub time_finished: Option<serde_json::Value>,
    pub credentials: Option<JobCredentials>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobProgress {
    pub percent: i32,
    pub description: Option<String>,
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCredentials {
    #[serde(rename = "type")]
    pub credential_type: Option<String>,
    pub data: Option<JobCredentialsData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCredentialsData {
    pub username: Option<String>,
    pub login_at: Option<HashMap<String, i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExcInfo {
    pub repr: Option<String>,
    #[serde(rename = "type")]
    pub exc_type: Option<String>,
    pub errno: Option<f64>,
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphResult {
    pub name: String,
    pub title: String,
    pub vertical_label: String,
    pub identifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReportingGraphQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<ReportingUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingGraphRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl ReportingGraphRequest {
    pub fn new(name: ReportingGraphName, identifier: Option<String>) -> Self {
        Self { name: name.value().to_string(), identifier }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingGraphResponse {
    pub name: String,
    pub identifier: Option<String>,
    pub aggregations: Option<HashMap<String, HashMap<String, f64>>>,
    pub data: Vec<Vec<f64>>,
    pub start: i32,
    pub end: i32,
    pub legend: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportingUnit {
    Hour, Day, Week, Month, Year,
}

#[derive(Debug, Clone)]
pub enum ReportingGraphName {
    Cpu, CpuTemp, Disk, Interface, Load, Processes,
    Memory, Uptime, ArcActualRate, ArcRate, ArcSize,
    ArcResult, DiskTemp, UpsCharge, UpsRuntime, UpsVoltage,
    UpsCurrent, UpsFrequency, UpsLoad, UpsTemperature,
}

impl ReportingGraphName {
    pub fn value(&self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::CpuTemp => "cputemp",
            Self::Disk => "disk",
            Self::Interface => "interface",
            Self::Load => "load",
            Self::Processes => "processes",
            Self::Memory => "memory",
            Self::Uptime => "uptime",
            Self::ArcActualRate => "arcactualrate",
            Self::ArcRate => "arcrate",
            Self::ArcSize => "arcsize",
            Self::ArcResult => "arcresult",
            Self::DiskTemp => "disktemp",
            Self::UpsCharge => "upscharge",
            Self::UpsRuntime => "upsruntime",
            Self::UpsVoltage => "upsvoltage",
            Self::UpsCurrent => "upscurrent",
            Self::UpsFrequency => "upsfrequency",
            Self::UpsLoad => "upsload",
            Self::UpsTemperature => "upstemperature",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskDetails {
    pub identifier: String,
    pub name: String,
    pub subsystem: String,
    pub number: i32,
    pub serial: String,
    pub lunid: Option<String>,
    pub size: i64,
    pub description: String,
    pub transfermode: String,
    pub hddstandby: String,
    pub advpowermgmt: String,
    pub togglesmart: Option<bool>,
    pub smartoptions: Option<String>,
    pub expiretime: Option<String>,
    pub critical: Option<String>,
    pub difference: Option<String>,
    pub informational: Option<String>,
    pub model: Option<String>,
    pub rotationrate: Option<i32>,
    #[serde(rename = "type")]
    pub disk_type: String,
    pub zfs_guid: Option<String>,
    pub bus: String,
    pub devname: String,
    pub enclosure: Option<String>,
    pub supports_smart: Option<bool>,
    pub pool: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    pub id: i32,
    pub name: String,
    pub guid: String,
    pub path: String,
    pub status: String,
    pub scan: Option<PoolScan>,
    pub expand: Option<PoolExpand>,
    pub topology: PoolTopology,
    pub healthy: bool,
    pub warning: bool,
    pub status_code: String,
    pub status_detail: Option<String>,
    pub size: i64,
    pub allocated: i64,
    pub free: i64,
    pub freeing: i64,
    pub fragmentation: String,
    pub size_str: String,
    pub allocated_str: String,
    pub free_str: String,
    pub freeing_str: String,
    pub dedup_table_quota: String,
    pub dedup_table_size: i64,
    pub autotrim: AutoTrim,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolScan {
    pub function: Option<String>,
    pub state: Option<String>,
    pub start_time: Option<HashMap<String, i64>>,
    pub end_time: Option<HashMap<String, i64>>,
    pub percentage: Option<f64>,
    pub bytes_to_process: Option<i64>,
    pub bytes_processed: Option<i64>,
    pub bytes_issued: Option<i64>,
    pub pause: Option<String>,
    pub errors: Option<i32>,
    pub total_secs_left: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolExpand {
    pub state: Option<String>,
    pub expanding_vdev: Option<serde_json::Value>,
    pub start_time: Option<HashMap<String, i64>>,
    pub end_time: Option<HashMap<String, i64>>,
    pub bytes_to_reflow: Option<i64>,
    pub bytes_reflowed: Option<i64>,
    pub waiting_for_resilver: Option<bool>,
    pub total_secs_left: Option<i64>,
    pub percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolTopology {
    pub data: Vec<PoolDevice>,
    pub log: Vec<PoolDevice>,
    pub cache: Vec<PoolDevice>,
    pub spare: Vec<PoolDevice>,
    pub special: Vec<PoolDevice>,
    pub dedup: Vec<PoolDevice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDevice {
    pub name: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub path: String,
    pub guid: String,
    pub status: String,
    pub stats: Option<PoolStats>,
    pub children: Vec<PoolDevice>,
    pub device: Option<String>,
    pub disk: Option<String>,
    pub unavail_disk: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    pub timestamp: i64,
    pub read_errors: i32,
    pub write_errors: i32,
    pub checksum_errors: i32,
    pub ops: Vec<i64>,
    pub bytes: Vec<i64>,
    pub size: i64,
    pub allocated: i64,
    pub fragmentation: i32,
    pub self_healed: i32,
    pub configured_ashift: i32,
    pub logical_ashift: i32,
    pub physical_ashift: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoTrim {
    pub value: String,
    pub rawvalue: String,
    pub parsed: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertResponse {
    pub uuid: String,
    pub source: String,
    pub klass: String,
    pub args: Option<serde_json::Value>,
    pub node: String,
    pub key: String,
    pub datetime: MongoDate,
    pub last_occurrence: MongoDate,
    pub dismissed: bool,
    pub mail: Option<serde_json::Value>,
    pub text: String,
    pub id: String,
    pub level: String,
    pub formatted: Option<String>,
    pub one_shot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoDate {
    #[serde(rename = "$date")]
    pub date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCategoriesResponse {
    pub id: String,
    pub title: String,
    pub classes: Vec<AlertCategoriesClasses>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCategoriesClasses {
    pub id: String,
    pub title: String,
    pub level: String,
    pub proactive_support: bool,
}