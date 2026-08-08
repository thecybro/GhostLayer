use serde::Serialize;

#[derive(Serialize)]
pub struct StorageWrite {
    pub key: String,
    pub value: String
}

#[derive(Serialize)]
pub struct FunctionResult {
    pub success: bool,
    pub username: Option<String>,
    pub error: Option<String>,
    pub display: String, // info to show to user, eg: key_id
    pub write: Vec<StorageWrite>,
}


#[derive(Serialize)]
pub struct CryptoFunctionResult {
    pub success: bool,
    pub error: Option<String>,
    pub nonce: Option<String>,
    pub display: String,
    pub message_key: Option<String>,
}

