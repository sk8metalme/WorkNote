# 単体テスト仕様: GitService

## 概要

GitServiceは、Git操作（commit, push, branch作成）を担当します。外部依存（Git CLI）をモック化してテストします。

---

## テスト対象

**モジュール**: `src-tauri/src/services/git_service.rs`

**主要関数**:
- `check_git_status() -> Result<StatusResult, WorkNoteError>`
- `pull_latest() -> Result<PullResult, WorkNoteError>`
- `commit_and_push(file: &Path, message: &str) -> Result<GitResult, WorkNoteError>`
- `create_feature_branch(title: &str) -> Result<BranchResult, WorkNoteError>`

---

## テストケース一覧

### TC-U-GS-001: Git status - 正常系

**前提**: Gitリポジトリが初期化されている

**実行**: `check_git_status()`

**期待結果**: `StatusResult { clean: true }`

**実装例**:
```rust
#[test]
fn tc_u_gs_001_git_status_success() {
    let git_service = GitService::new();
    let result = git_service.check_git_status();

    assert!(result.is_ok());
    let status = result.unwrap();
    assert!(status.clean);
}
```

---

### TC-U-GS-002: Git pull - 正常系

**前提**: リモートリポジトリが存在する

**実行**: `pull_latest()`

**期待結果**: `PullResult { success: true }`

**実装例**:
```rust
#[test]
fn tc_u_gs_002_git_pull_success() {
    let git_service = GitService::new();
    let result = git_service.pull_latest();

    assert!(result.is_ok());
    let pull = result.unwrap();
    assert!(pull.success);
}
```

---

### TC-U-GS-003: Git commit & push - 正常系

**前提**: ファイルが存在する

**実行**: `commit_and_push(Path::new("test.md"), "test commit")`

**期待結果**: `GitResult { success: true, branch: "main" }`

**実装例**:
```rust
#[test]
fn tc_u_gs_003_commit_and_push_success() {
    let git_service = GitService::new();
    let file = Path::new("test.md");
    let message = "test commit";

    let result = git_service.commit_and_push(file, message);

    assert!(result.is_ok());
    let git_result = result.unwrap();
    assert!(git_result.success);
    assert_eq!(git_result.branch, "main");
}
```

---

### TC-U-GS-004: Git push - 認証エラー

**前提**: Git認証が設定されていない

**実行**: `commit_and_push(...)`

**期待結果**: `Err(WorkNoteError::AuthError(...))`

**実装例**:
```rust
#[test]
fn tc_u_gs_004_push_auth_error() {
    let git_service = GitService::new();
    // モック: Git pushが認証エラーを返す
    let file = Path::new("test.md");
    let message = "test commit";

    let result = git_service.commit_and_push(file, message);

    assert!(result.is_err());
    match result.unwrap_err() {
        WorkNoteError::AuthError(_) => (),
        _ => panic!("Expected AuthError"),
    }
}
```

---

### TC-U-GS-005: Git push - ネットワークエラー

**前提**: ネットワークが利用不可

**実行**: `commit_and_push(...)`

**期待結果**: `Err(WorkNoteError::NetworkError(...))`

**実装例**:
```rust
#[test]
fn tc_u_gs_005_push_network_error() {
    let git_service = GitService::new();
    // モック: Git pushがネットワークエラーを返す
    let file = Path::new("test.md");
    let message = "test commit";

    let result = git_service.commit_and_push(file, message);

    assert!(result.is_err());
    match result.unwrap_err() {
        WorkNoteError::NetworkError(_) => (),
        _ => panic!("Expected NetworkError"),
    }
}
```

---

### TC-U-GS-006: featureブランチ作成 - 正常系

**前提**: デフォルトブランチが最新

**実行**: `create_feature_branch("cpu-high")`

**期待結果**: `BranchResult { name: "feature/worknote-cpu-high-{timestamp}" }`

**実装例**:
```rust
#[test]
fn tc_u_gs_006_create_feature_branch_success() {
    let git_service = GitService::new();
    let title = "cpu-high";

    let result = git_service.create_feature_branch(title);

    assert!(result.is_ok());
    let branch = result.unwrap();
    assert!(branch.name.starts_with("feature/worknote-cpu-high-"));
}
```

---

### TC-U-GS-007: コミットメッセージフォーマット検証

**前提**: ファイルが存在する

**実行**: `commit_and_push(Path::new("test.md"), "docs(worknote): add test")`

**期待結果**: コミットメッセージが正しいフォーマット

**実装例**:
```rust
#[test]
fn tc_u_gs_007_commit_message_format() {
    let git_service = GitService::new();
    let file = Path::new("test.md");
    let message = "docs(worknote): add test\n\nCreated by WorkNote\nCategory: alerts\nSeverity: high";

    let result = git_service.commit_and_push(file, message);

    assert!(result.is_ok());
    // モック検証: コミットメッセージが期待通り
}
```

---

## モック戦略

### Git CLIのモック化

Git CLIへの依存をモック化するため、以下のアプローチを使用：

**方法1: トレイトによる抽象化**

```rust
pub trait GitOperations {
    fn execute_command(&self, args: &[&str]) -> Result<String, WorkNoteError>;
}

pub struct RealGitOperations;
impl GitOperations for RealGitOperations {
    fn execute_command(&self, args: &[&str]) -> Result<String, WorkNoteError> {
        // 実際のGit CLI呼び出し
    }
}

pub struct MockGitOperations;
impl GitOperations for MockGitOperations {
    fn execute_command(&self, args: &[&str]) -> Result<String, WorkNoteError> {
        // モック実装
    }
}
```

**方法2: mockallクレートの使用**

```rust
#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

#[automock]
pub trait GitOperations {
    fn commit_and_push(&self, file: &Path, message: &str) -> Result<GitResult, WorkNoteError>;
}
```

---

## カバレッジ目標

- **行カバレッジ**: 95%以上
- **分岐カバレッジ**: 90%以上
- **関数カバレッジ**: 100%

---

## 実行方法

```bash
# GitServiceのテストのみ実行
cd src-tauri && cargo test git_service

# カバレッジ付き
cd src-tauri && cargo tarpaulin --package worknote --lib git_service
```

---

**文書情報**

| 項目 | 内容 |
|------|------|
| 作成日 | 2026-01-31 |
| 作成者 | Claude Code (michi:plan-tests) |
| バージョン | 1.0 |
