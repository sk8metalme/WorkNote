<script lang="ts">
  import { onMount } from 'svelte';
  import { validateKnowledgeInput } from '$lib/validation';
  import { saveKnowledge, renderMarkdown, proofreadAllFields, createDraft, listDrafts, loadDraft, deleteDraft, type DraftSummary } from '$lib/tauri-bridge';
  import { saveDraftToLocal, loadDraftFromLocal, clearDraftFromLocal } from '$lib/draft-storage';
  import ErrorDialog from './ErrorDialog.svelte';
  import PreviewPane from './PreviewPane.svelte';
  import MultiFieldDiffViewer from './MultiFieldDiffViewer.svelte';
  import CategoryFormFields from './CategoryFormFields.svelte';
  import type { KnowledgeInput, AppError, FieldDiff, ProofreadRequest } from '$lib/types';

  let input = $state<Partial<KnowledgeInput>>({
    title: '',
    category: '' as any,
    severity: '' as any,
    symptoms: '',
    procedure: '',
    notes: '',
    relatedLinks: ''
  });

  // カテゴリ別判断軸
  let judgment = $state<Record<string, string>>({});

  // 下書き関連の状態
  let showDraftList = $state(false);
  let draftList = $state<DraftSummary[]>([]);
  let savingDraft = $state(false);
  let currentDraftId = $state<string | null>(null);

  let errors = $state<Record<string, string>>({});
  let saving = $state(false);
  let error = $state<AppError | null>(null);
  let previewHtml = $state('');
  let previewLoading = $state(false);
  let debounceTimer: number | null = null;
  let autoSaveTimer: number | null = null;

  // AI一括添削関連の状態
  let fieldDiffs = $state<FieldDiff[]>([]);
  let showDiffViewer = $state(false);
  let proofreading = $state(false);

  // コンポーネントマウント時にlocalStorageから下書きを復元
  onMount(() => {
    const savedDraft = loadDraftFromLocal();
    if (savedDraft) {
      const shouldRestore = confirm('前回の入力内容が残っています。復元しますか？');
      if (shouldRestore) {
        input = savedDraft;
        // judgmentも復元
        if (savedDraft.judgment) {
          try {
            judgment = JSON.parse(savedDraft.judgment);
          } catch (e) {
            console.error('Failed to parse judgment:', e);
          }
        }
      } else {
        clearDraftFromLocal();
      }
    }
  });

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

  // 判断軸をJSON文字列に変換してinputに設定
  function updateJudgmentInInput() {
    if (Object.keys(judgment).length > 0) {
      input.judgment = JSON.stringify(judgment);
    } else {
      input.judgment = undefined;
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
      input.relatedLinks,
      JSON.stringify(judgment) // judgmentも監視対象に追加
    ];

    // judgmentをinputに反映
    updateJudgmentInInput();

    // localStorage自動保存（デバウンス）
    if (autoSaveTimer !== null) {
      clearTimeout(autoSaveTimer);
    }
    autoSaveTimer = window.setTimeout(() => {
      saveDraftToLocal(input);
    }, 1000);

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
      if (autoSaveTimer !== null) {
        clearTimeout(autoSaveTimer);
      }
    };
  });

  async function handleProofreadAll() {
    if (!input.symptoms?.trim() || !input.procedure?.trim()) {
      alert('症状と対応手順を入力してください');
      return;
    }

    proofreading = true;

    try {
      const request: ProofreadRequest = {
        symptoms: input.symptoms || '',
        procedure: input.procedure || '',
        notes: input.notes || undefined
      };

      const response = await proofreadAllFields(request);

      // FieldDiffの配列を作成
      fieldDiffs = [
        {
          field: 'symptoms',
          label: '症状',
          original: input.symptoms || '',
          modified: response.symptoms
        },
        {
          field: 'procedure',
          label: '対応手順',
          original: input.procedure || '',
          modified: response.procedure
        }
      ];

      // notesがある場合のみ追加
      if (input.notes?.trim()) {
        fieldDiffs.push({
          field: 'notes',
          label: '注意点',
          original: input.notes,
          modified: response.notes || ''
        });
      }

      showDiffViewer = true;
    } catch (e: any) {
      error = { type: 'ProofreadError', message: e.message || '添削に失敗しました' };
    } finally {
      proofreading = false;
    }
  }

  function handleAcceptField(field: 'symptoms' | 'procedure' | 'notes') {
    const diff = fieldDiffs.find(d => d.field === field);
    if (diff) {
      input[field] = diff.modified;
    }
  }

  function handleRejectField(field: 'symptoms' | 'procedure' | 'notes') {
    // 何もしない（元の値を保持）
  }

  async function handleSaveDraftToFile() {
    const validation = validateKnowledgeInput(input);
    if (!validation.valid) {
      alert('入力内容に不備があります。必須項目を入力してください。');
      return;
    }

    savingDraft = true;
    try {
      const draft = await createDraft(input as KnowledgeInput);
      currentDraftId = draft.id;
      alert('下書きをファイルに保存しました');
    } catch (e: any) {
      error = { type: 'FileError', message: e.message || '下書き保存に失敗しました' };
    } finally {
      savingDraft = false;
    }
  }

  async function handleShowDraftList() {
    try {
      draftList = await listDrafts();
      showDraftList = true;
    } catch (e: any) {
      error = { type: 'FileError', message: e.message || '下書き一覧の取得に失敗しました' };
    }
  }

  async function handleLoadDraft(id: string) {
    try {
      const draft = await loadDraft(id);
      input = draft.data;
      currentDraftId = draft.id;
      // judgmentも復元
      if (draft.data.judgment) {
        try {
          judgment = JSON.parse(draft.data.judgment);
        } catch (e) {
          console.error('Failed to parse judgment:', e);
        }
      }
      showDraftList = false;
      alert('下書きを読み込みました');
    } catch (e: any) {
      error = { type: 'FileError', message: e.message || '下書きの読み込みに失敗しました' };
    }
  }

  async function handleDeleteDraft(id: string) {
    if (!confirm('この下書きを削除しますか？')) return;

    try {
      await deleteDraft(id);
      draftList = await listDrafts();
      alert('下書きを削除しました');
    } catch (e: any) {
      error = { type: 'FileError', message: e.message || '下書きの削除に失敗しました' };
    }
  }

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
        input = { title: '', category: '' as any, severity: '' as any, symptoms: '', procedure: '', notes: '', relatedLinks: '' };
        judgment = {};
        currentDraftId = null;
        previewHtml = '';
        clearDraftFromLocal();
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

