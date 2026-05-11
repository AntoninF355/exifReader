<script lang="ts">
  import type { RankingEntryExt } from '../helpers';

  let { title, sub, entries, onrowclick }: {
    title: string;
    sub: string;
    entries: RankingEntryExt[];
    onrowclick: (entry: RankingEntryExt) => void;
  } = $props();
</script>

<div class="section">
  <div class="section-head">
    <div>
      <div class="s-title">{title}</div>
      <div class="s-sub">{sub}</div>
    </div>
  </div>
  <div class="ranking">
    {#each entries as e, i}
      <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
      <div class="rank-row" onclick={() => onrowclick(e)}>
        <span class="rank-pos">{i + 1}</span>
        <span class="rank-name">{e.name}</span>
        <div class="rank-prog"><div class="rank-track"><div class="rank-fill" style="width:{e.bw}%"></div></div></div>
        <span class="rank-pct">{e.percentage.toFixed(1)}%</span>
        <span class="rank-n">{e.count}</span>
        <svg class="rank-chev" width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"><path d="M6 4l4 4-4 4"/></svg>
      </div>
    {/each}
  </div>
</div>

<style>
  .section { margin-bottom: 44px; }

  .section-head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .s-title { font-size: 14px; font-weight: 600; letter-spacing: -0.01em; }
  .s-sub { font-size: 12px; color: var(--muted); margin-top: 2px; }

  .ranking {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--r);
    overflow: hidden;
  }

  .rank-row {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 13px 18px;
    border-bottom: 1px solid var(--border);
    cursor: pointer;
    transition: background var(--ease);
  }
  .rank-row:last-child { border-bottom: none; }
  .rank-row:hover { background: var(--accent-a); }

  .rank-pos {
    font-family: var(--fm);
    font-size: 12px;
    color: var(--muted);
    font-variant-numeric: tabular-nums;
    width: 18px;
    text-align: right;
    flex-shrink: 0;
  }

  .rank-name {
    flex: 1;
    min-width: 0;
    font-size: 13px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .rank-prog { width: 140px; flex-shrink: 0; }
  .rank-track { height: 3px; background: var(--border); border-radius: 2px; overflow: hidden; }
  .rank-fill { height: 100%; background: var(--accent); opacity: 0.65; border-radius: 2px; }

  .rank-pct {
    font-family: var(--fm);
    font-size: 12px;
    color: var(--muted);
    font-variant-numeric: tabular-nums;
    width: 44px;
    text-align: right;
    flex-shrink: 0;
  }
  .rank-n {
    font-family: var(--fm);
    font-size: 12px;
    color: var(--muted);
    font-variant-numeric: tabular-nums;
    width: 40px;
    text-align: right;
    flex-shrink: 0;
  }
  .rank-chev { color: var(--border); flex-shrink: 0; transition: color var(--ease); }
  .rank-row:hover .rank-chev { color: var(--muted); }
</style>
