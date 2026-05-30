pub struct Auth;

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

pub struct Connection;

impl Connection {
    pub const PING: &'static str = "core.ping";
}