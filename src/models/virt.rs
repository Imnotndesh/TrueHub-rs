use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VirtType {
    Container,
    Vm,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VirtStatus {
    Running,
    Stopped,
    Starting,
    Stopping,
    Unknown,
    Error,
    Frozen,
    Freezing,
    Thawed,
    Aborting,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AliasType {
    Inet,
    Inet6,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RootDiskIoBus {
    Nvme,
    #[serde(rename = "VIRTIO-BLK")]
    VirtioBlk,
    #[serde(rename = "VIRTIO-SCSI")]
    VirtioScsi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aliases {
    #[serde(rename = "type")]
    pub typ: AliasType,
    pub address: String,
    pub netmask: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub architecture: Option<String>,
    pub description: Option<String>,
    pub os: Option<String>,
    pub release: Option<String>,
    pub serial: Option<String>,
    #[serde(rename = "type")]
    pub typ: Option<String>,
    pub variant: Option<String>,
    pub secureboot: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uid {
    pub hostid: i32,
    pub maprange: i32,
    pub nsid: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gid {
    pub hostid: i32,
    pub maprange: i32,
    pub nsid: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernsIdmap {
    pub uid: Uid,
    pub gid: Gid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerResponse {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub typ: VirtType,
    pub status: VirtStatus,
    pub cpu: Option<String>,
    pub memory: Option<i32>,
    pub autostart: bool,
    pub environment: std::collections::HashMap<String, String>,
    pub aliases: Vec<Aliases>,
    pub image: Image,
    pub userns_idmap: Option<UsernsIdmap>,
    pub raw: Option<std::collections::HashMap<serde_json::Value, serde_json::Value>>,
    pub vnc_enabled: bool,
    pub vnc_port: Option<i32>,
    pub vnc_password: Option<String>,
    pub secure_boot: Option<bool>,
    pub root_disk_size: Option<i32>,
    pub root_disk_io_bus: Option<RootDiskIoBus>,
    pub storage_pool: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContainerUpdate {
    pub environment: Option<std::collections::HashMap<String, String>>,
    pub autostart: Option<bool>,
    pub cpu: Option<String>,
    pub memory: Option<i32>,
    pub vnc_port: Option<i32>,
    pub vnc_enabled: Option<bool>,
    pub vnc_password: Option<String>,
    pub secure_boot: Option<bool>,
    pub root_disk_size: Option<i32>,
    pub root_disk_io_bus: Option<RootDiskIoBus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstanceType {
    Container,
    Vm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageQueryResponse {
    pub label: String,
    pub os: String,
    pub release: String,
    pub archs: Vec<String>,
    pub variant: String,
    #[serde(rename = "instance_types")]
    pub instance_types: Vec<InstanceType>,
    pub secureboot: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "dev_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Device {
    Disk(DiskDevice),
    Gpu(GpuDevice),
    Proxy(ProxyDevice),
    Tpm(TPMDevice),
    Usb(USBDevice),
    Nic(NICDevice),
    Pci(PCIDevice),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskDevice {
    pub name: Option<String>,
    pub description: Option<String>,
    pub readonly: Option<bool>,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub boot_priority: Option<i32>,
    pub io_bus: Option<RootDiskIoBus>,
    pub storage_pool: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDevice {
    pub name: Option<String>,
    pub description: Option<String>,
    pub readonly: Option<bool>,
    #[serde(rename = "gpu_type")]
    pub gpu_type: String,
    pub id: Option<String>,
    pub gid: Option<i32>,
    pub uid: Option<i32>,
    pub mode: Option<String>,
    pub mdev: Option<String>,
    pub mig_uuid: Option<String>,
    pub pci: Option<String>,
    pub productid: Option<String>,
    pub vendorid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyDevice {
    pub name: Option<String>,
    pub description: Option<String>,
    pub readonly: Option<bool>,
    pub source_proto: String,
    pub source_port: i32,
    pub dest_proto: String,
    pub dest_port: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TPMDevice {
    pub name: Option<String>,
    pub description: Option<String>,
    pub readonly: Option<bool>,
    pub path: String,
    pub pathrm: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USBDevice {
    pub name: Option<String>,
    pub description: Option<String>,
    pub readonly: Option<bool>,
    pub bus: Option<i32>,
    pub dev: Option<i32>,
    pub product_id: Option<String>,
    pub vendor_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NICDevice {
    pub name: Option<String>,
    pub description: Option<String>,
    pub readonly: Option<bool>,
    pub network: Option<String>,
    pub nic_type: Option<String>,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PCIDevice {
    pub name: Option<String>,
    pub description: Option<String>,
    pub readonly: Option<bool>,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopArgs {
    pub timeout: Option<i32>,
    pub force: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageChoice {
    pub label: String,
    pub os: String,
    pub release: String,
    pub archs: Vec<String>,
    pub variant: String,
    pub instance_types: Vec<String>,
    pub secureboot: Option<bool>,
}