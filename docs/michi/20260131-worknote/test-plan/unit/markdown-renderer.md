# 単体テスト仕様: MarkdownRenderer

## 概要

MarkdownRendererは、Markdown→HTML変換とXSS対策を担当します。**セキュリティ上重要**なコンポーネントです。

---

## テスト対象

**モジュール**: `src/lib/markdown-renderer.ts`

**主要関数**:
- `renderMarkdownSafe(markdown: string): string`

---

## テストケース一覧

### セキュリティテスト（XSS対策）

#### TC-U-MR-001: XSS対策 - scriptタグの除去

**入力**: `'<script>alert("XSS")</script>'`

**期待結果**: `<script>`タグが含まれない

**実装例**:
```typescript
test('TC-U-MR-001: scriptタグは除去される', () => {
  const input = '<script>alert("XSS")</script>';
  const output = renderMarkdownSafe(input);
  expect(output).not.toContain('<script>');
  expect(output).not.toContain('alert');
});
```

---

#### TC-U-MR-002: XSS対策 - onerror属性の除去

**入力**: `'<img src=x onerror="alert(\'XSS\')">'`

**期待結果**: `onerror`属性が含まれない

**実装例**:
```typescript
test('TC-U-MR-002: onerror属性は除去される', () => {
  const input = '<img src=x onerror="alert(\'XSS\')">';
  const output = renderMarkdownSafe(input);
  expect(output).not.toContain('onerror');
  expect(output).not.toContain('alert');
});
```

---

#### TC-U-MR-003: XSS対策 - javascript:プロトコルの除去

**入力**: `'[Click me](javascript:alert("XSS"))'`

**期待結果**: `javascript:`プロトコルが含まれない

**実装例**:
```typescript
test('TC-U-MR-003: javascript:プロトコルは除去される', () => {
  const input = '[Click me](javascript:alert("XSS"))';
  const output = renderMarkdownSafe(input);
  expect(output).not.toContain('javascript:');
  expect(output).not.toContain('alert');
});
```

---

#### TC-U-MR-004: XSS対策 - iframeタグの除去

**入力**: `'<iframe src="https://evil.com"></iframe>'`

**期待結果**: `<iframe>`タグが含まれない

**実装例**:
```typescript
test('TC-U-MR-004: iframeタグは除去される', () => {
  const input = '<iframe src="https://evil.com"></iframe>';
  const output = renderMarkdownSafe(input);
  expect(output).not.toContain('<iframe>');
  expect(output).not.toContain('</iframe>');
});
```

---

#### TC-U-MR-005: XSS対策 - onload属性の除去

**入力**: `'<body onload="alert(\'XSS\')">'`

**期待結果**: `onload`属性が含まれない

**実装例**:
```typescript
test('TC-U-MR-005: onload属性は除去される', () => {
  const input = '<body onload="alert(\'XSS\')">';
  const output = renderMarkdownSafe(input);
  expect(output).not.toContain('onload');
  expect(output).not.toContain('alert');
});
```

---

### 正常系テスト

#### TC-U-MR-006: 正常系 - 見出し変換

**入力**: `'# Heading 1'`

**期待結果**: `<h1>Heading 1</h1>`を含む

**実装例**:
```typescript
test('TC-U-MR-006: 見出しが正しく変換される', () => {
  const input = '# Heading 1';
  const output = renderMarkdownSafe(input);
  expect(output).toContain('<h1');
  expect(output).toContain('Heading 1');
  expect(output).toContain('</h1>');
});
```

---

#### TC-U-MR-007: 正常系 - リスト変換

**入力**:
```markdown
- List item 1
- List item 2
```

**期待結果**: `<ul>`と`<li>`を含む

**実装例**:
```typescript
test('TC-U-MR-007: リストが正しく変換される', () => {
  const input = '- List item 1\n- List item 2';
  const output = renderMarkdownSafe(input);
  expect(output).toContain('<ul>');
  expect(output).toContain('<li>');
  expect(output).toContain('List item 1');
  expect(output).toContain('List item 2');
  expect(output).toContain('</li>');
  expect(output).toContain('</ul>');
});
```

---

#### TC-U-MR-008: 正常系 - 太字変換

**入力**: `'**Bold text**'`

**期待結果**: `<strong>Bold text</strong>`を含む

**実装例**:
```typescript
test('TC-U-MR-008: 太字が正しく変換される', () => {
  const input = '**Bold text**';
  const output = renderMarkdownSafe(input);
  expect(output).toContain('<strong>');
  expect(output).toContain('Bold text');
  expect(output).toContain('</strong>');
});
```

