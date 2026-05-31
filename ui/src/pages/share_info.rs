use gtk4::prelude::*;
use gtk4::{
    Stack, Orientation, Align, ScrolledWindow, PolicyType,
    Box as GBox, StackTransitionType,
};
use libadwaita::prelude::*;
use libadwaita::{
    HeaderBar, ToolbarView,
    ActionRow, ExpanderRow, PreferencesGroup,
};
use api::models::shares::{SmbShare, NfsShare};

pub enum ShareType {
    Smb(SmbShare),
    Nfs(NfsShare),
}

pub fn build(share: ShareType, nav_stack: Stack) -> ToolbarView {
    let toolbar_view = ToolbarView::new();
    let header = HeaderBar::new();

    let back_btn = gtk4::Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text("Back")
        .build();
    header.pack_start(&back_btn);

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

    match &share {
        ShareType::Smb(s) => {
            let title = gtk4::Label::builder()
                .label(&s.name)
                .css_classes(vec!["title"])
                .build();
            header.set_title_widget(Some(&title));

            let sub = gtk4::Label::builder()
                .label("SMB Share")
                .css_classes(vec!["subtitle"])
                .build();

            content.append(&smb_identity_group(s));
            content.append(&spacer(12));
            content.append(&smb_access_group(s));
            content.append(&spacer(12));
            content.append(&smb_features_group(s));
            content.append(&spacer(12));

            if !s.hostsallow.as_ref().map(|v| v.is_empty()).unwrap_or(true)
                || !s.hostsdeny.as_ref().map(|v| v.is_empty()).unwrap_or(true)
            {
                content.append(&smb_network_group(s));
                content.append(&spacer(12));
            }

            if s.timemachine.unwrap_or(false) {
                content.append(&smb_timemachine_group(s));
                content.append(&spacer(12));
            }

            if s.audit.enable {
                content.append(&smb_audit_group(s));
                content.append(&spacer(12));
            }

            if let Some(aux) = &s.auxsmbconf {
                if !aux.is_empty() {
                    content.append(&smb_aux_group(aux));
                }
            }
        }

        ShareType::Nfs(s) => {
            let name = s.path.split('/').last().unwrap_or(&s.path).to_string();
            let title = gtk4::Label::builder()
                .label(&name)
                .css_classes(vec!["title"])
                .build();
            header.set_title_widget(Some(&title));

            content.append(&nfs_identity_group(s));
            content.append(&spacer(12));
            content.append(&nfs_features_group(s));
            content.append(&spacer(12));

            if !s.networks.is_empty() || !s.hosts.is_empty() {
                content.append(&nfs_network_group(s));
                content.append(&spacer(12));
            }

            if s.maproot_user.is_some() || s.mapall_user.is_some() {
                content.append(&nfs_mapping_group(s));
                content.append(&spacer(12));
            }

            if !s.security.is_empty() {
                content.append(&nfs_security_group(s));
            }
        }
    }

    toolbar_view.add_top_bar(&header);
    scroll.set_child(Some(&content));
    toolbar_view.set_content(Some(&scroll));
    toolbar_view
}

fn smb_identity_group(s: &SmbShare) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Basic Information");

    let name_row = ActionRow::builder().title("Share Name").build();
    name_row.set_icon_name(Some("folder-symbolic"));
    name_row.add_suffix(&right_label(&s.name, &["caption"]));
    group.add(&name_row);

    let path_row = ActionRow::builder()
        .title("Path")
        .subtitle(&s.path)
        .build();
    path_row.set_icon_name(Some("folder-open-symbolic"));
    group.add(&path_row);

    if let Some(suffix) = &s.path_suffix {
        if !suffix.is_empty() {
            let suffix_row = ActionRow::builder().title("Path Suffix").build();
            suffix_row.add_suffix(&right_label(suffix, &["caption", "dim-label"]));
            group.add(&suffix_row);
        }
    }

    if let Some(comment) = &s.comment {
        if !comment.is_empty() {
            let comment_row = ActionRow::builder()
                .title("Description")
                .subtitle(comment)
                .build();
            group.add(&comment_row);
        }
    }

    let status_row = ActionRow::builder().title("Status").build();
    status_row.add_suffix(&pill_label(
        if s.enabled { "Active" } else { "Disabled" },
        &[if s.enabled { "success" } else { "error" }, "caption"],
    ));
    group.add(&status_row);

    if let Some(vuid) = &s.vuid {
        if !vuid.is_empty() {
            let vuid_row = ActionRow::builder().title("VUID").build();
            vuid_row.add_suffix(&right_label(vuid, &["caption", "monospace", "dim-label"]));
            group.add(&vuid_row);
        }
    }

    group
}

