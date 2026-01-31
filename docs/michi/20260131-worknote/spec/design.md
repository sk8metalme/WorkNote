# 詳細設計書: WorkNote

## 概要

本ドキュメントは、WorkNoteの詳細設計（データモデル、API仕様、エラーハンドリング、ディレクトリ構造等）を記述します。

---

## 1. データモデル

### 1.1 設定ファイル（config.json）

**保存先**: `~/Library/Application Support/WorkNote/config.json`

**スキーマ**:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["version", "git", "author", "shortcuts", "preferences"],
  "properties": {
    "version": {
      "type": "integer",
      "description": "設定ファイルバージョン（マイグレーション用）",
      "const": 1
    },
    "git": {
      "type": "object",
      "required": ["repositoryPath", "savePath", "defaultBranch", "commitMode"],
      "properties": {
        "repositoryPath": {
          "type": "string",
          "description": "リポジトリの絶対パス",
          "examples": ["~/projects/your-repo"]
        },
        "savePath": {
          "type": "string",
          "description": "ナレッジ保存先（リポジトリ内相対パス）",
          "examples": ["docs/runbooks"]
        },
        "defaultBranch": {
          "type": "string",
          "description": "デフォルトブランチ名",
          "default": "main"
        },
        "commitMode": {
          "type": "string",
          "enum": ["direct", "pr"],
          "description": "コミットモード（直接Push or PR作成）",
          "default": "direct"
        }
      }
    },
    "author": {
      "type": "object",
      "required": ["name", "email"],
      "properties": {
        "name": {
          "type": "string",
          "description": "Git Author名",
          "minLength": 1,
          "maxLength": 100
        },
        "email": {
          "type": "string",
          "description": "Git Authorメールアドレス",
          "format": "email"
        }
      }
    },
    "shortcuts": {
      "type": "object",
      "required": ["quickInput"],
      "properties": {
        "quickInput": {
          "type": "string",
          "description": "クイック入力起動ショートカット",
          "default": "CommandOrControl+Shift+K"
        }
      }
    },
    "preferences": {
      "type": "object",
      "required": ["launchAtLogin", "showInMenuBar", "showNotifications"],
      "properties": {
        "launchAtLogin": {
          "type": "boolean",
          "description": "ログイン時に自動起動",
          "default": false
        },
        "showInMenuBar": {
          "type": "boolean",
          "description": "メニューバーに常駐",
          "default": true
        },
        "showNotifications": {
          "type": "boolean",
          "description": "保存成功時に通知表示",
          "default": true
        }
      }
    }
  }
}
```

**デフォルト値**:

```json
{
  "version": 1,
  "git": {
    "repositoryPath": "",
    "savePath": "docs/runbooks",
    "defaultBranch": "main",
    "commitMode": "direct"
  },
  "author": {
    "name": "",
    "email": ""
  },
  "shortcuts": {
    "quickInput": "CommandOrControl+Shift+K"
  },
  "preferences": {
    "launchAtLogin": false,
    "showInMenuBar": true,
    "showNotifications": true
  }
}
```

---

### 1.2 下書きファイル（drafts/{uuid}.json）

**保存先**: `~/Library/Application Support/WorkNote/drafts/{uuid}.json`

**スキーマ**:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["id", "createdAt", "updatedAt", "data"],
  "properties": {
    "id": {
      "type": "string",
      "format": "uuid",
      "description": "下書きの一意識別子"
    },
    "createdAt": {
      "type": "string",
      "format": "date-time",
      "description": "作成日時（ISO 8601形式）"
    },
    "updatedAt": {
      "type": "string",
      "format": "date-time",
      "description": "更新日時（ISO 8601形式）"
    },
    "data": {
      "type": "object",
      "required": ["title", "category", "severity"],
      "properties": {
        "title": {
          "type": "string",
          "minLength": 1,
          "maxLength": 100
        },
        "category": {
          "type": "string",
          "enum": ["alerts", "maintenance", "troubleshooting"]
        },
        "severity": {
          "type": "string",
          "enum": ["low", "medium", "high", "critical"]
        },
        "symptoms": {
          "type": "string",
          "maxLength": 5000
        },
        "procedure": {
          "type": "string",
          "maxLength": 10000
        },
        "notes": {
          "type": "string",
          "maxLength": 3000
        },
        "links": {
          "type": "string",
          "maxLength": 1000
        }
      }
    }
  }
}
```

