use crate::models::{KnowledgeInput, Result, WorkNoteError};
use chrono::Local;
use std::fs;
use std::path::PathBuf;

/// FileGenerator - Markdownファイル生成を管理
pub struct FileGenerator {
    repository_path: PathBuf,
    save_path: String,
    author_name: String,
}

impl FileGenerator {
    /// 新しいFileGeneratorインスタンスを作成
    ///
    /// # Arguments
    /// * `repository_path` - Gitリポジトリパス
    /// * `save_path` - ナレッジ保存先パス（リポジトリルートからの相対パス）
    /// * `author_name` - Author名
    pub fn new(repository_path: PathBuf, save_path: String, author_name: String) -> Self {
        FileGenerator {
            repository_path,
            save_path,
            author_name,
        }
    }

    /// YAML文字列をエスケープ
    ///
    /// ダブルクォート、バックスラッシュ、改行をエスケープします。
    ///
    /// # Arguments
    /// * `s` - エスケープする文字列
    ///
    /// # Returns
    /// エスケープされた文字列
    fn escape_yaml_string(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    }

    /// タイトルをkebab-caseに変換
    ///
    /// ASCII文字（英数字）のみを残し、スペースと記号をハイフンに置換します。
    /// 日本語文字は除去されます。
    ///
    /// # Arguments
    /// * `title` - 変換するタイトル
    ///
    /// # Returns
    /// kebab-case形式のファイル名
    pub fn to_kebab_case(title: &str) -> String {
        // 小文字に変換
        let lowercase = title.to_lowercase();

        // ASCII英数字のみを残し、その他はハイフンに置換
        let replaced = lowercase
            .chars()
            .filter_map(|c| {
                if c.is_ascii_alphanumeric() {
                    Some(c)
                } else if c.is_whitespace() || c.is_ascii_punctuation() {
                    Some('-')
                } else {
                    // 非ASCII文字（日本語等）は除去
                    None
                }
            })
            .collect::<String>();

        // 連続するハイフンを1つにまとめる
        let mut result = String::new();
        let mut prev_was_hyphen = false;

        for c in replaced.chars() {
            if c == '-' {
                if !prev_was_hyphen {
                    result.push(c);
                }
                prev_was_hyphen = true;
            } else {
                result.push(c);
                prev_was_hyphen = false;
            }
        }

        // 先頭と末尾のハイフンを削除
        let trimmed = result.trim_matches('-').to_string();

        // 結果が空の場合はタイムスタンプを返す
        if trimmed.is_empty() {
            Local::now().format("%Y%m%d-%H%M%S").to_string()
        } else {
            trimmed
        }
    }

    /// Markdownファイルを生成（プレビュー用：author情報なし）
    ///
    /// # Arguments
    /// * `input` - ナレッジ入力データ
    ///
    /// # Returns
    /// Markdown形式の文字列
    pub fn generate_markdown_for_preview(input: &KnowledgeInput) -> String {
        let today = Local::now().format("%Y-%m-%d").to_string();

        let mut content = String::new();

        // Frontmatter
        content.push_str("---\n");
        content.push_str(&format!("title: \"{}\"\n", Self::escape_yaml_string(&input.title)));
        content.push_str(&format!("category: {}\n", input.category.as_str()));
        content.push_str(&format!("severity: {}\n", input.severity.as_str()));
        content.push_str("symptoms:\n");
        content.push_str(&format!("  - \"{}\"\n", Self::escape_yaml_string(&input.symptoms)));
        content.push_str("related_alerts: []\n");
        content.push_str(&format!("last_updated: {}\n", today));
        content.push_str("---\n\n");

        // タイトル
        content.push_str(&format!("# {}\n\n", input.title));

        // 概要
        content.push_str("## 概要\n\n");
        content.push_str(&format!("{}\n\n", input.symptoms));

        // 症状・検知条件
        content.push_str("## 症状・検知条件\n\n");
        content.push_str(&format!("{}\n\n", input.symptoms));

        // 対応手順
        content.push_str("## 対応手順\n\n");
        content.push_str(&format!("{}\n\n", input.procedure));

        // 注意点・落とし穴
        content.push_str("## 注意点・落とし穴\n\n");
        if let Some(notes) = &input.notes {
            content.push_str(&format!("{}\n\n", notes));
        } else {
            content.push_str("\n");
        }

        // 関連リンク
        content.push_str("## 関連リンク\n\n");
        if let Some(links) = &input.related_links {
            content.push_str(&format!("{}\n\n", links));
        } else {
            content.push_str("\n");
        }

        // 判断基準・判断軸（カテゴリ別）
        if let Some(judgment) = &input.judgment {
            if !judgment.is_empty() {
                content.push_str("## 判断基準・判断軸\n\n");
                content.push_str(&format!("{}\n\n", judgment));
            }
        }

        content
    }

