use gtk4::prelude::*;
use gtk4::{
    Stack, Orientation, Align, ScrolledWindow, PolicyType,
    Box as GBox, StackTransitionType, Entry,
};
use libadwaita::prelude::*;
use libadwaita::{
    ApplicationWindow, HeaderBar, ToolbarView,
    ActionRow, ExpanderRow, PreferencesGroup,
    StatusPage, SwitchRow,
};
use glib::clone;
use crate::state::AppState;
use crate::runtime;
use api::models::system::Pool;
use api::models::storage::{
    PoolScrubQueryResponse, UpdatePoolScrubDetails, DeletionSchedule,
    RunPoolScrubArgs, PoolScrubAction, TakeActionOnPoolScrubArgs,
};
use api::result::ApiResult;
use api::methods::{System as SystemMethods, Storage as StorageMethods};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct ScrubState {
    task: Option<PoolScrubQueryResponse>,
}

pub fn build(
    state: AppState,
    pool: Pool,
    nav_stack: Stack,
) -> ToolbarView {
    let toolbar_view = ToolbarView::new();

    let header = HeaderBar::new();
    let title = gtk4::Label::builder()
        .label(&pool.name)
        .css_classes(vec!["title"])
        .build();
    header.set_title_widget(Some(&title));

    let back_btn = gtk4::Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text("Back")
        .build();
    header.pack_start(&back_btn);

    let refresh_btn = gtk4::Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_text("Refresh")
        .build();
    header.pack_end(&refresh_btn);

    toolbar_view.add_top_bar(&header);

    let nav = nav_stack.clone();
    back_btn.connect_clicked(move |_| {
        nav.set_transition_type(StackTransitionType::SlideRight);
        nav.set_visible_child_name("home");
    });

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .vexpand(true)
        .build();

    let content = GBox::builder()
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
        .spacing(12)
        .vexpand(true)
        .build();
    let spinner = gtk4::Spinner::new();
    spinner.start();
    loading_box.append(&spinner);
    loading_box.append(
        &gtk4::Label::builder()
            .label("Loading scrub tasks…")
            .css_classes(vec!["dim-label"])
            .build()
    );

    let page_stack = Stack::new();
    page_stack.set_transition_type(StackTransitionType::Crossfade);
    page_stack.set_transition_duration(200);
    page_stack.add_named(&loading_box, Some("loading"));
    page_stack.add_named(&content, Some("content"));
    page_stack.set_visible_child_name("loading");

    scroll.set_child(Some(&page_stack));
    toolbar_view.set_content(Some(&scroll));

    let scrub_state: Arc<Mutex<Option<PoolScrubQueryResponse>>> = Arc::new(Mutex::new(None));

    let pool_id = pool.id;
    let pool_name = pool.name.clone();

    build_static_content(&content, &pool, &nav_stack, &state, scrub_state.clone());

    let load = {
        let state = state.clone();
        let page_stack = page_stack.clone();
        let content = content.clone();
        let pool = pool.clone();
        let scrub_state = scrub_state.clone();
        let nav_stack = nav_stack.clone();

        move || {
            page_stack.set_visible_child_name("loading");

            let (tx, rx) = async_channel::unbounded::<Option<PoolScrubQueryResponse>>();

            glib::MainContext::default().spawn_local(clone!(
                #[strong] page_stack,
                #[strong] content,
                #[strong] pool,
                #[strong] scrub_state,
                #[strong] nav_stack,
                #[strong] state,
                async move {
                    if let Ok(task) = rx.recv().await {
                        *scrub_state.lock().unwrap() = task.clone();
                        rebuild_scrub_section(&content, task.as_ref(), &pool, &state, &nav_stack, scrub_state.clone());
                        page_stack.set_visible_child_name("content");
                    }
                }
            ));

            let client = {
                let lock = state.manager.lock().unwrap();
                lock.as_ref().map(|m| m.client.clone())
            };

            if let Some(client) = client {
                runtime::spawn(
                    async move {
                        match client.call::<Vec<PoolScrubQueryResponse>>(StorageMethods::POOL_SCRUB_QUERY, vec![]).await {
                            ApiResult::Success(tasks) => tasks.into_iter().find(|t| t.pool == pool_id as i64),
                            _ => None,
                        }
                    },
                    tx,
                );
            } else {
                let _ = tx.send_blocking(None);
            }
        }
    };

    load();

    {
        let load = load.clone();
        refresh_btn.connect_clicked(move |_| load());
    }

    toolbar_view
}

