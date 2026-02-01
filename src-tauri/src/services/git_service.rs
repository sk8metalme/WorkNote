use crate::models::{Result, WorkNoteError};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// GitService - Git操作を管理
pub struct GitService {
    repository_path: PathBuf,
    default_branch: String,
}

impl GitService {
    /// 新しいGitServiceインスタンスを作成
    pub fn new(repository_path: PathBuf, default_branch: String) -> Self {
        GitService {
            repository_path,
            default_branch,
        }
    }

    /// Gitコマンドを実行
    fn execute_git(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("git")
            .current_dir(&self.repository_path)
            .args(args)
            .env("GIT_TERMINAL_PROMPT", "0")
            .stdin(Stdio::null())
            .output()
            .map_err(|e| WorkNoteError::GitError(format!("Failed to execute git: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(self.classify_error(&stderr));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// エラーメッセージからエラー種別を判定
    fn classify_error(&self, stderr: &str) -> WorkNoteError {
        let lower = stderr.to_lowercase();

        if lower.contains("authentication failed")
            || lower.contains("permission denied")
            || lower.contains("could not read from remote")
        {
            WorkNoteError::AuthError(stderr.to_string())
        } else if lower.contains("network")
            || lower.contains("unable to access")
            || lower.contains("connection")
        {
            WorkNoteError::NetworkError(stderr.to_string())
        } else {
            WorkNoteError::GitError(stderr.to_string())
        }
    }

    /// Git statusを確認
    pub fn check_git_status(&self) -> Result<bool> {
        let output = self.execute_git(&["status", "--porcelain"])?;
        Ok(output.trim().is_empty())
    }

    /// 最新の変更を取得
    pub fn pull_latest(&self) -> Result<()> {
        // デフォルトブランチにチェックアウト
        self.execute_git(&["checkout", &self.default_branch])?;
        // Pull実行
        self.execute_git(&["pull", "origin", &self.default_branch])?;
        Ok(())
    }

    /// ファイルをコミット＆プッシュ
    pub fn commit_and_push(&self, file_path: &Path, title: &str, category: &str, severity: &str) -> Result<String> {
        // ファイル名を相対パスに変換
        let relative_path = file_path.strip_prefix(&self.repository_path)
            .map_err(|e| WorkNoteError::FileError(format!("Invalid file path: {}", e)))?;

        // 相対パスをUTF-8文字列に変換
        let relative_path_str = relative_path
            .to_str()
            .ok_or_else(|| WorkNoteError::FileError("Invalid UTF-8 file path".to_string()))?;

        // Git add
        self.execute_git(&["add", relative_path_str])?;

        // コミットメッセージ生成
        let message = self.format_commit_message(title, category, severity);

        // Git commit
        self.execute_git(&["commit", "-m", &message])?;

        // Git push
        self.execute_git(&["push", "origin", &self.default_branch])?;

        // 最新のコミットハッシュを取得
        let hash = self.execute_git(&["rev-parse", "HEAD"])?;
        Ok(hash.trim().to_string())
    }

    /// コミットメッセージをフォーマット
    fn format_commit_message(&self, title: &str, category: &str, severity: &str) -> String {
        format!(
            "docs(worknote): add {}\n\nCategory: {}\nSeverity: {}",
            title, category, severity
        )
    }

    /// リモートURLからowner/repoを取得
    fn get_remote_info(&self) -> Result<(String, String)> {
        let remote_url = self.execute_git(&["remote", "get-url", "origin"])?;
        let remote_url = remote_url.trim();

        // Parse GitHub URL
        // Examples:
        // - https://github.com/owner/repo.git
        // - git@github.com:owner/repo.git
        let parts: Vec<&str> = if remote_url.starts_with("https://") {
            // HTTPS URL
            remote_url
                .trim_start_matches("https://github.com/")
                .trim_end_matches(".git")
                .split('/')
                .collect()
        } else if remote_url.starts_with("git@") {
            // SSH URL
            remote_url
                .trim_start_matches("git@github.com:")
                .trim_end_matches(".git")
                .split('/')
                .collect()
        } else {
            return Err(WorkNoteError::GitError(
                "Unsupported remote URL format".to_string(),
            ));
        };

        if parts.len() < 2 {
            return Err(WorkNoteError::GitError(
                "Invalid remote URL format".to_string(),
            ));
        }

        Ok((parts[0].to_string(), parts[1].to_string()))
    }

    /// GitHub PR作成URLを生成
    fn generate_pr_url(&self, branch: &str) -> Result<String> {
        let (owner, repo) = self.get_remote_info()?;
        Ok(format!(
            "https://github.com/{}/{}/compare/{}",
            owner, repo, branch
        ))
    }

    /// タイトルをブランチ名に変換（kebab-case + タイムスタンプ）
    fn sanitize_branch_name(&self, title: &str) -> String {
        let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S");
        let sanitized = title
            .to_lowercase()
            .chars()
            .filter_map(|c| {
                if c.is_ascii_alphanumeric() {
                    Some(c)
                } else if c.is_whitespace() || c.is_ascii_punctuation() {
                    Some('-')
                } else {
                    None
                }
            })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");

        // 日本語タイトルなど、ASCII文字がない場合は"knowledge"をデフォルトとして使用
        let prefix = if sanitized.is_empty() {
            "knowledge"
        } else {
            &sanitized
        };

        format!("feature/worknote-{}-{}", prefix, timestamp)
    }

    /// PR作成モード: featureブランチにコミット＆プッシュ
    pub fn commit_and_push_pr(
        &self,
        file_path: &Path,
        title: &str,
        category: &str,
        severity: &str,
    ) -> Result<(String, String)> {
        // デフォルトブランチから最新を取得
        self.pull_latest()?;

        // featureブランチ名を生成
        let branch_name = self.sanitize_branch_name(title);

        // featureブランチを作成してチェックアウト
        self.execute_git(&["checkout", "-b", &branch_name])?;

        // ファイル名を相対パスに変換
        let relative_path = file_path
            .strip_prefix(&self.repository_path)
            .map_err(|e| WorkNoteError::FileError(format!("Invalid file path: {}", e)))?;

        // 相対パスをUTF-8文字列に変換
        let relative_path_str = relative_path
            .to_str()
            .ok_or_else(|| WorkNoteError::FileError("Invalid UTF-8 file path".to_string()))?;

        // Git add
        self.execute_git(&["add", relative_path_str])?;

        // コミットメッセージ生成
        let message = self.format_commit_message(title, category, severity);

        // Git commit
        self.execute_git(&["commit", "-m", &message])?;

        // Git push (featureブランチ)
        self.execute_git(&["push", "origin", &branch_name])?;

        // 最新のコミットハッシュを取得
        let hash = self.execute_git(&["rev-parse", "HEAD"])?;

        // PR URL生成
        let pr_url = self.generate_pr_url(&branch_name)?;

        // デフォルトブランチに戻す（次回のDirect modeで誤ったブランチに コミットしないため）
        self.execute_git(&["checkout", &self.default_branch])?;

        Ok((hash.trim().to_string(), pr_url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_error_auth() {
        let service = GitService::new(PathBuf::from("/test"), "main".to_string());
        let error = service.classify_error("fatal: Authentication failed");

        match error {
            WorkNoteError::AuthError(_) => (),
            _ => panic!("Expected AuthError"),
        }
    }

    #[test]
    fn test_classify_error_network() {
        let service = GitService::new(PathBuf::from("/test"), "main".to_string());
        let error = service.classify_error("fatal: unable to access 'https://': Network error");

        match error {
            WorkNoteError::NetworkError(_) => (),
            _ => panic!("Expected NetworkError"),
        }
    }

    #[test]
    fn test_format_commit_message() {
        let service = GitService::new(PathBuf::from("/test"), "main".to_string());
        let message = service.format_commit_message("CPU高騰対応", "alerts", "high");

        assert!(message.contains("docs(worknote): add CPU高騰対応"));
        assert!(message.contains("Category: alerts"));
        assert!(message.contains("Severity: high"));
    }
}
