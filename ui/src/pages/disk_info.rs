use gtk4::prelude::*;
use gtk4::{
    Stack, Orientation, Align, ScrolledWindow, PolicyType,
    Box as GBox, StackTransitionType,
};
use libadwaita::prelude::*;
use libadwaita::{
    HeaderBar, ToolbarView,
    ActionRow, PreferencesGroup,
    ViewStack, ViewSwitcher, ViewSwitcherPolicy,
    ViewSwitcherBar,
};
use api::models::system::DiskDetails;

pub fn build(disks: Vec<DiskDetails>, nav_stack: Stack) -> ToolbarView {
    let toolbar_view = ToolbarView::new();

    let header = HeaderBar::new();
    let title = gtk4::Label::builder()
        .label("Disks")
        .css_classes(vec!["title"])
        .build();
    header.set_title_widget(Some(&title));

    let back_btn = gtk4::Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text("Back")
        .build();
    header.pack_start(&back_btn);
    toolbar_view.add_top_bar(&header);

    let nav = nav_stack.clone();
    back_btn.connect_clicked(move |_| {
        nav.set_transition_type(StackTransitionType::SlideRight);
        nav.set_visible_child_name("home");
    });

    if disks.is_empty() {
        let empty = libadwaita::StatusPage::builder()
            .icon_name("drive-harddisk-symbolic")
            .title("No Disks Found")
            .description("No disk information is available.")
            .build();
        toolbar_view.set_content(Some(&empty));
        return toolbar_view;
    }

    let disk_stack = ViewStack::new();
    let switcher = ViewSwitcher::builder()
        .stack(&disk_stack)
        .policy(ViewSwitcherPolicy::Wide)
        .build();

    let switcher_bar = ViewSwitcherBar::new();
    switcher_bar.set_stack(Some(&disk_stack));
    switcher_bar.set_reveal(disks.len() > 1);

    for disk in &disks {
        let page_content = build_disk_page(disk);
        let scroll = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .vexpand(true)
            .child(&page_content)
            .build();

        disk_stack.add_titled_with_icon(
            &scroll,
            Some(&disk.name),
            &disk.name,
            "drive-harddisk-symbolic",
        );
    }

    // Use the ViewSwitcher as the title widget if few disks
    if disks.len() <= 4 {
        header.set_title_widget(Some(&switcher));
    }

    let main_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .vexpand(true)
        .child(&disk_stack)
        .build();

    toolbar_view.set_content(Some(&main_scroll));

    toolbar_view
}

fn build_disk_page(disk: &DiskDetails) -> GBox {
    let page = GBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .margin_top(12)
        .margin_bottom(24)
        .margin_start(12)
        .margin_end(12)
        .build();

    page.append(&identity_group(disk));
    page.append(&spacer(12));
    page.append(&health_group(disk));
    page.append(&spacer(12));
    page.append(&power_group(disk));
    page.append(&spacer(12));

    if disk.pool.is_some() {
        page.append(&pool_group(disk));
        page.append(&spacer(12));
    }

    page.append(&technical_group(disk));
    page
}

fn identity_group(disk: &DiskDetails) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Identity");

    let name_row = ActionRow::builder().title("Device Name").build();
    name_row.set_icon_name(Some("drive-harddisk-symbolic"));
    name_row.add_suffix(&right_label(&disk.name, &["monospace", "caption"]));
    group.add(&name_row);

    let model_row = ActionRow::builder().title("Model").build();
    model_row.add_suffix(&right_label(
        disk.model.as_deref().unwrap_or("Unknown"),
        &["caption"],
    ));
    group.add(&model_row);

    let serial_row = ActionRow::builder().title("Serial Number").build();
    serial_row.add_suffix(&right_label(&disk.serial, &["caption", "monospace"]));
    group.add(&serial_row);

    let size_gb = disk.size / (1024 * 1024 * 1024);
    let size_row = ActionRow::builder().title("Capacity").build();
    size_row.add_suffix(&right_label(&format!("{} GB", size_gb), &["caption"]));
    group.add(&size_row);

    let type_row = ActionRow::builder().title("Type").build();
    type_row.add_suffix(&pill_label(&disk.disk_type, &["caption"]));
    group.add(&type_row);

    let bus_row = ActionRow::builder().title("Bus").build();
    bus_row.add_suffix(&right_label(&disk.bus, &["caption", "dim-label"]));
    group.add(&bus_row);

    if let Some(rotation) = disk.rotationrate {
        let rot_row = ActionRow::builder().title("Rotation Rate").build();
        rot_row.add_suffix(&right_label(
            &format!("{} RPM", rotation),
            &["caption"],
        ));
        group.add(&rot_row);
    }

    group
}