fn build_static_content(
    container: &GBox,
    pool: &Pool,
    nav_stack: &Stack,
    state: &AppState,
    scrub_state: Arc<Mutex<Option<PoolScrubQueryResponse>>>,
) {
    container.append(&identity_group(pool));
    container.append(&spacer(12));
    container.append(&storage_usage_group(pool));
    container.append(&spacer(12));
    container.append(&status_group(pool));
    container.append(&spacer(12));

    if let Some(scan) = &pool.scan {
        container.append(&scan_group(scan));
        container.append(&spacer(12));
    }

    let browse_btn = gtk4::Button::builder()
        .label("Browse Datasets")
        .icon_name("folder-symbolic")
        .css_classes(vec!["suggested-action", "pill"])
        .halign(Align::End)
        .margin_end(0)
        .build();

    let nav = nav_stack.clone();
    let pname = pool.name.clone();
    browse_btn.connect_clicked(move |_| {
        nav.set_transition_type(StackTransitionType::SlideLeft);
        nav.set_visible_child_name("datasets");
    });

    container.append(&browse_btn);
    container.append(&spacer(12));

    let placeholder = GBox::builder()
        .name("scrub-section-placeholder")
        .build();
    container.append(&placeholder);
}

fn rebuild_scrub_section(
    container: &GBox,
    task: Option<&PoolScrubQueryResponse>,
    pool: &Pool,
    state: &AppState,
    nav_stack: &Stack,
    scrub_state: Arc<Mutex<Option<PoolScrubQueryResponse>>>,
) {
    let mut child = container.first_child();
    while let Some(c) = child {
        let next = c.next_sibling();
        if c.widget_name() == "scrub-section-placeholder" {
            container.remove(&c);
            break;
        }
        child = next;
    }

    let section = scrub_group(task, pool, state, nav_stack, scrub_state);
    container.append(&section);
}

fn identity_group(pool: &Pool) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Pool Identity");

    let name_row = ActionRow::builder()
        .title("Name")
        .build();
    name_row.add_suffix(&right_label(&pool.name, &["monospace"]));
    group.add(&name_row);

    let guid_row = ActionRow::builder()
        .title("GUID")
        .build();
    guid_row.add_suffix(&right_label(&pool.guid, &["caption", "dim-label", "monospace"]));
    group.add(&guid_row);

    let path_row = ActionRow::builder()
        .title("Path")
        .build();
    path_row.add_suffix(&right_label(&pool.path, &["caption", "dim-label"]));
    group.add(&path_row);

    group
}