**例**:

```json
{
  "id": "a1b2c3d4-e5f6-4789-0123-456789abcdef",
  "createdAt": "2026-01-31T10:00:00Z",
  "updatedAt": "2026-01-31T10:30:00Z",
  "data": {
    "title": "CPU高騰対応",
    "category": "alerts",
    "severity": "high",
    "symptoms": "CPU使用率が90%を超えている",
    "procedure": "1. プロセス一覧を確認\n2. 原因プロセスを特定\n3. 必要に応じて再起動",
    "notes": "ピーク時間帯は注意",
    "links": "https://jira.example.com/TICKET-123"
  }
}
```

---

### 1.3 出力Markdownファイル

**保存先**: `{リポジトリパス}/{ナレッジ保存先}/{カテゴリ}/{filename}.md`

**例**: `~/projects/your-repo/docs/runbooks/alerts/cpu-high.md`

**テンプレート**:

```markdown
---
title: "{タイトル}"
category: {カテゴリ}
severity: {重要度}
symptoms:
  - "{症状1行目}"
  - "{症状2行目}"
related_alerts: []
last_updated: {YYYY-MM-DD}
author: "{Author名}"
---

# {タイトル}

## 概要

{症状・検知条件の1行目、または「このドキュメントは{タイトル}の対応手順です。」}

## 症状・検知条件

{症状・検知条件}

## 対応手順

{対応手順}

## 注意点・落とし穴

{注意点}

## 関連リンク

{関連リンク（各行を - [URL](URL) 形式に変換）}

## 対応履歴

| 日付 | 対応者 | 備考 |
|------|--------|------|
| {YYYY-MM-DD} | {Author名} | 初版作成 |
```

**ファイル名生成ルール**:

1. タイトルをkebab-caseに変換
2. 日本語 → ローマ字変換（例: "CPU高騰" → "cpu-gao-teng"）
3. 特殊文字を除去

**実装例（Rust）**:

```rust
fn to_kebab_case(title: &str) -> String {
    title
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}
```

---

## 2. API仕様（Tauri Command）

### 2.1 ナレッジ保存

**コマンド**: `save_knowledge`

**リクエスト**:

```typescript
interface SaveKnowledgeRequest {
  input: KnowledgeInput;
  mode: 'quick' | 'detail';
}
```

**レスポンス**:

```typescript
interface SaveKnowledgeResponse {
  success: boolean;
  filePath?: string;
  commitHash?: string;
  prUrl?: string;
  error?: string;
}
```

**Rust実装シグネチャ**:

```rust
#[tauri::command]
async fn save_knowledge(
    input: KnowledgeInput,
    mode: String,
    state: State<'_, AppState>,
) -> Result<SaveKnowledgeResponse, String>
```

---

### 2.2 設定読み込み

**コマンド**: `load_config`

**リクエスト**: なし

**レスポンス**:

```typescript
interface LoadConfigResponse {
  config: Config;
}
```

**Rust実装シグネチャ**:

```rust
#[tauri::command]
async fn load_config(
    state: State<'_, AppState>,
) -> Result<Config, String>
```

---

### 2.3 設定保存

**コマンド**: `save_config`

**リクエスト**:

```typescript
interface SaveConfigRequest {
  config: Config;
}
```

**レスポンス**:

```typescript
interface SaveConfigResponse {
  success: boolean;
  error?: string;
}
```

**Rust実装シグネチャ**:

```rust
#[tauri::command]
async fn save_config(
    config: Config,
    state: State<'_, AppState>,
) -> Result<SaveConfigResponse, String>
```

---

### 2.4 下書き保存

**コマンド**: `save_draft`

**リクエスト**:

```typescript
interface SaveDraftRequest {
  input: KnowledgeInput;
}
```

**レスポンス**:

```typescript
interface SaveDraftResponse {
  success: boolean;
  draftId: string;
  error?: string;
}
```

**Rust実装シグネチャ**:

```rust
#[tauri::command]
async fn save_draft(
    input: KnowledgeInput,
    state: State<'_, AppState>,
) -> Result<SaveDraftResponse, String>
```

---

### 2.5 下書き一覧取得

**コマンド**: `list_drafts`

**リクエスト**: なし

**レスポンス**:

```typescript
interface ListDraftsResponse {
  drafts: Draft[];
}
```

