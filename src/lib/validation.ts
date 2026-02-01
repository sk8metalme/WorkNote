import type { KnowledgeInput, ValidationResult, ValidationError, Category, Severity } from './types';

// バリデーション定数
const TITLE_MAX_LENGTH = 100;
const SYMPTOMS_MAX_LENGTH = 5000;
const PROCEDURE_MAX_LENGTH = 10000;
const NOTES_MAX_LENGTH = 5000;
const LINKS_MAX_LENGTH = 5000;

const VALID_CATEGORIES: Category[] = ['alerts', 'maintenance', 'ops', 'troubleshooting', 'inquiry'];
const VALID_SEVERITIES: Severity[] = ['low', 'medium', 'high', 'critical'];

/**
 * タイトルをバリデーション
 */
export function validateTitle(title: string): ValidationError | null {
  if (!title || title.trim().length === 0) {
    return { field: 'title', message: 'タイトルは必須です' };
  }

  if (title.length > TITLE_MAX_LENGTH) {
    return {
      field: 'title',
      message: `タイトルは${TITLE_MAX_LENGTH}文字以内で入力してください`
    };
  }

  return null;
}

/**
 * カテゴリをバリデーション
 */
export function validateCategory(category: Category | string): ValidationError | null {
  if (!category) {
    return { field: 'category', message: 'カテゴリを選択してください' };
  }

  if (!VALID_CATEGORIES.includes(category as Category)) {
    return { field: 'category', message: '無効なカテゴリです' };
  }

  return null;
}

/**
 * 重要度をバリデーション
 */
export function validateSeverity(severity: Severity | string): ValidationError | null {
  if (!severity) {
    return { field: 'severity', message: '重要度を選択してください' };
  }

  if (!VALID_SEVERITIES.includes(severity as Severity)) {
    return { field: 'severity', message: '無効な重要度です' };
  }

  return null;
}

/**
 * 症状をバリデーション
 */
export function validateSymptoms(symptoms: string): ValidationError | null {
  if (!symptoms || symptoms.trim().length === 0) {
    return { field: 'symptoms', message: '症状は必須です' };
  }

  if (symptoms.length > SYMPTOMS_MAX_LENGTH) {
    return {
      field: 'symptoms',
      message: `症状は${SYMPTOMS_MAX_LENGTH}文字以内で入力してください`
    };
  }

  return null;
}

/**
 * 対応手順をバリデーション
 */
export function validateProcedure(procedure: string): ValidationError | null {
  if (!procedure || procedure.trim().length === 0) {
    return { field: 'procedure', message: '対応手順は必須です' };
  }

  if (procedure.length > PROCEDURE_MAX_LENGTH) {
    return {
      field: 'procedure',
      message: `対応手順は${PROCEDURE_MAX_LENGTH}文字以内で入力してください`
    };
  }

  return null;
}

/**
 * 注意点をバリデーション
 */
export function validateNotes(notes: string | undefined): ValidationError | null {
  if (!notes) {
    return null;
  }

  if (notes.length > NOTES_MAX_LENGTH) {
    return {
      field: 'notes',
      message: `注意点は${NOTES_MAX_LENGTH}文字以内で入力してください`
    };
  }

  return null;
}

/**
 * 関連リンクをバリデーション
 */
export function validateLinks(relatedLinks: string | undefined): ValidationError | null {
  if (!relatedLinks) {
    return null;
  }

  if (relatedLinks.length > LINKS_MAX_LENGTH) {
    return {
      field: 'relatedLinks',
      message: `関連リンクは${LINKS_MAX_LENGTH}文字以内で入力してください`
    };
  }

  return null;
}

/**
 * ナレッジ入力データを統合バリデーション
 */
export function validateKnowledgeInput(input: Partial<KnowledgeInput>): ValidationResult {
  const errors: ValidationError[] = [];

  // 各フィールドをバリデーション
  const titleError = validateTitle(input.title || '');
  if (titleError) errors.push(titleError);

  const categoryError = validateCategory(input.category || '');
  if (categoryError) errors.push(categoryError);

  const severityError = validateSeverity(input.severity || '');
  if (severityError) errors.push(severityError);

  const symptomsError = validateSymptoms(input.symptoms || '');
  if (symptomsError) errors.push(symptomsError);

  const procedureError = validateProcedure(input.procedure || '');
  if (procedureError) errors.push(procedureError);

  const notesError = validateNotes(input.notes);
  if (notesError) errors.push(notesError);

  const linksError = validateLinks(input.relatedLinks);
  if (linksError) errors.push(linksError);

  return {
    valid: errors.length === 0,
    errors
  };
}