    /// Markdownファイルを生成
    ///
    /// # Arguments
    /// * `input` - ナレッジ入力データ
    ///
    /// # Returns
    /// Markdown形式の文字列
    pub fn generate_markdown(&self, input: &KnowledgeInput) -> String {
        let today = Local::now().format("%Y-%m-%d").to_string();

        let mut content = String::new();

        // Frontmatter
        content.push_str("---\n");
        content.push_str(&format!("title: \"{}\"\n", Self::escape_yaml_string(&input.title)));
        content.push_str(&format!("category: {}\n", input.category.as_str()));
        content.push_str(&format!("severity: {}\n", input.severity.as_str()));
        content.push_str("symptoms:\n");
        content.push_str(&format!("  - \"{}\"\n", Self::escape_yaml_string(&input.symptoms)));
        content.push_str("related_alerts: []\n");
        content.push_str(&format!("last_updated: {}\n", today));
        content.push_str(&format!("author: \"{}\"\n", Self::escape_yaml_string(&self.author_name)));
        content.push_str("---\n\n");

        // タイトル
        content.push_str(&format!("# {}\n\n", input.title));

        // 概要
        content.push_str("## 概要\n\n");
        content.push_str(&format!("{}\n\n", input.symptoms));

        // 症状・検知条件
        content.push_str("## 症状・検知条件\n\n");
        content.push_str(&format!("{}\n\n", input.symptoms));

        // 対応手順
        content.push_str("## 対応手順\n\n");
        content.push_str(&format!("{}\n\n", input.procedure));

        // 注意点・落とし穴
        content.push_str("## 注意点・落とし穴\n\n");
        if let Some(notes) = &input.notes {
            content.push_str(&format!("{}\n\n", notes));
        } else {
            content.push_str("\n");
        }

        // 関連リンク
        content.push_str("## 関連リンク\n\n");
        if let Some(links) = &input.related_links {
            content.push_str(&format!("{}\n\n", links));
        } else {
            content.push_str("\n");
        }

        // 判断基準・判断軸（カテゴリ別）
        if let Some(judgment) = &input.judgment {
            if !judgment.is_empty() {
                content.push_str("## 判断基準・判断軸\n\n");
                content.push_str(&format!("{}\n\n", judgment));
            }
        }

        // 対応履歴
        content.push_str("## 対応履歴\n\n");
        content.push_str("| 日付 | 対応者 | 備考 |\n");
        content.push_str("|------|--------|------|\n");
        content.push_str(&format!("| {} | {} | 初版作成 |\n", today, self.author_name));

        content
    }