**Rust実装シグネチャ**:

```rust
#[tauri::command]
async fn list_drafts(
    state: State<'_, AppState>,
) -> Result<Vec<Draft>, String>
```

---

### 2.6 下書き読み込み

**コマンド**: `load_draft`

**リクエスト**:

```typescript
interface LoadDraftRequest {
  draftId: string;
}
```

**レスポンス**:

```typescript
interface LoadDraftResponse {
  draft: Draft;
}
```

**Rust実装シグネチャ**:

```rust
#[tauri::command]
async fn load_draft(
    draft_id: String,
    state: State<'_, AppState>,
) -> Result<Draft, String>
```

---

### 2.7 下書き削除

**コマンド**: `delete_draft`

**リクエスト**:

```typescript
interface DeleteDraftRequest {
  draftId: string;
}
```

**レスポンス**:

```typescript
interface DeleteDraftResponse {
  success: boolean;
  error?: string;
}
```

**Rust実装シグネチャ**:

```rust
#[tauri::command]
async fn delete_draft(
    draft_id: String,
    state: State<'_, AppState>,
) -> Result<DeleteDraftResponse, String>
```

---

### 2.8 Markdownプレビュー生成

**コマンド**: `generate_preview`

**リクエスト**:

```typescript
interface GeneratePreviewRequest {
  input: KnowledgeInput;
}
```

**レスポンス**:

```typescript
interface GeneratePreviewResponse {
  markdown: string;
}
```

**Rust実装シグネチャ**:

```rust
#[tauri::command]
async fn generate_preview(
    input: KnowledgeInput,
    state: State<'_, AppState>,
) -> Result<String, String>
```

---

## 3. エラーハンドリング

### 3.1 エラー種別

```rust
#[derive(Debug, thiserror::Error)]
pub enum WorkNoteError {
    #[error("Git操作エラー: {0}")]
    GitError(String),

    #[error("ファイル操作エラー: {0}")]
    FileError(String),

    #[error("バリデーションエラー: {0}")]
    ValidationError(String),

    #[error("設定エラー: {0}")]
    ConfigError(String),

    #[error("ネットワークエラー: {0}")]
    NetworkError(String),

    #[error("認証エラー: {0}")]
    AuthError(String),
}
```

---

### 3.2 エラーマッピング

| エラー種別 | ユーザーメッセージ | アクション |
|------------|-------------------|-----------|
| **GitError (認証失敗)** | 「Git認証に失敗しました。SSH鍵またはCredential Helperを確認してください。」 | [設定画面を開く] [下書き保存] |
| **GitError (コンフリクト)** | 「マージコンフリクトが発生しました。手動で解決してください。」 | [リポジトリを開く] [下書き保存] |
| **NetworkError** | 「ネットワークに接続できません。接続を確認してください。」 | [リトライ] [下書き保存] [キャンセル] |
| **FileError (権限エラー)** | 「ファイルへのアクセス権限がありません。」 | [OK] |
| **FileError (ディスク容量不足)** | 「ディスク容量が不足しています。」 | [OK] |
| **ValidationError** | 「入力内容にエラーがあります: {詳細}」 | [OK] |
| **ConfigError (リポジトリパス無効)** | 「リポジトリパスが無効です。設定を確認してください。」 | [設定画面を開く] [キャンセル] |

---

### 3.3 エラーハンドリングフロー

```rust
impl GitService {
    pub fn commit_and_push(&self, file: &Path, message: &str) -> Result<GitResult, WorkNoteError> {
        // git add
        let output = Command::new("git")
            .args(&["add", file.to_str().unwrap()])
            .output()
            .map_err(|e| WorkNoteError::GitError(format!("git add失敗: {}", e)))?;

        if !output.status.success() {
            return Err(WorkNoteError::GitError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }

        // git commit
        let output = Command::new("git")
            .args(&["commit", "-m", message])
            .output()
            .map_err(|e| WorkNoteError::GitError(format!("git commit失敗: {}", e)))?;

        if !output.status.success() {
            return Err(WorkNoteError::GitError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }

        // git push
        let output = Command::new("git")
            .args(&["push", "origin", "main"])
            .output()
            .map_err(|e| WorkNoteError::GitError(format!("git push失敗: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);

            // エラー種別を判定
            if stderr.contains("Authentication failed") || stderr.contains("Permission denied") {
                return Err(WorkNoteError::AuthError("Git認証に失敗しました".to_string()));
            } else if stderr.contains("Could not resolve host") || stderr.contains("network") {
                return Err(WorkNoteError::NetworkError("ネットワークに接続できません".to_string()));
            } else {
                return Err(WorkNoteError::GitError(stderr.to_string()));
            }
        }

        Ok(GitResult {
            success: true,
            branch: "main".to_string(),
            commit_hash: None,
            pr_url: None,
        })
    }
}
```

