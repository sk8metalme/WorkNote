# E2Eテスト仕様: 設定保存とエラーハンドリング

## 概要

設定画面での設定保存と、各種エラーハンドリング（Git認証エラー、ネットワークエラー）のシナリオをテストします。

---

## テストシナリオ

### TC-E2E-101: 設定保存・読み込み（正常系）

**前提条件**:
- WorkNoteアプリがインストールされている
- 設定ファイルが存在しない（初回起動）

**シナリオ**:

1. **アプリ起動**
   - WorkNote.appを起動

2. **設定画面を開く**
   - `⌘ + ,` を押下、または「設定」メニューをクリック
   - 設定画面が表示される

3. **リポジトリパス入力**
   - リポジトリパス入力欄に `/Users/test/worknote-e2e-test` を入力

4. **ナレッジ保存先入力**
   - ナレッジ保存先入力欄に `docs/runbooks` を入力

5. **Author情報入力**
   - Author名に `Test User` を入力
   - Authorメールに `test@example.com` を入力

6. **保存実行**
   - 「保存」ボタンをクリック

7. **完了確認**
   - 「設定を保存しました」という通知が表示される

8. **アプリ再起動**
   - アプリを終了
   - アプリを再起動

9. **設定画面を開く**
   - 設定画面を再度開く

10. **設定値確認**
    - 入力した設定値が保持されていることを確認

**期待結果**:

1. **設定ファイル生成**
   - `~/Library/Application Support/WorkNote/config.json` が作成される

