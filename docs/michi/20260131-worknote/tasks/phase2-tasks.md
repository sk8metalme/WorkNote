# Phase 2 タスクリスト: WorkNote

## 概要

Phase 2では13のタスクを実装します。タスクは依存関係に基づいて順序付けされています。

## タスク一覧

### TASK-201: tauri-plugin-global-shortcut導入

**優先度:** 🔴 高
**予想工数:** 0.5日
**依存関係:** なし
**担当:** TBD

**目的:**
グローバルショートカット機能を有効化するため、tauri-plugin-global-shortcutを導入します。

**実装内容:**
1. `src-tauri/Cargo.toml`に依存追加
   ```toml
   [dependencies]
   tauri-plugin-global-shortcut = "2.0.0"
   ```
2. `src-tauri/src/lib.rs`でプラグイン登録
   ```rust
   use tauri_plugin_global_shortcut::GlobalShortcutExt;

   .plugin(tauri_plugin_global_shortcut::init())
   ```
3. 初期設定で`CommandOrControl+Shift+K`を登録

**受入条件:**
- [ ] Cargo.tomlに依存が追加されている
- [ ] `cargo build`が成功する
- [ ] プラグインが正しく初期化される

**テスト:**
- [ ] グローバルショートカット登録APIが呼び出せる
- [ ] アプリ起動時にエラーが発生しない

---

### TASK-202: ShortcutManager実装

**優先度:** 🔴 高
**予想工数:** 1日
**依存関係:** TASK-201
**担当:** TBD

**目的:**
グローバルショートカットの登録/解除、イベントハンドリングを管理するサービスを実装します。

**実装内容:**
1. `src-tauri/src/services/shortcut_manager.rs`作成
   ```rust
   pub struct ShortcutManager {
       app: AppHandle,
   }

   impl ShortcutManager {
       pub fn new(app: AppHandle) -> Self {
           Self { app }
       }

       pub fn register_shortcut(&self, shortcut: &str) -> Result<()> {
           // tauri-plugin-global-shortcut APIを使用
           // イベントハンドラでQuickInput Windowの表示/非表示切り替え
       }

       pub fn unregister_shortcut(&self) -> Result<()> {
           // ショートカット解除
       }
   }
   ```
2. `src-tauri/src/services/mod.rs`にモジュール追加
3. アプリ起動時にデフォルトショートカット登録

**受入条件:**
- [ ] ShortcutManagerが実装されている
- [ ] register_shortcut()でショートカット登録できる
- [ ] unregister_shortcut()でショートカット解除できる
- [ ] イベントハンドラが正しく動作する

**テスト:**
- [ ] 単体テスト: ショートカット登録/解除
- [ ] 統合テスト: イベント発火でウィンドウ表示切り替え
- [ ] エラーケース: 無効なショートカット文字列

---

### TASK-203: QuickInput Window UI実装

**優先度:** 🔴 高
**予想工数:** 1.5日
**依存関係:** なし
**担当:** TBD

**目的:**
Spotlight風のクイック入力UIを実装します。

**実装内容:**
1. `src/components/QuickInputWindow.svelte`作成
   - タイトル入力（required）
   - カテゴリ選択（required）
   - 重要度選択（required）
   - 「詳細入力へ →」ボタン
   - 「💾 クイック保存」ボタン
2. スタイリング（Tailwind CSS）
   - 固定サイズ: 480×280px
   - 角丸、影付き
   - 中央上部表示
3. キーボードショートカット
   - Enter: クイック保存実行
   - Esc: ウィンドウを閉じる
   - Tab: 次の入力欄へ移動
4. バリデーション
   - lib/validation.tsを再利用
   - 必須フィールドのみ検証

**受入条件:**
- [ ] UIが仕様通りのサイズで表示される
- [ ] タイトル入力欄に自動フォーカス
- [ ] Enterキーでクイック保存が実行される
- [ ] Escキーでウィンドウが閉じる
- [ ] バリデーションエラーが表示される

**テスト:**
- [ ] UI表示確認
- [ ] キーボードショートカット動作確認
- [ ] バリデーション動作確認

---

### TASK-204: QuickInput Window統合

**優先度:** 🔴 高
**予想工数:** 1日
**依存関係:** TASK-202, TASK-203
**担当:** TBD

**目的:**
QuickInput WindowをTauriアプリに統合し、グローバルショートカットで表示できるようにします。

**実装内容:**
1. `src-tauri/tauri.conf.json`にquick-inputウィンドウ定義追加
   ```json
   "windows": [
     {
       "label": "quick-input",
       "title": "WorkNote - クイック入力",
       "url": "/quick-input",
       "width": 480,
       "height": 280,
       "resizable": false,
       "center": true,
       "alwaysOnTop": true,
       "visible": false
     }
   ]
   ```