---

## 4. バリデーション仕様

### 4.1 フロントエンドバリデーション（即時フィードバック）

```typescript
interface ValidationRule {
  field: string;
  validate: (value: any) => ValidationError | null;
}

const validationRules: ValidationRule[] = [
  {
    field: 'title',
    validate: (value: string) => {
      if (!value || value.trim().length === 0) {
        return { field: 'title', message: 'タイトルは必須です' };
      }
      if (value.length > 100) {
        return { field: 'title', message: 'タイトルは100文字以内で入力してください' };
      }
      return null;
    }
  },
  {
    field: 'category',
    validate: (value: string) => {
      if (!value) {
        return { field: 'category', message: 'カテゴリを選択してください' };
      }
      return null;
    }
  },
  {
    field: 'severity',
    validate: (value: string) => {
      if (!value) {
        return { field: 'severity', message: '重要度を選択してください' };
      }
      return null;
    }
  },
  {
    field: 'symptoms',
    validate: (value: string) => {
      if (value && value.length > 5000) {
        return { field: 'symptoms', message: '症状・検知条件は5000文字以内で入力してください' };
      }
      return null;
    }
  },
  {
    field: 'procedure',
    validate: (value: string) => {
      if (value && value.length > 10000) {
        return { field: 'procedure', message: '対応手順は10000文字以内で入力してください' };
      }
      return null;
    }
  },
  {
    field: 'notes',
    validate: (value: string) => {
      if (value && value.length > 3000) {
        return { field: 'notes', message: '注意点は3000文字以内で入力してください' };
      }
      return null;
    }
  },
  {
    field: 'links',
    validate: (value: string) => {
      if (value && value.length > 1000) {
        return { field: 'links', message: '関連リンクは1000文字以内で入力してください' };
      }
      return null;
    }
  },
];
```

---

### 4.2 バックエンドバリデーション（二重チェック）

```rust
impl ValidationHelper {
    pub fn validate_knowledge_input(input: &KnowledgeInput) -> Result<(), WorkNoteError> {
        // タイトル
        if input.title.trim().is_empty() {
            return Err(WorkNoteError::ValidationError("タイトルは必須です".to_string()));
        }
        if input.title.len() > 100 {
            return Err(WorkNoteError::ValidationError("タイトルは100文字以内で入力してください".to_string()));
        }

        // 症状・検知条件
        if let Some(ref symptoms) = input.symptoms {
            if symptoms.len() > 5000 {
                return Err(WorkNoteError::ValidationError("症状・検知条件は5000文字以内で入力してください".to_string()));
            }
        }

        // 対応手順
        if let Some(ref procedure) = input.procedure {
            if procedure.len() > 10000 {
                return Err(WorkNoteError::ValidationError("対応手順は10000文字以内で入力してください".to_string()));
            }
        }

        // 注意点
        if let Some(ref notes) = input.notes {
            if notes.len() > 3000 {
                return Err(WorkNoteError::ValidationError("注意点は3000文字以内で入力してください".to_string()));
            }
        }

        // 関連リンク
        if let Some(ref links) = input.links {
            if links.len() > 1000 {
                return Err(WorkNoteError::ValidationError("関連リンクは1000文字以内で入力してください".to_string()));
            }
        }

        Ok(())
    }
}
```

---

## 5. ディレクトリ構造

