use tracing_subscriber::EnvFilter;
use std::sync::OnceLock;

static LOGGING_ENABLED: OnceLock<bool> = OnceLock::new();

pub fn init(enable_debug: bool) {
    LOGGING_ENABLED.get_or_init(|| enable_debug);

    if enable_debug {
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::new("truehub_api=debug"))
            )
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .init();
    }
}

pub fn is_enabled() -> bool {
    *LOGGING_ENABLED.get().unwrap_or(&false)
}

#[macro_export]
macro_rules! log_debug {
    ($tag:expr, $($arg:tt)*) => {
        if $crate::logger::is_enabled() {
            tracing::debug!(tag = $tag, $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($tag:expr, $($arg:tt)*) => {
        if $crate::logger::is_enabled() {
            tracing::error!(tag = $tag, $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($tag:expr, $($arg:tt)*) => {
        if $crate::logger::is_enabled() {
            tracing::info!(tag = $tag, $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! log_warn {
    ($tag:expr, $($arg:tt)*) => {
        if $crate::logger::is_enabled() {
            tracing::warn!(tag = $tag, $($arg)*);
        }
    };
}