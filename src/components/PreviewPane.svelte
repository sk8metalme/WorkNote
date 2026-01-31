<script lang="ts">
	import DOMPurify from 'dompurify';

	/**
	 * PreviewPane: Markdownプレビュー表示コンポーネント
	 *
	 * HTMLレンダリング済みのコンテンツを受け取り、
	 * DOMPurifyでサニタイズした上で、
	 * Tailwind CSS Proseスタイルで整形して表示します。
	 */
	export let htmlContent: string = '';
	export let isLoading: boolean = false;

	// XSS対策: DOMPurifyでHTMLをサニタイズ
	$: sanitizedHtml = htmlContent ? DOMPurify.sanitize(htmlContent) : '';
</script>

<div class="preview-pane h-full overflow-y-auto bg-white dark:bg-gray-900 border-l border-gray-200 dark:border-gray-700">
	{#if isLoading}
		<div class="flex items-center justify-center h-full text-gray-500 dark:text-gray-400">
			<div class="animate-pulse">プレビュー生成中...</div>
		</div>
	{:else if htmlContent}
		<article class="prose prose-slate dark:prose-invert max-w-none p-6">
			{@html sanitizedHtml}
		</article>
	{:else}
		<div class="flex items-center justify-center h-full text-gray-400 dark:text-gray-500">
			<p>プレビューなし</p>
		</div>
	{/if}
</div>

<style>
	.preview-pane {
		/* スクロールバーのスタイリング */
		scrollbar-width: thin;
		scrollbar-color: rgba(156, 163, 175, 0.5) transparent;
	}

	.preview-pane::-webkit-scrollbar {
		width: 8px;
	}

	.preview-pane::-webkit-scrollbar-track {
		background: transparent;
	}

	.preview-pane::-webkit-scrollbar-thumb {
		background-color: rgba(156, 163, 175, 0.5);
		border-radius: 4px;
	}

	.preview-pane::-webkit-scrollbar-thumb:hover {
		background-color: rgba(156, 163, 175, 0.7);
	}
</style>