---

#### TC-U-MR-009: 正常系 - リンク変換

**入力**: `'[Google](https://google.com)'`

**期待結果**: `<a href="https://google.com">Google</a>`を含む

**実装例**:
```typescript
test('TC-U-MR-009: リンクが正しく変換される', () => {
  const input = '[Google](https://google.com)';
  const output = renderMarkdownSafe(input);
  expect(output).toContain('<a');
  expect(output).toContain('href="https://google.com"');
  expect(output).toContain('Google');
  expect(output).toContain('</a>');
});
```

---

#### TC-U-MR-010: 正常系 - コードブロック変換

**入力**:
````markdown
```javascript
console.log('Hello');
```
````

**期待結果**: `<pre>`と`<code>`を含む

**実装例**:
```typescript
test('TC-U-MR-010: コードブロックが正しく変換される', () => {
  const input = '```javascript\nconsole.log(\'Hello\');\n```';
  const output = renderMarkdownSafe(input);
  expect(output).toContain('<pre>');
  expect(output).toContain('<code');
  expect(output).toContain('console.log');
  expect(output).toContain('</code>');
  expect(output).toContain('</pre>');
});
```

---

#### TC-U-MR-011: 正常系 - インラインコード変換

**入力**: ``'`code`'``

**期待結果**: `<code>code</code>`を含む

**実装例**:
```typescript
test('TC-U-MR-011: インラインコードが正しく変換される', () => {
  const input = '`code`';
  const output = renderMarkdownSafe(input);
  expect(output).toContain('<code>');
  expect(output).toContain('code');
  expect(output).toContain('</code>');
});
```

---

#### TC-U-MR-012: 正常系 - 複雑なMarkdown

**入力**:
```markdown
# Title

## Subtitle

- List 1
- List 2

**Bold** and *italic* text.

[Link](https://example.com)

```bash
echo "test"
```
```

**期待結果**: すべての要素が正しく変換され、XSSは含まれない

**実装例**:
```typescript
test('TC-U-MR-012: 複雑なMarkdownが正しく変換される', () => {
  const input = `# Title\n\n## Subtitle\n\n- List 1\n- List 2\n\n**Bold** and *italic* text.\n\n[Link](https://example.com)\n\n\`\`\`bash\necho "test"\n\`\`\``;
  const output = renderMarkdownSafe(input);

  // すべての要素が含まれることを確認
  expect(output).toContain('<h1');
  expect(output).toContain('<h2');
  expect(output).toContain('<ul>');
  expect(output).toContain('<strong>');
  expect(output).toContain('<em>');
  expect(output).toContain('<a');
  expect(output).toContain('<pre>');

  // XSSが含まれないことを確認
  expect(output).not.toContain('<script>');
  expect(output).not.toContain('onerror');
  expect(output).not.toContain('javascript:');
});
```

---

## エッジケース

#### TC-U-MR-013: エッジケース - 空文字

**入力**: `''`

**期待結果**: 空文字または空のHTML

**実装例**:
```typescript
test('TC-U-MR-013: 空文字は空のHTMLを返す', () => {
  const input = '';
  const output = renderMarkdownSafe(input);
  expect(output).toBe('');
});
```

---

#### TC-U-MR-014: エッジケース - HTMLエスケープ

**入力**: `'<p>Test</p>'`

**期待結果**: `<p>`タグはサニタイズされて`<p>Test</p>`として表示

**実装例**:
```typescript
test('TC-U-MR-014: 許可されたタグは通過する', () => {
  const input = '<p>Test</p>';
  const output = renderMarkdownSafe(input);
  expect(output).toContain('<p>');
  expect(output).toContain('Test');
  expect(output).toContain('</p>');
});
```

---

## カバレッジ目標

- **行カバレッジ**: 100%
- **分岐カバレッジ**: 100%
- **関数カバレッジ**: 100%

**重要**: XSS対策のテストは**すべて必須**です。1つでも失敗した場合はリリースブロック。

---

## 実行方法

```bash
# MarkdownRendererのテストのみ実行
npm run test:unit -- markdown-renderer

# カバレッジ付き
npm run test:coverage -- markdown-renderer
```

---

**文書情報**

| 項目 | 内容 |
|------|------|
| 作成日 | 2026-01-31 |
| 作成者 | Claude Code (michi:plan-tests) |
| バージョン | 1.0 |
