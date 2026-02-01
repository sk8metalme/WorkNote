// WorkNote TypeScript Type Definitions

/**
 * カテゴリ種別
 */
export type Category = 'alerts' | 'maintenance' | 'troubleshooting';

/**
 * 重要度
 */
export type Severity = 'low' | 'medium' | 'high' | 'critical';

/**
 * コミットモード
 */
export type CommitMode = 'direct' | 'feature-branch';

/**
 * ナレッジ入力データ
 */
export interface KnowledgeInput {
  title: string;
  category: Category;
  severity: Severity;
  symptoms: string;
  procedure: string;
  notes?: string;
  relatedLinks?: string;
}

/**
 * Git設定
 */
export interface GitConfig {
  repositoryPath: string;
  savePath: string;
  defaultBranch: string;
  commitMode: CommitMode;
}

/**
 * Author設定
 */
export interface AuthorConfig {
  name: string;
  email: string;
}

/**
 * ショートカット設定
 */
export interface ShortcutsConfig {
  quickInput: string;
}

/**
 * 環境設定
 */
export interface PreferencesConfig {
  launchAtLogin: boolean;
  showInMenuBar: boolean;
  showNotifications: boolean;
}

/**
 * アプリケーション設定
 */
export interface Config {
  version: number;
  git: GitConfig;
  author: AuthorConfig;
  shortcuts: ShortcutsConfig;
  preferences: PreferencesConfig;
}

/**
 * バリデーションエラー
 */
export interface ValidationError {
  field: string;
  message: string;
}

/**
 * バリデーション結果
 */
export interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
}

/**
 * ナレッジ保存レスポンス
 */
export interface SaveKnowledgeResponse {
  success: boolean;
  filePath?: string;
  branch?: string;
  commitHash?: string;
  prUrl?: string;
  error?: string;
}

/**
 * 下書きデータ
 */
export interface Draft {
  id: string;
  data: KnowledgeInput;
  createdAt: string;
  updatedAt: string;
}

/**
 * エラー種別
 */
export type ErrorType =
  | 'GitError'
  | 'FileError'
  | 'ValidationError'
  | 'ConfigError'
  | 'NetworkError'
  | 'AuthError'
  | 'ProofreadError';

/**
 * アプリケーションエラー
 */
export interface AppError {
  type: ErrorType;
  message: string;
  details?: string;
}