fn smb_access_group(s: &SmbShare) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Access Control");

    let ro_row = ActionRow::builder().title("Read-Only").build();
    ro_row.add_suffix(&bool_label(s.ro.unwrap_or(false)));
    group.add(&ro_row);

    let guest_row = ActionRow::builder().title("Guest Access").build();
    guest_row.add_suffix(&bool_label(s.guestok.unwrap_or(false)));
    group.add(&guest_row);

    let browse_row = ActionRow::builder().title("Browsable").build();
    browse_row.add_suffix(&bool_label(s.browsable));
    group.add(&browse_row);

    let locked_row = ActionRow::builder().title("Locked").build();
    locked_row.add_suffix(&bool_label(s.locked));
    group.add(&locked_row);

    group
}

fn smb_features_group(s: &SmbShare) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Features");

    let features: &[(&str, bool)] = &[
        ("ACL Support", s.acl.unwrap_or(false)),
        ("Alternate Data Streams", s.streams.unwrap_or(false)),
        ("Durable Handles", s.durablehandle.unwrap_or(false)),
        ("Shadow Copies", s.shadowcopy.unwrap_or(false)),
        ("Recycle Bin", s.recyclebin.unwrap_or(false)),
        ("Home Share", s.home.unwrap_or(false)),
        ("Time Machine", s.timemachine.unwrap_or(false)),
        ("Apple Name Mangling", s.aapl_name_mangling.unwrap_or(false)),
        ("ABE", s.abe.unwrap_or(false)),
        ("FSRVP", s.fsrvp.unwrap_or(false)),
        ("AFP", s.afp.unwrap_or(false)),
    ];

    for (label, enabled) in features {
        let row = ActionRow::builder().title(*label).build();
        row.add_suffix(&bool_label(*enabled));
        group.add(&row);
    }

    group
}

fn smb_network_group(s: &SmbShare) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Network Access");

    if let Some(allow) = &s.hostsallow {
        if !allow.is_empty() {
            let exp = ExpanderRow::builder()
                .title("Allowed Hosts")
                .subtitle(&format!("{} host(s)", allow.len()))
                .build();
            exp.set_icon_name(Some("emblem-ok-symbolic"));
            for host in allow {
                let row = ActionRow::builder().title(host).build();
                exp.add_row(&row);
            }
            group.add(&exp);
        }
    }

    if let Some(deny) = &s.hostsdeny {
        if !deny.is_empty() {
            let exp = ExpanderRow::builder()
                .title("Denied Hosts")
                .subtitle(&format!("{} host(s)", deny.len()))
                .build();
            exp.set_icon_name(Some("dialog-error-symbolic"));
            for host in deny {
                let row = ActionRow::builder().title(host).build();
                exp.add_row(&row);
            }
            group.add(&exp);
        }
    }

    group
}

fn smb_timemachine_group(s: &SmbShare) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Time Machine");

    let tm_row = ActionRow::builder()
        .title("Time Machine Backup")
        .subtitle("This share is configured as a Time Machine target")
        .build();
    tm_row.set_icon_name(Some("media-tape-symbolic"));
    group.add(&tm_row);

    if let Some(quota) = s.timemachine_quota {
        let quota_row = ActionRow::builder().title("Quota").build();
        quota_row.add_suffix(&right_label(
            &if quota > 0 { format!("{} GB", quota) } else { "No limit".to_string() },
            &["caption"],
        ));
        group.add(&quota_row);
    }

    group
}

fn smb_audit_group(s: &SmbShare) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Audit Settings");

    let enabled_row = ActionRow::builder().title("Auditing").build();
    enabled_row.add_suffix(&pill_label("Enabled", &["success", "caption"]));
    group.add(&enabled_row);

    if !s.audit.watch_list.is_empty() {
        let exp = ExpanderRow::builder()
            .title("Watch List")
            .subtitle(&format!("{} item(s)", s.audit.watch_list.len()))
            .build();
        for item in &s.audit.watch_list {
            exp.add_row(&ActionRow::builder().title(item).build());
        }
        group.add(&exp);
    }

    if !s.audit.ignore_list.is_empty() {
        let exp = ExpanderRow::builder()
            .title("Ignore List")
            .subtitle(&format!("{} item(s)", s.audit.ignore_list.len()))
            .build();
        for item in &s.audit.ignore_list {
            exp.add_row(&ActionRow::builder().title(item).build());
        }
        group.add(&exp);
    }

    group
}