```
WorkNote/
├── src-tauri/                      # Rustバックエンド
│   ├── Cargo.toml                  # Rust依存関係
│   ├── tauri.conf.json             # Tauri設定
│   ├── icons/                      # アプリアイコン
│   └── src/
│       ├── main.rs                 # エントリーポイント
│       ├── commands/               # Tauriコマンド
│       │   ├── mod.rs
│       │   ├── knowledge.rs        # ナレッジ保存関連
│       │   ├── config.rs           # 設定関連
│       │   └── draft.rs            # 下書き関連
│       ├── services/               # ビジネスロジック
│       │   ├── mod.rs
│       │   ├── git_service.rs      # Git操作
│       │   ├── file_generator.rs   # Markdown生成
│       │   ├── config_manager.rs   # 設定管理
│       │   └── draft_manager.rs    # 下書き管理
│       ├── models/                 # データ構造
│       │   ├── mod.rs
│       │   ├── knowledge.rs
│       │   ├── config.rs
│       │   └── draft.rs
│       └── utils/                  # ユーティリティ
│           ├── mod.rs
│           ├── validation.rs       # バリデーション
│           └── error.rs            # エラー定義
│
├── src/                            # Svelteフロントエンド
│   ├── main.ts                     # エントリーポイント
│   ├── App.svelte                  # ルートコンポーネント
│   ├── components/                 # UIコンポーネント
│   │   ├── QuickInputWindow.svelte
│   │   ├── DetailInputWindow.svelte
│   │   ├── SettingsWindow.svelte
│   │   ├── PreviewPane.svelte
│   │   ├── FormField.svelte
│   │   └── ErrorMessage.svelte
│   ├── lib/                        # ロジック
│   │   ├── tauri-bridge.ts         # Tauriコマンド呼び出し
│   │   ├── validation.ts           # バリデーション
│   │   ├── markdown-renderer.ts    # Markdown変換
│   │   └── types.ts                # 型定義
│   ├── stores/                     # Svelte Store
│   │   ├── config.ts               # 設定ストア
│   │   └── drafts.ts               # 下書きストア
│   └── styles/                     # スタイル
│       └── global.css              # Tailwind CSS
│
├── public/                         # 静的ファイル
│   └── favicon.ico
│
├── docs/                           # ドキュメント
│   └── michi/
│       └── 20260131-worknote/
│           └── spec/
│               ├── requirements.md
│               ├── architecture.md
│               ├── sequence.md
│               └── design.md
│
├── tests/                          # テスト
│   ├── unit/                       # 単体テスト
│   ├── integration/                # 統合テスト
│   └── e2e/                        # E2Eテスト
│
├── package.json                    # Node.js依存関係
├── tsconfig.json                   # TypeScript設定
├── vite.config.ts                  # Vite設定
├── tailwind.config.cjs             # Tailwind設定
└── README.md                       # プロジェクト概要
```

---

## 6. 技術的決定事項

### 6.1 Markdownレンダリング

**フロントエンド（プレビュー用）**:
- ライブラリ: `marked`
- **サニタイゼーション**: `DOMPurify`（**XSS対策必須**）
- シンタックスハイライト: `highlight.js`

**実装例（TypeScript）**:

```typescript
import DOMPurify from 'dompurify';
import { marked } from 'marked';
import hljs from 'highlight.js';

/**
 * Markdownを安全なHTMLに変換
 * XSS対策としてDOMPurifyでサニタイゼーションを必ず実行
 */
export function renderMarkdownSafe(markdown: string): string {
  // Step 1: MarkdownをHTMLに変換
  const rawHTML = marked(markdown, {
    highlight: (code, lang) => {
      if (lang && hljs.getLanguage(lang)) {
        try {
          return hljs.highlight(code, { language: lang }).value;
        } catch (err) {
          console.error('Highlight error:', err);
        }
      }
      return code;
    }
  });

  // Step 2: XSS対策 - DOMPurifyでサニタイゼーション
  const cleanHTML = DOMPurify.sanitize(rawHTML, {
    // 許可するHTMLタグ
    ALLOWED_TAGS: [
      'h1', 'h2', 'h3', 'h4', 'h5', 'h6',
      'p', 'br', 'hr',
      'ul', 'ol', 'li',
      'code', 'pre',
      'a', 'strong', 'em', 'del',
      'blockquote',
      'table', 'thead', 'tbody', 'tr', 'th', 'td'
    ],
    // 許可する属性
    ALLOWED_ATTR: ['href', 'class', 'id'],
    // リンクのプロトコルを制限（XSS防止）
    ALLOWED_URI_REGEXP: /^(?:(?:(?:f|ht)tps?|mailto|tel):|[^a-z]|[a-z+.\-]+(?:[^a-z+.\-:]|$))/i
  });

  return cleanHTML;
}
```