    /// ファイルを書き込む
    ///
    /// # Arguments
    /// * `input` - ナレッジ入力データ
    ///
    /// # Returns
    /// 作成されたファイルのパス
    pub fn write_file(&self, input: &KnowledgeInput) -> Result<PathBuf> {
        // ファイル名生成
        let base_filename = Self::to_kebab_case(&input.title);
        let filename = format!("{}.md", base_filename);

        // カテゴリディレクトリパス
        let category_dir = self
            .repository_path
            .join(&self.save_path)
            .join(input.category.as_str());

        // ディレクトリトラバーサル対策: repository_path 内に収まるか確認
        // まず必要なディレクトリを作成
        let save_dir = self.repository_path.join(&self.save_path);
        if !save_dir.exists() {
            fs::create_dir_all(&save_dir).map_err(|e| {
                WorkNoteError::FileError(format!("Failed to create save directory: {}", e))
            })?;
        }

        // repository_pathとcategory_dirをcanonicalizeして比較
        // repository_pathが存在しない場合は作成
        if !self.repository_path.exists() {
            fs::create_dir_all(&self.repository_path).map_err(|e| {
                WorkNoteError::FileError(format!("Failed to create repository directory: {}", e))
            })?;
        }

        let canonical_repo = self.repository_path.canonicalize().map_err(|e| {
            WorkNoteError::FileError(format!("Failed to canonicalize repository path: {}", e))
        })?;

        let canonical_category = category_dir.canonicalize().unwrap_or_else(|_| {
            // まだ存在しない場合は、親ディレクトリをベースに構築
            save_dir
                .canonicalize()
                .unwrap_or(save_dir.clone())
                .join(input.category.as_str())
        });

        // リポジトリパス外への書き込みを防止
        if !canonical_category.starts_with(&canonical_repo) {
            return Err(WorkNoteError::FileError(
                "Invalid save path: directory traversal detected".to_string(),
            ));
        }

        // ディレクトリが存在しない場合は作成
        if !category_dir.exists() {
            fs::create_dir_all(&category_dir).map_err(|e| {
                WorkNoteError::FileError(format!("Failed to create directory: {}", e))
            })?;
        }

        // ファイルパス
        let mut file_path = category_dir.join(&filename);

        // ファイル名衝突対応
        if file_path.exists() {
            let timestamp = Local::now().format("%Y%m%d-%H%M%S-%f").to_string();
            let collision_filename = format!("{}-{}.md", base_filename, timestamp);
            file_path = category_dir.join(&collision_filename);
        }

        // Markdown生成
        let markdown = self.generate_markdown(input);

        // ファイル書き込み
        fs::write(&file_path, markdown).map_err(|e| {
            WorkNoteError::FileError(format!("Failed to write file: {}", e))
        })?;

        Ok(file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Category, Severity};
    use std::env;

    #[test]
    fn test_to_kebab_case_english() {
        assert_eq!(FileGenerator::to_kebab_case("CPU High Alert"), "cpu-high-alert");
        assert_eq!(FileGenerator::to_kebab_case("Network Error"), "network-error");
        assert_eq!(FileGenerator::to_kebab_case("Database Connection"), "database-connection");
    }

    #[test]
    fn test_to_kebab_case_japanese() {
        // 日本語文字は除去され、ASCII文字のみが残る
        let result = FileGenerator::to_kebab_case("CPU高騰対応");
        assert_eq!(result, "cpu");
    }

    #[test]
    fn test_to_kebab_case_mixed() {
        let result = FileGenerator::to_kebab_case("Test テスト 123");
        // 英数字とハイフンのみ
        assert!(result.chars().all(|c| c.is_alphanumeric() || c == '-'));
        assert!(result.contains("test"));
        assert!(result.contains("123"));
    }

    #[test]
    fn test_to_kebab_case_special_chars() {
        assert_eq!(FileGenerator::to_kebab_case("Test@#$%123"), "test-123");
        assert_eq!(FileGenerator::to_kebab_case("Hello___World"), "hello-world");
    }

    #[test]
    fn test_generate_markdown() {
        let generator = FileGenerator::new(
            PathBuf::from("/test/repo"),
            "docs/runbooks".to_string(),
            "Test User".to_string(),
        );

        let input = KnowledgeInput {
            title: "CPU高騰対応".to_string(),
            category: Category::Alerts,
            severity: Severity::High,
            symptoms: "CPU使用率が90%を超えている".to_string(),
            procedure: "1. プロセス一覧を確認\n2. 原因プロセスを特定\n3. 必要に応じて再起動".to_string(),
            notes: Some("再起動前にログを確認すること".to_string()),
            related_links: Some("https://example.com/cpu-troubleshooting".to_string()),
            judgment: None,
        };

        let markdown = generator.generate_markdown(&input);

        // Frontmatter確認
        assert!(markdown.contains("---"));
        assert!(markdown.contains("title: \"CPU高騰対応\""));
        assert!(markdown.contains("category: alerts"));
        assert!(markdown.contains("severity: high"));
        assert!(markdown.contains("author: \"Test User\""));

        // セクション確認
        assert!(markdown.contains("# CPU高騰対応"));
        assert!(markdown.contains("## 概要"));
        assert!(markdown.contains("## 症状・検知条件"));
        assert!(markdown.contains("## 対応手順"));
        assert!(markdown.contains("## 注意点・落とし穴"));
        assert!(markdown.contains("## 関連リンク"));
        assert!(markdown.contains("## 対応履歴"));

        // 内容確認
        assert!(markdown.contains("CPU使用率が90%を超えている"));
        assert!(markdown.contains("プロセス一覧を確認"));
        assert!(markdown.contains("再起動前にログを確認すること"));
        assert!(markdown.contains("https://example.com/cpu-troubleshooting"));
    }

    #[test]
    fn test_write_file() {
        let temp_dir = env::temp_dir().join("worknote_test_filegen");
        let generator = FileGenerator::new(
            temp_dir.clone(),
            "docs/runbooks".to_string(),
            "Test User".to_string(),
        );

        let input = KnowledgeInput {
            title: "Test Knowledge".to_string(),
            category: Category::Alerts,
            severity: Severity::Medium,
            symptoms: "Test symptom".to_string(),
            procedure: "Test procedure".to_string(),
            notes: None,
            related_links: None,
            judgment: None,
        };

        let file_path = generator.write_file(&input).unwrap();

        // ファイルが作成されたことを確認
        assert!(file_path.exists());
        assert!(file_path.to_str().unwrap().ends_with("test-knowledge.md"));

        // ファイル内容確認
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("# Test Knowledge"));
        assert!(content.contains("Test symptom"));

        // クリーンアップ
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_write_file_collision() {
        let temp_dir = env::temp_dir().join("worknote_test_collision");
        let generator = FileGenerator::new(
            temp_dir.clone(),
            "docs/runbooks".to_string(),
            "Test User".to_string(),
        );

        let input = KnowledgeInput {
            title: "Collision Test".to_string(),
            category: Category::Maintenance,
            severity: Severity::Low,
            symptoms: "Test".to_string(),
            procedure: "Test".to_string(),
            notes: None,
            related_links: None,
            judgment: None,
        };

        // 1回目: 通常のファイル名
        let file_path1 = generator.write_file(&input).unwrap();
        assert!(file_path1.to_str().unwrap().contains("collision"));
        assert_eq!(file_path1.extension().unwrap(), "md");

        // 2回目: タイムスタンプ付きファイル名
        let file_path2 = generator.write_file(&input).unwrap();
        assert!(file_path2.to_str().unwrap().contains("collision-test-"));
        assert_eq!(file_path2.extension().unwrap(), "md");

        // 両方のファイルが存在することを確認
        assert!(file_path1.exists());
        assert!(file_path2.exists());
        assert_ne!(file_path1, file_path2);

        // クリーンアップ
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
