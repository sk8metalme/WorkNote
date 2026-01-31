import { invoke } from '@tauri-apps/api/core';
import type { KnowledgeInput, Config, SaveKnowledgeResponse, AppError } from './types';

/**
 * ナレッジを保存
 */
export async function saveKnowledge(input: KnowledgeInput): Promise<SaveKnowledgeResponse> {
  try {
    const commitHash = await invoke<string>('save_knowledge', { input });
    return {
      success: true,
      commitHash,
      filePath: undefined,
      branch: undefined,
      error: undefined
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
