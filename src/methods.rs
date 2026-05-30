pub struct Auth;
pub struct System;
pub struct Connection;
pub struct User;
pub struct Apps;
pub struct Virt;
pub struct Vm;
pub struct Shares;
pub struct Storage;

impl Auth {
    pub const LOGIN: &'static str = "auth.login";
    pub const API_LOGIN: &'static str = "auth.login_with_api_key";
    pub const TOKEN_LOGIN: &'static str = "auth.login_with_token";
    pub const LOGOUT: &'static str = "auth.logout";
    pub const ME: &'static str = "auth.me";
    pub const GEN_TOKEN: &'static str = "auth.generate_token";
    pub const GEN_ONETIME_PASSWORD: &'static str = "auth.generate_onetime_password";
    pub const MECHANISM_CHOICES: &'static str = "auth.mechanism_choices";
    pub const LOGIN_EX: &'static str = "auth.login_ex";
    pub const GET_SESSIONS: &'static str = "auth.sessions";
    pub const TERMINATE_OTHER_SESSION: &'static str = "auth.terminate_other_session";
    pub const TERMINATE_SESSION: &'static str = "auth.terminate_session";
}

impl System {
    pub const INFO: &'static str = "system.info";
    pub const GET_JOBS: &'static str = "core.get_jobs";
    pub const SHUTDOWN: &'static str = "system.shutdown";
    pub const GET_DISKS: &'static str = "disk.query";
    pub const GET_POOLS: &'static str = "pool.query";
    pub const GET_GRAPHS: &'static str = "reporting.graphs";
    pub const GET_GRAPH_DATA: &'static str = "reporting.get_data";
    /// Dismiss alert based on uuid (String)
    pub const DISMISS_ALERT: &'static str = "alert.dismiss";
    /// List all alerts from server
    pub const LIST_ALERTS: &'static str = "alert.list";
    /// List available alert categories
    pub const LIST_CATEGORIES: &'static str = "alert.list_categories";
    /// List all category policies
    pub const LIST_POLICIES: &'static str = "alert.list_policies";
    /// Restore a cleared alert based on uuid
    pub const RESTORE_ALERT: &'static str = "alert.restore";
}

impl Connection {
    pub const PING: &'static str = "core.ping";
}

impl User {
    pub const CHANGE_PASSWORD: &'static str = "user.set_password";
    pub const USER_UPDATE: &'static str = "user.update";
    pub const GET_USER_OBJ: &'static str = "user.get_obj";
}

impl Apps {
    pub const QUERY_APPS: &'static str = "app.query";
    pub const START_APP: &'static str = "app.start";
    pub const STOP_APP: &'static str = "app.stop";
    pub const UPGRADE_APP: &'static str = "app.upgrade";
    pub const UPDATE_APP_CONFIG: &'static str = "app.update";
    pub const GET_APP_CONFIG: &'static str = "app.config";
    pub const GET_UPGRADE_SUMMARY: &'static str = "app.upgrade_summary";
    pub const QUERY_MARKETPLACE_APPS: &'static str = "app.available";
    pub const GET_CATALOG_APP_DETAILS: &'static str = "catalog.get_app_details";
    pub const APP_CREATE: &'static str = "app.create";
    pub const CERTIFICATE_CHOICES: &'static str = "app.certificate_choices";
    pub const USED_APP_PORTS: &'static str = "app.used_ports";
    pub const APP_INSTANCE: &'static str = "app.get_instance";
    /// App rollback Method
    pub const ROLLBACK_APP: &'static str = "app.rollback";
    pub const APP_ROLLBACK_VERSIONS: &'static str = "app.rollback_versions";
    pub const DELETE_APP: &'static str = "app.delete";
    pub const SIMILAR_APPS: &'static str = "app.similar";
    pub const LATEST_APPS_TRAIN: &'static str = "latest";
    pub const STABLE_APPS_TRAIN: &'static str = "stable";
}

impl Virt {
    pub const GET_ALL_INSTANCES: &'static str = "virt.instance.query";
    pub const START_INSTANCE: &'static str = "virt.instance.start";
    pub const STOP_INSTANCE: &'static str = "virt.instance.stop";
    pub const RESTART_INSTANCE: &'static str = "virt.instance.restart";
    pub const DELETE_INSTANCE: &'static str = "virt.instance.delete";
    pub const UPDATE_INSTANCE: &'static str = "virt.instance.update";
    pub const DELETE_INSTANCE_DEVICE: &'static str = "virt.instance.device_delete";
    pub const GET_IMAGE_CHOICES: &'static str = "virt.instance.image_choice";
}

