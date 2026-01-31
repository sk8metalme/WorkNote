# 単体テスト仕様: FormValidator

## 概要

FormValidatorは、フロントエンドでの入力バリデーションを担当します。ユーザー入力の即時フィードバックを提供します。

---

## テスト対象

**モジュール**: `src/lib/validation.ts`

**主要関数**:
- `validateTitle(value: string): ValidationError | null`
- `validateCategory(value: string): ValidationError | null`
- `validateSeverity(value: string): ValidationError | null`
- `validateSymptoms(value: string): ValidationError | null`
- `validateProcedure(value: string): ValidationError | null`
- `validateNotes(value: string): ValidationError | null`
- `validateLinks(value: string): ValidationError | null`
- `validateKnowledgeInput(input: KnowledgeInput): ValidationResult`

---

## テストケース一覧

### TC-U-FV-001: タイトルバリデーション - 正常系

**入力**: `"CPU高騰対応"`

**期待結果**: `null`（エラーなし）

**実装例**:
```typescript
test('TC-U-FV-001: 正常なタイトルはエラーなし', () => {
  const result = validateTitle("CPU高騰対応");
  expect(result).toBeNull();
});
```

---

### TC-U-FV-002: タイトルバリデーション - 空文字

**入力**: `""`

**期待結果**: `{ field: 'title', message: 'タイトルは必須です' }`

**実装例**:
```typescript
test('TC-U-FV-002: 空文字はエラー', () => {
  const result = validateTitle("");
  expect(result).toEqual({
    field: 'title',
    message: 'タイトルは必須です'
  });
});
```

---

### TC-U-FV-003: タイトルバリデーション - 空白のみ

**入力**: `"   "`

**期待結果**: `{ field: 'title', message: 'タイトルは必須です' }`

**実装例**:
```typescript
test('TC-U-FV-003: 空白のみはエラー', () => {
  const result = validateTitle("   ");
  expect(result).toEqual({
    field: 'title',
    message: 'タイトルは必須です'
  });
});
```

---

### TC-U-FV-004: タイトルバリデーション - 100文字

**入力**: `"a".repeat(100)`

**期待結果**: `null`（エラーなし、境界値）

**実装例**:
```typescript
test('TC-U-FV-004: 100文字はOK（境界値）', () => {
  const result = validateTitle("a".repeat(100));
  expect(result).toBeNull();
});
```

---

### TC-U-FV-005: タイトルバリデーション - 101文字

**入力**: `"a".repeat(101)`

**期待結果**: `{ field: 'title', message: 'タイトルは100文字以内で入力してください' }`

**実装例**:
```typescript
test('TC-U-FV-005: 101文字はエラー（境界値）', () => {
  const result = validateTitle("a".repeat(101));
  expect(result).toEqual({
    field: 'title',
    message: 'タイトルは100文字以内で入力してください'
  });
});
```

---

### TC-U-FV-006: カテゴリバリデーション - 正常系

**入力**: `"alerts"`

**期待結果**: `null`

**実装例**:
```typescript
test('TC-U-FV-006: 正常なカテゴリはエラーなし', () => {
  const result = validateCategory("alerts");
  expect(result).toBeNull();
});
```

---

### TC-U-FV-007: カテゴリバリデーション - 空文字

**入力**: `""`

**期待結果**: `{ field: 'category', message: 'カテゴリを選択してください' }`

**実装例**:
```typescript
test('TC-U-FV-007: 空文字はエラー', () => {
  const result = validateCategory("");
  expect(result).toEqual({
    field: 'category',
    message: 'カテゴリを選択してください'
  });
});
```

---

### TC-U-FV-008: 重要度バリデーション - 正常系

**入力**: `"high"`

**期待結果**: `null`

**実装例**:
```typescript
test('TC-U-FV-008: 正常な重要度はエラーなし', () => {
  const result = validateSeverity("high");
  expect(result).toBeNull();
});
```

---

### TC-U-FV-009: 症状バリデーション - 5000文字（境界値）

**入力**: `"a".repeat(5000)`

**期待結果**: `null`

**実装例**:
```typescript
test('TC-U-FV-009: 5000文字はOK（境界値）', () => {
  const result = validateSymptoms("a".repeat(5000));
  expect(result).toBeNull();
});
```

---

### TC-U-FV-010: 症状バリデーション - 5001文字（境界値）

**入力**: `"a".repeat(5001)`

**期待結果**: `{ field: 'symptoms', message: '症状・検知条件は5000文字以内で入力してください' }`

**実装例**:
```typescript
test('TC-U-FV-010: 5001文字はエラー（境界値）', () => {
  const result = validateSymptoms("a".repeat(5001));
  expect(result).toEqual({
    field: 'symptoms',
    message: '症状・検知条件は5000文字以内で入力してください'
  });
});
```

---

### TC-U-FV-011: 対応手順バリデーション - 10000文字（境界値）

**入力**: `"a".repeat(10000)`

**期待結果**: `null`

**実装例**:
```typescript
test('TC-U-FV-011: 10000文字はOK（境界値）', () => {
  const result = validateProcedure("a".repeat(10000));
  expect(result).toBeNull();
});
```

---

### TC-U-FV-012: 統合バリデーション - 正常系

**入力**:
```typescript
{
  title: "CPU高騰対応",
  category: "alerts",
  severity: "high",
  symptoms: "CPU使用率が90%を超えている",
  procedure: "1. プロセス一覧を確認",
  notes: "ピーク時間帯は注意",
  links: "https://jira.example.com/TICKET-123"
}
```

**期待結果**: `{ valid: true, errors: [] }`

**実装例**:
```typescript
test('TC-U-FV-012: 正常な入力はエラーなし', () => {
  const input = {
    title: "CPU高騰対応",
    category: "alerts" as Category,
    severity: "high" as Severity,
    symptoms: "CPU使用率が90%を超えている",
    procedure: "1. プロセス一覧を確認",
    notes: "ピーク時間帯は注意",
    links: "https://jira.example.com/TICKET-123"
  };
  const result = validateKnowledgeInput(input);
  expect(result.valid).toBe(true);
  expect(result.errors).toHaveLength(0);
});
```

---

### TC-U-FV-013: 統合バリデーション - 複数エラー

**入力**:
```typescript
{
  title: "",
  category: "",
  severity: "",
}
```

**期待結果**:
```typescript
{
  valid: false,
  errors: [
    { field: 'title', message: 'タイトルは必須です' },
    { field: 'category', message: 'カテゴリを選択してください' },
    { field: 'severity', message: '重要度を選択してください' }
  ]
}
```

**実装例**:
```typescript
test('TC-U-FV-013: 複数エラーをすべて返す', () => {
  const input = {
    title: "",
    category: "" as Category,
    severity: "" as Severity,
  };
  const result = validateKnowledgeInput(input);
  expect(result.valid).toBe(false);
  expect(result.errors).toHaveLength(3);
});
```

---

## カバレッジ目標

- **行カバレッジ**: 100%
- **分岐カバレッジ**: 100%
- **関数カバレッジ**: 100%

---

## 実行方法

```bash
# FormValidatorのテストのみ実行
npm run test:unit -- form-validator

# カバレッジ付き
npm run test:coverage -- form-validator
```

---

**文書情報**

| 項目 | 内容 |
|------|------|
| 作成日 | 2026-01-31 |
| 作成者 | Claude Code (michi:plan-tests) |
| バージョン | 1.0 |
