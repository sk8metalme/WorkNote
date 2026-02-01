/**
 * localStorage draft storage utility
 * 入力中のデータを自動保存するためのユーティリティ
 */

import type { KnowledgeInput } from './types';

const DRAFT_KEY = 'worknote_draft';

/**
 * 下書きをlocalStorageに保存
 */
export function saveDraftToLocal(input: Partial<KnowledgeInput>): void {
  try {
    const draft = {
      ...input,
      _savedAt: new Date().toISOString()
    };
    localStorage.setItem(DRAFT_KEY, JSON.stringify(draft));
  } catch (e) {
    console.error('Failed to save draft to localStorage:', e);
  }
}

/**
 * localStorageから下書きを読み込み
 */
export function loadDraftFromLocal(): Partial<KnowledgeInput> | null {
  try {
    const saved = localStorage.getItem(DRAFT_KEY);
    if (!saved) return null;

    const draft = JSON.parse(saved);
    // _savedAtを除外して返す
    const { _savedAt, ...input } = draft;
    return input;
  } catch (e) {
    console.error('Failed to load draft from localStorage:', e);
    return null;
  }
}

/**
 * localStorageの下書きをクリア
 */
export function clearDraftFromLocal(): void {
  try {
    localStorage.removeItem(DRAFT_KEY);
  } catch (e) {
    console.error('Failed to clear draft from localStorage:', e);
  }
}
