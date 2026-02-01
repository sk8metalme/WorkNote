<script lang="ts">
  import { validateKnowledgeInput } from '$lib/validation';
  import { saveKnowledge, renderMarkdown } from '$lib/tauri-bridge';
  import ErrorDialog from './ErrorDialog.svelte';
  import PreviewPane from './PreviewPane.svelte';
  import type { KnowledgeInput, AppError } from '$lib/types';

  let input = $state<Partial<KnowledgeInput>>({
    title: '',
    category: '' as any,
    severity: '' as any,
    symptoms: '',
    procedure: '',
    notes: '',
    relatedLinks: ''
  });

  let errors = $state<Record<string, string>>({});
  let saving = $state(false);
  let error = $state<AppError | null>(null);
  let previewHtml = $state('');
  let previewLoading = $state(false);
  let debounceTimer: number | null = null;

  // デバウンス付きプレビュー更新
  async function updatePreview() {
    // 最低限のバリデーション（title, category, severity, symptoms, procedureが必要）
    if (!input.title || !input.category || !input.severity || !input.symptoms || !input.procedure) {
      previewHtml = '';
      return;
    }

    previewLoading = true;

    try {
      const html = await renderMarkdown(input as KnowledgeInput);
      previewHtml = html;
    } catch (e) {
      console.error('プレビュー生成エラー:', e);
      previewHtml = '<p class="text-red-600">プレビュー生成に失敗しました</p>';
    } finally {
      previewLoading = false;
    }
  }

  // inputが変更されたらデバウンス付きでプレビュー更新
  $effect(() => {
    // inputの変更を監視（プリミティブ値に展開して依存関係を明示）
    const watchedValues = [
      input.title,
      input.category,
      input.severity,
      input.symptoms,
      input.procedure,
      input.notes,
      input.relatedLinks
    ];

    if (debounceTimer !== null) {
      clearTimeout(debounceTimer);
    }

    debounceTimer = window.setTimeout(() => {
      updatePreview();
    }, 500);

    return () => {
      if (debounceTimer !== null) {
        clearTimeout(debounceTimer);
      }
    };
  });

  async function handleSave() {
    const validation = validateKnowledgeInput(input);

    if (!validation.valid) {
      errors = validation.errors.reduce((acc, e) => ({ ...acc, [e.field]: e.message }), {});
      return;
    }

    errors = {};
    saving = true;

    try {
      const result = await saveKnowledge(input as KnowledgeInput);
      if (result.success) {
        let message = '保存に成功しました';
        if (result.prUrl) {
          message += `\n\nPR作成URL:\n${result.prUrl}`;
        }
        alert(message);
        // フォームクリア
        input = { title: '', category: undefined, severity: undefined, symptoms: '', procedure: '', notes: '', relatedLinks: '' };
        previewHtml = '';
      } else {
        error = { type: 'GitError', message: result.error || '不明なエラー' };
      }
    } catch (e: any) {
      error = { type: 'GitError', message: e.message };
    } finally {
      saving = false;
    }
  }
</script>

<div class="h-screen flex">
  <!-- 左側: 入力フォーム -->
  <div class="flex-1 overflow-y-auto p-6">
    <h1 class="text-2xl font-bold mb-6">ナレッジ入力</h1>

    <form on:submit|preventDefault={handleSave} class="space-y-4">
      <div>
        <label class="block text-sm font-medium mb-1">タイトル *</label>
        <input type="text" bind:value={input.title} class="w-full border rounded px-3 py-2" />
        {#if errors.title}<p class="text-red-600 text-sm">{errors.title}</p>{/if}
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium mb-1">カテゴリ *</label>
          <select bind:value={input.category} class="w-full border rounded px-3 py-2">
            <option value="">選択してください</option>
            <option value="alerts">alerts</option>
            <option value="maintenance">maintenance</option>
            <option value="troubleshooting">troubleshooting</option>
          </select>
          {#if errors.category}<p class="text-red-600 text-sm">{errors.category}</p>{/if}
        </div>

        <div>
          <label class="block text-sm font-medium mb-1">重要度 *</label>
          <select bind:value={input.severity} class="w-full border rounded px-3 py-2">
            <option value="">選択してください</option>
            <option value="low">low</option>
            <option value="medium">medium</option>
            <option value="high">high</option>
            <option value="critical">critical</option>
          </select>
          {#if errors.severity}<p class="text-red-600 text-sm">{errors.severity}</p>{/if}
        </div>
      </div>

      <div>
        <label class="block text-sm font-medium mb-1">症状 *</label>
        <textarea bind:value={input.symptoms} rows="3" class="w-full border rounded px-3 py-2"></textarea>
        {#if errors.symptoms}<p class="text-red-600 text-sm">{errors.symptoms}</p>{/if}
      </div>

      <div>
        <label class="block text-sm font-medium mb-1">対応手順 *</label>
        <textarea bind:value={input.procedure} rows="6" class="w-full border rounded px-3 py-2"></textarea>
        {#if errors.procedure}<p class="text-red-600 text-sm">{errors.procedure}</p>{/if}
      </div>

      <div>
        <label class="block text-sm font-medium mb-1">注意点・落とし穴</label>
        <textarea bind:value={input.notes} rows="3" class="w-full border rounded px-3 py-2"></textarea>
      </div>

      <div>
        <label class="block text-sm font-medium mb-1">関連リンク</label>
        <textarea bind:value={input.relatedLinks} rows="2" class="w-full border rounded px-3 py-2"></textarea>
      </div>

      <button type="submit" disabled={saving} class="bg-blue-600 text-white px-6 py-3 rounded hover:bg-blue-700 disabled:opacity-50">
        {saving ? '保存中...' : 'Git Commit & Push'}
      </button>
    </form>
  </div>

  <!-- 右側: プレビュー -->
  <div class="w-1/2">
    <PreviewPane htmlContent={previewHtml} isLoading={previewLoading} />
  </div>
</div>

<ErrorDialog {error} onClose={() => error = null} />
