use crate::models::knowledge::KnowledgeInput;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 下書きデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Draft {
    pub id: String,
    pub data: KnowledgeInput,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Draft {
    /// 新しいDraftを作成
    pub fn new(data: KnowledgeInput) -> Self {
        let now = Utc::now();
        let id = uuid::Uuid::new_v4().to_string();

        Self {
            id,
            data,
            created_at: now,
            updated_at: now,
        }
    }

    /// Draftを更新
    pub fn update(&mut self, data: KnowledgeInput) {
        self.data = data;
        self.updated_at = Utc::now();
    }
}

/// 下書きサマリー（一覧表示用）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftSummary {
    pub id: String,
    pub title: String,
    pub category: String,
    pub updated_at: DateTime<Utc>,
}

impl From<&Draft> for DraftSummary {
    fn from(draft: &Draft) -> Self {
        Self {
            id: draft.id.clone(),
            title: draft.data.title.clone(),
            category: draft.data.category.as_str().to_string(),
            updated_at: draft.updated_at,
        }
    }
}
