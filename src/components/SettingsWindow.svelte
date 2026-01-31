<script lang="ts">
  import { onMount } from 'svelte';
  import { loadConfig, saveConfig } from '$lib/tauri-bridge';
  import type { Config } from '$lib/types';

  let config: Config | null = null;
  let loading = true;
  let saving = false;
  let message = '';

  onMount(async () => {
    try {
      config = await loadConfig();
    } catch (e: any) {
      message = `設定の読み込みに失敗しました: ${e.message}`;
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!config) return;
    saving = true;
    message = '';
    try {
      await saveConfig(config);
      message = '設定を保存しました';
    } catch (e: any) {
      message = `保存に失敗しました: ${e.message}`;
    } finally {
      saving = false;
    }
  }
</script>

<div class="p-6 max-w-2xl mx-auto">
  <h1 class="text-2xl font-bold mb-6">設定</h1>

  {#if loading}
    <p>読み込み中...</p>
  {:else if config}
    <form on:submit|preventDefault={handleSave} class="space-y-4">
      <div>
        <label class="block text-sm font-medium mb-1">リポジトリパス</label>
        <input type="text" bind:value={config.git.repositoryPath} class="w-full border rounded px-3 py-2" required />
      </div>

      <div>
        <label class="block text-sm font-medium mb-1">保存先</label>
        <input type="text" bind:value={config.git.savePath} class="w-full border rounded px-3 py-2" required />
      </div>

      <div>
        <label class="block text-sm font-medium mb-1">Author名</label>
        <input type="text" bind:value={config.author.name} class="w-full border rounded px-3 py-2" required />
      </div>

      <div>
        <label class="block text-sm font-medium mb-1">Authorメール</label>
        <input type="email" bind:value={config.author.email} class="w-full border rounded px-3 py-2" required />
      </div>

      <div>
        <label class="block text-sm font-medium mb-2">コミットモード</label>
        <div class="space-y-2">
          <label class="flex items-center">
            <input
              type="radio"
              bind:group={config.git.commitMode}
              value="direct"
              class="mr-2"
            />
            <span class="text-sm">直接Push（デフォルトブランチに直接）</span>
          </label>
          <label class="flex items-center">
            <input
              type="radio"
              bind:group={config.git.commitMode}
              value="feature-branch"
              class="mr-2"
            />
            <span class="text-sm">PR作成（featureブランチを作成）</span>
          </label>
        </div>
      </div>

      <button type="submit" disabled={saving} class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 disabled:opacity-50">
        {saving ? '保存中...' : '保存'}
      </button>

      {#if message}
        <p class="text-sm {message.includes('失敗') ? 'text-red-600' : 'text-green-600'}">{message}</p>
      {/if}
    </form>
  {/if}
</div>
