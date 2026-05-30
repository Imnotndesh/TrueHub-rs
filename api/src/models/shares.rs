use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbShare {
    pub id: i32,
    pub purpose: Option<String>,
    pub path: String,
    pub path_suffix: Option<String>,
    pub home: Option<bool>,
    pub name: String,
    pub comment: Option<String>,
    pub ro: Option<bool>,
    pub browsable: bool,
    pub recyclebin: Option<bool>,
    pub guestok: Option<bool>,
    pub hostsallow: Option<Vec<String>>,
    pub hostsdeny: Option<Vec<String>>,
    pub auxsmbconf: Option<String>,
    pub aapl_name_mangling: Option<bool>,
    pub abe: Option<bool>,
    pub acl: Option<bool>,
    pub durablehandle: Option<bool>,
    pub streams: Option<bool>,
    pub timemachine: Option<bool>,
    pub timemachine_quota: Option<i32>,
    pub vuid: Option<String>,
    pub shadowcopy: Option<bool>,
    pub fsrvp: Option<bool>,
    pub enabled: bool,
    pub afp: Option<bool>,
    pub audit: SmbAudit,
    pub path_local: Option<String>,
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbAudit {
    pub enable: bool,
    pub watch_list: Vec<String>,
    pub ignore_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfsShare {
    pub id: i32,
    pub path: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub comment: String,
    #[serde(default)]
    pub networks: Vec<String>,
    #[serde(default)]
    pub hosts: Vec<String>,
    #[serde(default)]
    pub ro: bool,
    #[serde(default)]
    pub maproot_user: Option<String>,
    #[serde(default)]
    pub maproot_group: Option<String>,
    #[serde(default)]
    pub mapall_user: Option<String>,
    #[serde(default)]
    pub mapall_group: Option<String>,
    #[serde(default)]
    pub security: Vec<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub expose_snapshots: bool,
}