2. `src/routes/quick-input/+page.svelte`作成
   - QuickInputWindowコンポーネントを表示
3. `src-tauri/src/commands/window.rs`作成
   ```rust
   #[tauri::command]
   pub fn show_quick_input_window(app: AppHandle) -> Result<()> {
       // quick-inputウィンドウを表示
   }

   #[tauri::command]
   pub fn hide_quick_input_window(app: AppHandle) -> Result<()> {
       // quick-inputウィンドウを非表示
   }
   ```
4. ShortcutManagerからwindow commandsを呼び出す

**受入条件:**
- [ ] グローバルショートカットでウィンドウが表示される
- [ ] Escキーでウィンドウが非表示になる
- [ ] 常に最前面に表示される
- [ ] 画面中央上部に配置される

**テスト:**
- [ ] グローバルショートカット起動
- [ ] ウィンドウ表示/非表示切り替え
- [ ] alwaysOnTop動作確認

---

### TASK-205: クイック保存機能実装

**優先度:** 🔴 高
**予想工数:** 1日
**依存関係:** TASK-204
**担当:** TBD

**目的:**
QuickInputから最小限の入力でMarkdownファイルを生成し、Git commit & pushを実行します。

**実装内容:**
1. `src/lib/tauri-bridge.ts`に`quickSaveKnowledge`関数追加
   ```typescript
   export async function quickSaveKnowledge(
     title: string,
     category: Category,
     severity: Severity
   ): Promise<SaveKnowledgeResponse> {
     // ...
   }
   ```
2. `src-tauri/src/commands/knowledge.rs`に`quick_save_knowledge`コマンド追加
   ```rust
   #[tauri::command]
   pub async fn quick_save_knowledge(
       app: AppHandle,
       title: String,
       category: Category,
       severity: Severity,
   ) -> std::result::Result<SaveKnowledgeResponse, ErrorInfo> {
       // 他のフィールドは空文字列で KnowledgeInput を生成
       // FileGenerator, GitService を呼び出す
   }
   ```
3. QuickInputWindow.svelteから呼び出し
4. 成功時にウィンドウを閉じる

**受入条件:**
- [ ] クイック保存でMarkdownファイルが生成される
- [ ] frontmatterに必須フィールド（title, category, severity）が含まれる
- [ ] 他のフィールドは空または空配列
- [ ] Git commit & pushが実行される
- [ ] 成功時にウィンドウが閉じる

**テスト:**
- [ ] クイック保存のE2Eテスト
- [ ] 生成されたMarkdownファイルの検証
- [ ] Git commit履歴の確認

---

### TASK-206: GitServiceにPR作成モード追加

**優先度:** 🔴 高
**予想工数:** 2日
**依存関係:** なし
**担当:** TBD

**目的:**
featureブランチを作成してpushし、GitHub PR作成URLを生成する機能を追加します。

**実装内容:**
1. `src-tauri/src/services/git_service.rs`を拡張
   ```rust
   impl GitService {
       pub fn commit_and_push_pr(
           &self,
           file_path: &Path,
           title: &str,
       ) -> Result<(String, String)> {
           // 1. git checkout main && git pull
           // 2. featureブランチ作成: feature/worknote-{title}-{timestamp}
           // 3. git add & git commit
           // 4. git push origin feature/worknote-xxx
           // 5. PR URL生成
           // return (commit_hash, pr_url)
       }

       fn get_remote_info(&self) -> Result<(String, String)> {
           // git remote get-url origin
           // URLからowner/repoを抽出
           // return (owner, repo)
       }

       fn generate_pr_url(&self, branch: &str) -> Result<String> {
           let (owner, repo) = self.get_remote_info()?;
           Ok(format!("https://github.com/{}/{}/compare/{}", owner, repo, branch))
       }
   }
   ```
2. タイムスタンプ生成ロジック追加
3. ブランチ名のサニタイズ（特殊文字除去）

**受入条件:**
- [ ] featureブランチが正しく作成される
- [ ] featureブランチにpushされる
- [ ] PR URL生成が正しい
- [ ] デフォルトブランチに影響しない
- [ ] エラーハンドリングが適切

**テスト:**
- [ ] 単体テスト: get_remote_info()
- [ ] 単体テスト: generate_pr_url()
- [ ] 統合テスト: commit_and_push_pr()の全フロー
- [ ] エラーケース: リモートURLが無効

---

### TASK-207: Config型にcommitMode追加

