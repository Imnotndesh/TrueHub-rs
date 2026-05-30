use libadwaita::prelude::*;
use libadwaita::{Application, ApplicationWindow};
use gtk4::{Stack, StackTransitionType};
use crate::state::AppState;
use crate::pages::{login, home};
use crate::runtime;
use api::store::{accounts, prefs};
use api::store::accounts::{get_current_session, get_last_used_profile, get_server, get_account};
use api::manager::TrueNasApiManager;
use api::models::auth::TokenRequest;
use api::result::ApiResult;

pub enum StartupResult {
    NeedsLogin,
    NeedsCredentialLogin {
        manager: TrueNasApiManager,
        token: String,
        username: String,
    },
    Ready {
        manager: TrueNasApiManager,
        token: String,
        username: String,
    },
}

pub fn build_ui(app: &Application) {
    let state = AppState::new();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("TrueHub")
        .default_width(520)
        .default_height(640)
        .build();

    let stack = Stack::new();
    stack.set_transition_type(StackTransitionType::Crossfade);
    stack.set_transition_duration(300);

    let login_page = login::build(state.clone(), stack.clone());
    let home_page = home::build(state.clone(), stack.clone(), window.clone());

    stack.add_named(&login_page, Some("login"));
    stack.add_named(&home_page, Some("home"));
    stack.set_visible_child_name("login");

    window.set_content(Some(&stack));
    window.present();
    let (tx, rx) = async_channel::unbounded::<StartupResult>();

    glib::MainContext::default().spawn_local(glib::clone!(
        #[strong] state,
        #[strong] stack,
        async move {
            if let Ok(result) = rx.recv().await {
                match result {
                    StartupResult::NeedsLogin => {

                    }
                    StartupResult::NeedsCredentialLogin { manager, token, username } |
                    StartupResult::Ready { manager, token, username } => {
                        state.set_manager(manager);
                        state.set_token(token);
                        state.set_username(username);
                        stack.set_visible_child_name("home");
                    }
                }
            }
        }
    ));

    runtime::spawn(do_startup_check(), tx);
}

async fn do_startup_check() -> StartupResult {
    let prefs = prefs::load_sync();
    let server_url = match prefs.server_url {
        Some(url) => url,
        None => return StartupResult::NeedsLogin,
    };

    let session = accounts::get_current_session().await;

    if let Some((server_id, account_id, token)) = session {
        let server = get_server(&server_id).await;
        let account = get_account(&account_id).await;

        if let (Some(server), Some(account)) = (server, account) {
            let manager = TrueNasApiManager::new(&server.server_url, server.insecure);

            if manager.connect().await {
                match manager.auth.login_with_token(&token).await {
                    ApiResult::Success(true) => {
                        match manager.auth.generate_token(TokenRequest::default()).await {
                            ApiResult::Success(new_token) => {
                                let _ = accounts::save_current_session(
                                    &server_id, &account_id, &new_token
                                ).await;
                                return StartupResult::Ready {
                                    manager,
                                    token: new_token,
                                    username: account.username,
                                };
                            }
                            _ => {}
                        }
                    }
                    _ => {
                    }
                }
            }
        }
    }

    let profile = get_last_used_profile().await;

    if let Some((server_id, account_id)) = profile {
        let server = get_server(&server_id).await;
        let account = get_account(&account_id).await;

        if let (Some(server), Some(account)) = (server, account) {
            if account.auto_login_enabled {
                let (cred1, cred2) = accounts::get_account_credentials(
                    &account.id, &account.login_method
                );

                let manager = TrueNasApiManager::new(&server.server_url, server.insecure);

                if manager.connect().await {
                    let login_result = match account.login_method {
                        accounts::LoginMethod::ApiKey => {
                            if let Some(key) = cred1 {
                                manager.auth.login_with_api_key(&key).await
                            } else {
                                return StartupResult::NeedsLogin;
                            }
                        }
                        accounts::LoginMethod::Password => {
                            if let (Some(user), Some(pass)) = (cred1, cred2) {
                                manager.auth.login(&user, &pass).await
                            } else {
                                return StartupResult::NeedsLogin;
                            }
                        }
                    };

                    if let ApiResult::Success(true) = login_result {
                        match manager.auth.generate_token(TokenRequest::default()).await {
                            ApiResult::Success(token) => {
                                let _ = accounts::save_current_session(
                                    &server_id, &account_id, &token
                                ).await;
                                return StartupResult::NeedsCredentialLogin {
                                    manager,
                                    token,
                                    username: account.username,
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    StartupResult::NeedsLogin
}