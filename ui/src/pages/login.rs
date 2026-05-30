use gtk4::prelude::*;
use gtk4::{Stack, Align, Orientation};
use libadwaita::prelude::*;
use libadwaita::{
    EntryRow, PasswordEntryRow, PreferencesGroup,
    PreferencesPage, SwitchRow, ViewStack, ViewSwitcher,
    ViewSwitcherPolicy, Banner,
};
use gtk4::Spinner;
use glib::clone;
use api::manager::TrueNasApiManager;
use api::models::auth::TokenRequest;
use api::result::ApiResult;
use api::store::{accounts, prefs};
use api::store::accounts::{
    LoginMethod, SavedAccount, SavedServer,
    save_account, save_server, save_current_session,
    save_last_used_profile, save_account_credentials,
};
use api::store::prefs::AppPrefs;
use crate::state::AppState;
use crate::runtime;

pub enum LoginResult {
    Success {
        manager: TrueNasApiManager,
        token: String,
        username: String,
    },
    Error(String),
}

pub fn build(state: AppState, root_stack: Stack) -> libadwaita::ToolbarView {
    let toolbar_view = libadwaita::ToolbarView::new();

    let header = libadwaita::HeaderBar::new();
    header.set_title_widget(Some(
        &gtk4::Label::builder()
            .label("Sign In to TrueHub")
            .css_classes(vec!["title"])
            .build()
    ));
    toolbar_view.add_top_bar(&header);

    let banner = Banner::new("");
    banner.set_revealed(false);
    toolbar_view.add_top_bar(&banner);

    let mode_stack = ViewStack::new();
    let switcher = ViewSwitcher::builder()
        .stack(&mode_stack)
        .policy(ViewSwitcherPolicy::Wide)
        .halign(Align::Center)
        .build();

    let pw_page = PreferencesPage::new();
    let server_group = PreferencesGroup::new();
    server_group.set_title("Server");

    let server_entry = EntryRow::new();
    server_entry.set_title("Server URL");
    server_entry.set_show_apply_button(false);
    server_entry.set_text("wss://");

    let insecure_row = SwitchRow::new();
    insecure_row.set_title("Allow self-signed certificate");
    insecure_row.set_subtitle("Enable for TrueNAS with self-signed TLS");
    insecure_row.set_active(true);

    server_group.add(&server_entry);
    server_group.add(&insecure_row);
    pw_page.add(&server_group);

    let creds_group = PreferencesGroup::new();
    creds_group.set_title("Credentials");

    let username_entry = EntryRow::new();
    username_entry.set_title("Username");

    let password_entry = PasswordEntryRow::new();
    password_entry.set_title("Password");

    creds_group.add(&username_entry);
    creds_group.add(&password_entry);
    pw_page.add(&creds_group);

    mode_stack.add_titled_with_icon(
        &pw_page, Some("password"), "Password", "dialog-password-symbolic"
    );

    // --- API Key page ---
    let api_page = PreferencesPage::new();
    let api_server_group = PreferencesGroup::new();
    api_server_group.set_title("Server");

    let api_server_entry = EntryRow::new();
    api_server_entry.set_title("Server URL");
    api_server_entry.set_text("wss://");

    let api_insecure_row = SwitchRow::new();
    api_insecure_row.set_title("Allow self-signed certificate");
    api_insecure_row.set_subtitle("Enable for TrueNAS with self-signed TLS");
    api_insecure_row.set_active(true);

    api_server_group.add(&api_server_entry);
    api_server_group.add(&api_insecure_row);
    api_page.add(&api_server_group);

    let api_key_group = PreferencesGroup::new();
    api_key_group.set_title("API Key");

    let api_key_entry = PasswordEntryRow::new();
    api_key_entry.set_title("API Key");

    api_key_group.add(&api_key_entry);
    api_page.add(&api_key_group);

    mode_stack.add_titled_with_icon(
        &api_page, Some("apikey"), "API Key", "key-symbolic"
    );

    let content = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(24)
        .build();

    content.append(&switcher);
    content.append(&mode_stack);

    let btn_box = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(Align::Center)
        .margin_top(8)
        .build();

    let spinner = Spinner::new();
    spinner.set_visible(false);

    let login_btn = gtk4::Button::builder()
        .label("Sign In")
        .css_classes(vec!["suggested-action", "pill"])
        .build();

    btn_box.append(&spinner);
    btn_box.append(&login_btn);
    content.append(&btn_box);

    toolbar_view.set_content(Some(&content));

    let fill_server = server_entry.clone();
    let fill_api_server = api_server_entry.clone();
    let saved = prefs::load_sync();
    if let Some(url) = saved.server_url {
        fill_server.set_text(&url);
        fill_api_server.set_text(&url);
    }


    login_btn.connect_clicked(clone!(
        #[strong] state,
        #[strong] root_stack,
        #[strong] mode_stack,
        #[strong] server_entry,
        #[strong] username_entry,
        #[strong] password_entry,
        #[strong] api_server_entry,
        #[strong] api_key_entry,
        #[strong] insecure_row,
        #[strong] api_insecure_row,
        #[strong] banner,
        #[strong] spinner,
        #[strong] login_btn,
        move |_| {
            let mode = mode_stack.visible_child_name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "password".to_string());

            let (server_url, insecure) = if mode == "apikey" {
                (api_server_entry.text().to_string(), api_insecure_row.is_active())
            } else {
                (server_entry.text().to_string(), insecure_row.is_active())
            };

            let clean_url = normalize_server_url(&server_url);

            if clean_url.is_empty() {
                banner.set_title("Please enter a server URL");
                banner.set_revealed(true);
                return;
            }

            let username = username_entry.text().to_string();
            let password = password_entry.text().to_string();
            let api_key = api_key_entry.text().to_string();

            spinner.set_visible(true);
            spinner.start();
            login_btn.set_sensitive(false);
            banner.set_revealed(false);

            let (tx, rx) = async_channel::unbounded::<LoginResult>();

            glib::MainContext::default().spawn_local(clone!(
                #[strong] state,
                #[strong] root_stack,
                #[strong] banner,
                #[strong] spinner,
                #[strong] login_btn,
                async move {
                    if let Ok(result) = rx.recv().await {
                        spinner.stop();
                        spinner.set_visible(false);
                        login_btn.set_sensitive(true);

                        match result {
                            LoginResult::Success { manager, token, username } => {
                                state.set_manager(manager);
                                state.set_token(token);
                                state.set_username(username);
                                root_stack.set_visible_child_name("home");
                            }
                            LoginResult::Error(msg) => {
                                banner.set_title(&msg);
                                banner.set_revealed(true);
                            }
                        }
                    }
                }
            ));

            runtime::spawn(
                do_login(clean_url, insecure, mode, username, password, api_key),
                tx,
            );
        }
    ));

    toolbar_view
}

