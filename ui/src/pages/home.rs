
use gtk4::prelude::*;
use gtk4::{
    Stack, Orientation, Align, ScrolledWindow,
    PolicyType, Box as GBox, StackTransitionType,
};
use libadwaita::prelude::*;
use libadwaita::{
    ApplicationWindow, HeaderBar, ToolbarView,
    ActionRow, ExpanderRow, PreferencesGroup,
    Banner, StatusPage,
};
use glib::clone;
use std::sync::{Arc, Mutex};
use gtk4::accessible::State;
use crate::state::AppState;
use crate::runtime;
use api::client::TrueNasClient;
use api::models::system::{SystemInfo, Pool, DiskDetails};
use api::models::shares::{SmbShare, NfsShare};
use api::result::ApiResult;
use api::methods::{System as SystemMethods, Shares as ShareMethods};
use crate::pages::{disk_info, performance, pool_details, share_info};
use crate::pages::performance::MetricType;
use crate::pages::share_info::ShareType;

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

    let loading_box = GBox::builder()
        .orientation(Orientation::Vertical)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(16)
        .vexpand(true)
        .build();

    let spinner = gtk4::Spinner::new();
    spinner.start();
    spinner.set_size_request(32, 32);

    let load_label = gtk4::Label::builder()
        .label("Loading dashboard…")
        .css_classes(vec!["dim-label"])
        .build();

    loading_box.append(&spinner);
    loading_box.append(&load_label);

    let error_page = StatusPage::builder()
        .icon_name("network-error-symbolic")
        .title("Connection Failed")
        .description("Could not reach the TrueNAS server.")
        .build();

    let page_stack = Stack::new();
    page_stack.set_transition_type(StackTransitionType::Crossfade);
    page_stack.set_transition_duration(250);
    page_stack.add_named(&loading_box, Some("loading"));
    page_stack.add_named(&content_box, Some("content"));
    page_stack.add_named(&error_page, Some("error"));
    page_stack.set_visible_child_name("loading");

    scroll.set_child(Some(&page_stack));
    toolbar_view.set_content(Some(&scroll));

    let data_store: Arc<Mutex<Option<HomeData>>> = Arc::new(Mutex::new(None));

    let load = {
        let state = state.clone();
        let content_box = content_box.clone();
        let page_stack = page_stack.clone();
        let banner = banner.clone();
        let error_page = error_page.clone();
        let data_store = data_store.clone();
        let title_label = title_label.clone();
        let nav_stack = _root_stack.clone();
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
                #[strong] state,
                async move {
                    if let Ok(result) = rx.recv().await {
                        match result {
                            HomeLoad::Success(data) => {
                                title_label.set_label(&data.system_info.hostname);
                                while let Some(child) = content_box.first_child() {
                                    content_box.remove(&child);
                                }
                                build_content(&content_box, &data, &nav_stack, &window, &state);
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

            let client = {
                let lock = state.manager.lock().unwrap();
                lock.as_ref().map(|m| m.client.clone())
            };

            match client {
                Some(c) => runtime::spawn(fetch_home_data(c), tx),
                None => {
                    let _ = tx.send_blocking(HomeLoad::Error("Not connected to any server.".to_string()));
                }
            }
        }
    };

    load();

    refresh_btn.connect_clicked(move |_| {
        load();
    });

    toolbar_view
}

async fn fetch_home_data(client: std::sync::Arc<TrueNasClient>) -> HomeLoad {
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

fn build_content(
    container: &GBox,
    data: &HomeData,
    nav_stack: &Stack,
    window: &ApplicationWindow,
    state: &AppState,
) {
    let state_clone = state.clone();
    container.append(&server_identity_group(data));
    container.append(&spacer(12));
    container.append(&hardware_group(data));
    container.append(&spacer(12));
    container.append(&performance_group(nav_stack, state));
    container.append(&spacer(12));
    container.append(&storage_pools_group(data, nav_stack,&state_clone));
    container.append(&spacer(12));
    container.append(&disks_group(data, nav_stack));
    container.append(&spacer(12));
    container.append(&shares_group(data, nav_stack));
}

fn server_identity_group(data: &HomeData) -> PreferencesGroup {
    let group = PreferencesGroup::new();

    let host_row = ActionRow::builder()
        .title(&data.system_info.hostname)
        .subtitle(&data.system_info.version)
        .build();
    host_row.set_icon_name(Some("network-server-symbolic"));

    let online_badge = pill_label("Online", &["success", "caption"]);
    host_row.add_suffix(&online_badge);
    group.add(&host_row);

    let uptime_row = ActionRow::builder()
        .title("Uptime")
        .subtitle(&data.system_info.uptime)
        .build();
    uptime_row.set_icon_name(Some("preferences-system-time-symbolic"));
    group.add(&uptime_row);

    if let Some(product) = &data.system_info.system_product {
        if !product.is_empty() {
            let model_row = ActionRow::builder()
                .title("Hardware")
                .subtitle(product)
                .build();
            model_row.set_icon_name(Some("computer-symbolic"));
            group.add(&model_row);
        }
    }

    let tz_row = ActionRow::builder()
        .title("Timezone")
        .subtitle(&data.system_info.timezone)
        .build();
    tz_row.set_icon_name(Some("globe-symbolic"));
    group.add(&tz_row);

    group
}
fn performance_group(nav_stack: &Stack, state: &AppState) -> PreferencesGroup {
    let group = PreferencesGroup::new();

    let row = ActionRow::builder()
        .title("Performance Monitor")
        .subtitle("CPU, Memory, Temperature")
        .activatable(true)
        .build();
    row.set_icon_name(Some("utilities-system-monitor-symbolic"));
    row.add_suffix(&gtk4::Image::from_icon_name("go-next-symbolic"));

    let nav = nav_stack.clone();
    let state = state.clone();
    row.connect_activated(move |_| {
        if let Some(old) = nav.child_by_name("performance") {
            nav.remove(&old);
        }
        let page = performance::build(state.clone(), MetricType::Cpu, nav.clone());
        nav.add_named(&page, Some("performance"));
        nav.set_transition_type(StackTransitionType::SlideLeft);
        nav.set_visible_child_name("performance");
    });

    group.add(&row);
    group
}
fn hardware_group(data: &HomeData) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Hardware");

    let cores_total = data.system_info.cores as i32;
    let cores_phys = data.system_info.physical_cores.unwrap_or(cores_total);

    let cpu_row = ActionRow::builder()
        .title("CPU")
        .subtitle(&format!("{cores_total} threads  /  {cores_phys} cores"))
        .build();
    cpu_row.set_icon_name(Some("processor-symbolic"));

    let core_label = right_label(&format!("{cores_phys}C / {cores_total}T"), &["caption", "dim-label"]);
    cpu_row.add_suffix(&core_label);
    group.add(&cpu_row);

    if let Some(mem_bytes) = data.system_info.physmem {
        let mem_gb = mem_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        let ecc_tag = if data.system_info.ecc_memory { "ECC" } else { "Non-ECC" };

        let mem_row = ActionRow::builder()
            .title("Memory")
            .subtitle(&format!("{:.1} GB  ·  {}", mem_gb, ecc_tag))
            .build();
        mem_row.set_icon_name(Some("memory-chip-symbolic"));

        let ecc_badge = if data.system_info.ecc_memory {
            pill_label("ECC", &["success", "caption"])
        } else {
            pill_label("Non-ECC", &["caption", "dim-label"])
        };
        mem_row.add_suffix(&ecc_badge);
        group.add(&mem_row);
    }

    if data.system_info.loadavg.len() >= 3 {
        let la = &data.system_info.loadavg;
        let load_row = ActionRow::builder()
            .title("Load Average")
            .subtitle("1 min  /  5 min  /  15 min")
            .build();
        load_row.set_icon_name(Some("speedometer-symbolic"));

        let load_label = right_label(
            &format!("{:.2}  {:.2}  {:.2}", la[0], la[1], la[2]),
            &["caption"],
        );
        load_row.add_suffix(&load_label);
        group.add(&load_row);
    }

    group
}

fn storage_pools_group(data: &HomeData, nav_stack: &Stack, state: &AppState) -> PreferencesGroup{
    let group = PreferencesGroup::new();
    group.set_title("Storage Pools");
    group.set_description(Some(&format!("{} pool(s) configured", data.pools.len())));

    if data.pools.is_empty() {
        let empty = ActionRow::builder()
            .title("No storage pools configured")
            .build();
        empty.set_icon_name(Some("drive-harddisk-symbolic"));
        group.add(&empty);
        return group;
    }

    for pool in &data.pools {
        let used_pct = if pool.size > 0 {
            pool.allocated as f64 / pool.size as f64
        } else {
            0.0
        };
        let used_pct_int = (used_pct * 100.0) as i32;

        let health_icon = match (pool.healthy, pool.warning) {
            (true, false) => "emblem-ok-symbolic",
            (true, true) => "dialog-warning-symbolic",
            (false, _) => "dialog-error-symbolic",
        };

        let health_text = match (pool.healthy, pool.warning) {
            (true, false) => "Healthy",
            (true, true) => "Warning",
            (false, _) => "Degraded",
        };

        let expander = ExpanderRow::builder()
            .title(&pool.name)
            .subtitle(&format!("{health_text}  ·  {used_pct_int}% used  ·  {}", pool.status_code))
            .build();
        expander.set_icon_name(Some(health_icon));

        let health_badge_class = match (pool.healthy, pool.warning) {
            (true, false) => "success",
            (true, true) => "warning",
            (false, _) => "error",
        };
        let badge = pill_label(health_text, &[health_badge_class, "caption"]);
        expander.add_suffix(&badge);

        let bar_row = storage_bar_row(pool.allocated, pool.free, pool.size, used_pct);
        expander.add_row(&bar_row);

        let alloc_row = ActionRow::builder()
            .title("Allocated")
            .build();
        alloc_row.add_suffix(&right_label(&format_bytes(pool.allocated), &["caption"]));
        expander.add_row(&alloc_row);

        let free_row = ActionRow::builder()
            .title("Free")
            .build();
        free_row.add_suffix(&right_label(&format_bytes(pool.free), &["caption", "success"]));
        expander.add_row(&free_row);

        let total_row = ActionRow::builder()
            .title("Total")
            .build();
        total_row.add_suffix(&right_label(&format_bytes(pool.size), &["caption"]));
        expander.add_row(&total_row);

        let frag_row = ActionRow::builder()
            .title("Fragmentation")
            .build();
        frag_row.add_suffix(&right_label(&format!("{}%", pool.fragmentation), &["caption"]));
        expander.add_row(&frag_row);

        let trim_row = ActionRow::builder()
            .title("Auto Trim")
            .build();
        trim_row.add_suffix(&right_label(&pool.autotrim.parsed, &["caption"]));
        expander.add_row(&trim_row);

        if let Some(detail) = &pool.status_detail {
            if !detail.is_empty() {
                let detail_row = ActionRow::builder()
                    .title("Status Detail")
                    .subtitle(detail)
                    .build();
                expander.add_row(&detail_row);
            }
        }

        let details_btn = ActionRow::builder()
            .title("View Pool Details")
            .activatable(true)
            .build();
        details_btn.add_suffix(&gtk4::Image::from_icon_name("go-next-symbolic"));
        details_btn.set_icon_name(Some("drive-harddisk-symbolic"));

        let nav = nav_stack.clone();
        let pool_clone = pool.clone();
        let state_clone = state.clone();
        details_btn.connect_activated(move |_| {
            let page = pool_details::build(state_clone.clone(), pool_clone.clone(), nav.clone());
            if nav.child_by_name("pool-details").is_none() {
                nav.add_named(&page, Some("pool-details"));
            }
            nav.set_transition_type(StackTransitionType::SlideLeft);
            nav.set_visible_child_name("pool-details");
        });
        expander.add_row(&details_btn);

        group.add(&expander);
    }

    group
}

fn storage_bar_row(allocated: i64, free: i64, total: i64, used_fraction: f64) -> libadwaita::PreferencesRow {
    let wrapper = GBox::builder()
        .orientation(Orientation::Vertical)
        .margin_top(8)
        .margin_bottom(10)
        .margin_start(16)
        .margin_end(16)
        .spacing(6)
        .build();

    let bar = gtk4::LevelBar::new();
    bar.set_min_value(0.0);
    bar.set_max_value(1.0);
    bar.set_value(used_fraction.clamp(0.0, 1.0));
    bar.set_mode(gtk4::LevelBarMode::Continuous);
    bar.add_css_class("storage-bar");

    let stats_row = GBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(0)
        .build();

    let used_lbl = gtk4::Label::builder()
        .label(&format!("Used: {}", format_bytes(allocated)))
        .css_classes(vec!["caption", "dim-label"])
        .halign(Align::Start)
        .hexpand(true)
        .build();

    let free_lbl = gtk4::Label::builder()
        .label(&format!("Free: {}", format_bytes(free)))
        .css_classes(vec!["caption", "success"])
        .halign(Align::End)
        .build();

    stats_row.append(&used_lbl);
    stats_row.append(&free_lbl);

    wrapper.append(&bar);
    wrapper.append(&stats_row);

    let row = libadwaita::PreferencesRow::new();
    row.set_child(Some(&wrapper));
    row
}

fn disks_group(data: &HomeData, nav_stack: &Stack) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Disks");
    group.set_description(Some(&format!("{} disk(s) detected", data.disks.len())));

    if data.disks.is_empty() {
        let empty = ActionRow::builder().title("No disks found").build();
        empty.set_icon_name(Some("drive-harddisk-symbolic"));
        group.add(&empty);
        return group;
    }

    let expander = ExpanderRow::builder()
        .title("All Disks")
        .subtitle(&format!("{} disk(s)", data.disks.len()))
        .build();
    expander.set_icon_name(Some("drive-harddisk-symbolic"));

    let count_badge = pill_label(&data.disks.len().to_string(), &["caption"]);
    expander.add_suffix(&count_badge);

    for disk in &data.disks {
        let size_str = format_bytes(disk.size);
        let model = disk.model.as_deref().unwrap_or("Unknown");
        let pool_hint = disk.pool.as_deref().unwrap_or("—");

        let row = ActionRow::builder()
            .title(&disk.name)
            .subtitle(&format!("{model}  ·  Pool: {pool_hint}"))
            .activatable(true)
            .build();
        row.set_icon_name(Some("drive-harddisk-symbolic"));

        let type_badge = pill_label(&disk.disk_type, &["caption", "dim-label"]);
        row.add_suffix(&type_badge);

        let size_badge = pill_label(&size_str, &["caption"]);
        row.add_suffix(&size_badge);
        row.add_suffix(&gtk4::Image::from_icon_name("go-next-symbolic"));

        let nav = nav_stack.clone();
        let dname = disk.name.clone();
        let disk_data = data.disks.clone();

        row.connect_activated(move |_| {
            let page = disk_info::build(disk_data.clone(), nav.clone());
            if nav.child_by_name("disk-info").is_none() {
                nav.add_named(&page, Some("disk-info"));
            }
            nav.set_visible_child_name("disk-info");
        });

        expander.add_row(&row);
    }

    group.add(&expander);
    group
}

fn shares_group(data: &HomeData, nav_stack: &Stack) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Network Shares");
    group.set_description(Some(&format!(
        "{} SMB  ·  {} NFS",
        data.smb_shares.len(), data.nfs_shares.len()
    )));

    let smb_expander = ExpanderRow::builder()
        .title("SMB Shares")
        .subtitle(&format!("{} share(s)", data.smb_shares.len()))
        .build();
    smb_expander.set_icon_name(Some("folder-remote-symbolic"));
    smb_expander.add_suffix(&pill_label(&data.smb_shares.len().to_string(), &["caption"]));

    if data.smb_shares.is_empty() {
        smb_expander.add_row(&empty_row("No SMB shares configured"));
    } else {
        for share in &data.smb_shares {
            let row = ActionRow::builder()
                .title(&share.name)
                .subtitle(&share.path)
                .activatable(true)
                .build();
            row.set_icon_name(Some("folder-symbolic"));

            let (label_text, label_classes): (&str, &[&str]) = if share.enabled {
                ("Active", &["caption", "success"])
            } else {
                ("Off", &["caption", "error"])
            };
            row.add_suffix(&pill_label(label_text, label_classes));
            row.add_suffix(&gtk4::Image::from_icon_name("go-next-symbolic"));

            let nav = nav_stack.clone();

            let sname = share.name.clone();
            let share_clone = share.clone();
            row.connect_activated(move |_| {
                let page = share_info::build(ShareType::Smb(share_clone.clone()), nav.clone());
                if nav.child_by_name("share-smb").is_none() {
                    nav.add_named(&page, Some("share-smb"));
                }
                nav.set_visible_child_name("share-smb");
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
    nfs_expander.add_suffix(&pill_label(&data.nfs_shares.len().to_string(), &["caption"]));

    if data.nfs_shares.is_empty() {
        nfs_expander.add_row(&empty_row("No NFS shares configured"));
    } else {
        for share in &data.nfs_shares {
            let display = share.path.split('/').last().unwrap_or(&share.path).to_string();
            let row = ActionRow::builder()
                .title(&display)
                .subtitle(&share.path)
                .activatable(true)
                .build();
            row.set_icon_name(Some("folder-symbolic"));

            let (label_text, label_classes): (&str, &[&str]) = if share.enabled {
                ("Active", &["caption", "success"])
            } else {
                ("Off", &["caption", "error"])
            };
            row.add_suffix(&pill_label(label_text, label_classes));
            row.add_suffix(&gtk4::Image::from_icon_name("go-next-symbolic"));

            let nav = nav_stack.clone();
            let share_clone = share.clone();
            row.connect_activated(move |_| {
                let page = share_info::build(ShareType::Nfs(share_clone.clone()), nav.clone());
                if nav.child_by_name("share-nfs").is_none() {
                    nav.add_named(&page, Some("share-nfs"));
                }
                nav.set_visible_child_name("share-nfs");
            });
            nfs_expander.add_row(&row);
        }
    }
    group.add(&nfs_expander);

    group
}

fn pill_label(text: &str, css_classes: &[&str]) -> gtk4::Label {
    let label = gtk4::Label::builder()
        .label(text)
        .valign(Align::Center)
        .build();
    for cls in css_classes {
        label.add_css_class(cls);
    }
    label
}

fn right_label(text: &str, css_classes: &[&str]) -> gtk4::Label {
    let label = gtk4::Label::builder()
        .label(text)
        .valign(Align::Center)
        .halign(Align::End)
        .build();
    for cls in css_classes {
        label.add_css_class(cls);
    }
    label
}

fn empty_row(text: &str) -> ActionRow {
    let row = ActionRow::builder()
        .title(text)
        .build();
    row.set_sensitive(false);
    row
}

fn push_stub_page(nav_stack: &Stack, name: &str, title: &str, icon: &str, description: &str) {
    if nav_stack.child_by_name(name).is_none() {
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
    }

    nav_stack.set_transition_type(StackTransitionType::SlideLeft);
    nav_stack.set_visible_child_name(name);
}

fn spacer(height: i32) -> gtk4::Box {
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