fn health_group(disk: &DiskDetails) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Health &amp; SMART");   // <-- fix: escaped ampersand

    let smart_enabled = disk.togglesmart.unwrap_or(true);
    let smart_row = ActionRow::builder()
        .title("SMART Monitoring")
        .subtitle(if smart_enabled { "Active and monitoring" } else { "Disabled — consider enabling" })
        .build();
    smart_row.set_icon_name(Some("emblem-system-symbolic"));
    smart_row.add_suffix(&pill_label(
        if smart_enabled { "ON" } else { "OFF" },
        &[if smart_enabled { "success" } else { "error" }, "caption"],
    ));
    group.add(&smart_row);

    if let Some(supports) = disk.supports_smart {
        let cap_row = ActionRow::builder().title("SMART Capable").build();
        cap_row.add_suffix(&right_label(
            if supports { "Yes" } else { "No" },
            &[if supports { "success" } else { "dim-label" }, "caption"],
        ));
        group.add(&cap_row);
    }

    if let Some(opts) = &disk.smartoptions {
        if !opts.is_empty() {
            let opts_row = ActionRow::builder()
                .title("SMART Options")
                .subtitle(opts)
                .build();
            group.add(&opts_row);
        }
    }

    if let Some(critical) = &disk.critical {
        let crit_row = ActionRow::builder().title("Critical Temp").build();
        crit_row.add_suffix(&right_label(critical, &["caption", "error"]));
        group.add(&crit_row);
    }

    if let Some(inform) = &disk.informational {
        let info_row = ActionRow::builder().title("Informational Temp").build();
        info_row.add_suffix(&right_label(inform, &["caption"]));
        group.add(&info_row);
    }

    group
}

fn power_group(disk: &DiskDetails) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Power Management");

    let transfer_row = ActionRow::builder().title("Transfer Mode").build();
    transfer_row.set_icon_name(Some("drive-harddisk-symbolic"));
    transfer_row.add_suffix(&right_label(&disk.transfermode, &["caption"]));
    group.add(&transfer_row);

    let standby_row = ActionRow::builder().title("HDD Standby").build();
    standby_row.add_suffix(&right_label(&disk.hddstandby, &["caption"]));
    group.add(&standby_row);

    let apm_row = ActionRow::builder().title("Adv. Power Management").build();
    apm_row.add_suffix(&right_label(&disk.advpowermgmt, &["caption"]));
    group.add(&apm_row);

    group
}

fn pool_group(disk: &DiskDetails) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Pool Association");

    if let Some(pool_name) = &disk.pool {
        let pool_row = ActionRow::builder()
            .title("Pool")
            .subtitle(pool_name)
            .build();
        pool_row.set_icon_name(Some("drive-harddisk-symbolic"));
        group.add(&pool_row);
    }

    if let Some(guid) = &disk.zfs_guid {
        let guid_row = ActionRow::builder().title("ZFS GUID").build();
        guid_row.add_suffix(&right_label(guid, &["caption", "monospace", "dim-label"]));
        group.add(&guid_row);
    }

    group
}

fn technical_group(disk: &DiskDetails) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Technical Details");

    let devname_row = ActionRow::builder().title("Device Name").build();
    devname_row.add_suffix(&right_label(&disk.devname, &["caption", "monospace"]));
    group.add(&devname_row);

    let subsys_row = ActionRow::builder().title("Subsystem").build();
    subsys_row.add_suffix(&right_label(&disk.subsystem, &["caption"]));
    group.add(&subsys_row);

    let desc_row = ActionRow::builder().title("Description").build();
    desc_row.add_suffix(&right_label(&disk.description, &["caption", "dim-label"]));
    group.add(&desc_row);

    if let Some(lunid) = &disk.lunid {
        if !lunid.is_empty() {
            let lun_row = ActionRow::builder().title("LUN ID").build();
            lun_row.add_suffix(&right_label(lunid, &["caption", "monospace", "dim-label"]));
            group.add(&lun_row);
        }
    }

    group
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