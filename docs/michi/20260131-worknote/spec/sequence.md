# シーケンス図: WorkNote

## 概要

本ドキュメントは、WorkNoteの主要な処理フローをシーケンス図で記述します。

---

## 1. クイック入力からの保存フロー（直接Pushモード）

```mermaid
sequenceDiagram
    actor User as 運用エンジニア
    participant QW as QuickInputWindow
    participant Val as FormValidator
    participant TB as TauriBridge
    participant GS as GitService
    participant FG as FileGenerator
    participant Git as Git CLI
    participant Repo as リモートリポジトリ

    User->>QW: ⌘+Shift+K（グローバルショートカット）
    activate QW
    QW->>User: ウィンドウ表示（0.3秒以内）
    User->>QW: タイトル、カテゴリ、重要度を入力
    User->>QW: Enter or 「クイック保存」ボタンクリック

    QW->>Val: validate(input)
    activate Val
    Val-->>QW: ValidationResult { valid: true }
    deactivate Val

    QW->>TB: save_knowledge(input, mode: "direct")
    activate TB

    TB->>GS: check_git_status()
    activate GS
    GS->>Git: git status
    Git-->>GS: 出力
    GS-->>TB: StatusResult
    deactivate GS

    TB->>GS: pull_latest()
    activate GS
    GS->>Git: git checkout main
    GS->>Git: git pull origin main
    Git-->>GS: 成功
    GS-->>TB: PullResult
    deactivate GS

    TB->>FG: generate_markdown(input)
    activate FG
    FG->>FG: タイトル → kebab-case変換
    FG->>FG: frontmatter生成
    FG->>FG: セクション構成
    FG-->>TB: MarkdownFile { path, content }
    deactivate FG

    TB->>GS: commit_and_push(file, message)
    activate GS
    GS->>Git: git add {file}
    GS->>Git: git commit -m "docs(worknote): add {title}"
    GS->>Git: git push origin main
    Git-->>Repo: push
    Repo-->>Git: 成功
    GS-->>TB: GitResult { success: true }
    deactivate GS

    TB-->>QW: SaveResult { success: true }
    deactivate TB

    QW->>User: 完了通知表示
    QW->>QW: ウィンドウを閉じる
    deactivate QW
```

---

## 2. 詳細入力からの保存フロー（直接Pushモード）

```mermaid
sequenceDiagram
    actor User as 運用エンジニア
    participant DW as DetailInputWindow
    participant PV as PreviewPane
    participant MR as MarkdownRenderer
    participant Val as FormValidator
    participant TB as TauriBridge
    participant GS as GitService
    participant FG as FileGenerator
    participant Git as Git CLI
    participant Repo as リモートリポジトリ

    User->>DW: Dockアイコンクリック or メニューバー
    activate DW
    DW->>User: 詳細入力ウィンドウ表示

    User->>DW: 全フィールドに入力
    User->>DW: プレビュー表示チェック

    DW->>PV: show_preview(input)
    activate PV
    PV->>MR: render(input)
    activate MR
    MR-->>PV: HTML
    deactivate MR
    PV->>User: プレビュー表示
    deactivate PV

    User->>DW: ⌘+Enter or 「Git Commit & Push」ボタン

    DW->>Val: validate(input)
    activate Val
    Val-->>DW: ValidationResult { valid: true }
    deactivate Val

    DW->>TB: save_knowledge(input, mode: "direct")
    activate TB

    Note over TB,Git: クイック入力と同じフロー
    TB->>GS: check_git_status()
    TB->>GS: pull_latest()
    TB->>FG: generate_markdown(input)
    TB->>GS: commit_and_push(file, message)
    GS->>Git: git add, commit, push
    Git-->>Repo: push
    GS-->>TB: GitResult { success: true }

    TB-->>DW: SaveResult { success: true }
    deactivate TB

    DW->>User: 完了通知表示
    DW->>DW: ウィンドウを閉じる
    deactivate DW
```

---

## 3. PR作成モードのフロー

