# テスト戦略: WorkNote

## 概要

本ドキュメントは、WorkNoteアプリケーションのテスト戦略を定義します。

---

## テスト方針

### 基本方針

- **TDD（テスト駆動開発）** を基本とする
- **カバレッジ95%以上** を目標（Phase 1 MVP）
- **型安全性** を活用（TypeScript + Rust）
- **継続的インテグレーション** でテストを自動実行

### スコープ

本プロジェクトでは、以下のテストレベルに焦点を当てます：

1. **単体テスト（Unit Test）** - 個別のコンポーネント・関数のテスト
2. **E2Eテスト（End-to-End Test）** - ユーザーシナリオのテスト

**除外**: 統合テスト、パフォーマンステストは今回のスコープ外

---

## テストレベル

### 1. 単体テスト（Unit Test）

**目的**: 個別のコンポーネント・関数が正しく動作することを保証

**対象**:
- フロントエンド（TypeScript/Svelte）
  - FormValidator - 入力バリデーションロジック
  - MarkdownRenderer - Markdown→HTML変換（XSS対策含む）
  - TauriBridge - Tauriコマンド呼び出しラッパー

- バックエンド（Rust）
  - GitService - Git操作（commit, push, branch作成）
  - FileGenerator - Markdownファイル生成、kebab-case変換
  - ConfigManager - 設定ファイル読み書き
  - DraftManager - 下書き保存・復元
  - ValidationHelper - サーバーサイドバリデーション
  - ErrorHandler - エラーハンドリング

**ツール**:
- フロントエンド: Vitest + @testing-library/svelte
- バックエンド: Rust標準テスト (`cargo test`)

**カバレッジ目標**: 95%以上

**実行タイミング**:
- コード変更時（ローカル）
- PR作成時（CI/CD）
- mainマージ時（CI/CD）

---

### 2. E2Eテスト（End-to-End Test）

**目的**: ユーザーシナリオが正しく動作することを保証

**対象シナリオ**:
1. **ナレッジ入力→Git push（直接Pushモード）**
   - 詳細入力ウィンドウでナレッジを入力
   - Markdownファイルが生成される
   - Git commit & pushが成功する

2. **設定画面での設定保存**
   - 設定画面でリポジトリパス、Author情報を入力
   - 設定が保存される
   - 設定が再起動後も保持される

3. **エラーハンドリング**
   - Git認証エラー時のエラー表示
   - ネットワークエラー時のリトライ・下書き保存

**ツール**: WebDriver（tauri-driver）またはPlaywright with Tauri plugin

**カバレッジ目標**: 主要シナリオ100%

**実行タイミング**:
- PR作成時（CI/CD、主要シナリオのみ）
- リリース前（すべてのシナリオ）

---

## テスト環境

### ローカル環境

| 項目 | 設定 |
|------|------|
| OS | macOS 12 (Monterey) 以降 |
| Git | 2.30+ |
| Node.js | 18+ |
| Rust | 1.70+ |

### CI/CD環境（GitHub Actions）

| 項目 | 設定 |
|------|------|
| OS | macos-latest |
| Git | プリインストール版 |
| Node.js | actions/setup-node@v4 |
| Rust | actions-rust-lang/setup-rust-toolchain@v1 |

---

## テストデータ

### 単体テスト

- **モック**: Git CLI、ファイルシステムはモック化
- **Fixture**: テストデータは固定値（例: `test-title`, `alerts`, `high`）
- **クリーンアップ**: 各テスト後にモックをリセット

### E2Eテスト

- **テストリポジトリ**: 専用のテストリポジトリを使用（例: `worknote-e2e-test`）
- **テストデータ**: 各テスト前に初期状態にリセット
- **クリーンアップ**: テスト後にテストリポジトリの変更をrevert

---

## カバレッジ目標

### Phase 1 MVP

| テストレベル | 目標カバレッジ | 測定方法 |
|--------------|----------------|----------|
| 単体テスト | 95%以上 | Istanbul (c8) / Tarpaulin |
| E2Eテスト | 主要シナリオ100% | シナリオベース |

### 除外項目

以下はカバレッジ測定から除外：
- 型定義ファイル（`.d.ts`）
- 設定ファイル（`vite.config.ts`, `tauri.conf.json`）
- テストコード自体

---

## 継続的インテグレーション

### GitHub Actions ワークフロー

**トリガー**:
- PR作成・更新時
- mainブランチへのpush時

**ジョブ**:

```yaml
name: Test

on:
  pull_request:
  push:
    branches: [main]

jobs:
  unit-test-frontend:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 18
      - run: npm ci
      - run: npm run test:unit
      - run: npm run test:coverage

  unit-test-backend:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cd src-tauri && cargo test
      - run: cd src-tauri && cargo tarpaulin --out Xml

  e2e-test:
    runs-on: macos-latest
    if: github.event_name == 'pull_request'
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 18
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: npm ci
      - run: npm run test:e2e
```

**テスト失敗時**:
- PRマージをブロック
- Slackに通知（オプション）

---

## テスト実行コマンド

### ローカル実行

```bash
# 単体テスト（フロントエンド）
npm run test:unit

# 単体テスト（バックエンド）
cd src-tauri && cargo test

# カバレッジ（フロントエンド）
npm run test:coverage

# カバレッジ（バックエンド）
cd src-tauri && cargo tarpaulin

# E2Eテスト
npm run test:e2e

# すべてのテスト
npm run test:all
```

### CI/CD実行

```bash
# GitHub Actionsで自動実行
# PRマージ前に必ずすべてのテストが実行される
```

---

## テストメンテナンス

### テストコードの品質

- テストコードも本番コードと同等の品質を保つ
- DRY原則を適用（テストヘルパー関数の活用）
- 可読性を重視（Arrange-Act-Assert パターン）

### テストの保守性

- 設計変更時にテストも同時に更新
- 不要になったテストは削除
- テストの実行時間を監視（遅いテストは改善）

---

## まとめ

WorkNoteのテスト戦略は以下の特徴を持ちます：

1. **TDD重視**: テストファーストで開発を進める
2. **高カバレッジ**: 95%以上を目標
3. **CI/CD統合**: PR時に自動テスト実行
4. **実用的スコープ**: 単体テストとE2Eテストに焦点

---

**文書情報**

| 項目 | 内容 |
|------|------|
| 作成日 | 2026-01-31 |
| 作成者 | Claude Code (michi:plan-tests) |
| バージョン | 1.0 |
| ステータス | 承認待ち |
