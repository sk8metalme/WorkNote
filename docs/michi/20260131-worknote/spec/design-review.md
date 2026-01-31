# 設計レビュー結果: WorkNote

## サマリー

| 項目 | 結果 |
|------|------|
| **総合評価** | ✅ **合格** |
| **重大な問題** | 0件（修正完了） |
| **軽微な問題** | 3件 |
| **改善提案** | 5件 |
| **レビュー日** | 2026-01-31 |
| **レビュアー** | Claude Code (michi:review-design) |
| **修正日** | 2026-01-31 |

**総評**:
WorkNoteの設計は全体的に良好です。C4モデルによる明確なアーキテクチャ設計、型安全なデータモデル、堅牢なエラーハンドリングが実装されています。**XSS対策（CRI-001）も修正完了し、設計レビューは合格となりました。**

---

## 1. 技術的妥当性

### 1.1 アーキテクチャの整合性 ✅

**評価**: 合格

**良い点**:
- C4モデルの4レベル（Context, Container, Component, Code）で明確に設計
- フロントエンド（Svelte）とバックエンド（Rust）の責務分離が明確
- Tauri Commandによる型安全な通信設計
- 一方向の依存関係（UI → ロジック → インフラ）

**確認項目**:
- ✅ レイヤー分離が適切
- ✅ 依存関係が一方向
- ✅ 責任分担が明確

---

### 1.2 データモデルの設計 ✅

**評価**: 合格

