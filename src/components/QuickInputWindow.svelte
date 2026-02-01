<script lang="ts">
  import { validateTitle, validateCategory, validateSeverity } from '$lib/validation';
  import type { Category, Severity } from '$lib/types';
  import { goto } from '$app/navigation';
  import { quickSaveKnowledge, hideQuickInputWindow } from '$lib/tauri-bridge';

  let title = '';
  let category: Category | undefined = undefined;
  let severity: Severity | undefined = undefined;

  let errors: Record<string, string> = {};
  let saving = false;

  function handleDetailInput() {
    // Navigate to detail input window
    goto('/');
  }

  async function handleQuickSave() {
    // Validate required fields
    errors = {};

    const titleValidation = validateTitle(title);
    if (titleValidation) {
      errors.title = titleValidation.message;
    }

    const categoryValidation = validateCategory(category || '');
    if (categoryValidation) {
      errors.category = categoryValidation.message;
    }

    const severityValidation = validateSeverity(severity || '');
    if (severityValidation) {
      errors.severity = severityValidation.message;
    }

    if (Object.keys(errors).length > 0) {
      return;
    }

    saving = true;
    try {
      const result = await quickSaveKnowledge(title, category!, severity!);

      if (result.success) {
        // Show success message with PR URL if available
        if (result.prUrl) {
          alert(`保存に成功しました\n\nPR作成URL:\n${result.prUrl}`);
        }

        // Clear form
        title = '';
        category = undefined;
        severity = undefined;
        errors = {};

        // Close window (hide)
        await hideQuickInputWindow();
      } else {
        errors.save = result.error || '保存に失敗しました';
      }
    } catch (e: any) {
      errors.save = e.message || '保存中にエラーが発生しました';
    } finally {
      saving = false;
    }
  }

  async function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      event.preventDefault();
      await handleQuickSave();
    } else if (event.key === 'Escape') {
      await hideQuickInputWindow();
    }
  }
</script>

<svelte:window on:keydown={handleKeyDown} />

<div class="min-h-screen bg-white flex items-start justify-center pt-20">
  <div class="bg-white rounded-lg shadow-2xl p-6 w-[480px]">
    <div class="flex justify-between items-center mb-4">
      <h1 class="text-lg font-bold text-gray-900">📝 WorkNote</h1>
    </div>

    <form on:submit|preventDefault={handleQuickSave} class="space-y-4">
      <!-- タイトル -->
      <div>
        <label class="block text-sm font-medium mb-1">タイトル *</label>
        <input
          type="text"
          bind:value={title}
          class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="例: CPU高騰対応"
          autofocus
        />
        {#if errors.title}
          <p class="text-red-600 text-sm mt-1">{errors.title}</p>
        {/if}
      </div>

      <!-- カテゴリと重要度 -->
      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium mb-1">カテゴリ *</label>
          <select
            bind:value={category}
            class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value={undefined}>選択してください</option>
            <option value="alerts">alerts</option>
            <option value="maintenance">maintenance</option>
            <option value="troubleshooting">troubleshooting</option>
          </select>
          {#if errors.category}
            <p class="text-red-600 text-sm mt-1">{errors.category}</p>
          {/if}
        </div>

        <div>
          <label class="block text-sm font-medium mb-1">重要度 *</label>
          <div class="space-y-1">
            <label class="flex items-center">
              <input type="radio" bind:group={severity} value="low" class="mr-2" />
              <span class="text-sm">low</span>
            </label>
            <label class="flex items-center">
              <input type="radio" bind:group={severity} value="medium" class="mr-2" />
              <span class="text-sm">medium</span>
            </label>
            <label class="flex items-center">
              <input type="radio" bind:group={severity} value="high" class="mr-2" />
              <span class="text-sm">high</span>
            </label>
            <label class="flex items-center">
              <input type="radio" bind:group={severity} value="critical" class="mr-2" />
              <span class="text-sm">critical</span>
            </label>
          </div>
          {#if errors.severity}
            <p class="text-red-600 text-sm mt-1">{errors.severity}</p>
          {/if}
        </div>
      </div>

      {#if errors.save}
        <p class="text-red-600 text-sm">{errors.save}</p>
      {/if}

      <!-- ボタン -->
      <div class="flex gap-2 pt-2">
        <button
          type="button"
          on:click={handleDetailInput}
          class="flex-1 bg-gray-100 text-gray-700 px-4 py-2 rounded hover:bg-gray-200 text-sm"
        >
          詳細入力へ →
        </button>
        <button
          type="submit"
          disabled={saving}
          class="flex-1 bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 disabled:opacity-50 text-sm"
        >
          {saving ? '保存中...' : '💾 クイック保存'}
        </button>
      </div>
    </form>
  </div>
</div>
