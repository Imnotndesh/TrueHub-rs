use gtk4::prelude::*;
use gtk4::{
    Stack, Orientation, Align, ScrolledWindow,
    PolicyType, Box as GBox, StackTransitionType,
};
use libadwaita::prelude::*;
use libadwaita::{
    ApplicationWindow, HeaderBar, ToolbarView,
    ActionRow, ExpanderRow, PreferencesGroup,
    Banner, StatusPage, BreakpointCondition, Breakpoint,
    BreakpointConditionLengthType,
};
use glib::clone;
use std::sync::{Arc, Mutex};
use crate::state::AppState;
use crate::runtime;
use api::manager::TrueNasApiManager;
use api::models::system::{SystemInfo, Pool, DiskDetails};
use api::models::shares::{SmbShare, NfsShare};
use api::result::ApiResult;
use api::methods::{System as SystemMethods, Shares as ShareMethods};

#[derive(Clone)]
pub struct HomeData {
    pub system_info: SystemInfo,
    pub pools: Vec<Pool>,
    pub disks: Vec<DiskDetails>,
    pub smb_shares: Vec<SmbShare>,
    pub nfs_shares: Vec<NfsShare>,
}

pub enum HomeLoad {
    Success(HomeData),
    Error(String),
}

pub fn build(
    state: AppState,
    _root_stack: Stack,
    window: ApplicationWindow,
) -> ToolbarView {
    let toolbar_view = ToolbarView::new();

    let header = HeaderBar::new();
    let title_label = gtk4::Label::builder()
        .label("TrueHub")
        .css_classes(vec!["title"])
        .build();
    header.set_title_widget(Some(&title_label));

    let refresh_btn = gtk4::Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_text("Refresh")
        .build();
    header.pack_end(&refresh_btn);

    let user_btn = gtk4::Button::builder()
        .icon_name("person-symbolic")
        .tooltip_text(state.get_username().as_str())
        .build();
    header.pack_end(&user_btn);

    toolbar_view.add_top_bar(&header);

    let banner = Banner::new("");
    banner.set_revealed(false);
    toolbar_view.add_top_bar(&banner);

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .vexpand(true)
        .build();

    let content_box = GBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .margin_top(12)
        .margin_bottom(24)
        .margin_start(12)
        .margin_end(12)
        .build();

    let loading = GBox::builder()
        .orientation(Orientation::Vertical)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .vexpand(true)
        .build();
    let load_spinner = gtk4::Spinner::new();
    load_spinner.start();
    let load_label = gtk4::Label::builder()
        .label("Loading dashboard...")
        .css_classes(vec!["dim-label"])
        .build();
    loading.append(&load_spinner);
    loading.append(&load_label);

    let page_stack = Stack::new();
    page_stack.set_transition_duration(200);
    page_stack.add_named(&loading, Some("loading"));
    page_stack.add_named(&content_box, Some("content"));

    let error_page = StatusPage::builder()
        .icon_name("network-error-symbolic")
        .title("Failed to load")
        .build();
    page_stack.add_named(&error_page, Some("error"));
    page_stack.set_visible_child_name("loading");

    scroll.set_child(Some(&page_stack));
    toolbar_view.set_content(Some(&scroll));

    let data_store: Arc<Mutex<Option<HomeData>>> = Arc::new(Mutex::new(None));

    let nav_stack = _root_stack.clone();

    let load = {
        let state = state.clone();
        let content_box = content_box.clone();
        let page_stack = page_stack.clone();
        let banner = banner.clone();
        let error_page = error_page.clone();
        let data_store = data_store.clone();
        let title_label = title_label.clone();
        let nav_stack = nav_stack.clone();
        let window = window.clone();

        move || {
            page_stack.set_visible_child_name("loading");
            banner.set_revealed(false);

            let (tx, rx) = async_channel::unbounded::<HomeLoad>();

            glib::MainContext::default().spawn_local(clone!(
                #[strong] content_box,
                #[strong] page_stack,
                #[strong] banner,
                #[strong] error_page,
                #[strong] data_store,
                #[strong] title_label,
                #[strong] nav_stack,
                #[strong] window,
                async move {
                    if let Ok(result) = rx.recv().await {
                        match result {
                            HomeLoad::Success(data) => {
                                title_label.set_label(&data.system_info.hostname);

                                while let Some(child) = content_box.first_child() {
                                    content_box.remove(&child);
                                }

                                build_content(&content_box, &data, &nav_stack, &window);
                                *data_store.lock().unwrap() = Some(data);
                                page_stack.set_visible_child_name("content");
                            }
                            HomeLoad::Error(msg) => {
                                error_page.set_description(Some(&msg));
                                page_stack.set_visible_child_name("error");
                                banner.set_title(&msg);
                                banner.set_revealed(true);
                            }
                        }
                    }
                }
            ));

            let has_manager = state.manager.lock().unwrap().is_some();
            if has_manager {
                let client = {
                    let lock = state.manager.lock().unwrap();
                    lock.as_ref().map(|m| m.client.clone())
                };
                if let Some(client) = client {
                    runtime::spawn(fetch_home_data(client), tx);
                } else {
                    let _ = tx.send_blocking(HomeLoad::Error("Not connected".to_string()));
                }
            } else {
                let _ = tx.send_blocking(HomeLoad::Error("Not connected".to_string()));
            }
        }
    };

    load();

    refresh_btn.connect_clicked(move |_| {
        load();
    });

    toolbar_view
}