fn storage_usage_group(pool: &Pool) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Storage");
    group.set_description(Some(&format!("Total capacity: {}", format_bytes(pool.size))));

    let used_frac = if pool.size > 0 {
        (pool.allocated as f64 / pool.size as f64).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let used_pct = (used_frac * 100.0) as i32;

    let bar_wrapper = GBox::builder()
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
    bar.set_value(used_frac);
    bar.set_mode(gtk4::LevelBarMode::Continuous);

    if used_frac > 0.9 {
        bar.add_css_class("error");
    } else if used_frac > 0.75 {
        bar.add_css_class("warning");
    }

    let stats = GBox::builder()
        .orientation(Orientation::Horizontal)
        .build();

    let used_lbl = gtk4::Label::builder()
        .label(&format!("Used: {} ({}%)", format_bytes(pool.allocated), used_pct))
        .css_classes(vec!["caption", "dim-label"])
        .halign(Align::Start)
        .hexpand(true)
        .build();

    let free_lbl = gtk4::Label::builder()
        .label(&format!("Free: {}", format_bytes(pool.free)))
        .css_classes(vec!["caption", "success"])
        .halign(Align::End)
        .build();

    stats.append(&used_lbl);
    stats.append(&free_lbl);
    bar_wrapper.append(&bar);
    bar_wrapper.append(&stats);

    let bar_row = libadwaita::PreferencesRow::new();
    bar_row.set_child(Some(&bar_wrapper));
    group.add(&bar_row);

    let alloc_row = ActionRow::builder().title("Allocated").build();
    alloc_row.add_suffix(&right_label(&format_bytes(pool.allocated), &["caption"]));
    group.add(&alloc_row);

    let free_row = ActionRow::builder().title("Free").build();
    free_row.add_suffix(&right_label(&format_bytes(pool.free), &["caption", "success"]));
    group.add(&free_row);

    let freeing_row = ActionRow::builder().title("Freeing").build();
    freeing_row.add_suffix(&right_label(&format_bytes(pool.freeing), &["caption", "dim-label"]));
    group.add(&freeing_row);

    let frag_row = ActionRow::builder().title("Fragmentation").build();
    frag_row.add_suffix(&right_label(&format!("{}%", pool.fragmentation), &["caption"]));
    group.add(&frag_row);

    let dedup_row = ActionRow::builder().title("Dedup Table Size").build();
    dedup_row.add_suffix(&right_label(&format_bytes(pool.dedup_table_size), &["caption", "dim-label"]));
    group.add(&dedup_row);

    group
}

fn status_group(pool: &Pool) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Health &amp; Status");

    let (health_str, health_class) = match (pool.healthy, pool.warning) {
        (true, false) => ("Healthy", "success"),
        (true, true) => ("Warning", "warning"),
        (false, _) => ("Degraded", "error"),
    };

    let status_row = ActionRow::builder()
        .title("Status")
        .subtitle(&pool.status_code)
        .build();
    status_row.set_icon_name(Some(if pool.healthy { "emblem-ok-symbolic" } else { "dialog-error-symbolic" }));
    status_row.add_suffix(&pill_label(health_str, &[health_class, "caption"]));
    group.add(&status_row);

    if let Some(detail) = &pool.status_detail {
        if !detail.is_empty() {
            let detail_row = ActionRow::builder()
                .title("Detail")
                .subtitle(detail)
                .build();
            group.add(&detail_row);
        }
    }

    let trim_row = ActionRow::builder().title("Auto Trim").build();
    trim_row.add_suffix(&right_label(&pool.autotrim.parsed, &["caption"]));
    group.add(&trim_row);

    let warning_row = ActionRow::builder().title("Warning Flag").build();
    warning_row.add_suffix(&pill_label(
        if pool.warning { "Yes" } else { "No" },
        &[if pool.warning { "warning" } else { "dim-label" }, "caption"],
    ));
    group.add(&warning_row);

    group
}

fn scan_group(scan: &api::models::system::PoolScan) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Last Scan");

    let func_row = ActionRow::builder().title("Function").build();
    func_row.add_suffix(&right_label(scan.function.as_deref().unwrap_or("—"), &["caption"]));
    group.add(&func_row);

    let state_row = ActionRow::builder().title("State").build();
    let state_str = scan.state.as_deref().unwrap_or("—");
    let state_class = match state_str {
        "FINISHED" => "success",
        "SCANNING" => "accent",
        "CANCELED" => "warning",
        _ => "dim-label",
    };
    state_row.add_suffix(&pill_label(state_str, &[state_class, "caption"]));
    group.add(&state_row);

    let errors_row = ActionRow::builder().title("Errors").build();
    let err_count = scan.errors.unwrap_or(0);
    errors_row.add_suffix(&right_label(
        &err_count.to_string(),
        &[if err_count > 0 { "error" } else { "success" }, "caption"],
    ));
    group.add(&errors_row);

    if let Some(pct) = scan.percentage {
        let bar_wrapper = GBox::builder()
            .orientation(Orientation::Vertical)
            .margin_top(8)
            .margin_bottom(10)
            .margin_start(16)
            .margin_end(16)
            .spacing(4)
            .build();

        let bar = gtk4::LevelBar::new();
        bar.set_min_value(0.0);
        bar.set_max_value(100.0);
        bar.set_value(pct);

        let pct_label = gtk4::Label::builder()
            .label(&format!("Progress: {:.1}%", pct))
            .css_classes(vec!["caption", "dim-label"])
            .halign(Align::Start)
            .build();

        bar_wrapper.append(&bar);
        bar_wrapper.append(&pct_label);

        if let Some(secs_left) = scan.total_secs_left {
            let eta = gtk4::Label::builder()
                .label(&format!("ETA: {}m {}s", secs_left / 60, secs_left % 60))
                .css_classes(vec!["caption", "dim-label"])
                .halign(Align::End)
                .build();
            bar_wrapper.append(&eta);
        }

        let bar_row = libadwaita::PreferencesRow::new();
        bar_row.set_child(Some(&bar_wrapper));
        group.add(&bar_row);
    }

    group
}