impl Vm {
    pub const GET_ALL_VM_INSTANCES: &'static str = "vm.query";
    pub const START_VM_INSTANCE: &'static str = "vm.start";
    pub const STOP_INSTANCE: &'static str = "vm.stop";
    pub const RESTART_INSTANCE: &'static str = "vm.restart";
    pub const DELETE_INSTANCE: &'static str = "vm.delete";
    pub const SUSPEND_VM: &'static str = "vm.suspend";
    pub const RESUME_VM: &'static str = "vm.resume";
    pub const POWER_OFF_VM: &'static str = "vm.poweroff";
    pub const CLONE_VM: &'static str = "vm.clone";
    pub const GET_VM_MEMORY_USAGE: &'static str = "vm.get_memory_usage";
    pub const GET_INSTANCE: &'static str = "vm.get_instance";
    pub const GET_VM_STATUS: &'static str = "vm.status";
    pub const GET_DISPLAY_URL: &'static str = "vm.get_display_web_uri";
}

impl Shares {
    pub const GET_NFS_SHARES: &'static str = "sharing.nfs.query";
    pub const GET_SMB_SHARES: &'static str = "sharing.smb.query";
}

impl Storage {
    /// Creates a new directory at the specified path.
    pub const FILESYSTEM_MKDIR: &'static str = "filesystem.mkdir";
    /// Retrieves filesystem information for a specific directory.
    pub const FILESYSTEM_STAT: &'static str = "filesystem.stat";
    /// Returns statistics of the filesystem for a given path.
    pub const FILESYSTEM_STATFS: &'static str = "filesystem.statfs";

    // Dataset operations
    pub const DATASET_CREATE: &'static str = "pool.dataset.create";
    /// Removes snapshots from a dataset.
    pub const DATASET_DESTROY_SNAPSHOTS: &'static str = "pool.dataset.destroy_snapshots";
    /// Fetches detailed information for a specific dataset.
    pub const DATASET_DETAILS: &'static str = "pool.dataset.details";
    /// Queries all datasets on the system.
    pub const DATASET_QUERY: &'static str = "pool.dataset.query";
    pub const DATASET_DELETE: &'static str = "pool.dataset.delete";

    // Pool scrub operations
    pub const POOL_SCRUB_QUERY: &'static str = "pool.scrub.query";
    pub const POOL_SCRUB_CREATE: &'static str = "pool.scrub.create";
    /// Retrieves a single pool scrub task instance.
    pub const POOL_SCRUB_GET_INSTANCE: &'static str = "pool.scrub.get_instance";
    /// Initiates a pool scrub if the threshold has been met. Returns a job ID.
    pub const POOL_SCRUB_RUN: &'static str = "pool.scrub.run";
    /// Performs an action (START, STOP, PAUSE) on a pool scrub job.
    pub const POOL_SCRUB_ACTION: &'static str = "pool.scrub.scrub";
    /// Updates an existing pool scrub task. Returns a job ID.
    pub const POOL_SCRUB_UPDATE: &'static str = "pool.scrub.update";
    /// Deletes a pool scrub task. Returns a job ID.
    pub const POOL_SCRUB_DELETE: &'static str = "pool.scrub.delete";

    // Snapshot task operations
    /// Creates a periodic snapshot task for a dataset.
    pub const SNAPSHOT_TASK_CREATE: &'static str = "pool.snapshottask.create";
    /// Deletes a periodic snapshot task.
    pub const SNAPSHOT_TASK_DELETE: &'static str = "pool.snapshottask.delete";
    /// Returns a list of snapshots which will change the retention if periodic snapshot task id is deleted.
    pub const SNAPSHOT_TASK_DELETE_WILL_CHANGE_RETENTION: &'static str = "pool.snapshottask.delete_will_change_retention_for";
    /// Fetch an instance of a periodic snapshot task.
    pub const SNAPSHOT_TASK_GET_INSTANCE: &'static str = "pool.snapshottask.get_instance";
    /// Query all snapshot tasks and return a list of SnapshotCreationResponse.
    pub const SNAPSHOT_TASK_QUERY: &'static str = "pool.snapshottask.query";
    /// Execute a periodic snapshot task of `id`.
    pub const SNAPSHOT_TASK_RUN: &'static str = "pool.snapshottask.run";
    /// Updates a periodic snapshot task.
    pub const SNAPSHOT_TASK_UPDATE: &'static str = "pool.snapshottask.update";
    /// Returns a list of snapshots which will change the retention if periodic snapshot task `id` is updated with `data`.
    pub const SNAPSHOT_TASK_UPDATE_WILL_CHANGE_RETENTION: &'static str = "pool.snapshottask.update_will_change_retention_for";
}