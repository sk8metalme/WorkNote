<script lang="ts">
  import { diffLines, type Change } from 'diff';

  interface DiffViewerProps {
    original: string;
    modified: string;
    onAccept?: () => void;
    onReject?: () => void;
  }

  let { original, modified, onAccept, onReject }: DiffViewerProps = $props();

  let changes = $derived.by(() => {
    return diffLines(original, modified);
  });

  function handleAccept() {
    onAccept?.();
  }

  function handleReject() {
    onReject?.();
  }

  function getLineClass(change: Change): string {
    if (change.added) return 'diff-added';
    if (change.removed) return 'diff-removed';
    return 'diff-unchanged';
  }

  function getLinePrefix(change: Change): string {
    if (change.added) return '+';
    if (change.removed) return '-';
    return ' ';
  }
</script>

<div class="diff-viewer">
  <div class="diff-header">
    <h3 class="diff-title">添削結果</h3>
    <div class="diff-actions">
      <button class="btn-accept" onclick={handleAccept}>
        受け入れる
      </button>
      <button class="btn-reject" onclick={handleReject}>
        却下する
      </button>
    </div>
  </div>

  <div class="diff-content">
    <div class="diff-lines">
      {#each changes as change, idx (idx)}
        {#each change.value.split('\n') as line, lineIdx}
          {#if line || lineIdx === 0}
            <div class="diff-line {getLineClass(change)}">
              <span class="diff-prefix">{getLinePrefix(change)}</span>
              <span class="diff-text">{line}</span>
            </div>
          {/if}
        {/each}
      {/each}
    </div>
  </div>
</div>

<style>
  .diff-viewer {
    display: flex;
    flex-direction: column;
    height: 100%;
    border: 1px solid #D1D6DE;
    border-radius: 6px;
    overflow: hidden;
    background-color: #ffffff;
  }

  .diff-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background-color: #F7F8FA;
    border-bottom: 1px solid #D1D6DE;
  }

  .diff-title {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
    color: #2A303A;
  }

  .diff-actions {
    display: flex;
    gap: 8px;
  }

  .btn-accept,
  .btn-reject {
    padding: 5px 12px;
    font-size: 13px;
    font-weight: 500;
    border-radius: 6px;
    border: 1px solid;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-accept {
    background-color: #06C755;
    color: #ffffff;
    border-color: rgba(27, 31, 36, 0.15);
  }

  .btn-accept:hover {
    background-color: #05b34c;
  }

  .btn-reject {
    background-color: #ffffff;
    color: #24292f;
    border-color: rgba(27, 31, 36, 0.15);
  }

  .btn-reject:hover {
    background-color: #E9ECF0;
  }

  .diff-content {
    flex: 1;
    overflow: auto;
  }

  .diff-lines {
    font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas,
      'Liberation Mono', monospace;
    font-size: 12px;
    line-height: 20px;
  }

  .diff-line {
    display: flex;
    min-height: 20px;
    padding: 0 10px;
    white-space: pre-wrap;
    word-wrap: break-word;
  }

  .diff-prefix {
    flex-shrink: 0;
    width: 20px;
    user-select: none;
    text-align: center;
    color: rgba(31, 35, 40, 0.5);
  }

  .diff-text {
    flex: 1;
    padding-left: 10px;
  }

  .diff-added {
    background-color: #d1f4d1;
    color: #1a7f37;
  }

  .diff-added .diff-prefix {
    background-color: #a6e9a6;
    color: #1a7f37;
  }

  .diff-removed {
    background-color: #ffccc7;
    color: #d1242f;
  }

  .diff-removed .diff-prefix {
    background-color: #ff9a8c;
    color: #d1242f;
  }

  .diff-unchanged {
    color: #1f2328;
  }

  .diff-unchanged .diff-prefix {
    color: rgba(31, 35, 40, 0.3);
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .diff-viewer {
      background-color: #0d1117;
      border-color: #30363d;
    }

    .diff-header {
      background-color: #161b22;
      border-bottom-color: #30363d;
    }

    .diff-title {
      color: #e6edf3;
    }

    .btn-accept {
      background-color: #06C755;
    }

    .btn-accept:hover {
      background-color: #05b34c;
    }

    .btn-reject {
      background-color: #21262d;
      color: #e6edf3;
    }

    .btn-reject:hover {
      background-color: #30363d;
    }

    .diff-added {
      background-color: rgba(46, 160, 67, 0.15);
      color: #7ee787;
    }

    .diff-added .diff-prefix {
      background-color: rgba(46, 160, 67, 0.3);
      color: #7ee787;
    }

    .diff-removed {
      background-color: rgba(248, 81, 73, 0.15);
      color: #ff7b72;
    }

    .diff-removed .diff-prefix {
      background-color: rgba(248, 81, 73, 0.3);
      color: #ff7b72;
    }

    .diff-unchanged {
      color: #e6edf3;
    }
  }
</style>
