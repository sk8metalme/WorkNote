# E2Eテスト仕様: ナレッジ入力→Git push

## 概要

ユーザーが詳細入力ウィンドウでナレッジを入力し、Git commit & pushが成功するまでのシナリオをテストします。

---

## テストシナリオ

### TC-E2E-001: ナレッジ入力→直接Push（正常系）

**前提条件**:
- WorkNoteアプリがインストールされている
- Git設定が完了している（リポジトリパス、Author情報）
- テストリポジトリが初期化されている

**シナリオ**:

1. **アプリ起動**
   - WorkNote.appを起動
   - 詳細入力ウィンドウが表示される

2. **タイトル入力**
   - タイトル入力欄に「CPU高騰対応」を入力

3. **カテゴリ選択**
   - カテゴリドロップダウンから「alerts」を選択

4. **重要度選択**
   - 重要度ドロップダウンから「high」を選択

5. **症状入力**
   - 症状・検知条件欄に「CPU使用率が90%を超えている」を入力

6. **対応手順入力**
   - 対応手順欄に以下を入力:
     ```
     1. プロセス一覧を確認
     2. 原因プロセスを特定
     3. 必要に応じて再起動
     ```

7. **保存実行**
   - 「Git Commit & Push」ボタンをクリック

8. **処理待機**
   - Git操作が実行される（ローディング表示）

9. **完了確認**
   - 「保存に成功しました」という通知が表示される
   - ウィンドウが閉じる

**期待結果**:

1. **Markdownファイル生成**
   - `{リポジトリパス}/docs/runbooks/alerts/cpu.md` が作成される（日本語文字は除去され、ASCII英数字のみが残る）

2. **ファイル内容**
   ```markdown
   ---
   title: "CPU高騰対応"
   category: alerts
   severity: high
   symptoms:
     - "CPU使用率が90%を超えている"
   related_alerts: []
   last_updated: 2026-01-31
   author: "{Author名}"
   ---

   # CPU高騰対応

   ## 概要

   CPU使用率が90%を超えている

   ## 症状・検知条件

   CPU使用率が90%を超えている

   ## 対応手順

   1. プロセス一覧を確認
   2. 原因プロセスを特定
   3. 必要に応じて再起動

   ## 注意点・落とし穴



   ## 関連リンク



   ## 対応履歴

   | 日付 | 対応者 | 備考 |
   |------|--------|------|
   | 2026-01-31 | {Author名} | 初版作成 |
   ```

3. **Gitコミット**
   - コミットメッセージ: `docs(worknote): add CPU高騰対応`

4. **Gitプッシュ**
   - リモートリポジトリに正常にpush

**実装例（Playwright）**:

```typescript
test('TC-E2E-001: ナレッジ入力→直接Push（正常系）', async ({ page }) => {
  // Step 1: アプリ起動
  await page.goto('tauri://localhost');

  // Step 2-3: タイトル、カテゴリ、重要度を入力
  await page.fill('[data-testid="title-input"]', 'CPU高騰対応');
  await page.selectOption('[data-testid="category-select"]', 'alerts');
  await page.selectOption('[data-testid="severity-select"]', 'high');

  // Step 4-6: 症状、対応手順を入力
  await page.fill('[data-testid="symptoms-input"]', 'CPU使用率が90%を超えている');
  await page.fill('[data-testid="procedure-input"]', '1. プロセス一覧を確認\n2. 原因プロセスを特定\n3. 必要に応じて再起動');

  // Step 7: 保存実行
  await page.click('[data-testid="save-button"]');

  // Step 8-9: 完了確認
  await expect(page.locator('[data-testid="notification"]')).toContainText('保存に成功しました');

  // ファイル生成確認（バックエンドテスト）
  const fs = require('fs');
  const filePath = `${TEST_REPO_PATH}/docs/runbooks/alerts/cpu.md`;
  expect(fs.existsSync(filePath)).toBe(true);

  // ファイル内容確認
  const content = fs.readFileSync(filePath, 'utf-8');
  expect(content).toContain('title: "CPU高騰対応"');
  expect(content).toContain('category: alerts');
  expect(content).toContain('severity: high');

  // Gitコミット確認
  const { execSync } = require('child_process');
  const gitLog = execSync('git log -1 --pretty=%B', { cwd: TEST_REPO_PATH }).toString();
  expect(gitLog).toContain('docs(worknote): add CPU高騰対応');
});
```

