import { invoke } from '@tauri-apps/api/core';
import type { KnowledgeInput, Config, SaveKnowledgeResponse, Category, Severity } from './types';

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
