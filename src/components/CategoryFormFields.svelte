<script lang="ts">
  import type { Category } from '$lib/types';

  interface Props {
    category: Category | '';
    judgment: Record<string, string>;
    onUpdate: (judgment: Record<string, string>) => void;
  }

  let { category, judgment, onUpdate }: Props = $props();

  function handleFieldChange(field: string, value: string) {
    onUpdate({
      ...judgment,
      [field]: value
    });
  }
</script>

{#if category === 'alerts'}
  <div class="space-y-4 mt-4 p-4 bg-ly-gray-50 rounded border">
    <h3 class="font-medium text-sm text-ly-gray-700">判断基準・判断軸（アラート）</h3>

    <div>
      <label class="block text-sm font-medium mb-1">閾値・条件</label>
      <textarea
        value={judgment.threshold || ''}
        oninput={(e) => handleFieldChange('threshold', e.currentTarget.value)}
        rows="2"
        class="w-full border rounded px-3 py-2 text-sm"
        placeholder="例: CPU使用率が90%を超えた場合、または5分間継続した場合"
      ></textarea>
    </div>

    <div>
      <label class="block text-sm font-medium mb-1">エスカレーション基準</label>
      <textarea
        value={judgment.escalationCriteria || ''}
        oninput={(e) => handleFieldChange('escalationCriteria', e.currentTarget.value)}
        rows="2"
        class="w-full border rounded px-3 py-2 text-sm"
        placeholder="例: 10分以内に対応できない場合は上長へ報告"
      ></textarea>
    </div>
  </div>
{:else if category === 'ops'}
  <div class="space-y-4 mt-4 p-4 bg-ly-gray-50 rounded border">
    <h3 class="font-medium text-sm text-ly-gray-700">判断基準・判断軸（運用作業）</h3>

    <div>
      <label class="block text-sm font-medium mb-1">作業基準</label>
      <textarea
        value={judgment.workCriteria || ''}
        oninput={(e) => handleFieldChange('workCriteria', e.currentTarget.value)}
        rows="2"
        class="w-full border rounded px-3 py-2 text-sm"
        placeholder="例: バックアップ完了後に実施、変更管理チケット必須"
      ></textarea>
    </div>

    <div>
      <label class="block text-sm font-medium mb-1">実施タイミング</label>
      <textarea
        value={judgment.timing || ''}
        oninput={(e) => handleFieldChange('timing', e.currentTarget.value)}
        rows="2"
        class="w-full border rounded px-3 py-2 text-sm"
        placeholder="例: 毎週日曜日 03:00-05:00、サービス影響を最小化するため深夜帯"
      ></textarea>
    </div>
  </div>
{:else if category === 'troubleshooting'}
  <div class="space-y-4 mt-4 p-4 bg-ly-gray-50 rounded border">
    <h3 class="font-medium text-sm text-ly-gray-700">判断基準・判断軸（トラブルシューティング）</h3>

    <div>
      <label class="block text-sm font-medium mb-1">原因特定プロセス</label>
      <textarea
        value={judgment.rootCauseProcess || ''}
        oninput={(e) => handleFieldChange('rootCauseProcess', e.currentTarget.value)}
        rows="2"
        class="w-full border rounded px-3 py-2 text-sm"
        placeholder="例: ログ確認 → メトリクス分析 → コード変更履歴確認の順で調査"
      ></textarea>
    </div>

    <div>
      <label class="block text-sm font-medium mb-1">調査手順</label>
      <textarea
        value={judgment.investigationSteps || ''}
        oninput={(e) => handleFieldChange('investigationSteps', e.currentTarget.value)}
        rows="2"
        class="w-full border rounded px-3 py-2 text-sm"
        placeholder="例: 1. エラーログの時系列確認、2. 関連サービスの状態確認、3. ネットワーク疎通確認"
      ></textarea>
    </div>
  </div>
{:else if category === 'inquiry'}
  <div class="space-y-4 mt-4 p-4 bg-ly-gray-50 rounded border">
    <h3 class="font-medium text-sm text-ly-gray-700">判断基準・判断軸（問い合わせ）</h3>

    <div>
      <label class="block text-sm font-medium mb-1">対応優先度基準</label>
      <textarea
        value={judgment.priorityCriteria || ''}
        oninput={(e) => handleFieldChange('priorityCriteria', e.currentTarget.value)}
        rows="2"
        class="w-full border rounded px-3 py-2 text-sm"
        placeholder="例: 業務停止につながる場合は即時対応、その他は24時間以内に回答"
      ></textarea>
    </div>

    <div>
      <label class="block text-sm font-medium mb-1">回答指針</label>
      <textarea
        value={judgment.responseGuideline || ''}
        oninput={(e) => handleFieldChange('responseGuideline', e.currentTarget.value)}
        rows="2"
        class="w-full border rounded px-3 py-2 text-sm"
        placeholder="例: 公式ドキュメントURLを含める、再現手順を必ず確認"
      ></textarea>
    </div>
  </div>
{/if}