**優先度:** 🔴 高
**予想工数:** 0.5日
**依存関係:** なし
**担当:** TBD

**目的:**
Git操作モード（直接Push / PR作成）を設定で切り替えられるようにします。

**実装内容:**
1. `src-tauri/src/models/config.rs`を拡張
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
   #[serde(rename_all = "lowercase")]
   pub enum CommitMode {
       #[serde(rename = "direct")]
       DirectPush,
       #[serde(rename = "pr")]
       CreatePR,
   }

   impl Default for CommitMode {
       fn default() -> Self {
           CommitMode::DirectPush
       }
   }

   pub struct GitConfig {
       pub repository_path: String,
       pub save_path: String,
       pub default_branch: String,
       #[serde(default)]
       pub commit_mode: CommitMode,
   }
   ```
2. `src/lib/types.ts`にCommitMode型追加
   ```typescript
   export type CommitMode = 'direct' | 'pr';

   export interface GitConfig {
       repositoryPath: string;
       savePath: string;
       defaultBranch: string;
       commitMode?: CommitMode;
   }
   ```

**受入条件:**
- [ ] CommitMode enumが定義されている
- [ ] GitConfigにcommitModeフィールドが追加されている
- [ ] デフォルト値がDirectPush
- [ ] 既存のconfig.jsonが読み込める（後方互換性）

**テスト:**
- [ ] 単体テスト: CommitModeのシリアライズ/デシリアライズ
- [ ] 統合テスト: commitModeがないconfig.jsonを読み込む

---

### TASK-208: SettingsWindowにcommitMode選択追加

**優先度:** 🔴 高
**予想工数:** 0.5日
**依存関係:** TASK-207
**担当:** TBD

**目的:**
設定画面でGit操作モードを選択できるようにします。

**実装内容:**
1. `src/components/SettingsWindow.svelte`を拡張
   ```svelte
   <div>
     <label class="block text-sm font-medium mb-1">コミットモード</label>
     <div class="flex gap-4">
       <label class="flex items-center">
         <input type="radio" bind:group={config.git.commitMode} value="direct" />
         <span class="ml-2">直接Push（デフォルトブランチに直接）</span>
       </label>
       <label class="flex items-center">
         <input type="radio" bind:group={config.git.commitMode} value="pr" />
         <span class="ml-2">PR作成（feature ブランチを作成）</span>
       </label>
     </div>
   </div>
   ```
2. デフォルト値: `direct`
3. 保存時にconfig.jsonに書き込み

**受入条件:**
- [ ] ラジオボタンで選択できる
- [ ] commitMode変更が保存される
- [ ] アプリ再起動後も設定が保持される

**テスト:**
- [ ] UI表示確認
- [ ] 設定保存確認
- [ ] 永続化確認

---

### TASK-209: save_knowledge CommandでcommitMode対応

**優先度:** 🔴 高
**予想工数:** 1日
**依存関係:** TASK-206, TASK-207
**担当:** TBD

**目的:**
ナレッジ保存時にcommitModeに応じてGit操作を切り替えます。

**実装内容:**
1. `src-tauri/src/commands/knowledge.rs`を修正
   ```rust
   #[tauri::command]
   pub async fn save_knowledge(
       app: AppHandle,
       input: KnowledgeInput,
   ) -> std::result::Result<SaveKnowledgeResponse, ErrorInfo> {
       let config = config_manager.load_config()?;

       // Markdown生成
       let file_path = file_generator.write_file(&input)?;

       // commitModeに応じて分岐
       let (commit_hash, pr_url) = match config.git.commit_mode {
           CommitMode::DirectPush => {
               let hash = git_service.commit_and_push(&file_path, &input.title)?;
               (hash, None)
           }
           CommitMode::CreatePR => {
               let (hash, url) = git_service.commit_and_push_pr(&file_path, &input.title)?;
               (hash, Some(url))
           }
       };

       Ok(SaveKnowledgeResponse {
           commit_hash,
           file_path: file_path.to_string_lossy().to_string(),
           pr_url,
       })
   }
   ```
2. SaveKnowledgeResponse型を拡張
   ```rust
   #[derive(Debug, Serialize)]
   pub struct SaveKnowledgeResponse {
       pub commit_hash: String,
       pub file_path: String,
       pub pr_url: Option<String>,
   }
   ```
3. フロントエンドでpr_urlを処理
   - pr_urlがある場合、通知にリンクを表示
   - クリックでブラウザを開く

**受入条件:**
- [ ] DirectPushモードで直接pushされる
- [ ] CreatePRモードでfeatureブランチが作成される
- [ ] レスポンスにpr_urlが含まれる
- [ ] pr_url URLクリックでブラウザが開く

**テスト:**
- [ ] DirectPushモードのE2Eテスト
- [ ] CreatePRモードのE2Eテスト
- [ ] エラーケース: Git操作失敗

---

### TASK-210: Markdownレンダラー実装

**優先度:** 🟡 中
**予想工数:** 1日
**依存関係:** なし
**担当:** TBD

**目的:**
KnowledgeInputからHTML形式のMarkdownプレビューを生成します。

**実装内容:**
1. `src-tauri/Cargo.toml`に依存追加
   ```toml
   pulldown-cmark = "0.9"
   ```
2. `src-tauri/src/services/markdown_renderer.rs`作成
   ```rust
   use pulldown_cmark::{Parser, html};

   pub struct MarkdownRenderer;

   impl MarkdownRenderer {
       pub fn render_markdown(input: &KnowledgeInput) -> Result<String> {
           // 1. FileGenerator::generate_markdown() を呼び出してMarkdown文字列を取得
           // 2. pulldown-cmarkでHTML変換
           // 3. frontmatterセクションはYAMLとしてプリフォーマット表示
           // return HTML文字列
       }
   }
   ```
3. `src-tauri/src/services/mod.rs`にモジュール追加

**受入条件:**
- [ ] Markdown見出し、リスト、コードブロックが正しくレンダリングされる
- [ ] frontmatterがYAMLとして表示される
- [ ] XSS対策: HTMLエスケープが適用されている

**テスト:**
- [ ] 単体テスト: 各Markdown要素のレンダリング
- [ ] 単体テスト: frontmatter表示
- [ ] セキュリティテスト: XSS攻撃のテスト

---

### TASK-211: PreviewPane UI実装

**優先度:** 🟡 中
**予想工数:** 0.5日
**依存関係:** なし
**担当:** TBD

**目的:**
Markdownプレビューを表示するUIコンポーネントを実装します。

**実装内容:**
1. `src/components/PreviewPane.svelte`作成
   ```svelte
   <script lang="ts">
     export let content: string; // レンダリング済みHTML
   </script>

   <div class="preview-pane prose prose-sm max-w-none overflow-y-auto h-full p-6">
     {@html content}
   </div>

   <style>
     .preview-pane {
       /* Tailwind CSS Prose plugin */
     }
   </style>
   ```
2. Tailwind CSS Proseプラグイン導入
   ```bash
   npm install -D @tailwindcss/typography
   ```
3. `tailwind.config.js`に追加
   ```js
   plugins: [require('@tailwindcss/typography')]
   ```

**受入条件:**
- [ ] HTMLコンテンツが正しく表示される
- [ ] スクロールが動作する
- [ ] Proseスタイルが適用されている

**テスト:**
- [ ] UI表示確認
- [ ] スクロール動作確認
- [ ] スタイリング確認

---

### TASK-212: DetailInputWindowにプレビュー統合

**優先度:** 🟡 中
**予想工数:** 1.5日
**依存関係:** TASK-210, TASK-211
**担当:** TBD

**目的:**
詳細入力ウィンドウにMarkdownプレビュー機能を統合します。

**実装内容:**
1. `src/components/DetailInputWindow.svelte`を拡張
   ```svelte
   <script lang="ts">
     import PreviewPane from './PreviewPane.svelte';
     import { debounce } from 'lodash-es';

     let showPreview = false;
     let previewContent = '';

     const updatePreview = debounce(async () => {
       if (showPreview) {
         previewContent = await renderMarkdown(input);
       }
     }, 100);

     $: if (showPreview) {
       updatePreview();
     }
   </script>

   <div class={showPreview ? 'grid grid-cols-2 gap-4' : ''}>
     <div>
       <!-- 既存のフォーム -->
     </div>
     {#if showPreview}
       <PreviewPane content={previewContent} />
     {/if}
   </div>

   <label class="flex items-center">
     <input type="checkbox" bind:checked={showPreview} />
     <span class="ml-2">プレビュー表示</span>
   </label>
   ```
2. `src-tauri/src/commands/markdown.rs`作成
   ```rust
   #[tauri::command]
   pub fn render_markdown(input: KnowledgeInput) -> Result<String, ErrorInfo> {
       MarkdownRenderer::render_markdown(&input)
           .map_err(ErrorInfo::from)
   }
   ```
3. `src/lib/tauri-bridge.ts`に`renderMarkdown`関数追加
4. lodash-esをインストール
   ```bash
   npm install lodash-es
   npm install -D @types/lodash-es
   ```

**受入条件:**
- [ ] チェックボックスON/OFFでレイアウトが切り替わる
- [ ] 入力変更がリアルタイムで反映される（デバウンス100ms）
- [ ] プレビューが正確
- [ ] パフォーマンスが良好（長文でも遅延なし）

**テスト:**
- [ ] UI切り替え確認
- [ ] デバウンス動作確認
- [ ] プレビュー正確性確認
- [ ] パフォーマンステスト

---

### TASK-213: E2Eテスト追加（オプション）

**優先度:** 🟢 低
**予想工数:** 3日
**依存関係:** すべてのタスク完了後
**担当:** TBD

**目的:**
Phase 2機能のE2Eテストを追加します。

**実装内容:**
1. tauri-driverセットアップ（Phase 1で保留）
2. グローバルショートカットのテスト
   - ショートカット押下でQuickInputが表示されるか
3. クイック保存のE2Eテスト
   - QuickInput → 入力 → 保存 → ファイル生成確認
4. PR作成モードのE2Eテスト
   - 設定変更 → 保存 → featureブランチ確認
5. プレビュー表示のE2Eテスト
   - チェックボックスON → 入力 → プレビュー更新確認

**受入条件:**
- [ ] tauri-driverが動作する
- [ ] すべてのE2Eテストが合格する

**テスト:**
- Phase 2の全機能

---

## タスク依存関係図

```
TASK-201 (plugin導入)
  └─> TASK-202 (ShortcutManager)
         └─> TASK-204 (QuickInput統合)
                └─> TASK-205 (クイック保存)

TASK-203 (QuickInput UI)
  └─> TASK-204 (QuickInput統合)

TASK-206 (PR作成モード) ─┐
                          ├─> TASK-209 (commitMode対応)
TASK-207 (Config拡張) ────┤
  └─> TASK-208 (Settings UI)

TASK-210 (Markdownレンダラー) ─┐
                                ├─> TASK-212 (プレビュー統合)
TASK-211 (PreviewPane UI) ──────┘

すべてのタスク完了
  └─> TASK-213 (E2Eテスト)
```

## 実装優先順位

### フェーズ1: クイック入力（4タスク）
1. TASK-201: plugin導入
2. TASK-202: ShortcutManager
3. TASK-203: QuickInput UI
4. TASK-204: QuickInput統合
5. TASK-205: クイック保存

**目標:** グローバルショートカットでクイック入力が動作する

### フェーズ2: PR作成モード（4タスク）
1. TASK-206: PR作成モード
2. TASK-207: Config拡張
3. TASK-208: Settings UI
4. TASK-209: commitMode対応

**目標:** PR作成モードが動作する

### フェーズ3: Markdownプレビュー（3タスク）
1. TASK-210: Markdownレンダラー
2. TASK-211: PreviewPane UI
3. TASK-212: プレビュー統合

**目標:** 入力内容がリアルタイムでプレビュー表示される

### フェーズ4: E2Eテスト（1タスク、オプション）
1. TASK-213: E2Eテスト

**目標:** すべての機能がE2Eテストで検証される

## 進捗管理

| タスクID | ステータス | 担当 | 開始日 | 完了日 | 備考 |
|---------|-----------|------|--------|--------|------|
| TASK-201 | ⬜ 未着手 | - | - | - | |
| TASK-202 | ⬜ 未着手 | - | - | - | |
| TASK-203 | ⬜ 未着手 | - | - | - | |
| TASK-204 | ⬜ 未着手 | - | - | - | |
| TASK-205 | ⬜ 未着手 | - | - | - | |
| TASK-206 | ⬜ 未着手 | - | - | - | |
| TASK-207 | ⬜ 未着手 | - | - | - | |
| TASK-208 | ⬜ 未着手 | - | - | - | |
| TASK-209 | ⬜ 未着手 | - | - | - | |
| TASK-210 | ⬜ 未着手 | - | - | - | |
| TASK-211 | ⬜ 未着手 | - | - | - | |
| TASK-212 | ⬜ 未着手 | - | - | - | |
| TASK-213 | ⬜ 未着手 | - | - | - | オプション |

## 総工数見積もり

| 優先度 | タスク数 | 合計工数 |
|--------|---------|---------|
| 🔴 高 | 9タスク | 9.5日 |
| 🟡 中 | 3タスク | 3日 |
| 🟢 低 | 1タスク | 3日 |
| **合計** | **13タスク** | **15.5日** |

## 参照

- Phase 2要件定義: `docs/michi/20260131-worknote/spec/phase2-requirements.md`
- Phase 2設計書: `docs/michi/20260131-worknote/spec/phase2-design.md`
- Phase 1タスクリスト: `docs/michi/20260131-worknote/tasks/tasks.md`