fn scrub_group(
    task: Option<&PoolScrubQueryResponse>,
    pool: &Pool,
    state: &AppState,
    nav_stack: &Stack,
    scrub_state: Arc<Mutex<Option<PoolScrubQueryResponse>>>,
) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Scrub Schedule");

    match task {
        None => {
            let empty_row = ActionRow::builder()
                .title("No scrub task configured")
                .subtitle("Create one to schedule automatic scrubs")
                .build();
            empty_row.set_icon_name(Some("dialog-information-symbolic"));
            group.add(&empty_row);

            let create_btn = gtk4::Button::builder()
                .label("Create Scrub Task")
                .icon_name("list-add-symbolic")
                .css_classes(vec!["suggested-action", "pill"])
                .halign(Align::Center)
                .margin_top(8)
                .build();

            let state2 = state.clone();
            let pool2 = pool.clone();
            let nav = nav_stack.clone();
            let ss = scrub_state.clone();
            create_btn.connect_clicked(move |_| {
                show_scrub_dialog(None, &pool2, &state2, &nav, ss.clone());
            });

            let btn_row = libadwaita::PreferencesRow::new();
            let btn_wrap = GBox::builder()
                .margin_top(4).margin_bottom(4)
                .margin_start(16).margin_end(16)
                .build();
            btn_wrap.append(&create_btn);
            btn_row.set_child(Some(&btn_wrap));
            group.add(&btn_row);
        }

        Some(t) => {
            let enabled_row = ActionRow::builder().title("Status").build();
            enabled_row.add_suffix(&pill_label(
                if t.enabled { "Enabled" } else { "Disabled" },
                &[if t.enabled { "success" } else { "dim-label" }, "caption"],
            ));
            group.add(&enabled_row);

            if !t.description.is_empty() {
                let desc_row = ActionRow::builder()
                    .title("Description")
                    .subtitle(&t.description)
                    .build();
                group.add(&desc_row);
            }

            let threshold_row = ActionRow::builder().title("Threshold").build();
            threshold_row.add_suffix(&right_label(&format!("{} days", t.threshold), &["caption"]));
            group.add(&threshold_row);

            let cron = format!(
                "{} {} {} {} {}",
                t.schedule.minute, t.schedule.hour,
                t.schedule.dom, t.schedule.month, t.schedule.dow
            );
            let sched_row = ActionRow::builder()
                .title("Cron Schedule")
                .build();
            sched_row.add_suffix(&right_label(&cron, &["caption", "monospace", "dim-label"]));
            group.add(&sched_row);

            let pool_name = pool.name.clone();
            let task_clone = t.clone();

            let actions_box = GBox::builder()
                .orientation(Orientation::Horizontal)
                .spacing(8)
                .margin_top(8)
                .margin_bottom(8)
                .margin_start(16)
                .margin_end(16)
                .build();

            let edit_btn = gtk4::Button::builder()
                .label("Edit")
                .icon_name("document-edit-symbolic")
                .css_classes(vec!["pill"])
                .hexpand(true)
                .build();

            let run_btn = gtk4::Button::builder()
                .label("Run Now")
                .icon_name("media-playback-start-symbolic")
                .css_classes(vec!["suggested-action", "pill"])
                .hexpand(true)
                .build();

            let del_btn = gtk4::Button::builder()
                .label("Delete")
                .icon_name("user-trash-symbolic")
                .css_classes(vec!["destructive-action", "pill"])
                .hexpand(true)
                .build();

            {
                let state2 = state.clone();
                let pool2 = pool.clone();
                let nav = nav_stack.clone();
                let ss = scrub_state.clone();
                let tc = task_clone.clone();
                edit_btn.connect_clicked(move |_| {
                    show_scrub_dialog(Some(&tc), &pool2, &state2, &nav, ss.clone());
                });
            }

            {
                let state2 = state.clone();
                let pname = pool_name.clone();
                let threshold = t.threshold;
                run_btn.connect_clicked(move |_| {
                    let client = {
                        let lock = state2.manager.lock().unwrap();
                        lock.as_ref().map(|m| m.client.clone())
                    };
                    if let Some(client) = client {
                        let (tx, rx) = async_channel::unbounded::<()>();
                        let args = RunPoolScrubArgs { name: pname.clone(), threshold };
                        runtime::spawn(
                            async move {
                                let _ = client.call::<bool>(
                                    StorageMethods::POOL_SCRUB_RUN,
                                    vec![serde_json::json!(args.name), serde_json::json!(args.threshold)],
                                ).await;
                            },
                            tx,
                        );
                    }
                });
            }

            {
                let state2 = state.clone();
                let task_id = t.id;
                del_btn.connect_clicked(move |_| {
                    let client = {
                        let lock = state2.manager.lock().unwrap();
                        lock.as_ref().map(|m| m.client.clone())
                    };
                    if let Some(client) = client {
                        let (tx, rx) = async_channel::unbounded::<()>();
                        runtime::spawn(
                            async move {
                                let _ = client.call::<bool>(
                                    StorageMethods::POOL_SCRUB_DELETE,
                                    vec![serde_json::json!(task_id)],
                                ).await;
                            },
                            tx,
                        );
                    }
                });
            }

            actions_box.append(&edit_btn);
            actions_box.append(&run_btn);
            actions_box.append(&del_btn);

            let actions_row = libadwaita::PreferencesRow::new();
            actions_row.set_child(Some(&actions_box));
            group.add(&actions_row);
        }
    }

    group
}

