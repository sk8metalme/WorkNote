use serde::{Deserialize, Serialize};

/// 一括添削リクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProofreadRequest {
    pub symptoms: String,
    pub procedure: String,
    pub notes: Option<String>,
}

/// 一括添削レスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProofreadResponse {
    pub symptoms: String,
    pub procedure: String,
    pub notes: Option<String>,
}
