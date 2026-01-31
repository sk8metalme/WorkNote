<script lang="ts">
  import SettingsWindow from '$lib/../components/SettingsWindow.svelte';
  import { goto } from '$app/navigation';

  function goBack() {
    goto('/');
  }
</script>

<div class="min-h-screen bg-gray-50">
  <header class="bg-white shadow-sm">
    <div class="max-w-7xl mx-auto px-4 py-4 flex justify-between items-center">
      <button
        on:click={goBack}
        class="text-sm text-blue-600 hover:text-blue-700"
      >
        ← 戻る
      </button>
      <h1 class="text-xl font-bold text-gray-900">設定</h1>
      <div class="w-16"></div>
    </div>
  </header>

  <main>
    <SettingsWindow />
  </main>
</div>