fn smb_aux_group(aux: &str) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Additional Configuration");

    let text_view = gtk4::TextView::builder()
        .editable(false)
        .monospace(true)
        .wrap_mode(gtk4::WrapMode::Word)
        .top_margin(12)
        .bottom_margin(12)
        .left_margin(16)
        .right_margin(16)
        .build();
    text_view.buffer().set_text(aux);
    text_view.add_css_class("card");

    let row = libadwaita::PreferencesRow::new();
    row.set_child(Some(&text_view));
    group.add(&row);

    group
}

fn nfs_identity_group(s: &NfsShare) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Basic Information");

    let path_row = ActionRow::builder()
        .title("Export Path")
        .subtitle(&s.path)
        .build();
    path_row.set_icon_name(Some("folder-symbolic"));
    group.add(&path_row);

    let id_row = ActionRow::builder().title("Share ID").build();
    id_row.add_suffix(&right_label(&s.id.to_string(), &["caption", "dim-label"]));
    group.add(&id_row);

    if !s.comment.is_empty() {
        let comment_row = ActionRow::builder()
            .title("Description")
            .subtitle(&s.comment)
            .build();
        group.add(&comment_row);
    }

    let status_row = ActionRow::builder().title("Status").build();
    status_row.add_suffix(&pill_label(
        if s.enabled { "Active" } else { "Disabled" },
        &[if s.enabled { "success" } else { "error" }, "caption"],
    ));
    group.add(&status_row);

    if !s.aliases.is_empty() {
        let exp = ExpanderRow::builder()
            .title("Aliases")
            .subtitle(&format!("{} alias(es)", s.aliases.len()))
            .build();
        for alias in &s.aliases {
            exp.add_row(&ActionRow::builder().title(alias).build());
        }
        group.add(&exp);
    }

    group
}

fn nfs_features_group(s: &NfsShare) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Features");

    let ro_row = ActionRow::builder().title("Read-Only").build();
    ro_row.add_suffix(&bool_label(s.ro));
    group.add(&ro_row);

    let locked_row = ActionRow::builder().title("Locked").build();
    locked_row.add_suffix(&bool_label(s.locked));
    group.add(&locked_row);

    let snap_row = ActionRow::builder().title("Expose Snapshots").build();
    snap_row.add_suffix(&bool_label(s.expose_snapshots));
    group.add(&snap_row);

    group
}

fn nfs_network_group(s: &NfsShare) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Network Access");

    if !s.networks.is_empty() {
        let exp = ExpanderRow::builder()
            .title("Allowed Networks")
            .subtitle(&format!("{} network(s)", s.networks.len()))
            .build();
        exp.set_icon_name(Some("network-wired-symbolic"));
        for net in &s.networks {
            exp.add_row(&ActionRow::builder().title(net).build());
        }
        group.add(&exp);
    }

    if !s.hosts.is_empty() {
        let exp = ExpanderRow::builder()
            .title("Allowed Hosts")
            .subtitle(&format!("{} host(s)", s.hosts.len()))
            .build();
        exp.set_icon_name(Some("computer-symbolic"));
        for host in &s.hosts {
            exp.add_row(&ActionRow::builder().title(host).build());
        }
        group.add(&exp);
    }

    group
}

fn nfs_mapping_group(s: &NfsShare) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("User Mapping");

    let fields: &[(&str, Option<&str>)] = &[
        ("Map Root User", s.maproot_user.as_deref()),
        ("Map Root Group", s.maproot_group.as_deref()),
        ("Map All User", s.mapall_user.as_deref()),
        ("Map All Group", s.mapall_group.as_deref()),
    ];

    for (label, value) in fields {
        if let Some(v) = value {
            if !v.is_empty() {
                let row = ActionRow::builder().title(*label).build();
                row.add_suffix(&right_label(v, &["caption"]));
                group.add(&row);
            }
        }
    }

    group
}

fn nfs_security_group(s: &NfsShare) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Security Flavors");

    for flavor in &s.security {
        let row = ActionRow::builder().title(flavor).build();
        row.set_icon_name(Some("emblem-system-symbolic"));
        group.add(&row);
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

fn bool_label(value: bool) -> gtk4::Label {
    pill_label(
        if value { "Yes" } else { "No" },
        &[if value { "success" } else { "dim-label" }, "caption"],
    )
}

fn spacer(height: i32) -> gtk4::Box {
    let b = gtk4::Box::new(Orientation::Vertical, 0);
    b.set_height_request(height);
    b
}