**良い点**:
- JSONスキーマで厳密に定義（config.json, drafts/*.json）
- TypeScriptとRustの型定義が一致
- バージョニングによるマイグレーション対応（config.version）

**確認項目**:
- ✅ データ正規化: 第3正規形相当（ファイルベースのため厳密には適用外だが、適切に構造化）
- ✅ 型安全性: TypeScript + Rustで二重保証
- ✅ 拡張性: バージョンフィールドによる将来のマイグレーション対応

---

### 1.3 API設計 ✅

**評価**: 合格

**良い点**:
- Tauri Commandベースで型安全
- リクエスト・レスポンスの型定義が明確
- 8つのコマンドが適切に分離（save_knowledge, load_config, save_draft等）

**確認項目**:
- ✅ 命名規則の一貫性: snake_caseで統一
- ✅ エラーハンドリング: Result型で適切に処理
- ✅ 非同期処理: `async fn` で適切に実装予定

**注意点**:
- Tauri CommandはREST APIではないため、RESTful準拠は評価対象外

---

## 2. セキュリティチェック

### 2.1 OWASP Top 10 準拠 ⚠️

**評価**: 条件付き合格（修正必須項目あり）

#### ✅ 対策済み

| 項目 | 対策 | 評価 |
|------|------|------|
| **SQL Injection** | データベース不使用（ファイルベース） | N/A |
| **CSRF** | デスクトップアプリのためCSRF不適用 | N/A |
| **認証・認可** | Git認証をOS/ツールに委譲 | ✅ 適切 |
| **機密データ露出** | Git認証情報をアプリ内に保存しない | ✅ 適切 |
| **入力検証** | フロントエンド+バックエンドで二重チェック | ✅ 適切 |

#### ❌ 対策不足（重大）

| 項目 | 問題 | 影響 | 推奨対応 |
|------|------|------|----------|
| **XSS (Cross-Site Scripting)** | Markdownプレビュー時のサニタイゼーションが不明確 | ユーザーが入力した悪意あるHTMLが実行される可能性 | DOMPurifyによるサニタイゼーション必須 |

---

### 2.2 データ暗号化 ✅

**評価**: 合格

| 項目 | 対策 | 評価 |
|------|------|------|
| **パスワード保存** | パスワードは保存しない | ✅ 適切 |
| **通信の暗号化** | Git pushはGit CLIが処理（SSH/HTTPS） | ✅ 適切 |
| **設定ファイル** | ユーザー権限でのみアクセス可能 | ✅ 適切 |

---

### 2.3 セキュリティ上の懸念事項

#### CRI-001: XSS対策の欠如 ❌ **重大**

**説明**:
design.md の 6.1節でMarkdownレンダリングライブラリ（marked or markdown-it）とDOMPurifyが記載されていますが、**実装の詳細が不明確**です。

**影響**:
ユーザーが以下のようなMarkdownを入力した場合、XSSが発生する可能性があります：

```markdown
<img src=x onerror="alert('XSS')">
<script>alert('XSS')</script>
```

**推奨対応**:
1. **必ずDOMPurifyでサニタイゼーション**を実行
2. 実装例をdesign.mdに追加:

```typescript
import DOMPurify from 'dompurify';
import { marked } from 'marked';

function renderMarkdown(markdown: string): string {
  const rawHTML = marked(markdown);
  const cleanHTML = DOMPurify.sanitize(rawHTML);
  return cleanHTML;
}
```

3. サニタイゼーションのテストケースを追加

---

## 3. テスト計画準備状況

### 3.1 テストしやすい設計 ⚠️

**評価**: 改善の余地あり

**良い点**:
- コンポーネントの責務分離が明確
- 型定義が明確でモック作成が容易

**改善点**:

#### LI-001: 依存性注入（DI）の明示的な設計が不足

**説明**:
GitServiceやConfigManagerなどのコンポーネントが、Git CLIやファイルシステムに直接依存しています。テスト時にモック化が難しくなります。

**影響**:
単体テスト時に実際のGit CLIを呼び出してしまい、テストが遅くなる、環境依存になる。

**推奨対応**:
トレイト（Rust）やインターフェース（TypeScript）で抽象化する：

```rust
// 抽象化
pub trait GitOperations {
    fn commit_and_push(&self, file: &Path, message: &str) -> Result<GitResult, WorkNoteError>;
}

// 本番実装
pub struct GitServiceImpl;
impl GitOperations for GitServiceImpl {
    fn commit_and_push(&self, file: &Path, message: &str) -> Result<GitResult, WorkNoteError> {
        // Git CLI呼び出し
    }
}

// テスト用モック
pub struct MockGitService;
impl GitOperations for MockGitService {
    fn commit_and_push(&self, file: &Path, message: &str) -> Result<GitResult, WorkNoteError> {
        // モック実装
        Ok(GitResult { success: true, ... })
    }
}
```

---

### 3.2 モックしやすいインターフェース ⚠️

**評価**: 改善の余地あり

**良い点**:
- 型定義が明確
- エラー型が定義されている

**改善点**:

#### LI-002: 外部依存の抽象化不足

**説明**:
以下の外部依存がハードコードされています：
- Git CLI（std::process::Command）
- ファイルシステム（std::fs）
- tauri-plugin-store

**推奨対応**:
インターフェース（トレイト）で抽象化し、テスト時にモックを注入できるようにする。

---

## 4. その他の問題

### 4.1 軽微な問題

#### LI-003: ファイル名変換ロジックの不完全性

**説明**:
design.md の 1.3節でファイル名生成ルールが記載されていますが、**日本語→ローマ字変換の実装が不明**です。

現在の実装例:
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

**問題点**:
- 日本語文字が単純に除去されるだけ
- "CPU高騰対応" → "cpu" となり、情報が失われる

**推奨対応**:
1. ローマ字変換ライブラリを使用（例: `wana_kana`）
2. または、タイムスタンプ+連番で一意性を保証:

```rust
fn generate_filename(title: &str, timestamp: &str) -> String {
    let slug = title
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "-");
    format!("{}-{}.md", timestamp, slug)
}
```

---

## 5. 改善提案

### REC-001: パフォーマンスモニタリング設計の追加

**説明**:
NFR-001でパフォーマンス要件（起動1秒以内、メモリ50MB以下等）が定義されていますが、**実測方法が不明**です。

**効果**:
開発中にパフォーマンス劣化を早期検知できる。

**推奨対応**:
1. パフォーマンスメトリクスの収集ポイントを設計に追加:
   - アプリ起動時間
   - ウィンドウ表示時間
   - メモリ使用量
   - Git操作時間

2. ログ設計に組み込む:

```rust
use std::time::Instant;

fn measure_performance<F, T>(name: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    log::info!("Performance: {} took {:?}", name, duration);
    result
}
```

---

### REC-002: ログ設計の明示化

**説明**:
エラーハンドリングは設計されていますが、**ログ設計が不明**です。

**効果**:
運用時のトラブルシューティングが容易になる。

**推奨対応**:
1. ログレベルの定義（ERROR, WARN, INFO, DEBUG）
2. ログ出力先の設計:
   - macOS: `~/Library/Logs/WorkNote/`
   - ローテーション: 日次、最大7日分保持

3. ログライブラリ: `log` + `env_logger` or `tracing`

---

### REC-003: 設定ファイルのバリデーション強化

**説明**:
config.jsonのスキーマは定義されていますが、**不正な設定値への対応が不明**です。

**効果**:
ユーザーが手動で設定ファイルを編集した際のエラーを防止。

**推奨対応**:
1. 設定読み込み時のバリデーション:
   - リポジトリパスの存在確認
   - Author emailの形式確認
   - ショートカットキーの妥当性確認

2. バリデーション失敗時はデフォルト値で復元

---

### REC-004: Git操作のリトライロジック追加

**説明**:
ネットワークエラー時のリトライがユーザー操作に依存しています。

**効果**:
一時的なネットワーク障害時に自動復旧できる。

**推奨対応**:
1. Git push時の自動リトライ（最大3回、指数バックオフ）
2. リトライ状況をUIに表示

```rust
async fn push_with_retry(max_retries: u32) -> Result<GitResult, WorkNoteError> {
    for attempt in 1..=max_retries {
        match self.push() {
            Ok(result) => return Ok(result),
            Err(WorkNoteError::NetworkError(_)) if attempt < max_retries => {
                tokio::time::sleep(Duration::from_secs(2u64.pow(attempt))).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Err(WorkNoteError::NetworkError("Max retries exceeded".to_string()))
}
```

---

### REC-005: 下書きの有効期限設定

**説明**:
下書きが無期限に保存され、ディスク容量を消費し続けます。

**効果**:
古い下書きを自動削除し、ディスク容量を節約。

**推奨対応**:
1. 下書きに有効期限（デフォルト: 30日）を追加
2. アプリ起動時に期限切れ下書きを自動削除
3. 設定画面で有効期限をカスタマイズ可能に

---

## 6. 合格条件

### 必須修正項目（完了）

- [x] **CRI-001: XSS対策の実装** - DOMPurifyによるサニタイゼーションをdesign.mdに明記し、実装時に必ず適用すること ✅ **修正完了（2026-01-31）**

### 推奨修正項目（軽微な問題）

- [ ] **LI-001: 依存性注入の設計** - テスト容易性向上のため、トレイトで抽象化
- [ ] **LI-002: 外部依存の抽象化** - Git CLI、ファイルシステムのインターフェース化
- [ ] **LI-003: ファイル名変換ロジックの改善** - ローマ字変換またはタイムスタンプ使用

### 推奨実施項目（改善提案）

- [ ] **REC-001: パフォーマンスモニタリング設計** - メトリクス収集ポイント追加
- [ ] **REC-002: ログ設計の明示化** - ログレベル、出力先、ローテーション定義
- [ ] **REC-003: 設定ファイルバリデーション強化** - 不正値への対応
- [ ] **REC-004: Git操作のリトライロジック** - 自動リトライ実装
- [ ] **REC-005: 下書き有効期限設定** - 古い下書き自動削除

---

## 7. 次のステップ

### Step 1: 必須修正の実施

**CRI-001（XSS対策）** を修正してください。

**修正内容**:
1. `docs/michi/20260131-worknote/spec/design.md` の 6.1節を以下に更新:

```markdown
### 6.1 Markdownレンダリング

**フロントエンド（プレビュー用）**:
- ライブラリ: `marked`
- **サニタイゼーション**: `DOMPurify`（XSS対策必須）
- シンタックスハイライト: `highlight.js`

**実装例**:

\`\`\`typescript
import DOMPurify from 'dompurify';
import { marked } from 'marked';

function renderMarkdownSafe(markdown: string): string {
  const rawHTML = marked(markdown);
  const cleanHTML = DOMPurify.sanitize(rawHTML, {
    ALLOWED_TAGS: ['h1', 'h2', 'h3', 'p', 'ul', 'ol', 'li', 'code', 'pre', 'a', 'strong', 'em'],
    ALLOWED_ATTR: ['href', 'class']
  });
  return cleanHTML;
}
\`\`\`

**セキュリティ要件**:
- すべてのMarkdown→HTML変換後にDOMPurifyでサニタイゼーション必須
- `<script>`, `<iframe>`, `onerror`等の危険なタグ・属性は除去
```

### Step 2: 設計書の更新

修正後、以下のコマンドで設計書を再確認:

```bash
/michi review-design 20260131-worknote
```

### Step 3: テスト計画または タスク分割へ進む

設計レビューが合格したら、次のステップへ進みます：

**推奨**: テスト計画作成
```bash
/michi plan-tests 20260131-worknote
```

または

**代替**: タスク分割（テスト計画スキップ）
```bash
/michi create-tasks 20260131-worknote
```

---

## まとめ

WorkNoteの設計は**全体的に高品質**です。明確なアーキテクチャ、型安全な設計、堅牢なエラーハンドリングが実装されています。

**ただし、XSS対策の欠如という重大な問題があります。** この問題を修正すれば、設計レビューは合格となります。

軽微な問題や改善提案は、実装の優先度に応じて対応してください。特に、テスト容易性向上のための依存性注入設計は、Phase 1 MVP実装前に対応することを強く推奨します。

---

**文書情報**

| 項目 | 内容 |
|------|------|
| 作成日 | 2026-01-31 |
| 作成者 | Claude Code (michi:review-design) |
| バージョン | 1.0 |
| ステータス | レビュー完了 - 条件付き合格 |
