use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemMkdirArgs {
    pub path: String,
    pub options: MkdirOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MkdirOptions {
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default = "default_true")]
    pub raise_chmod_errors: bool,
}

fn default_mode() -> String { "755".to_string() }
fn default_true() -> bool { true }

impl Default for MkdirOptions {
    fn default() -> Self {
        Self {
            mode: default_mode(),
            raise_chmod_errors: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemMkdirResult {
    pub name: String,
    pub path: String,
    pub realpath: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub size: i32,
    pub allocation_size: i32,
    pub mode: i32,
    pub mount_id: i32,
    pub acl: bool,
    pub uid: i32,
    pub gid: i32,
    pub is_mountpoint: bool,
    pub is_ctldir: bool,
    pub attributes: Vec<String>,
    pub xattrs: Vec<String>,
    pub zfs_attrs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemStatArgs {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemStatResult {
    pub realpath: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub size: i64,
    pub allocation_size: i64,
    pub mode: i32,
    pub mount_id: i64,
    pub uid: i32,
    pub gid: i32,
    pub atime: f64,
    pub mtime: f64,
    pub ctime: f64,
    pub btime: f64,
    pub dev: i64,
    pub inode: i64,
    pub nlink: i32,
    pub acl: bool,
    pub is_mountpoint: bool,
    pub is_ctldir: bool,
    pub attributes: Vec<String>,
    pub user: String,
    pub group: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemStatfsArgs {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemStatfsResult {
    pub flags: Vec<String>,
    pub fsid: String,
    pub fstype: String,
    pub source: String,
    pub dest: String,
    pub blocksize: i64,
    pub total_blocks: i64,
    pub free_blocks: i64,
    pub avail_blocks: i64,
    pub total_blocks_str: String,
    pub free_blocks_str: String,
    pub avail_blocks_str: String,
    pub files: i64,
    pub free_files: i64,
    pub name_max: i32,
    pub total_bytes: i64,
    pub free_bytes: i64,
    pub avail_bytes: i64,
    pub total_bytes_str: String,
    pub free_bytes_str: String,
    pub avail_bytes_str: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestroySnapshotsArgs {
    pub name: String,
    pub snapshots: SnapshotOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotOptions {
    #[serde(default)]
    pub all: bool,
    #[serde(default)]
    pub recursive: bool,
    pub snapshots: Option<Vec<SnapshotRange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotRange {
    pub start: Option<String>,
    pub end: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetDetailsResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub name: String,
    pub pool: String,
    pub encrypted: bool,
    pub encryption_root: Option<String>,
    pub key_loaded: bool,
    pub children: Vec<DatasetDetailsResponse>,
    pub snapshot_count: i32,
    pub deduplication: ZfsSettingProperty,
    pub mountpoint: Option<String>,
    pub sync: ZfsSettingProperty,
    pub compression: ZfsSettingProperty,
    pub compressratio: ZfsSettingProperty,
    pub origin: ZfsSettingProperty,
    pub quota: ZfsSizeProperty,
    pub refquota: ZfsSizeProperty,
    pub reservation: ZfsSizeProperty,
    pub refreservation: ZfsSizeProperty,
    pub key_format: ZfsSettingProperty,
    pub encryption_algorithm: ZfsSettingProperty,
    pub used: ZfsSizeProperty,
    pub usedbychildren: ZfsSizeProperty,
    pub usedbydataset: ZfsSizeProperty,
    pub usedbysnapshots: ZfsSizeProperty,
    pub available: ZfsSizeProperty,
    pub user_properties: HashMap<String, String>,
    pub locked: bool,
    pub atime: bool,
    pub casesensitive: bool,
    pub readonly: bool,
    pub thick_provisioned: bool,
    pub nfs_shares: Vec<String>,
    pub smb_shares: Vec<String>,
    pub iscsi_shares: Vec<String>,
    pub vms: Vec<String>,
    pub apps: Vec<String>,
    pub virt_instances: Vec<String>,
    pub replication_tasks_count: i32,
    pub snapshot_tasks_count: i32,
    pub cloudsync_tasks_count: i32,
    pub rsync_tasks_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsSettingProperty {
    pub parsed: Option<String>,
    pub rawvalue: String,
    pub value: Option<String>,
    pub source: String,
    pub source_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsSizeProperty {
    pub parsed: Option<i64>,
    pub rawvalue: String,
    pub value: Option<String>,
    pub source: String,
    pub source_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetFetchSnapshotCountArgs {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsGenericProperty {
    pub parsed: Option<serde_json::Value>,
    pub rawvalue: String,
    pub value: Option<String>,
    pub source: String,
    pub source_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsCreationProperty {
    pub parsed: MongoDate,
    pub rawvalue: String,
    pub value: Option<String>,
    pub source: String,
    pub source_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoDate {
    #[serde(rename = "$date")]
    pub date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDataset {
    pub id: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub name: String,
    pub pool: String,
    pub encrypted: bool,
    pub encryption_root: Option<String>,
    pub key_loaded: bool,
    pub children: Vec<ZfsDataset>,
    pub comments: Option<ZfsGenericProperty>,
    pub deduplication: ZfsGenericProperty,
    pub mountpoint: String,
    pub aclmode: ZfsGenericProperty,
    pub acltype: ZfsGenericProperty,
    pub xattr: ZfsGenericProperty,
    pub atime: ZfsGenericProperty,
    pub casesensitivity: ZfsGenericProperty,
    pub checksum: ZfsGenericProperty,
    pub exec: ZfsGenericProperty,
    pub sync: ZfsGenericProperty,
    pub compression: ZfsGenericProperty,
    pub compressratio: ZfsGenericProperty,
    pub origin: ZfsGenericProperty,
    pub quota: ZfsGenericProperty,
    pub refquota: ZfsGenericProperty,
    pub reservation: ZfsGenericProperty,
    pub refreservation: ZfsGenericProperty,
    pub copies: ZfsGenericProperty,
    pub snapdir: ZfsGenericProperty,
    pub readonly: ZfsGenericProperty,
    pub recordsize: ZfsGenericProperty,
    pub key_format: ZfsGenericProperty,
    pub encryption_algorithm: ZfsGenericProperty,
    pub used: ZfsGenericProperty,
    pub usedbychildren: ZfsGenericProperty,
    pub usedbydataset: ZfsGenericProperty,
    pub usedbyrefreservation: ZfsGenericProperty,
    pub usedbysnapshots: ZfsGenericProperty,
    pub available: ZfsGenericProperty,
    pub special_small_block_size: ZfsGenericProperty,
    pub pbkdf2iters: ZfsGenericProperty,
    pub creation: ZfsCreationProperty,
    pub snapdev: ZfsGenericProperty,
    pub user_properties: HashMap<String, serde_json::Value>,
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolScrubQueryArgs {
    #[serde(default)]
    pub filters: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub options: PoolScrubQueryOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolScrubQueryOptions {
    #[serde(default = "default_true")]
    pub relationships: bool,
    #[serde(default)]
    pub count: bool,
    #[serde(default)]
    pub get: bool,
    #[serde(default)]
    pub force_sql_filters: bool,
    pub extend: Option<Vec<String>>,
    pub extend_context: Option<String>,
    pub prefix: Option<String>,
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub order_by: Vec<String>,
    #[serde(default)]
    pub select: Vec<String>,
    #[serde(default)]
    pub offset: i32,
    #[serde(default)]
    pub limit: i32,
}

impl Default for PoolScrubQueryOptions {
    fn default() -> Self {
        Self {
            relationships: true,
            count: false,
            get: false,
            force_sql_filters: false,
            extend: None,
            extend_context: None,
            prefix: None,
            extra: HashMap::new(),
            order_by: Vec::new(),
            select: Vec::new(),
            offset: 0,
            limit: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletionSchedule {
    pub minute: String,
    pub hour: String,
    pub dom: String,
    pub month: String,
    pub dow: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolScrubQueryResponse {
    pub pool: i64,
    pub threshold: i32,
    pub description: String,
    pub schedule: DeletionSchedule,
    pub enabled: bool,
    pub id: i64,
    pub pool_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolScrubQuerySingleArgs {
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunPoolScrubArgs {
    pub name: String,
    #[serde(default = "default_threshold")]
    pub threshold: i32,
}
fn default_threshold() -> i32 { 35 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeActionOnPoolScrubArgs {
    pub name: String,
    pub action: PoolScrubAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PoolScrubAction {
    Start,
    Stop,
    Pause,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePoolScrubArgs {
    #[serde(rename = "id_")]
    pub id_: i32,
    pub data: UpdatePoolScrubDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePoolScrubDetails {
    pub pool: i64,
    pub threshold: i32,
    pub description: String,
    pub schedule: DeletionSchedule,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePoolScrubArgs {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotTaskCreateArgs {
    pub dataset: String,
    #[serde(default)]
    pub recursive: bool,
    #[serde(default = "default_lifetime_value")]
    pub lifetime_value: i32,
    #[serde(default)]
    pub lifetime_unit: LifetimeUnits,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default = "default_naming_schema")]
    pub naming_schema: String,
    #[serde(default = "default_true")]
    pub allow_empty: bool,
    pub schedule: SnapshotSchedule,
}
fn default_lifetime_value() -> i32 { 2 }
fn default_naming_schema() -> String { "auto-%Y-%m-%d_%H-%M".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum LifetimeUnits {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl Default for LifetimeUnits {
    fn default() -> Self {
        LifetimeUnits::Week
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotSchedule {
    #[serde(default = "default_minute")]
    pub minute: String,
    #[serde(default = "default_hour")]
    pub hour: String,
    #[serde(default = "default_dom")]
    pub dom: String,
    #[serde(default = "default_month")]
    pub month: String,
    #[serde(default = "default_dow")]
    pub dow: String,
    #[serde(default = "default_begin")]
    pub begin: String,
    #[serde(default = "default_end")]
    pub end: String,
}
fn default_minute() -> String { "00".to_string() }
fn default_hour() -> String { "*".to_string() }
fn default_dom() -> String { "*".to_string() }
fn default_month() -> String { "*".to_string() }
fn default_dow() -> String { "*".to_string() }
fn default_begin() -> String { "00:00".to_string() }
fn default_end() -> String { "23:59".to_string() }

impl Default for SnapshotSchedule {
    fn default() -> Self {
        Self {
            minute: default_minute(),
            hour: default_hour(),
            dom: default_dom(),
            month: default_month(),
            dow: default_dow(),
            begin: default_begin(),
            end: default_end(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotCreationResponse {
    pub dataset: String,
    pub recursive: bool,
    pub lifetime_value: i32,
    pub lifetime_unit: LifetimeUnits,
    pub enabled: bool,
    pub exclude: Vec<String>,
    pub naming_schema: String,
    pub allow_empty: bool,
    pub schedule: SnapshotSchedule,
    pub id: i32,
    pub vmware_sync: bool,
    pub state: HashMap<Option<serde_json::Value>, Option<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSnapshotTaskArgs {
    pub id: i32,
    #[serde(default)]
    pub options: DeleteSnapshotTaskOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSnapshotTaskOptions {
    #[serde(default)]
    pub fixate_removal_date: bool,
}

impl Default for DeleteSnapshotTaskOptions {
    fn default() -> Self {
        Self { fixate_removal_date: false }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWillChangeRetentionForArgs {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSnapshotTaskInstanceArgs {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteSnapshotTaskArgs {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSnapshotTaskArgs {
    pub id: i32,
    pub data: UpdateSnapshotTaskDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSnapshotTaskDetails {
    pub dataset: String,
    #[serde(default)]
    pub recursive: bool,
    #[serde(default = "default_lifetime_value")]
    pub lifetime_value: i32,
    #[serde(default)]
    pub lifetime_unit: LifetimeUnits,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default = "default_naming_schema")]
    pub naming_schema: String,
    #[serde(default = "default_true")]
    pub allow_empty: bool,
    pub schedule: SnapshotSchedule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWillChangeRetentionForArgs {
    pub id: i32,
    pub data: UpdateSnapshotTaskDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DatasetType {
    Filesystem,
    Volume,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum AclType {
    Nfsv4,
    Posix,
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum AclMode {
    Passthrough,
    Discard,
    Restricted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum CaseSensitivity {
    Sensitive,
    Insensitive,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Atime {
    On,
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetCreatePayload {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: DatasetType,
    pub acltype: AclType,
    pub aclmode: AclMode,
    pub casesensitivity: Option<CaseSensitivity>,
    pub atime: Option<Atime>,
    pub comments: Option<String>,
    pub volume_size: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatasetOptions {
    Share,
    Generic,
}

pub struct Presets;

impl Presets {
    pub fn share_dataset() -> DatasetCreatePayload {
        DatasetCreatePayload {
            name: String::new(),
            typ: DatasetType::Filesystem,
            acltype: AclType::Nfsv4,
            aclmode: AclMode::Passthrough,
            casesensitivity: None,
            atime: None,
            comments: None,
            volume_size: None,
        }
    }

    pub fn generic_dataset() -> DatasetCreatePayload {
        DatasetCreatePayload {
            name: String::new(),
            typ: DatasetType::Filesystem,
            acltype: AclType::Posix,
            aclmode: AclMode::Discard,
            casesensitivity: None,
            atime: None,
            comments: None,
            volume_size: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsProperty {
    pub value: Option<String>,
    pub rawvalue: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetCreationResponse {
    pub id: String,
    pub name: String,
    pub pool: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub encrypted: bool,
    pub mountpoint: String,
    pub aclmode: ZfsProperty,
    pub acltype: ZfsProperty,
    pub casesensitivity: ZfsProperty,
    pub atime: ZfsProperty,
    pub compression: ZfsProperty,
    pub checksum: ZfsProperty,
    pub deduplication: ZfsProperty,
    pub used: ZfsProperty,
    pub available: ZfsProperty,
    pub quota: ZfsProperty,
    pub creation: CreationProperty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreationProperty {
    pub parsed: MongoDate,
    pub rawvalue: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetDeleteOptions {
    pub id: String,
}