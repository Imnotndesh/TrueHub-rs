use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CpuMode {
    Custom,
    #[serde(rename = "HOST-MODEL")]
    HostModel,
    #[serde(rename = "HOST-PASSTHROUGH")]
    HostPassthrough,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmDevice {
    pub id: i32,
    pub attributes: std::collections::HashMap<String, serde_json::Value>,
    pub vm: i32,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmStatus {
    pub state: String,
    pub pid: Option<i32>,
    pub domain_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmQueryResponse {
    pub command_line_args: String,
    pub cpu_mode: CpuMode,
    pub cpu_model: Option<String>,
    pub name: String,
    pub description: String,
    pub vcpus: i32,
    pub cores: i32,
    pub threads: i32,
    pub cpuset: Option<String>,
    pub nodeset: Option<String>,
    pub enable_cpu_topology_extension: bool,
    pub pin_vcpus: bool,
    pub suspend_on_snapshot: bool,
    pub trusted_platform_module: bool,
    pub memory: i32,
    pub min_memory: Option<i32>,
    pub hyperv_enlightenments: bool,
    pub bootloader: String,
    pub bootloader_ovmf: String,
    pub autostart: bool,
    pub hide_from_msr: bool,
    pub ensure_display_device: bool,
    pub time: String,
    pub shutdown_timeout: i32,
    pub arch_type: Option<String>,
    pub machine_type: Option<String>,
    pub uuid: Option<String>,
    pub devices: Option<Vec<VmDevice>>,
    pub display_available: bool,
    pub id: i32,
    pub status: VmStatus,
    pub enable_secure_boot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmDisplayUriQueryResponse {
    pub uri: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopOptions {
    #[serde(default)]
    pub force: bool,
    #[serde(default)]
    pub force_after_timeout: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartOptions {
    #[serde(default)]
    pub overcommit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteOptions {
    #[serde(default)]
    pub zvols: bool,
    #[serde(default)]
    pub force: bool,
}