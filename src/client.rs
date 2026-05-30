use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use dashmap::DashMap;
use tokio::task;
use tokio::sync::oneshot;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use crate::models::connection::{RpcRequest, RpcResponse, RpcError};
use crate::result::ApiResult;
use rustls::client::danger::{ServerCertVerified, ServerCertVerifier, HandshakeSignatureValid};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::DigitallySignedStruct;

type WsSink = futures_util::stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type PendingMap = Arc<DashMap<u32, oneshot::Sender<RpcResponse>>>;

#[derive(Debug)]
struct NoCertVerifier;

impl ServerCertVerifier for NoCertVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer,
        _intermediates: &[CertificateDer],
        _server_name: &ServerName,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        rustls::crypto::ring::default_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

#[derive(Debug)]
pub struct TrueNasRpcError {
    pub code: i32,
    pub message: String,
}

impl std::fmt::Display for TrueNasRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RPC Error {}: {}", self.code, self.message)
    }
}
impl std::error::Error for TrueNasRpcError {}

pub struct TrueNasClient {
    pub server_url: String,
    pub insecure: bool,
    id_counter: Arc<AtomicU32>,
    pending_requests: PendingMap,
    sender: Arc<Mutex<Option<WsSink>>>,
    connection_state: Arc<tokio::sync::watch::Sender<ConnectionState>>,
}

impl TrueNasClient {
    pub fn new(server_url: impl Into<String>, insecure: bool) -> Self {
        let (state_tx, _) = tokio::sync::watch::channel(ConnectionState::Disconnected);
        Self {
            server_url: server_url.into(),
            insecure,
            id_counter: Arc::new(AtomicU32::new(1)),
            pending_requests: Arc::new(DashMap::new()),
            sender: Arc::new(Mutex::new(None)),
            connection_state: Arc::new(state_tx),
        }
    }

    pub fn state_receiver(&self) -> tokio::sync::watch::Receiver<ConnectionState> {
        self.connection_state.subscribe()
    }

    pub async fn connect(&self) -> bool {
        let _ = rustls::crypto::ring::default_provider().install_default();
        let _ = self.connection_state.send(ConnectionState::Connecting);

        let result = if self.insecure {
            let config = rustls::ClientConfig::builder()
                .dangerous()
                .with_custom_certificate_verifier(Arc::new(NoCertVerifier))
                .with_no_client_auth();
            let connector = tokio_tungstenite::Connector::Rustls(Arc::new(config));
            tokio_tungstenite::connect_async_tls_with_config(
                &self.server_url,
                None,
                false,
                Some(connector),
            ).await
        } else {
            connect_async(&self.server_url).await
        };

        match result {
            Ok((ws_stream, _)) => {
                let (sink, mut stream) = ws_stream.split();
                *self.sender.lock().await = Some(sink);
                let _ = self.connection_state.send(ConnectionState::Connected);

                let pending = Arc::clone(&self.pending_requests);
                let state_tx = Arc::clone(&self.connection_state);
                task::spawn(async move {
                    while let Some(msg) = stream.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                Self::handle_message(&text, &pending);
                            }
                            Err(e) => {
                                let _ = state_tx.send(ConnectionState::Error(e.to_string()));
                                break;
                            }
                            _ => {}
                        }
                    }
                    let _ = state_tx.send(ConnectionState::Disconnected);
                });

                true
            }
            Err(e) => {
                let _ = self.connection_state.send(ConnectionState::Error(e.to_string()));
                false
            }
        }
    }

    fn handle_message(text: &str, pending: &PendingMap) {
        let Ok(resp) = serde_json::from_str::<RpcResponse>(text) else {
            return;
        };
        if resp.id.is_none() {
            return;
        }
        if let Some(id) = resp.id {
            if let Some((_, tx)) = pending.remove(&id) {
                let _ = tx.send(resp);
            }
        }
    }

    pub async fn call<T>(&self, method: &str, params: Vec<serde_json::Value>) -> ApiResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let id = self.id_counter.fetch_add(1, Ordering::SeqCst);
        let request = RpcRequest::new(id, method, params);

        let (tx, rx) = oneshot::channel::<RpcResponse>();
        self.pending_requests.insert(id, tx);

        let json = match serde_json::to_string(&request) {
            Ok(j) => j,
            Err(e) => return ApiResult::Error { message: e.to_string() },
        };

        {
            let mut sender = self.sender.lock().await;
            match sender.as_mut() {
                Some(s) => {
                    if s.send(Message::Text(json.into())).await.is_err() {
                        self.pending_requests.remove(&id);
                        return ApiResult::Error { message: "Failed to send message".to_string() };
                    }
                }
                None => {
                    self.pending_requests.remove(&id);
                    return ApiResult::Error { message: "Not connected".to_string() };
                }
            }
        }

        match rx.await {
            Ok(resp) => {
                if let Some(err) = resp.error {
                    return ApiResult::Error { message: self.extract_error_message(&err) };
                }
                match resp.result {
                    Some(val) => match serde_json::from_value::<T>(val) {
                        Ok(data) => ApiResult::Success(data),
                        Err(e) => ApiResult::Error { message: e.to_string() },
                    },
                    None => ApiResult::Error { message: "Null result".to_string() },
                }
            }
            Err(_) => ApiResult::Error { message: "Response channel dropped".to_string() },
        }
    }

    fn extract_error_message(&self, err: &RpcError) -> String {
        if let Some(data) = &err.data {
            if let Some(reason) = data.get("reason").and_then(|r| r.as_str()) {
                return reason.to_string();
            }
        }
        err.message.clone()
    }

    pub async fn disconnect(&self) {
        if let Some(mut sink) = self.sender.lock().await.take() {
            let _ = sink.close().await;
        }
        let _ = self.connection_state.send(ConnectionState::Disconnected);
        self.pending_requests.clear();
    }
}