```mermaid
sequenceDiagram
    actor User as 運用エンジニア
    participant DW as DetailInputWindow
    participant TB as TauriBridge
    participant CM as ConfigManager
    participant GS as GitService
    participant FG as FileGenerator
    participant Git as Git CLI
    participant Repo as リモートリポジトリ

    User->>DW: ナレッジ入力
    User->>DW: 「Git Commit & Push」ボタン

    DW->>TB: save_knowledge(input, mode: "pr")
    activate TB

    TB->>CM: get_config()
    activate CM
    CM-->>TB: Config { commitMode: "pr" }
    deactivate CM

    TB->>GS: pull_latest()
    activate GS
    GS->>Git: git checkout main
    GS->>Git: git pull origin main
    GS-->>TB: PullResult
    deactivate GS

    TB->>GS: create_feature_branch(title)
    activate GS
    GS->>GS: ブランチ名生成: "feature/worknote-{title}-{timestamp}"
    GS->>Git: git checkout -b feature/worknote-xxx
    GS-->>TB: BranchResult { name: "feature/worknote-xxx" }
    deactivate GS

    TB->>FG: generate_markdown(input)
    activate FG
    FG-->>TB: MarkdownFile
    deactivate FG

    TB->>GS: commit_and_push_branch(file, branch)
    activate GS
    GS->>Git: git add {file}
    GS->>Git: git commit -m "docs(worknote): add {title}"
    GS->>Git: git push origin feature/worknote-xxx
    Git-->>Repo: push
    GS-->>TB: GitResult { success: true, pr_url: "https://github.com/..." }
    deactivate GS

    TB-->>DW: SaveResult { success: true, pr_url: "..." }
    deactivate TB

    DW->>User: 完了通知 + PR作成リンク表示
    User->>User: [GitHubでPRを作成]をクリック → ブラウザ起動
```

---

## 4. 設定読み込み・保存フロー

```mermaid
sequenceDiagram
    actor User as 運用エンジニア
    participant SW as SettingsWindow
    participant TB as TauriBridge
    participant CM as ConfigManager
    participant Store as tauri-plugin-store
    participant FS as FileSystem

    User->>SW: ⌘+, or メニュー → 設定
    activate SW

    SW->>TB: load_config()
    activate TB
    TB->>CM: load()
    activate CM
    CM->>Store: get("config")
    activate Store
    Store->>FS: read ~/Library/Application Support/WorkNote/config.json
    FS-->>Store: JSON
    Store-->>CM: Config
    deactivate Store
    CM-->>TB: Config
    deactivate CM
    TB-->>SW: Config
    deactivate TB

    SW->>User: 設定値を表示

    User->>SW: リポジトリパス、Author名等を編集
    User->>SW: 「保存」ボタンクリック

    SW->>TB: save_config(new_config)
    activate TB
    TB->>CM: save(new_config)
    activate CM
    CM->>Store: set("config", new_config)
    activate Store
    Store->>FS: write ~/Library/Application Support/WorkNote/config.json
    FS-->>Store: 成功
    Store-->>CM: 成功
    deactivate Store
    CM-->>TB: 成功
    deactivate CM
    TB-->>SW: SaveResult { success: true }
    deactivate TB

    SW->>User: 保存完了通知
    SW->>SW: ウィンドウを閉じる
    deactivate SW
```

---

## 5. 下書き保存・復元フロー

```mermaid
sequenceDiagram
    actor User as 運用エンジニア
    participant DW as DetailInputWindow
    participant TB as TauriBridge
    participant DM as DraftManager
    participant FS as FileSystem

    Note over User,FS: 下書き保存

    User->>DW: ナレッジ入力中
    User->>DW: ⌘+S（下書き保存）

    DW->>TB: save_draft(input)
    activate TB
    TB->>DM: save(input)
    activate DM
    DM->>DM: UUID生成
    DM->>DM: Draft構造体作成
    DM->>FS: write ~/Library/Application Support/WorkNote/drafts/{uuid}.json
    FS-->>DM: 成功
    DM-->>TB: DraftResult { id: "uuid" }
    deactivate DM
    TB-->>DW: SaveResult { success: true }
    deactivate TB

    DW->>User: 下書き保存完了通知

    Note over User,FS: 下書き復元

    User->>DW: メニュー → 下書き一覧
    DW->>TB: list_drafts()
    activate TB
    TB->>DM: list()
    activate DM
    DM->>FS: read ~/Library/Application Support/WorkNote/drafts/*.json
    FS-->>DM: Draft[]
    DM-->>TB: Draft[]
    deactivate DM
    TB-->>DW: Draft[]
    deactivate TB

    DW->>User: 下書き一覧表示
    User->>DW: 下書きを選択

    DW->>TB: load_draft(id)
    activate TB
    TB->>DM: load(id)
    activate DM
    DM->>FS: read ~/Library/Application Support/WorkNote/drafts/{id}.json
    FS-->>DM: Draft
    DM-->>TB: Draft
    deactivate DM
    TB-->>DW: Draft
    deactivate TB

    DW->>User: 入力フォームに復元
```

---

## 6. Markdownプレビューフロー

