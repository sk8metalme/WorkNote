use serde::{Deserialize, Serialize};

/// コミットモード
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CommitMode {
    Direct,
    FeatureBranch,
}

impl Default for CommitMode {
    fn default() -> Self {
        CommitMode::Direct
    }
}

/// Git設定
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitConfig {
    pub repository_path: String,
    pub save_path: String,
    pub default_branch: String,
    pub commit_mode: CommitMode,
}

impl Default for GitConfig {
    fn default() -> Self {
        GitConfig {
            repository_path: String::new(),
            save_path: "docs/runbooks".to_string(),
            default_branch: "main".to_string(),
            commit_mode: CommitMode::default(),
        }
    }
}

/// Author設定
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorConfig {
    pub name: String,
    pub email: String,
}

impl Default for AuthorConfig {
    fn default() -> Self {
        AuthorConfig {
            name: String::new(),
            email: String::new(),
        }
    }
}

/// ショートカット設定
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutsConfig {
    pub quick_input: String,
}

impl Default for ShortcutsConfig {
    fn default() -> Self {
        ShortcutsConfig {
            quick_input: "CommandOrControl+J".to_string(),
        }
    }
}

/// 環境設定
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferencesConfig {
    pub launch_at_login: bool,
    pub show_in_menu_bar: bool,
    pub show_notifications: bool,
}

impl Default for PreferencesConfig {
    fn default() -> Self {
        PreferencesConfig {
            launch_at_login: false,
            show_in_menu_bar: true,
            show_notifications: true,
        }
    }
}

/// アプリケーション設定
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub version: u32,
    pub git: GitConfig,
    pub author: AuthorConfig,
    pub shortcuts: ShortcutsConfig,
    pub preferences: PreferencesConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: 1,
            git: GitConfig::default(),
            author: AuthorConfig::default(),
            shortcuts: ShortcutsConfig::default(),
            preferences: PreferencesConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.version, 1);
        assert_eq!(config.git.default_branch, "main");
        assert_eq!(config.git.save_path, "docs/runbooks");
        assert_eq!(config.git.commit_mode, CommitMode::Direct);
        assert_eq!(config.shortcuts.quick_input, "CommandOrControl+J");
        assert!(config.preferences.show_in_menu_bar);
        assert!(config.preferences.show_notifications);
        assert!(!config.preferences.launch_at_login);
    }

    #[test]
    fn test_commit_mode_serialization() {
        let direct = CommitMode::Direct;
        let json = serde_json::to_string(&direct).unwrap();
        assert_eq!(json, "\"direct\"");

        let feature = CommitMode::FeatureBranch;
        let json = serde_json::to_string(&feature).unwrap();
        assert_eq!(json, "\"feature-branch\"");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"version\":1"));
        assert!(json.contains("\"defaultBranch\":\"main\""));
    }
}