async fn fetch_home_data(client: std::sync::Arc<api::client::TrueNasClient>) -> HomeLoad {
    use api::models::system::{SystemInfo, Pool, DiskDetails};
    use api::models::shares::{SmbShare, NfsShare};
    use api::methods::{System as SystemMethods, Shares as ShareMethods};

    let system_info = match client.call::<SystemInfo>(SystemMethods::INFO, vec![]).await {
        ApiResult::Success(info) => info,
        ApiResult::Error { message } => return HomeLoad::Error(message),
        ApiResult::Loading => return HomeLoad::Error("Unexpected loading state".to_string()),
    };

    let pools = match client.call::<Vec<Pool>>(SystemMethods::GET_POOLS, vec![]).await {
        ApiResult::Success(p) => p,
        _ => vec![],
    };

    let disks = match client.call::<Vec<DiskDetails>>(SystemMethods::GET_DISKS, vec![]).await {
        ApiResult::Success(d) => d,
        _ => vec![],
    };

    let smb_shares = match client.call::<Vec<SmbShare>>(ShareMethods::GET_SMB_SHARES, vec![]).await {
        ApiResult::Success(s) => s,
        _ => vec![],
    };

    let nfs_shares = match client.call::<Vec<NfsShare>>(ShareMethods::GET_NFS_SHARES, vec![]).await {
        ApiResult::Success(s) => s,
        _ => vec![],
    };

    HomeLoad::Success(HomeData { system_info, pools, disks, smb_shares, nfs_shares })
}

fn build_content(container: &GBox, data: &HomeData, nav_stack: &Stack, window: &ApplicationWindow) {
    container.append(&system_overview_card(data));
    container.append(&make_spacer(12));
    container.append(&metrics_group(data));
    container.append(&make_spacer(12));
    container.append(&storage_stats_group(data, nav_stack, window));
    container.append(&make_spacer(12));
    container.append(&storage_group(data, nav_stack, window));
    container.append(&make_spacer(12));
    container.append(&shares_group(data, nav_stack, window));
}

fn system_overview_card(data: &HomeData) -> PreferencesGroup {
    let group = PreferencesGroup::new();

    let row = ActionRow::builder()
        .title(&data.system_info.hostname)
        .subtitle(&data.system_info.version)
        .build();
    row.set_icon_name(Some("computer-symbolic"));

    let badge = gtk4::Label::builder()
        .label("Online")
        .css_classes(vec!["success", "caption"])
        .valign(Align::Center)
        .build();
    row.add_suffix(&badge);

    let uptime_row = ActionRow::builder()
        .title("Uptime")
        .subtitle(&data.system_info.uptime)
        .build();
    uptime_row.set_icon_name(Some("preferences-system-time-symbolic"));

    let model_row = ActionRow::builder()
        .title("Model")
        .subtitle(&data.system_info.model)
        .build();
    model_row.set_icon_name(Some("computer-symbolic"));

    group.add(&row);
    group.add(&uptime_row);
    group.add(&model_row);
    group
}