---

### TC-E2E-002: バリデーションエラーの表示

**前提条件**: TC-E2E-001と同じ

**シナリオ**:

1. アプリ起動
2. タイトルを**空欄のまま**にする
3. カテゴリ、重要度も選択しない
4. 「Git Commit & Push」ボタンをクリック

**期待結果**:

1. エラーメッセージが表示される:
   - 「タイトルは必須です」
   - 「カテゴリを選択してください」
   - 「重要度を選択してください」

2. Git操作は実行されない

**実装例（Playwright）**:

```typescript
test('TC-E2E-002: バリデーションエラーの表示', async ({ page }) => {
  await page.goto('tauri://localhost');

  // 何も入力せずに保存
  await page.click('[data-testid="save-button"]');

  // エラーメッセージ確認
  await expect(page.locator('[data-testid="title-error"]')).toContainText('タイトルは必須です');
  await expect(page.locator('[data-testid="category-error"]')).toContainText('カテゴリを選択してください');
  await expect(page.locator('[data-testid="severity-error"]')).toContainText('重要度を選択してください');

  // Git操作が実行されていないことを確認
  const { execSync } = require('child_process');
  const gitLog = execSync('git log -1 --pretty=%B', { cwd: TEST_REPO_PATH }).toString();
  expect(gitLog).not.toContain('docs(worknote):');
});
```

---

### TC-E2E-003: 文字数制限の検証

**前提条件**: TC-E2E-001と同じ

**シナリオ**:

1. アプリ起動
2. タイトルに**101文字**を入力
3. 「Git Commit & Push」ボタンをクリック

**期待結果**:

1. エラーメッセージが表示される:
   - 「タイトルは100文字以内で入力してください」

2. Git操作は実行されない

**実装例（Playwright）**:

```typescript
test('TC-E2E-003: 文字数制限の検証', async ({ page }) => {
  await page.goto('tauri://localhost');

  // 101文字入力
  const longTitle = 'a'.repeat(101);
  await page.fill('[data-testid="title-input"]', longTitle);
  await page.selectOption('[data-testid="category-select"]', 'alerts');
  await page.selectOption('[data-testid="severity-select"]', 'high');

  await page.click('[data-testid="save-button"]');

  // エラーメッセージ確認
  await expect(page.locator('[data-testid="title-error"]')).toContainText('タイトルは100文字以内で入力してください');

  // Git操作が実行されていないことを確認
  const { execSync } = require('child_process');
  const gitLog = execSync('git log -1 --pretty=%B', { cwd: TEST_REPO_PATH }).toString();
  expect(gitLog).not.toContain('docs(worknote):');
});
```

---

## テスト環境

### 前提条件

- **テストリポジトリ**: `worknote-e2e-test` という専用リポジトリを使用
- **Git設定**: テスト用のAuthor名・メールを設定
- **クリーンアップ**: 各テスト前にリポジトリを初期状態にリセット

### テストリポジトリのセットアップ

```bash
# テストリポジトリの初期化
git clone https://github.com/your-org/worknote-e2e-test.git
cd worknote-e2e-test
git config user.name "E2E Test"
git config user.email "e2e@test.com"

# ディレクトリ構造作成
mkdir -p docs/runbooks/{alerts,maintenance,troubleshooting}
git add .
git commit -m "Initial setup"
git push origin main
```

---

## 実行方法

```bash
# E2Eテストのみ実行
npm run test:e2e

# 特定のシナリオのみ実行
npm run test:e2e -- knowledge-input-to-push

# ヘッドレスモード（CI/CD）
npm run test:e2e:ci
```

---

**文書情報**

| 項目 | 内容 |
|------|------|
| 作成日 | 2026-01-31 |
| 作成者 | Claude Code (michi:plan-tests) |
| バージョン | 1.0 |
