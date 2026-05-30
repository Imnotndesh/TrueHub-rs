// apps.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppQueryResponse {
    pub name: String,
    pub id: String,
    pub state: String,
    #[serde(rename = "upgrade_available")]
    #[serde(default)]
    pub upgrade_available: bool,
    #[serde(rename = "latest_version")]
    pub latest_version: Option<String>,
    #[serde(rename = "image_updates_available")]
    #[serde(default)]
    pub image_updates_available: bool,
    #[serde(rename = "custom_app")]
    #[serde(default)]
    pub custom_app: bool,
    #[serde(default)]
    pub migrated: bool,
    #[serde(rename = "human_version")]
    pub human_version: Option<String>,
    pub version: Option<String>,
    pub metadata: Option<Metadata>,
    #[serde(rename = "active_workloads")]
    pub active_workloads: Option<ActiveWorkloads>,
    pub notes: Option<String>,
    pub portals: Option<HashMap<String, String>>,
    #[serde(rename = "migrated_from_kubernetes")]
    #[serde(default)]
    pub migrated_from_kubernetes: bool,
    #[serde(rename = "version_info")]
    pub version_info: Option<VersionInfo>,
    pub config: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub changelog: Option<String>,
    #[serde(rename = "upgrade_notes")]
    pub upgrade_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "app_version")]
    pub app_version: Option<String>,
    pub categories: Option<Vec<String>>,
    pub description: Option<String>,
    pub home: Option<String>,
    pub icon: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub title: Option<String>,
    pub train: Option<String>,
    pub screenshots: Option<Vec<String>>,
    pub sources: Option<Vec<String>>,
    pub maintainers: Option<Vec<Maintainer>>,
    pub capabilities: Option<Vec<Capability>>,
    #[serde(rename = "date_added")]
    pub date_added: Option<String>,
    #[serde(rename = "last_update")]
    pub last_update: Option<String>,
    #[serde(rename = "changelog_url")]
    pub changelog_url: Option<String>,
    #[serde(rename = "lib_version")]
    pub lib_version: Option<String>,
    #[serde(rename = "lib_version_hash")]
    pub lib_version_hash: Option<String>,
    #[serde(rename = "run_as_context")]
    pub run_as_context: Option<Vec<RunAsContext>>,
    #[serde(rename = "host_mounts")]
    pub host_mounts: Option<Vec<HostMount>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruenasDate {
    #[serde(rename = "$date")]
    pub date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAvailableItem {
    #[serde(rename = "app_readme")]
    pub app_readme: Option<String>,
    pub categories: Option<Vec<String>>,
    pub description: String,
    pub healthy: Option<bool>,
    #[serde(rename = "healthy_error")]
    pub healthy_error: Option<String>,
    pub home: String,
    pub location: String,
    #[serde(rename = "latest_version")]
    pub latest_version: Option<String>,
    #[serde(rename = "latest_app_version")]
    pub latest_app_version: Option<String>,
    #[serde(rename = "latest_human_version")]
    pub latest_human_version: Option<String>,
    #[serde(rename = "last_update")]
    pub last_update: Option<TruenasDate>,
    pub name: String,
    pub recommended: bool,
    pub title: String,
    pub maintainers: Vec<Maintainer>,
    pub tags: Option<Vec<String>>,
    pub screenshots: Option<Vec<String>>,
    pub sources: Option<Vec<String>>,
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    pub catalog: Option<String>,
    pub installed: bool,
    pub train: String,
}

impl AppAvailableItem {
    pub fn last_update_string(&self) -> String {
        self.last_update
            .as_ref()
            .and_then(|d| d.date)
            .map(|d| d.to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn tags_string(&self) -> String {
        self.tags
            .as_ref()
            .map(|t| t.join(", "))
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maintainer {
    pub name: String,
    pub email: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunAsContext {
    pub description: Option<String>,
    pub uid: Option<i32>,
    pub gid: Option<i32>,
    #[serde(rename = "user_name")]
    pub user_name: Option<String>,
    #[serde(rename = "group_name")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostMount {
    #[serde(rename = "host_path")]
    pub host_path: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveWorkloads {
    #[serde(default)]
    pub containers: i32,
    #[serde(rename = "used_ports")]
    pub used_ports: Option<Vec<UsedPort>>,
    #[serde(rename = "container_details")]
    pub container_details: Option<Vec<ContainerDetail>>,
    pub volumes: Option<Vec<Volume>>,
    pub images: Option<Vec<String>>,
    pub networks: Option<Vec<Network>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsedPort {
    #[serde(rename = "container_port")]
    pub container_port: i32,
    pub protocol: String,
    #[serde(rename = "host_ports")]
    pub host_ports: Vec<HostPort>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostPort {
    #[serde(rename = "host_port")]
    pub host_port: i32,
    #[serde(rename = "host_ip")]
    pub host_ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerDetail {
    pub id: String,
    #[serde(rename = "service_name")]
    pub service_name: String,
    pub image: String,
    #[serde(rename = "port_config")]
    pub port_config: Option<Vec<UsedPort>>,
    pub state: String,
    #[serde(rename = "volume_mounts")]
    pub volume_mounts: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub source: String,
    pub destination: String,
    pub mode: Option<String>,
    #[serde(rename = "type")]
    pub typ: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Created")]
    pub created: Option<String>,
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    pub enable_ipv6: Option<bool>,
    #[serde(rename = "IPAM")]
    pub ipam: Option<Ipam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipam {
    pub driver: Option<String>,
    pub options: Option<HashMap<String, String>>,
    pub config: Option<Vec<IpamConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamConfig {
    pub subnet: Option<String>,
    pub gateway: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeOptions {
    #[serde(default = "default_app_version")]
    pub app_version: String,
    #[serde(default)]
    pub values: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub snapshot_hostpaths: bool,
}

fn default_app_version() -> String {
    "latest".to_string()
}

impl Default for UpgradeOptions {
    fn default() -> Self {
        Self {
            app_version: "latest".to_string(),
            values: HashMap::new(),
            snapshot_hostpaths: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUpgradeSummaryResult {
    pub latest_version: String,
    pub latest_human_version: String,
    pub upgrade_version: String,
    pub upgrade_human_version: String,
    pub available_versions_for_upgrade: Vec<AvailableVersionForUpgrade>,
    pub changelog: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableVersionForUpgrade {
    pub version: String,
    pub human_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUpgradeRequest {
    #[serde(default = "default_app_version")]
    pub app_version: String,
}

impl Default for AppUpgradeRequest {
    fn default() -> Self {
        Self {
            app_version: "latest".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackOptions {
    #[serde(default = "default_app_version")]
    pub app_version: String,
    #[serde(default = "default_true")]
    pub rollback_snapshot: bool,
}

fn default_true() -> bool {
    true
}

impl Default for RollbackOptions {
    fn default() -> Self {
        Self {
            app_version: "latest".to_string(),
            rollback_snapshot: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAppOptions {
    #[serde(default = "default_true")]
    pub remove_images: bool,
    #[serde(default)]
    pub remove_ix_volumes: bool,
    #[serde(default)]
    pub force_remove_ix_volumes: bool,
    #[serde(default)]
    pub force_remove_custom_app: bool,
}

impl Default for DeleteAppOptions {
    fn default() -> Self {
        Self {
            remove_images: true,
            remove_ix_volumes: false,
            force_remove_ix_volumes: false,
            force_remove_custom_app: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppQueryExtra {
    #[serde(rename = "host_ip")]
    pub host_ip: Option<String>,
    #[serde(rename = "include_app_schema")]
    #[serde(default)]
    pub include_app_schema: bool,
    #[serde(rename = "retrieve_config")]
    #[serde(default)]
    pub retrieve_config: bool,
}

impl Default for AppQueryExtra {
    fn default() -> Self {
        Self {
            host_ip: None,
            include_app_schema: false,
            retrieve_config: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppQueryOptions {
    #[serde(default)]
    pub extra: AppQueryExtra,
    #[serde(default)]
    pub order_by: Vec<String>,
    #[serde(default)]
    pub select: Vec<String>,
    #[serde(default)]
    pub count: bool,
    #[serde(default)]
    pub get: bool,
    #[serde(default)]
    pub offset: i32,
    #[serde(default)]
    pub limit: i32,
    #[serde(default)]
    pub force_sql_filters: bool,
}

impl Default for AppQueryOptions {
    fn default() -> Self {
        Self {
            extra: AppQueryExtra::default(),
            order_by: Vec::new(),
            select: Vec::new(),
            count: false,
            get: false,
            offset: 0,
            limit: 0,
            force_sql_filters: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSimilarResponse {
    #[serde(rename = "app_readme")]
    pub app_readme: Option<String>,
    pub categories: Option<Vec<String>>,
    pub description: Option<String>,
    pub healthy: Option<bool>,
    #[serde(rename = "healthy_error")]
    pub healthy_error: Option<String>,
    pub home: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "latest_version")]
    pub latest_version: Option<String>,
    #[serde(rename = "latest_app_version")]
    pub latest_app_version: Option<String>,
    #[serde(rename = "human_version")]
    pub human_version: Option<String>,
    #[serde(rename = "last_update")]
    pub last_update: Option<String>,
    pub name: String,
    #[serde(default)]
    pub recommended: bool,
    pub title: Option<String>,
    pub maintainers: Option<Vec<Maintainer>>,
    pub tags: Option<Vec<String>>,
    pub screenshots: Option<Vec<String>>,
    pub sources: Option<Vec<String>>,
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    pub catalog: Option<String>,
    #[serde(default)]
    pub installed: bool,
    pub train: Option<String>,
    pub popularity: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogAppDetails {
    #[serde(rename = "app_readme")]
    pub app_readme: Option<String>,
    pub categories: Option<Vec<String>>,
    pub description: Option<String>,
    pub healthy: Option<bool>,
    #[serde(rename = "healthy_error")]
    pub healthy_error: Option<String>,
    pub home: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "latest_version")]
    pub latest_version: Option<String>,
    #[serde(rename = "latest_app_version")]
    pub latest_app_version: Option<String>,
    #[serde(rename = "latest_human_version")]
    pub latest_human_version: Option<String>,
    #[serde(rename = "last_update")]
    pub last_update: Option<TruenasDate>,
    pub name: String,
    #[serde(default)]
    pub recommended: bool,
    pub title: Option<String>,
    pub maintainers: Option<Vec<Maintainer>>,
    pub tags: Option<Vec<String>>,
    pub screenshots: Option<Vec<String>>,
    pub sources: Option<Vec<String>>,
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    pub capabilities: Option<Vec<Capability>>,
    #[serde(rename = "run_as_context")]
    pub run_as_context: Option<Vec<RunAsContext>>,
    pub versions: Option<HashMap<String, CatalogAppVersion>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogAppVersion {
    pub healthy: Option<bool>,
    pub supported: Option<bool>,
    #[serde(rename = "healthy_error")]
    pub healthy_error: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "last_update")]
    pub last_update: Option<String>,
    #[serde(rename = "human_version")]
    pub human_version: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "app_metadata")]
    pub app_metadata: Option<CatalogAppMetadata>,
    pub schema: Option<CatalogAppSchema>,
    pub readme: Option<String>,
    pub changelog: Option<String>,
    pub values: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogAppMetadata {
    #[serde(rename = "app_version")]
    pub app_version: Option<String>,
    pub categories: Option<Vec<String>>,
    pub description: Option<String>,
    pub home: Option<String>,
    pub icon: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub title: Option<String>,
    pub train: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub maintainers: Option<Vec<Maintainer>>,
    pub capabilities: Option<Vec<Capability>>,
    #[serde(rename = "run_as_context")]
    pub run_as_context: Option<Vec<RunAsContext>>,
    #[serde(rename = "date_added")]
    pub date_added: Option<String>,
    #[serde(rename = "changelog_url")]
    pub changelog_url: Option<String>,
    #[serde(rename = "lib_version")]
    pub lib_version: Option<String>,
    #[serde(rename = "lib_version_hash")]
    pub lib_version_hash: Option<String>,
    #[serde(rename = "host_mounts")]
    pub host_mounts: Option<Vec<HostMount>>,
    pub screenshots: Option<Vec<String>>,
    pub sources: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogAppSchema {
    pub groups: Option<Vec<SchemaGroup>>,
    pub questions: Option<Vec<SchemaQuestion>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaGroup {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaQuestion {
    pub variable: String,
    pub label: Option<String>,
    pub group: Option<String>,
    pub description: Option<String>,
    pub schema: Option<SchemaDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDefinition {
    #[serde(rename = "type")]
    pub typ: Option<String>,
    pub default: Option<serde_json::Value>,
    pub required: Option<bool>,
    pub hidden: Option<bool>,
    #[serde(rename = "null")]
    pub null: Option<bool>,
    pub min: Option<i32>,
    pub max: Option<i32>,
    #[serde(rename = "private")]
    pub private: Option<bool>,
    #[serde(rename = "show_if")]
    pub show_if: Option<Vec<Vec<Option<serde_json::Value>>>>,
    #[serde(rename = "$ref")]
    pub ref_: Option<Vec<String>>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<SchemaEnum>>,
    pub attrs: Option<Vec<SchemaQuestion>>,
    pub items: Option<Vec<SchemaQuestion>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaEnum {
    pub value: Option<serde_json::Value>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppInstanceState {
    Crashed,
    Deploying,
    Running,
    Stopped,
    Stopping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInstanceResponse {
    pub name: String,
    pub id: i32,
    pub state: AppInstanceState,
    pub upgrade_available: bool,
    pub latest_version: Option<String>,
    pub image_updates_available: bool,
    pub custom_app: bool,
    pub migrated: bool,
    pub human_version: String,
    pub version: String,
    #[serde(default)]
    pub metadata: HashMap<String, Option<serde_json::Value>>,
    #[serde(rename = "activeWorkloads")]
    pub active_workloads: Option<ActiveWorkloads>,
    pub notes: Option<String>,
    #[serde(default)]
    pub portals: HashMap<String, String>,
    #[serde(rename = "version_details")]
    pub version_details: Option<HashMap<String, serde_json::Value>>,
    pub config: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAppConfigOptions {
    #[serde(default)]
    pub values: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub custom_compose_config: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub custom_compose_config_string: String,
}

impl Default for UpdateAppConfigOptions {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
            custom_compose_config: HashMap::new(),
            custom_compose_config_string: String::new(),
        }
    }
}