fn metrics_group(data: &HomeData) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("System");
    group.set_description(Some("Hardware resources"));

    let cores = data.system_info.cores as i32;
    let physical = data.system_info.physical_cores.unwrap_or(cores);
    let cpu_row = ActionRow::builder()
        .title("CPU Cores")
        .subtitle(&format!("{cores} logical  /  {physical} physical"))
        .build();
    cpu_row.set_icon_name(Some("processor-symbolic"));
    group.add(&cpu_row);

    let mem_gb = data.system_info.physmem
        .map(|m| m as f64 / (1024.0 * 1024.0 * 1024.0))
        .unwrap_or(0.0);
    let ecc = if data.system_info.ecc_memory { " (ECC)" } else { "" };
    let mem_row = ActionRow::builder()
        .title("Memory")
        .subtitle(&format!("{:.1} GB{ecc}", mem_gb))
        .build();
    mem_row.set_icon_name(Some("memory-chip-symbolic"));
    group.add(&mem_row);

    if data.system_info.loadavg.len() >= 3 {
        let la = &data.system_info.loadavg;
        let load_row = ActionRow::builder()
            .title("Load Average")
            .subtitle(&format!("{:.2}  {:.2}  {:.2}  (1m / 5m / 15m)", la[0], la[1], la[2]))
            .build();
        load_row.set_icon_name(Some("speedometer-symbolic"));
        group.add(&load_row);
    }

    group
}

fn storage_stats_group(data: &HomeData, nav_stack: &Stack, window: &ApplicationWindow) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Storage Overview");

    let disk_count = data.disks.len();
    let disk_row = ActionRow::builder()
        .title("Disks")
        .subtitle(&format!("{disk_count} disk(s) detected"))
        .activatable(true)
        .build();
    disk_row.set_icon_name(Some("drive-harddisk-symbolic"));
    let arrow = gtk4::Image::from_icon_name("go-next-symbolic");
    disk_row.add_suffix(&arrow);

    let nav = nav_stack.clone();
    disk_row.connect_activated(move |_| {
        ensure_stub_page(&nav, "disks", "Drive Info", "drive-harddisk-symbolic",
                         "Disk details coming soon");
        nav.set_visible_child_name("disks");
    });

    group.add(&disk_row);
    group
}

fn storage_group(data: &HomeData, nav_stack: &Stack, _window: &ApplicationWindow) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Storage Pools");

    if data.pools.is_empty() {
        let empty = ActionRow::builder()
            .title("No pools configured")
            .build();
        empty.set_icon_name(Some("drive-harddisk-symbolic"));
        group.add(&empty);
        return group;
    }

    for pool in &data.pools {
        let used_pct = if pool.size > 0 {
            (pool.allocated as f64 / pool.size as f64 * 100.0) as i32
        } else { 0 };

        let status_icon = if pool.healthy { "emblem-ok-symbolic" } else { "dialog-error-symbolic" };

        let expander = ExpanderRow::builder()
            .title(&pool.name)
            .subtitle(&format!(
                "{}  •  {}% used",
                if pool.healthy { "Healthy" } else { "Degraded" },
                used_pct
            ))
            .build();
        expander.set_icon_name(Some(status_icon));

        let bar_box = GBox::builder()
            .orientation(Orientation::Vertical)
            .margin_top(4)
            .margin_bottom(8)
            .margin_start(12)
            .margin_end(12)
            .spacing(4)
            .build();

        let bar = gtk4::LevelBar::new();
        bar.set_min_value(0.0);
        bar.set_max_value(100.0);
        bar.set_value(used_pct as f64);
        bar.set_mode(gtk4::LevelBarMode::Continuous);

        let bar_label = gtk4::Label::builder()
            .label(&format!(
                "Used: {}  •  Free: {}  •  Total: {}",
                format_bytes(pool.allocated),
                format_bytes(pool.free),
                format_bytes(pool.size)
            ))
            .css_classes(vec!["caption", "dim-label"])
            .halign(Align::Start)
            .build();

        bar_box.append(&bar);
        bar_box.append(&bar_label);

        let bar_row = libadwaita::PreferencesRow::new();
        bar_row.set_child(Some(&bar_box));
        expander.add_row(&bar_row);

        if let Some(detail) = &pool.status_detail {
            if !detail.is_empty() {
                let detail_row = ActionRow::builder()
                    .title("Status")
                    .subtitle(detail)
                    .build();
                expander.add_row(&detail_row);
            }
        }

        let frag_row = ActionRow::builder()
            .title("Fragmentation")
            .subtitle(&format!("{}%", pool.fragmentation))
            .build();
        expander.add_row(&frag_row);

        let details_row = ActionRow::builder()
            .title("View Pool Details")
            .activatable(true)
            .build();
        details_row.add_suffix(&gtk4::Image::from_icon_name("go-next-symbolic"));

        let nav = nav_stack.clone();
        let pool_name = pool.name.clone();
        details_row.connect_activated(move |_| {
            ensure_stub_page(&nav, "pool-details", "Pool Details", "drive-harddisk-symbolic",
                             &format!("Pool details for '{}' coming soon", pool_name));
            nav.set_visible_child_name("pool-details");
        });
        expander.add_row(&details_row);

        group.add(&expander);
    }

    group
}