fn show_scrub_dialog(
    existing: Option<&PoolScrubQueryResponse>,
    pool: &Pool,
    state: &AppState,
    nav_stack: &Stack,
    scrub_state: Arc<Mutex<Option<PoolScrubQueryResponse>>>,
) {
    let dialog = libadwaita::AlertDialog::new(
        Some(if existing.is_some() { "Edit Scrub Task" } else { "Create Scrub Task" }),
        None,
    );
    dialog.add_response("cancel", "Cancel");
    dialog.add_response("confirm", if existing.is_some() { "Update" } else { "Create" });
    dialog.set_response_appearance("confirm", libadwaita::ResponseAppearance::Suggested);

    let form = GBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let desc_entry = Entry::builder()
        .placeholder_text("Description")
        .text(existing.map(|t| t.description.as_str()).unwrap_or(""))
        .build();

    let threshold_entry = Entry::builder()
        .placeholder_text("Threshold (days)")
        .text(existing.map(|t| t.threshold.to_string()).as_deref().unwrap_or("35"))
        .build();

    let minute_entry = Entry::builder()
        .placeholder_text("Min (00)")
        .text(existing.map(|t| t.schedule.minute.as_str()).unwrap_or("00"))
        .hexpand(true)
        .build();

    let hour_entry = Entry::builder()
        .placeholder_text("Hour (00)")
        .text(existing.map(|t| t.schedule.hour.as_str()).unwrap_or("00"))
        .hexpand(true)
        .build();

    let dom_entry = Entry::builder()
        .placeholder_text("Day of Month (*)")
        .text(existing.map(|t| t.schedule.dom.as_str()).unwrap_or("*"))
        .hexpand(true)
        .build();

    let month_entry = Entry::builder()
        .placeholder_text("Month (*)")
        .text(existing.map(|t| t.schedule.month.as_str()).unwrap_or("*"))
        .hexpand(true)
        .build();

    let dow_entry = Entry::builder()
        .placeholder_text("Day of Week (7)")
        .text(existing.map(|t| t.schedule.dow.as_str()).unwrap_or("7"))
        .hexpand(true)
        .build();

    let enabled_switch = SwitchRow::builder()
        .title("Enabled")
        .active(existing.map(|t| t.enabled).unwrap_or(true))
        .build();

    let cron_box = GBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .build();
    cron_box.append(&minute_entry);
    cron_box.append(&hour_entry);
    cron_box.append(&dom_entry);

    let cron_box2 = GBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .build();
    cron_box2.append(&month_entry);
    cron_box2.append(&dow_entry);

    let cron_lbl = gtk4::Label::builder()
        .label("Cron Schedule")
        .halign(Align::Start)
        .css_classes(vec!["heading"])
        .build();

    form.append(&desc_entry);
    form.append(&threshold_entry);
    form.append(&cron_lbl);
    form.append(&cron_box);
    form.append(&cron_box2);

    let enabled_box = GBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .build();
    let enabled_lbl = gtk4::Label::builder()
        .label("Enabled")
        .hexpand(true)
        .halign(Align::Start)
        .build();
    let enabled_sw = gtk4::Switch::builder()
        .active(existing.map(|t| t.enabled).unwrap_or(true))
        .valign(Align::Center)
        .build();
    enabled_box.append(&enabled_lbl);
    enabled_box.append(&enabled_sw);
    form.append(&enabled_box);

    dialog.set_extra_child(Some(&form));

    let pool_id = pool.id as i64;
    let existing_id = existing.map(|t| t.id);
    let state2 = state.clone();

    dialog.connect_response(None, move |_dlg, response| {
        if response != "confirm" { return; }

        let schedule = DeletionSchedule {
            minute: minute_entry.text().to_string(),
            hour: hour_entry.text().to_string(),
            dom: dom_entry.text().to_string(),
            month: month_entry.text().to_string(),
            dow: dow_entry.text().to_string(),
        };

        let details = UpdatePoolScrubDetails {
            pool: pool_id,
            threshold: threshold_entry.text().to_string().parse().unwrap_or(35),
            description: desc_entry.text().to_string(),
            schedule,
            enabled: enabled_sw.is_active(),
        };

        let client = {
            let lock = state2.manager.lock().unwrap();
            lock.as_ref().map(|m| m.client.clone())
        };

        if let Some(client) = client {
            let (tx, rx) = async_channel::unbounded::<()>();
            if let Some(id) = existing_id {
                runtime::spawn(
                    async move {
                        let _ = client.call::<serde_json::Value>(
                            StorageMethods::POOL_SCRUB_UPDATE,
                            vec![serde_json::json!(id), serde_json::json!(details)],
                        ).await;
                    },
                    tx,
                );
            } else {
                runtime::spawn(
                    async move {
                        let _ = client.call::<serde_json::Value>(
                            StorageMethods::POOL_SCRUB_CREATE,
                            vec![serde_json::json!(details)],
                        ).await;
                    },
                    tx,
                );
            }
        }
    });

    dialog.present(gtk4::Window::NONE);
}

fn pill_label(text: &str, css_classes: &[&str]) -> gtk4::Label {
    let lbl = gtk4::Label::builder().label(text).valign(Align::Center).build();
    for c in css_classes { lbl.add_css_class(c); }
    lbl
}

fn right_label(text: &str, css_classes: &[&str]) -> gtk4::Label {
    let lbl = gtk4::Label::builder()
        .label(text).valign(Align::Center).halign(Align::End).build();
    for c in css_classes { lbl.add_css_class(c); }
    lbl
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
    while size >= 1024.0 && i < units.len() - 1 { size /= 1024.0; i += 1; }
    format!("{:.1} {}", size, units[i])
}