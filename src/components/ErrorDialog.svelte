<script lang="ts">
  import type { AppError } from '$lib/types';

  export let error: AppError | null = null;
  export let onRetry: (() => void) | null = null;
  export let onClose: () => void;

  function handleRetry() {
    if (onRetry) {
      onRetry();
    }
    onClose();
  }
</script>

{#if error}
  <div
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    role="presentation"
  >
    <div
      class="bg-white rounded-lg p-6 max-w-md w-full mx-4"
      role="dialog"
      aria-labelledby="error-dialog-title"
      aria-modal="true"
    >
      <h2 id="error-dialog-title" class="text-xl font-bold text-red-600 mb-4">エラーが発生しました</h2>

      <div class="mb-4">
        <p class="text-gray-700 font-semibold mb-2">{error.type}</p>
        <p class="text-gray-600">{error.message}</p>
        {#if error.details}
          <p class="text-gray-500 text-sm mt-2">{error.details}</p>
        {/if}
      </div>

      <div class="flex gap-2 justify-end">
        {#if onRetry}
          <button
            on:click={handleRetry}
            class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            リトライ
          </button>
        {/if}
        <button
          on:click={onClose}
          class="px-4 py-2 bg-gray-300 text-gray-700 rounded hover:bg-gray-400"
        >
          閉じる
        </button>
      </div>
    </div>
  </div>
{/if}