2. **ファイル内容**
   ```json
   {
     "version": 1,
     "git": {
       "repositoryPath": "/Users/test/worknote-e2e-test",
       "savePath": "docs/runbooks",
       "defaultBranch": "main",
       "commitMode": "direct"
     },
     "author": {
       "name": "Test User",
       "email": "test@example.com"
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

3. **再起動後も設定が保持**
   - リポジトリパス、Author情報が表示される

**実装例（Playwright）**:

```typescript
test('TC-E2E-101: 設定保存・読み込み（正常系）', async ({ page }) => {
  // Step 1-2: アプリ起動、設定画面を開く
  await page.goto('tauri://localhost');
  await page.keyboard.press('Meta+Comma'); // ⌘ + ,

  // Step 3-5: 設定入力
  await page.fill('[data-testid="repository-path-input"]', '/Users/test/worknote-e2e-test');
  await page.fill('[data-testid="save-path-input"]', 'docs/runbooks');
  await page.fill('[data-testid="author-name-input"]', 'Test User');
  await page.fill('[data-testid="author-email-input"]', 'test@example.com');

  // Step 6-7: 保存実行
  await page.click('[data-testid="save-settings-button"]');
  await expect(page.locator('[data-testid="notification"]')).toContainText('設定を保存しました');

  // Step 8: アプリ再起動（Tauriの場合は特殊な処理が必要）
  await page.close();
  await page.goto('tauri://localhost');

  // Step 9-10: 設定値確認
  await page.keyboard.press('Meta+Comma');
  await expect(page.locator('[data-testid="repository-path-input"]')).toHaveValue('/Users/test/worknote-e2e-test');
  await expect(page.locator('[data-testid="author-name-input"]')).toHaveValue('Test User');
  await expect(page.locator('[data-testid="author-email-input"]')).toHaveValue('test@example.com');

  // 設定ファイル確認
  const fs = require('fs');
  const configPath = `${process.env.HOME}/Library/Application Support/WorkNote/config.json`;
  expect(fs.existsSync(configPath)).toBe(true);

  const config = JSON.parse(fs.readFileSync(configPath, 'utf-8'));
  expect(config.author.name).toBe('Test User');
  expect(config.author.email).toBe('test@example.com');
});
```

---

### TC-E2E-102: Git認証エラーのハンドリング

**前提条件**:
- WorkNoteアプリがインストールされている
- Git設定が完了しているが、**認証情報が無効**

**シナリオ**:

1. アプリ起動
2. ナレッジを入力（タイトル、カテゴリ、重要度）
3. 「Git Commit & Push」ボタンをクリック
4. Git pushが**認証エラー**で失敗

**期待結果**:

1. **エラーダイアログ表示**
   - タイトル: 「Git認証エラー」
   - メッセージ: 「Git認証に失敗しました。SSH鍵またはCredential Helperを確認してください。」
   - アクション:
     - [設定画面を開く]
     - [下書き保存]
     - [キャンセル]

2. **下書き保存を選択した場合**
   - 入力内容が下書きとして保存される
   - 下書きID（UUID）が通知される

**実装例（Playwright）**:

```typescript
test('TC-E2E-102: Git認証エラーのハンドリング', async ({ page }) => {
  // Git認証を無効化（モック）
  await page.evaluate(() => {
    window.__MOCK_GIT_AUTH_FAIL__ = true;
  });

  await page.goto('tauri://localhost');

  // ナレッジ入力
  await page.fill('[data-testid="title-input"]', 'Test Knowledge');
  await page.selectOption('[data-testid="category-select"]', 'alerts');
  await page.selectOption('[data-testid="severity-select"]', 'high');

  // 保存実行
  await page.click('[data-testid="save-button"]');

  // エラーダイアログ確認
  await expect(page.locator('[data-testid="error-dialog-title"]')).toContainText('Git認証エラー');
  await expect(page.locator('[data-testid="error-dialog-message"]')).toContainText('Git認証に失敗しました');

  // 下書き保存を選択
  await page.click('[data-testid="save-draft-button"]');

  // 下書き保存完了確認
  await expect(page.locator('[data-testid="notification"]')).toContainText('下書きを保存しました');

  // 下書きファイル確認
  const fs = require('fs');
  const draftsDir = `${process.env.HOME}/Library/Application Support/WorkNote/drafts`;
  const draftFiles = fs.readdirSync(draftsDir);
  expect(draftFiles.length).toBe(1);

  const draftContent = JSON.parse(fs.readFileSync(`${draftsDir}/${draftFiles[0]}`, 'utf-8'));
  expect(draftContent.data.title).toBe('Test Knowledge');
});
```

---

### TC-E2E-103: ネットワークエラーのハンドリング

**前提条件**:
- WorkNoteアプリがインストールされている
- Git設定が完了しているが、**ネットワークが利用不可**

**シナリオ**:

1. アプリ起動
2. ナレッジを入力
3. 「Git Commit & Push」ボタンをクリック
4. Git pushが**ネットワークエラー**で失敗

**期待結果**:

1. **エラーダイアログ表示**
   - タイトル: 「ネットワークエラー」
   - メッセージ: 「ネットワークに接続できません。接続を確認してください。」
   - アクション:
     - [リトライ]
     - [下書き保存]
     - [キャンセル]

2. **リトライを選択した場合**
   - Git push操作を再実行

**実装例（Playwright）**:

```typescript
test('TC-E2E-103: ネットワークエラーのハンドリング', async ({ page }) => {
  // ネットワークを無効化（モック）
  await page.evaluate(() => {
    window.__MOCK_NETWORK_FAIL__ = true;
  });

  await page.goto('tauri://localhost');

  // ナレッジ入力
  await page.fill('[data-testid="title-input"]', 'Test Knowledge');
  await page.selectOption('[data-testid="category-select"]', 'alerts');
  await page.selectOption('[data-testid="severity-select"]', 'high');

  // 保存実行
  await page.click('[data-testid="save-button"]');

  // エラーダイアログ確認
  await expect(page.locator('[data-testid="error-dialog-title"]')).toContainText('ネットワークエラー');
  await expect(page.locator('[data-testid="error-dialog-message"]')).toContainText('ネットワークに接続できません');

  // リトライボタンが表示されることを確認
  await expect(page.locator('[data-testid="retry-button"]')).toBeVisible();

  // ネットワークを復旧（モック解除）
  await page.evaluate(() => {
    window.__MOCK_NETWORK_FAIL__ = false;
  });

  // リトライを選択
  await page.click('[data-testid="retry-button"]');

  // 成功通知確認
  await expect(page.locator('[data-testid="notification"]')).toContainText('保存に成功しました');
});
```

---

### TC-E2E-104: 設定バリデーション - 無効なリポジトリパス

**前提条件**: TC-E2E-101と同じ

**シナリオ**:

1. アプリ起動
2. 設定画面を開く
3. リポジトリパスに**存在しないパス**を入力 (`/invalid/path`)
4. 「保存」ボタンをクリック

**期待結果**:

1. **エラーメッセージ表示**
   - 「リポジトリパスが無効です。設定を確認してください。」

2. **設定は保存されない**

**実装例（Playwright）**:

```typescript
test('TC-E2E-104: 設定バリデーション - 無効なリポジトリパス', async ({ page }) => {
  await page.goto('tauri://localhost');
  await page.keyboard.press('Meta+Comma');

  // 無効なパスを入力
  await page.fill('[data-testid="repository-path-input"]', '/invalid/path');
  await page.fill('[data-testid="author-name-input"]', 'Test User');
  await page.fill('[data-testid="author-email-input"]', 'test@example.com');

  await page.click('[data-testid="save-settings-button"]');

  // エラーメッセージ確認
  await expect(page.locator('[data-testid="repository-path-error"]')).toContainText('リポジトリパスが無効です');

  // 設定ファイルが作成されていないことを確認
  const fs = require('fs');
  const configPath = `${process.env.HOME}/Library/Application Support/WorkNote/config.json`;
  expect(fs.existsSync(configPath)).toBe(false);
});
```

---

## テスト環境

### モック戦略

E2Eテストでは、以下のエラーをモック化します：

- **Git認証エラー**: `window.__MOCK_GIT_AUTH_FAIL__` フラグ
- **ネットワークエラー**: `window.__MOCK_NETWORK_FAIL__` フラグ

### クリーンアップ

各テスト後に以下をクリーンアップ：

- 設定ファイル: `~/Library/Application Support/WorkNote/config.json`
- 下書きファイル: `~/Library/Application Support/WorkNote/drafts/*.json`

---

## 実行方法

```bash
# 設定・エラーハンドリングのE2Eテストのみ実行
npm run test:e2e -- settings-and-errors

# すべてのE2Eテスト実行
npm run test:e2e
```

---

**文書情報**

| 項目 | 内容 |
|------|------|
| 作成日 | 2026-01-31 |
| 作成者 | Claude Code (michi:plan-tests) |
| バージョン | 1.0 |
