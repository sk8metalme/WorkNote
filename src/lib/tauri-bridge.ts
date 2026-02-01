import { invoke } from '@tauri-apps/api/core';
import type { KnowledgeInput, Config, SaveKnowledgeResponse, Category, Severity, ProofreadRequest, ProofreadResponse, Draft } from './types';

/**
 * 下書きサマリー（一覧表示用）
 */
export interface DraftSummary {
  id: string;
  title: string;
  category: string;
  updatedAt: string;
}

/**
 * ナレッジを保存
 */
export async function saveKnowledge(input: KnowledgeInput): Promise<SaveKnowledgeResponse> {
  try {
    const result = await invoke<{
      commitHash: string;
      filePath: string;
      prUrl?: string;
    }>('save_knowledge', { input });
    return {
      success: true,
      commitHash: result.commitHash,
      filePath: result.filePath,
      prUrl: result.prUrl
    };
  } catch (error: any) {
    return {
      success: false,
      error: error.message || 'Unknown error occurred'
    };
  }
}

/**
 * 設定を読み込み
 */
export async function loadConfig(): Promise<Config> {
  return invoke<Config>('load_config');
}

/**
 * 設定を保存
 */
export async function saveConfig(config: Config): Promise<void> {
  await invoke('save_config', { config });
}

/**
 * クイック保存（タイトル、カテゴリ、重要度のみ）
 */
export async function quickSaveKnowledge(
  title: string,
  category: Category,
  severity: Severity
): Promise<SaveKnowledgeResponse> {
  try {
    const result = await invoke<{
      commitHash: string;
      filePath: string;
      prUrl?: string;
    }>('quick_save_knowledge', {
      title,
      category,
      severity
    });
    return {
      success: true,
      commitHash: result.commitHash,
      filePath: result.filePath,
      prUrl: result.prUrl
    };
  } catch (error: any) {
    return {
      success: false,
      error: error.message || 'Unknown error occurred'
    };
  }
}

/**
 * Quick-inputウィンドウを表示
 */
export async function showQuickInputWindow(): Promise<void> {
  await invoke('show_quick_input_window');
}

/**
 * Quick-inputウィンドウを非表示
 */
export async function hideQuickInputWindow(): Promise<void> {
  await invoke('hide_quick_input_window');
}

/**
 * MarkdownをHTMLにレンダリング
 */
export async function renderMarkdown(input: KnowledgeInput): Promise<string> {
  return invoke<string>('render_markdown', { input });
}

/**
 * Markdown文章を添削
 */
export async function proofreadMarkdown(content: string): Promise<string> {
  return invoke<string>('proofread_markdown', { content });
}

/**
 * 複数フィールドを一括添削
 */
export async function proofreadAllFields(request: ProofreadRequest): Promise<ProofreadResponse> {
  return invoke<ProofreadResponse>('proofread_all_fields', { request });
}

/**
 * 下書きを保存
 */
export async function saveDraft(draft: Draft): Promise<void> {
  await invoke('save_draft', { draft });
}

/**
 * 下書きを作成
 */
export async function createDraft(data: KnowledgeInput): Promise<Draft> {
  return invoke<Draft>('create_draft', { data });
}

/**
 * 下書きを読み込み
 */
export async function loadDraft(id: string): Promise<Draft> {
  return invoke<Draft>('load_draft', { id });
}

/**
 * 下書き一覧を取得
 */
export async function listDrafts(): Promise<DraftSummary[]> {
  return invoke<DraftSummary[]>('list_drafts');
}

/**
 * 下書きを削除
 */
export async function deleteDraft(id: string): Promise<void> {
  await invoke('delete_draft', { id });
}

/**
 * 下書きを更新
 */
export async function updateDraft(id: string, data: KnowledgeInput): Promise<Draft> {
  return invoke<Draft>('update_draft', { id, data });
}