**セキュリティ要件**:
- ✅ すべてのMarkdown→HTML変換後にDOMPurifyでサニタイゼーション必須
- ✅ `<script>`, `<iframe>`, `onerror`等の危険なタグ・属性は除去
- ✅ `javascript:` プロトコルのリンクは除去
- ✅ XSS攻撃ベクトル（例: `<img src=x onerror="alert('XSS')">`）をブロック

**テストケース**:

```typescript
// テスト: 悪意あるHTMLがサニタイズされることを確認
describe('renderMarkdownSafe', () => {
  it('should sanitize script tags', () => {
    const input = '<script>alert("XSS")</script>';
    const output = renderMarkdownSafe(input);
    expect(output).not.toContain('<script>');
  });

  it('should sanitize onerror attributes', () => {
    const input = '<img src=x onerror="alert(\'XSS\')">';
    const output = renderMarkdownSafe(input);
    expect(output).not.toContain('onerror');
  });

  it('should sanitize javascript: protocol', () => {
    const input = '[Click me](javascript:alert("XSS"))';
    const output = renderMarkdownSafe(input);
    expect(output).not.toContain('javascript:');
  });

  it('should allow safe markdown', () => {
    const input = '# Heading\n\n- List item\n\n**Bold** text';
    const output = renderMarkdownSafe(input);
    expect(output).toContain('<h1>');
    expect(output).toContain('<li>');
    expect(output).toContain('<strong>');
  });
});
```

**バックエンド（Markdown生成）**:
- 標準ライブラリのみ使用（テンプレート文字列）
- バックエンドで生成するMarkdownは信頼できるため、サニタイゼーション不要

---

### 6.2 グローバルショートカット

**ライブラリ**: `tauri-plugin-global-shortcut`

**実装例**:

```rust
use tauri_plugin_global_shortcut::GlobalShortcutExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::init())
        .setup(|app| {
            let handle = app.handle();
            app.global_shortcut().register("CommandOrControl+Shift+K", move || {
                // QuickInputWindowを表示
                handle.emit("show-quick-input", ()).unwrap();
            })?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

### 6.3 設定ストレージ

**ライブラリ**: `tauri-plugin-store`

**実装例**:

```rust
use tauri_plugin_store::StoreBuilder;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let store = StoreBuilder::new("config.json").build(app.handle())?;
            // 設定読み込み
            let config: Config = store.get("config").unwrap_or_default();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

### 6.4 通知表示

**ライブラリ**: Tauri標準API (`tauri::api::notification`)

**実装例**:

```rust
use tauri::api::notification::Notification;

fn show_success_notification(app_handle: &tauri::AppHandle, message: &str) {
    Notification::new(&app_handle.config().tauri.bundle.identifier)
        .title("WorkNote")
        .body(message)
        .show()
        .unwrap();
}
```

---

## 7. パフォーマンス最適化

### 7.1 起動時間短縮

| 施策 | 効果 |
|------|------|
| 遅延ロード | 初期ロードを最小化、必要に応じて読み込み |
| コード分割 | ウィンドウごとにバンドル分割 |
| 最小限の初期化 | 設定読み込みのみ、他は遅延 |

---

### 7.2 メモリ使用量削減

| 施策 | 効果 |
|------|------|
| 不要なデータ解放 | 下書き一覧など、使用後に解放 |
| キャッシュ制御 | プレビューのキャッシュサイズ制限 |
| リソース最適化 | 画像サイズ、フォント最小化 |

---

### 7.3 バンドルサイズ削減

| 施策 | 効果 |
|------|------|
| Tree shaking | 未使用コードの削除 |
| 依存関係最小化 | 必要最小限のライブラリのみ使用 |
| 圧縮 | Viteの最適化設定 |

---

## まとめ

WorkNoteの詳細設計は以下の特徴を持ちます：

1. **明確なデータモデル**: JSONスキーマで厳密に定義
2. **型安全なAPI**: TypeScript + Rustで二重チェック
3. **堅牢なエラーハンドリング**: ユーザーフレンドリーなメッセージ
4. **シンプルな構造**: 責務分離、保守しやすい

---

**文書情報**

| 項目 | 内容 |
|------|------|
| 作成日 | 2026-01-31 |
| 作成者 | Claude Code (michi:create-design) |
| バージョン | 1.0 |
| ステータス | 承認待ち |