fn shares_group(data: &HomeData, nav_stack: &Stack, _window: &ApplicationWindow) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Shares");

    let smb_expander = ExpanderRow::builder()
        .title("SMB Shares")
        .subtitle(&format!("{} share(s)", data.smb_shares.len()))
        .build();
    smb_expander.set_icon_name(Some("folder-remote-symbolic"));

    if data.smb_shares.is_empty() {
        let none = ActionRow::builder().title("No SMB shares").build();
        smb_expander.add_row(&none);
    } else {
        for share in &data.smb_shares {
            let row = ActionRow::builder()
                .title(&share.name)
                .subtitle(&share.path)
                .activatable(true)
                .build();
            let status = gtk4::Label::builder()
                .label(if share.enabled { "Active" } else { "Off" })
                .css_classes(vec![
                    "caption",
                    if share.enabled { "success" } else { "error" }
                ])
                .valign(Align::Center)
                .build();
            row.add_suffix(&status);
            row.add_suffix(&gtk4::Image::from_icon_name("go-next-symbolic"));
            row.set_icon_name(Some("folder-symbolic"));

            let nav = nav_stack.clone();
            let share_name = share.name.clone();
            row.connect_activated(move |_| {
                ensure_stub_page(&nav, "share-info", "Share Info", "folder-remote-symbolic",
                                 &format!("Share details for '{}' coming soon", share_name));
                nav.set_visible_child_name("share-info");
            });
            smb_expander.add_row(&row);
        }
    }
    group.add(&smb_expander);

    let nfs_expander = ExpanderRow::builder()
        .title("NFS Shares")
        .subtitle(&format!("{} share(s)", data.nfs_shares.len()))
        .build();
    nfs_expander.set_icon_name(Some("folder-remote-symbolic"));

    if data.nfs_shares.is_empty() {
        let none = ActionRow::builder().title("No NFS shares").build();
        nfs_expander.add_row(&none);
    } else {
        for share in &data.nfs_shares {
            let name = share.path.split('/').last().unwrap_or(&share.path).to_string();
            let row = ActionRow::builder()
                .title(&name)
                .subtitle(&share.path)
                .activatable(true)
                .build();
            let status = gtk4::Label::builder()
                .label(if share.enabled { "Active" } else { "Off" })
                .css_classes(vec![
                    "caption",
                    if share.enabled { "success" } else { "error" }
                ])
                .valign(Align::Center)
                .build();
            row.add_suffix(&status);
            row.add_suffix(&gtk4::Image::from_icon_name("go-next-symbolic"));
            row.set_icon_name(Some("folder-symbolic"));

            let nav = nav_stack.clone();
            let share_path = share.path.clone();
            row.connect_activated(move |_| {
                ensure_stub_page(&nav, "share-info", "Share Info", "folder-remote-symbolic",
                                 &format!("NFS share details for '{}' coming soon", share_path));
                nav.set_visible_child_name("share-info");
            });
            nfs_expander.add_row(&row);
        }
    }
    group.add(&nfs_expander);

    group
}

fn ensure_stub_page(nav_stack: &Stack, name: &str, title: &str, icon: &str, description: &str) {
    if nav_stack.child_by_name(name).is_some() {
        return;
    }

    let stub_toolbar = ToolbarView::new();

    let header = HeaderBar::new();
    let title_label = gtk4::Label::builder()
        .label(title)
        .css_classes(vec!["title"])
        .build();
    header.set_title_widget(Some(&title_label));

    let back_btn = gtk4::Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text("Back")
        .build();
    header.pack_start(&back_btn);
    stub_toolbar.add_top_bar(&header);

    let status = StatusPage::builder()
        .icon_name(icon)
        .title(title)
        .description(description)
        .build();

    stub_toolbar.set_content(Some(&status));

    let nav = nav_stack.clone();
    back_btn.connect_clicked(move |_| {
        nav.set_visible_child_name("home");
    });

    nav_stack.add_named(&stub_toolbar, Some(name));
    nav_stack.set_transition_type(StackTransitionType::SlideLeftRight);
}

fn make_spacer(height: i32) -> gtk4::Box {
    let b = gtk4::Box::new(Orientation::Vertical, 0);
    b.set_height_request(height);
    b
}

fn format_bytes(bytes: i64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut i = 0;
    while size >= 1024.0 && i < units.len() - 1 {
        size /= 1024.0;
        i += 1;
    }
    format!("{:.1} {}", size, units[i])
}