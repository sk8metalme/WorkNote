<script lang="ts">
  import DetailInputWindow from '$lib/../components/DetailInputWindow.svelte';
  import { goto } from '$app/navigation';

  function openSettings() {
    goto('/settings');
  }
</script>

<div class="h-screen flex flex-col bg-ly-gray-50 overflow-hidden">
  <header class="bg-white shadow-sm flex-shrink-0">
    <div class="max-w-7xl mx-auto px-4 py-4 flex justify-between items-center">
      <h1 class="text-xl font-bold text-ly-gray-900">WorkNote</h1>
      <button
        on:click={openSettings}
        class="text-sm text-ly-green hover:text-ly-green/90"
      >
        ⚙️ 設定
      </button>
    </div>
  </header>

  <main class="flex-1 overflow-hidden">
    <DetailInputWindow />
  </main>
</div>
