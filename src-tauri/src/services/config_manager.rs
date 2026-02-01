use crate::models::{Config, WorkNoteError, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// 設定ファイル名
const CONFIG_FILE_NAME: &str = "config.json";

/// ConfigManager - 設定ファイルの読み書きを管理
pub struct ConfigManager {
    config_dir: PathBuf,
}

impl ConfigManager {
    /// 新しいConfigManagerインスタンスを作成
    ///
    /// # Arguments
    /// * `app_data_dir` - アプリケーションデータディレクトリ
    pub fn new(app_data_dir: PathBuf) -> Self {
        ConfigManager {
            config_dir: app_data_dir,
        }
    }

    /// 設定ファイルのパスを取得
    fn config_path(&self) -> PathBuf {
        self.config_dir.join(CONFIG_FILE_NAME)
    }

    /// 設定を読み込む
    ///
    /// ファイルが存在しない場合はデフォルト値を返す
    pub fn load_config(&self) -> Result<Config> {
        let path = self.config_path();

        if !path.exists() {
            // ファイルが存在しない場合はデフォルト値を返す
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&path).map_err(|e| {
            WorkNoteError::ConfigError(format!("Failed to read config file: {}", e))
        })?;

        let config: Config = serde_json::from_str(&content).map_err(|e| {
            WorkNoteError::ConfigError(format!("Failed to parse config file: {}", e))
        })?;

        Ok(config)
    }

    /// 設定を保存する
    ///
    /// # Arguments
    /// * `config` - 保存する設定
    pub fn save_config(&self, config: &Config) -> Result<()> {
        // 設定バリデーション
        self.validate_config(config)?;

        // ディレクトリが存在しない場合は作成
        if !self.config_dir.exists() {
            fs::create_dir_all(&self.config_dir).map_err(|e| {
                WorkNoteError::ConfigError(format!("Failed to create config directory: {}", e))
            })?;
        }

        let path = self.config_path();
        let content = serde_json::to_string_pretty(config).map_err(|e| {
            WorkNoteError::ConfigError(format!("Failed to serialize config: {}", e))
        })?;

        fs::write(&path, content).map_err(|e| {
            WorkNoteError::ConfigError(format!("Failed to write config file: {}", e))
        })?;

        Ok(())
    }

    /// 設定をバリデーション
    ///
    /// # Arguments
    /// * `config` - バリデーションする設定
    fn validate_config(&self, config: &Config) -> Result<()> {
        // リポジトリパスが空でないことを確認
        if config.git.repository_path.is_empty() {
            return Err(WorkNoteError::ValidationError(
                "Repository path is empty".to_string(),
            ));
        }

        // リポジトリパスが存在することを確認
        let repo_path = Path::new(&config.git.repository_path);
        if !repo_path.exists() {
            return Err(WorkNoteError::ValidationError(format!(
                "Repository path does not exist: {}",
                config.git.repository_path
            )));
        }

        // .gitディレクトリが存在することを確認（Gitリポジトリであることを検証）
        let git_dir = repo_path.join(".git");
        if !git_dir.exists() {
            return Err(WorkNoteError::ValidationError(format!(
                "Not a Git repository: {}",
                config.git.repository_path
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn create_test_config_manager() -> ConfigManager {
        let temp_dir = env::temp_dir().join("worknote_test_config");
        ConfigManager::new(temp_dir)
    }

    fn create_test_git_repo(path: &Path) -> std::io::Result<()> {
        fs::create_dir_all(path)?;
        fs::create_dir_all(path.join(".git"))?;
        Ok(())
    }

    #[test]
    fn test_load_config_default() {
        let manager = create_test_config_manager();

        // 設定ファイルが存在しない場合、デフォルト値が返される
        let config = manager.load_config().unwrap();
        assert_eq!(config.version, 1);
        assert_eq!(config.git.default_branch, "main");
    }

    #[test]
    fn test_save_and_load_config() {
        let manager = create_test_config_manager();

        // テスト用リポジトリパス作成
        let test_repo = env::temp_dir().join("test_repo");
        create_test_git_repo(&test_repo).unwrap();

        let mut config = Config::default();
        config.git.repository_path = test_repo.to_str().unwrap().to_string();

        // 保存
        manager.save_config(&config).unwrap();

        // 読み込み
        let loaded_config = manager.load_config().unwrap();
        assert_eq!(loaded_config.version, config.version);
        assert_eq!(loaded_config.git.repository_path, config.git.repository_path);

        // クリーンアップ
        let _ = fs::remove_dir_all(&test_repo);
        let _ = fs::remove_file(manager.config_path());
    }

    #[test]
    fn test_validate_config_empty_repository_path() {
        let manager = create_test_config_manager();
        let config = Config::default();

        let result = manager.validate_config(&config);
        assert!(result.is_err());
        match result.unwrap_err() {
            WorkNoteError::ValidationError(msg) => {
                assert!(msg.contains("Repository path is empty"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_validate_config_nonexistent_repository() {
        let manager = create_test_config_manager();
        let mut config = Config::default();
        config.git.repository_path = "/nonexistent/path".to_string();

        let result = manager.validate_config(&config);
        assert!(result.is_err());
        match result.unwrap_err() {
            WorkNoteError::ValidationError(msg) => {
                assert!(msg.contains("does not exist"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_validate_config_not_git_repository() {
        let manager = create_test_config_manager();
        let test_dir = env::temp_dir().join("not_a_git_repo");
        fs::create_dir_all(&test_dir).unwrap();

        let mut config = Config::default();
        config.git.repository_path = test_dir.to_str().unwrap().to_string();

        let result = manager.validate_config(&config);
        assert!(result.is_err());
        match result.unwrap_err() {
            WorkNoteError::ValidationError(msg) => {
                assert!(msg.contains("Not a Git repository"));
            }
            _ => panic!("Expected ValidationError"),
        }

        // クリーンアップ
        let _ = fs::remove_dir_all(&test_dir);
    }
}
