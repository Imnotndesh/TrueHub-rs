use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: u32,
    pub method: String,
    pub params: Vec<serde_json::Value>,
}

impl RpcRequest {
    pub fn new(id: u32, method: impl Into<String>, params: Vec<serde_json::Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.into(),
            params,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub id: Option<u32>,
    pub result: Option<serde_json::Value>,
    pub error: Option<RpcError>,
    pub method: Option<String>,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}