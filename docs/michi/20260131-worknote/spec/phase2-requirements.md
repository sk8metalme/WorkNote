# Phase 2 要件定義書: WorkNote

## 概要

Phase 1で実装した基本機能に加えて、日常的に使いやすい機能を追加します。

## Phase 1で実装済み

- ✅ 詳細入力ウィンドウ
- ✅ Git直接Pushモード
- ✅ リポジトリ設定、Author情報設定
- ✅ バリデーション
- ✅ エラーハンドリング（GitError/NetworkError/AuthError分類）

## Phase 2で実装する機能

### FR-201: Spotlight風クイック入力ウィンドウ

**EARS形式要件:**
- **Event-driven**: ユーザーがグローバルショートカット（⌘ + Shift + K）を押下した場合、システムはSpotlight風の小さなウィンドウを画面中央上部に表示すること
- **Ubiquitous**: Spotlight風ウィンドウは常に最小限のフィールド（タイトル、カテゴリ、重要度）のみを表示すること
- **State-driven**: Spotlight風ウィンドウが表示されている状態において、Escキーが押下された場合、システムはウィンドウを閉じること
- **Event-driven**: ユーザーがSpotlight風ウィンドウで「クイック保存」ボタンをクリックした場合、システムは最小限のMarkdownファイルを生成してGit commit & pushを実行すること

**受入条件:**
- [ ] グローバルショートカット（⌘ + Shift + K）でウィンドウが起動する
- [ ] ウィンドウサイズは 480px × 280px（固定）
- [ ] 画面中央上部に表示される
- [ ] タイトル、カテゴリ、重要度のみ入力可能
- [ ] Enterキーでクイック保存が実行される
- [ ] Escキーでウィンドウが閉じる
- [ ] 「詳細入力へ」ボタンで詳細入力ウィンドウに切り替わる

### FR-202: PR作成モード

**EARS形式要件:**
- **State-driven**: 設定画面でコミットモードが「PR作成」に設定されている状態において、ユーザーが保存を実行した場合、システムはfeature/worknote-{title}-{timestamp}ブランチを作成してpushすること
- **Event-driven**: feature ブランチのpushが完了した場合、システムはGitHub PR作成URLを通知ダイアログに表示すること
- **Unwanted**: PR作成モードにおいて、システムはデフォルトブランチに直接pushしてはならない

**受入条件:**
- [ ] 設定画面に「コミットモード」選択肢（直接Push / PR作成）が追加される
- [ ] PR作成モード選択時、feature/worknote-{title}-{timestamp}ブランチが作成される
- [ ] featureブランチにpushされる
- [ ] 完了通知にGitHub PR作成URLが含まれる
- [ ] URLクリックでブラウザが開く

### FR-203: Markdownプレビュー

**EARS形式要件:**
- **State-driven**: 詳細入力ウィンドウで「プレビュー表示」チェックボックスがONの状態において、システムはウィンドウ右側にMarkdownプレビューペインを表示すること
- **Ubiquitous**: プレビューペインは常に入力内容をリアルタイムでMarkdownレンダリングして表示すること
- **State-driven**: frontmatterセクションはYAML形式でプレビューに表示されること

**受入条件:**
- [ ] 詳細入力ウィンドウに「プレビュー表示」チェックボックスが追加される
- [ ] チェックON時、ウィンドウが左右2分割される（入力フォーム | プレビュー）
- [ ] 入力内容の変更がリアルタイムでプレビューに反映される
- [ ] frontmatterセクションがYAML形式で表示される
- [ ] Markdownの見出し、リスト、コードブロックが正しくレンダリングされる

### FR-204: グローバルショートカット設定

**EARS形式要件:**
- **Ubiquitous**: システムは常にグローバルショートカット設定を保持すること
- **Event-driven**: ユーザーがグローバルショートカットを押下した場合、システムは他のアプリがフォーカスされていてもSpotlight風ウィンドウを起動すること
- **Optional**: ユーザーは必要に応じて、設定画面でグローバルショートカットをカスタマイズしてもよい

**受入条件:**
- [ ] tauri-plugin-global-shortcutが導入される
- [ ] デフォルトショートカットは ⌘ + Shift + K
- [ ] 設定画面でショートカットをカスタマイズ可能
- [ ] 他のアプリがフォーカスされていてもショートカットが動作する
- [ ] 既存のシステムショートカットと競合しない

## 非機能要件

### NFR-201: パフォーマンス

- Spotlight風ウィンドウ表示: 0.3秒以内
- Markdownプレビュー更新遅延: 100ms以内
- メモリ使用量増加: Phase 1比で+20MB以内

### NFR-202: ユーザビリティ

- グローバルショートカットは macOS標準のショートカット規約に従う
- Spotlight風ウィンドウは常に最前面に表示される
- プレビューペインはスクロール可能

### NFR-203: 互換性

- Phase 1で作成したMarkdownファイル形式と互換性を保つ
- Phase 1の設定ファイル（config.json）を拡張するが、既存フィールドは維持

## 制約条件

- macOS専用（グローバルショートカットはmacOS APIを使用）
- Tauriのセキュリティポリシーに準拠
- GitHub URLは https://github.com/{owner}/{repo}/compare/{branch} 形式

## 参照

- 元の要件定義: [GitHub Issue #1](https://github.com/sk8metalme/WorkNote/issues/1)
- Phase 1実装: [PR #2](https://github.com/sk8metalme/WorkNote/pull/2)