<div class="h-full flex">
  <!-- 左側: 入力フォーム -->
  <div class="flex-1 overflow-y-auto p-6">
    <h1 class="text-2xl font-bold mb-6">ナレッジ入力</h1>

    <form onsubmit={(e) => { e.preventDefault(); handleSave(); }} class="space-y-4">
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
            <option value="ops">ops（運用作業）</option>
            <option value="troubleshooting">troubleshooting</option>
            <option value="inquiry">inquiry（問い合わせ）</option>
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

      <!-- カテゴリ別判断軸フィールド -->
      <CategoryFormFields
        category={input.category || ''}
        {judgment}
        onUpdate={(updated) => judgment = updated}
      />

      <div class="flex gap-4">
        <button
          type="button"
          onclick={handleShowDraftList}
          class="bg-ly-gray-600 text-white px-4 py-2 rounded hover:bg-ly-gray-700 text-sm"
        >
          📂 下書き一覧
        </button>

        <button
          type="button"
          onclick={handleSaveDraftToFile}
          disabled={savingDraft}
          class="bg-ly-navy text-white px-4 py-2 rounded hover:bg-ly-navy/90 disabled:opacity-50 text-sm"
        >
          {savingDraft ? '保存中...' : '💾 下書き保存'}
        </button>

        <button
          type="button"
          onclick={handleProofreadAll}
          disabled={proofreading}
          class="bg-ly-navy text-white px-6 py-3 rounded hover:bg-ly-navy/90 disabled:opacity-50"
        >
          {proofreading ? 'AI添削中...' : 'AI一括添削'}
        </button>

        <button type="submit" disabled={saving} class="bg-ly-green text-white px-6 py-3 rounded hover:bg-ly-green/90 disabled:opacity-50">
          {saving ? '保存中...' : 'Git Commit & Push'}
        </button>
      </div>
    </form>
  </div>

  <!-- 右側: プレビュー -->
  <div class="w-1/2">
    <PreviewPane htmlContent={previewHtml} isLoading={previewLoading} />
  </div>
</div>

<ErrorDialog {error} onClose={() => error = null} />

<!-- 下書き一覧モーダル -->
{#if showDraftList}
  <div
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    role="dialog"
    aria-modal="true"
    aria-labelledby="draft-list-title"
  >
    <div class="bg-white rounded-lg shadow-xl w-3/5 max-h-4/5 flex flex-col">
      <div class="flex justify-between items-center p-4 border-b">
        <h2 id="draft-list-title" class="text-xl font-bold">下書き一覧</h2>
        <button
          onclick={() => showDraftList = false}
          aria-label="閉じる"
          class="text-gray-500 hover:text-gray-700"
        >
          ✕
        </button>
      </div>
      <div class="flex-1 overflow-auto p-4">
        {#if draftList.length === 0}
          <p class="text-gray-500 text-center py-8">下書きがありません</p>
        {:else}
          <div class="space-y-2">
            {#each draftList as draft}
              <div class="border rounded p-4 hover:bg-gray-50">
                <div class="flex justify-between items-start">
                  <div class="flex-1">
                    <h3 class="font-medium">{draft.title}</h3>
                    <p class="text-sm text-gray-600">
                      カテゴリ: {draft.category} |
                      更新: {new Date(draft.updatedAt).toLocaleString('ja-JP')}
                    </p>
                  </div>
                  <div class="flex gap-2">
                    <button
                      onclick={() => handleLoadDraft(draft.id)}
                      class="bg-ly-green text-white px-3 py-1 rounded hover:bg-ly-green/90 text-sm"
                    >
                      読み込み
                    </button>
                    <button
                      onclick={() => handleDeleteDraft(draft.id)}
                      class="bg-ly-red text-white px-3 py-1 rounded hover:bg-ly-red/90 text-sm"
                    >
                      削除
                    </button>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<!-- AI添削結果モーダル -->
{#if showDiffViewer}
  <div
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    role="dialog"
    aria-modal="true"
    aria-labelledby="diff-viewer-title"
  >
    <div class="bg-white rounded-lg shadow-xl w-4/5 h-4/5 flex flex-col">
      <div class="flex justify-between items-center p-4 border-b">
        <h2 id="diff-viewer-title" class="text-xl font-bold">AI一括添削結果</h2>
        <button
          onclick={() => showDiffViewer = false}
          aria-label="閉じる"
          class="text-gray-500 hover:text-gray-700"
        >
          ✕
        </button>
      </div>
      <div class="flex-1 overflow-hidden">
        <MultiFieldDiffViewer
          diffs={fieldDiffs}
          onAcceptField={handleAcceptField}
          onRejectField={handleRejectField}
        />
      </div>
    </div>
  </div>
{/if}
