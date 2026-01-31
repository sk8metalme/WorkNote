<script lang="ts">
  import DetailInputWindow from '$lib/../components/DetailInputWindow.svelte';
  import { goto } from '$app/navigation';

  function openSettings() {
    goto('/settings');
  }
</script>

<div class="min-h-screen bg-gray-50">
  <header class="bg-white shadow-sm">
    <div class="max-w-7xl mx-auto px-4 py-4 flex justify-between items-center">
      <h1 class="text-xl font-bold text-gray-900">WorkNote</h1>
      <button
        on:click={openSettings}
        class="text-sm text-blue-600 hover:text-blue-700"
      >
        ⚙️ 設定
      </button>
    </div>
  </header>

  <main>
    <DetailInputWindow />
  </main>
</div>
