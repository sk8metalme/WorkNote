<script lang="ts">
  import DiffViewer from './DiffViewer.svelte';
  import type { FieldDiff } from '$lib/types';

  interface MultiFieldDiffViewerProps {
    diffs: FieldDiff[];
    onAcceptField?: (field: 'symptoms' | 'procedure' | 'notes') => void;
    onRejectField?: (field: 'symptoms' | 'procedure' | 'notes') => void;
  }

  let { diffs, onAcceptField, onRejectField }: MultiFieldDiffViewerProps = $props();

  let activeTab = $state<'symptoms' | 'procedure' | 'notes'>('symptoms');

  function handleAccept(field: 'symptoms' | 'procedure' | 'notes') {
    onAcceptField?.(field);
  }

  function handleReject(field: 'symptoms' | 'procedure' | 'notes') {
    onRejectField?.(field);
  }

  // 現在のタブの差分を取得
  const currentDiff = $derived(diffs.find(d => d.field === activeTab));
</script>

<div class="multi-field-diff-viewer">
  <div class="tabs">
    <button
      class="tab {activeTab === 'symptoms' ? 'active' : ''}"
      onclick={() => (activeTab = 'symptoms')}
    >
      症状
    </button>
    <button
      class="tab {activeTab === 'procedure' ? 'active' : ''}"
      onclick={() => (activeTab = 'procedure')}
    >
      対応手順
    </button>
    <button
      class="tab {activeTab === 'notes' ? 'active' : ''}"
      onclick={() => (activeTab = 'notes')}
    >
      注意点
    </button>
  </div>

  <div class="tab-content">
    {#if currentDiff}
      <DiffViewer
        original={currentDiff.original}
        modified={currentDiff.modified}
        onAccept={() => handleAccept(currentDiff.field)}
        onReject={() => handleReject(currentDiff.field)}
      />
    {:else}
      <div class="no-diff">差分がありません</div>
    {/if}
  </div>
</div>

<style>
  .multi-field-diff-viewer {
    display: flex;
    flex-direction: column;
    height: 100%;
    border: 1px solid #D1D6DE;
    border-radius: 6px;
    overflow: hidden;
    background-color: #ffffff;
  }

  .tabs {
    display: flex;
    border-bottom: 1px solid #D1D6DE;
    background-color: #F7F8FA;
  }

  .tab {
    padding: 10px 20px;
    font-size: 14px;
    font-weight: 500;
    color: #6B7684;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    transition: all 0.2s;
  }

  .tab:hover {
    color: #2A303A;
    background-color: #E9ECF0;
  }

  .tab.active {
    color: #2A303A;
    border-bottom-color: #06C755;
    font-weight: 600;
  }

  .tab-content {
    flex: 1;
    overflow: hidden;
  }

  .no-diff {
    padding: 20px;
    text-align: center;
    color: #6B7684;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .multi-field-diff-viewer {
      background-color: #0d1117;
      border-color: #30363d;
    }

    .tabs {
      background-color: #161b22;
      border-bottom-color: #30363d;
    }

    .tab {
      color: #8b949e;
    }

    .tab:hover {
      color: #e6edf3;
      background-color: #21262d;
    }

    .tab.active {
      color: #e6edf3;
      border-bottom-color: #06C755;
    }

    .no-diff {
      color: #8b949e;
    }
  }
</style>