async fn do_login(
    server_url: String,
    insecure: bool,
    mode: String,
    username: String,
    password: String,
    api_key: String,
) -> LoginResult {
    let _ = prefs::save_sync(&AppPrefs {
        server_url: Some(server_url.clone()),
        insecure,
        enable_debug_logging: false,
    });

    let manager = TrueNasApiManager::new(&server_url, insecure);

    if !manager.connect().await {
        return LoginResult::Error("Failed to connect to server".to_string());
    }

    let login_result = if mode == "apikey" {
        manager.auth.login_with_api_key(&api_key).await
    } else {
        manager.auth.login(&username, &password).await
    };

    match login_result {
        ApiResult::Success(true) => {
            match manager.auth.generate_token(TokenRequest::default()).await {
                ApiResult::Success(token) => {
                    let server = SavedServer::new(server_url.clone(), insecure, None);
                    let _ = save_server(server.clone()).await;

                    let login_method = if mode == "apikey" {
                        LoginMethod::ApiKey
                    } else {
                        LoginMethod::Password
                    };

                    let account_username = if mode == "apikey" {
                        "api_key_user".to_string()
                    } else {
                        username.clone()
                    };

                    let mut account = SavedAccount::new(
                        server.id.clone(),
                        account_username.clone(),
                        login_method.clone(),
                    );
                    account.auto_login_enabled = true;
                    let _ = save_account(account.clone()).await;

                    let _ = save_account_credentials(
                        &account.id,
                        &login_method,
                        if mode == "apikey" { Some(&api_key) } else { None },
                        if mode != "apikey" { Some(&username) } else { None },
                        if mode != "apikey" { Some(&password) } else { None },
                    );

                    let _ = save_current_session(&server.id, &account.id, &token).await;
                    let _ = save_last_used_profile(&server.id, &account.id).await;

                    LoginResult::Success { manager, token, username: account_username }
                }
                ApiResult::Error { message } => {
                    LoginResult::Error(format!("Token error: {message}"))
                }
                _ => LoginResult::Error("Unexpected token state".to_string()),
            }
        }
        ApiResult::Success(false) => LoginResult::Error("Invalid credentials".to_string()),
        ApiResult::Error { message } => LoginResult::Error(format!("Login failed: {message}")),
        _ => LoginResult::Error("Unexpected state".to_string()),
    }
}

fn normalize_server_url(input: &str) -> String {
    let trimmed = input.trim().trim_end_matches('/');
    if trimmed.is_empty() { return String::new(); }

    let base = trimmed.strip_suffix("/api/current").unwrap_or(trimmed);

    let base = if base.starts_with("ws://") || base.starts_with("wss://") {
        base.to_string()
    } else if base.starts_with("https://") {
        base.replacen("https://", "wss://", 1)
    } else if base.starts_with("http://") {
        base.replacen("http://", "ws://", 1)
    } else {
        format!("wss://{base}")
    };
    format!("{base}/api/current")
}