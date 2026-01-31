use serde::{Deserialize, Serialize};

/// ナレッジ保存レスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveKnowledgeResponse {
    pub commit_hash: String,
    pub file_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr_url: Option<String>,
}
