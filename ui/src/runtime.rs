use std::sync::OnceLock;
use tokio::runtime::{Handle, Runtime};

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

pub fn init() {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to build Tokio runtime")
    });
}

pub fn spawn<F, T>(future: F, sender: async_channel::Sender<T>)
where
    F: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let rt = RUNTIME.get().expect("Runtime not initialized");
    rt.spawn(async move {
        let result = future.await;
        let _ = sender.send(result).await;
    });
}

pub fn handle() -> &'static Handle {
    RUNTIME.get().expect("Runtime not initialized").handle()
}