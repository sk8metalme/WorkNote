// WorkNote TypeScript Type Definitions

/**
 * カテゴリ種別
 */
export type Category = 'alerts' | 'maintenance' | 'ops' | 'troubleshooting' | 'inquiry';

/**
 * 重要度
 */
export type Severity = 'low' | 'medium' | 'high' | 'critical';

/**
 * コミットモード
 */
export type CommitMode = 'direct' | 'feature-branch';

/**
 * カテゴリ別判断軸
 */
export interface CategoryJudgment {
  alerts?: {
    threshold?: string;           // 閾値・条件
    escalationCriteria?: string;  // エスカレーション基準
  };
  ops?: {
    workCriteria?: string;        // 作業基準
    timing?: string;              // 実施タイミング
  };
  troubleshooting?: {
    rootCauseProcess?: string;    // 原因特定プロセス
    investigationSteps?: string;  // 調査手順
  };
  inquiry?: {
    priorityCriteria?: string;    // 対応優先度基準
    responseGuideline?: string;   // 回答指針
  };
}

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
  judgment?: string; // JSON文字列として保存
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
 * 添削設定
 */
export interface ProofreadConfig {
  prompt: string;
}

/**
 * アプリケーション設定
 */
export interface Config {
  version: number;
  git: GitConfig;
  author?: AuthorConfig; // 後方互換性のためオプショナル（git configから取得するため不要）
  shortcuts: ShortcutsConfig;
  preferences: PreferencesConfig;
  proofread?: ProofreadConfig;
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

/**
 * 一括添削リクエスト
 */
export interface ProofreadRequest {
  symptoms: string;
  procedure: string;
  notes?: string;
}

/**
 * 一括添削レスポンス
 */
export interface ProofreadResponse {
  symptoms: string;
  procedure: string;
  notes?: string;
}

/**
 * フィールド差分
 */
export interface FieldDiff {
  field: 'symptoms' | 'procedure' | 'notes';
  label: string;
  original: string;
  modified: string;
}