```mermaid
sequenceDiagram
    actor User as 運用エンジニア
    participant DW as DetailInputWindow
    participant PV as PreviewPane
    participant MR as MarkdownRenderer
    participant FG as FileGenerator

    User->>DW: 「プレビュー表示」チェック

    DW->>PV: show()
    activate PV
    PV->>User: プレビューペイン表示
    deactivate PV

    loop リアルタイム更新
        User->>DW: テキスト入力

        DW->>FG: generate_preview_markdown(input)
        activate FG
        FG->>FG: frontmatter生成
        FG->>FG: セクション構成
        FG-->>DW: Markdown文字列
        deactivate FG

        DW->>PV: update(markdown)
        activate PV
        PV->>MR: render(markdown)
        activate MR
        MR-->>PV: HTML
        deactivate MR
        PV->>User: プレビュー更新表示
        deactivate PV
    end
```

---

## 7. エラーハンドリングフロー

### 7.1 Git認証エラー

```mermaid
sequenceDiagram
    actor User as 運用エンジニア
    participant DW as DetailInputWindow
    participant TB as TauriBridge
    participant GS as GitService
    participant Git as Git CLI
    participant EH as ErrorHandler

    User->>DW: 「Git Commit & Push」ボタン

    DW->>TB: save_knowledge(input)
    TB->>GS: commit_and_push()
    GS->>Git: git push origin main
    Git-->>GS: エラー: 認証失敗

    GS->>EH: handle_git_error(error)
    activate EH
    EH->>EH: エラー種別を判定
    EH-->>GS: UserFriendlyError { message, action }
    deactivate EH

    GS-->>TB: Err(GitError)
    TB-->>DW: SaveResult { success: false, error: "..." }

    DW->>User: エラーダイアログ表示<br/>「Git認証に失敗しました。<br/>SSH鍵またはCredential Helperを確認してください。<br/>[設定画面を開く] [下書き保存]」

    alt 下書き保存を選択
        User->>DW: [下書き保存]
        DW->>TB: save_draft(input)
        TB-->>DW: DraftResult
        DW->>User: 下書き保存完了
    else 設定画面を選択
        User->>DW: [設定画面を開く]
        DW->>DW: open_settings()
    end
```

### 7.2 ネットワークエラー

```mermaid
sequenceDiagram
    actor User as 運用エンジニア
    participant DW as DetailInputWindow
    participant TB as TauriBridge
    participant GS as GitService
    participant Git as Git CLI

    User->>DW: 「Git Commit & Push」ボタン

    DW->>TB: save_knowledge(input)
    TB->>GS: commit_and_push()
    GS->>Git: git push origin main
    Git-->>GS: エラー: ネットワークタイムアウト

    GS-->>TB: Err(NetworkError)
    TB-->>DW: SaveResult { success: false, error: "..." }

    DW->>User: エラーダイアログ表示<br/>「ネットワークに接続できません。<br/>[リトライ] [下書き保存] [キャンセル]」

    alt リトライを選択
        User->>DW: [リトライ]
        DW->>TB: save_knowledge(input)
        Note over TB,Git: 再度Push処理
    else 下書き保存を選択
        User->>DW: [下書き保存]
        DW->>TB: save_draft(input)
        TB-->>DW: DraftResult
        DW->>User: 下書き保存完了
    end
```

---

## 8. グローバルショートカット登録フロー

```mermaid
sequenceDiagram
    actor User as 運用エンジニア
    participant App as WorkNote App
    participant CM as ConfigManager
    participant GS as GlobalShortcut Plugin
    participant OS as macOS

    App->>App: アプリ起動

    App->>CM: load_config()
    CM-->>App: Config { shortcuts: { quickInput: "⌘+Shift+K" } }

    App->>GS: register("⌘+Shift+K", callback)
    activate GS
    GS->>OS: グローバルショートカット登録
    OS-->>GS: 登録成功
    GS-->>App: 登録完了
    deactivate GS

    Note over User,OS: アプリがバックグラウンドでも動作

    User->>OS: ⌘+Shift+K 押下
    OS->>GS: ショートカットイベント
    GS->>App: callback()
    App->>App: QuickInputWindowを表示
    App->>User: ウィンドウ表示
```

---

## まとめ

WorkNoteの主要フローは以下の特徴を持ちます：

1. **シンプルな処理**: フロントエンド → バックエンド → Git CLI の明確な流れ
2. **エラーハンドリング**: 各エラーに対して適切な対処方法を提示
3. **非同期処理**: Tauri Commandによる非ブロッキング操作
4. **下書き保存**: ネットワークエラー時の損失防止

---

**文書情報**

| 項目 | 内容 |
|------|------|
| 作成日 | 2026-01-31 |
| 作成者 | Claude Code (michi:create-design) |
| バージョン | 1.0 |
| ステータス | 承認待ち |
