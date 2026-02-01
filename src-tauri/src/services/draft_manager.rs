use crate::models::{Draft, DraftSummary, KnowledgeInput, Result, WorkNoteError};
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};

/// DraftManager - 下書きの保存・読み込みを管理
pub struct DraftManager {
    drafts_dir: PathBuf,
}

impl DraftManager {
    /// 新しいDraftManagerインスタンスを作成
    ///
    /// # Arguments
    /// * `app_data_dir` - アプリケーションデータディレクトリ
    pub fn new(app_data_dir: PathBuf) -> Self {
        let drafts_dir = app_data_dir.join("drafts");

        DraftManager { drafts_dir }
    }

    /// draftsディレクトリが存在することを確認（なければ作成）
    fn ensure_drafts_dir(&self) -> Result<()> {
        if !self.drafts_dir.exists() {
            fs::create_dir_all(&self.drafts_dir).map_err(|e| {
                WorkNoteError::FileError(format!("Failed to create drafts directory: {}", e))
            })?;
        }
        Ok(())
    }

    /// 下書きを保存
    ///
    /// # Arguments
    /// * `draft` - 保存する下書き
    pub fn save_draft(&self, draft: &Draft) -> Result<()> {
        self.ensure_drafts_dir()?;

        let file_path = self.drafts_dir.join(format!("{}.json", draft.id));

        let json = serde_json::to_string_pretty(draft).map_err(|e| {
            WorkNoteError::FileError(format!("Failed to serialize draft: {}", e))
        })?;

        fs::write(&file_path, json).map_err(|e| {
            WorkNoteError::FileError(format!("Failed to write draft file: {}", e))
        })?;

        info!(draft_id = %draft.id, "Draft saved successfully");
        Ok(())
    }

    /// 下書きを読み込み
    ///
    /// # Arguments
    /// * `id` - 下書きID
    pub fn load_draft(&self, id: &str) -> Result<Draft> {
        let file_path = self.drafts_dir.join(format!("{}.json", id));

        if !file_path.exists() {
            return Err(WorkNoteError::FileError(format!(
                "Draft not found: {}",
                id
            )));
        }

        let json = fs::read_to_string(&file_path).map_err(|e| {
            WorkNoteError::FileError(format!("Failed to read draft file: {}", e))
        })?;

        let draft: Draft = serde_json::from_str(&json).map_err(|e| {
            WorkNoteError::FileError(format!("Failed to deserialize draft: {}", e))
        })?;

        info!(draft_id = %id, "Draft loaded successfully");
        Ok(draft)
    }

    /// 下書き一覧を取得
    pub fn list_drafts(&self) -> Result<Vec<DraftSummary>> {
        if !self.drafts_dir.exists() {
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(&self.drafts_dir).map_err(|e| {
            WorkNoteError::FileError(format!("Failed to read drafts directory: {}", e))
        })?;

        let mut summaries = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|e| {
                WorkNoteError::FileError(format!("Failed to read directory entry: {}", e))
            })?;

            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                match fs::read_to_string(&path) {
                    Ok(json) => match serde_json::from_str::<Draft>(&json) {
                        Ok(draft) => {
                            summaries.push(DraftSummary::from(&draft));
                        }
                        Err(e) => {
                            error!(file = ?path, error = %e, "Failed to deserialize draft");
                        }
                    },
                    Err(e) => {
                        error!(file = ?path, error = %e, "Failed to read draft file");
                    }
                }
            }
        }

        // 更新日時の降順でソート
        summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        info!(count = summaries.len(), "Listed drafts");
        Ok(summaries)
    }

    /// 下書きを削除
    ///
    /// # Arguments
    /// * `id` - 下書きID
    pub fn delete_draft(&self, id: &str) -> Result<()> {
        let file_path = self.drafts_dir.join(format!("{}.json", id));

        if !file_path.exists() {
            return Err(WorkNoteError::FileError(format!(
                "Draft not found: {}",
                id
            )));
        }

        fs::remove_file(&file_path).map_err(|e| {
            WorkNoteError::FileError(format!("Failed to delete draft file: {}", e))
        })?;

        info!(draft_id = %id, "Draft deleted successfully");
        Ok(())
    }

    /// 新しい下書きを作成して保存
    ///
    /// # Arguments
    /// * `data` - ナレッジ入力データ
    pub fn create_draft(&self, data: KnowledgeInput) -> Result<Draft> {
        let draft = Draft::new(data);
        self.save_draft(&draft)?;
        Ok(draft)
    }

    /// 既存の下書きを更新
    ///
    /// # Arguments
    /// * `id` - 下書きID
    /// * `data` - ナレッジ入力データ
    pub fn update_draft(&self, id: &str, data: KnowledgeInput) -> Result<Draft> {
        let mut draft = self.load_draft(id)?;
        draft.update(data);
        self.save_draft(&draft)?;
        Ok(draft)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Category, Severity};
    use std::env;

    #[test]
    fn test_save_and_load_draft() {
        let temp_dir = env::temp_dir().join("worknote_test_draft");
        let manager = DraftManager::new(temp_dir.clone());

        let input = KnowledgeInput {
            title: "Test Draft".to_string(),
            category: Category::Alerts,
            severity: Severity::High,
            symptoms: "Test symptoms".to_string(),
            procedure: "Test procedure".to_string(),
            notes: Some("Test notes".to_string()),
            related_links: None,
            judgment: None,
        };

        // 下書き作成
        let draft = manager.create_draft(input.clone()).unwrap();
        assert!(!draft.id.is_empty());

        // 下書き読み込み
        let loaded = manager.load_draft(&draft.id).unwrap();
        assert_eq!(loaded.id, draft.id);
        assert_eq!(loaded.data.title, "Test Draft");

        // クリーンアップ
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_list_drafts() {
        let temp_dir = env::temp_dir().join("worknote_test_draft_list");
        let manager = DraftManager::new(temp_dir.clone());

        let input1 = KnowledgeInput {
            title: "Draft 1".to_string(),
            category: Category::Alerts,
            severity: Severity::Medium,
            symptoms: "Symptoms 1".to_string(),
            procedure: "Procedure 1".to_string(),
            notes: None,
            related_links: None,
            judgment: None,
        };

        let input2 = KnowledgeInput {
            title: "Draft 2".to_string(),
            category: Category::Ops,
            severity: Severity::Low,
            symptoms: "Symptoms 2".to_string(),
            procedure: "Procedure 2".to_string(),
            notes: None,
            related_links: None,
            judgment: None,
        };

        manager.create_draft(input1).unwrap();
        manager.create_draft(input2).unwrap();

        let summaries = manager.list_drafts().unwrap();
        assert_eq!(summaries.len(), 2);

        // クリーンアップ
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_delete_draft() {
        let temp_dir = env::temp_dir().join("worknote_test_draft_delete");
        let manager = DraftManager::new(temp_dir.clone());

        let input = KnowledgeInput {
            title: "To Delete".to_string(),
            category: Category::Troubleshooting,
            severity: Severity::Critical,
            symptoms: "Symptoms".to_string(),
            procedure: "Procedure".to_string(),
            notes: None,
            related_links: None,
            judgment: None,
        };

        let draft = manager.create_draft(input).unwrap();
        manager.delete_draft(&draft.id).unwrap();

        let result = manager.load_draft(&draft.id);
        assert!(result.is_err());

        // クリーンアップ
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
