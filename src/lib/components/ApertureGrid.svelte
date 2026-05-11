<script lang="ts">
  import type { ApertureCellExt } from '../helpers';
  let { stops }: { stops: ApertureCellExt[] } = $props();
</script>

<div class="section">
  <div class="section-head">
    <div>
      <div class="s-title">Aperture</div>
      <div class="s-sub">Standard f-stops · peak highlighted</div>
    </div>
  </div>
  <div class="ap-grid">
    {#each stops as s}
      <div class="ap-cell" class:peak={s.is_peak}>
        <div class="ap-stop">{s.label}</div>
        <div class="ap-pct">{s.percentage.toFixed(1)}%</div>
        <div class="ap-count">{s.count}</div>
        <div class="ap-bar" style="width:{s.bw}%"></div>
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

  .ap-grid {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 10px;
  }

  .ap-cell {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--r);
    padding: 14px 12px 10px;
    position: relative;
    overflow: hidden;
    transition: border-color var(--ease);
  }
  .ap-cell.peak { border-color: var(--accent); }

  .ap-stop {
    font-family: var(--fd);
    font-size: 14px;
    font-weight: 600;
    letter-spacing: -0.01em;
    margin-bottom: 5px;
  }
  .ap-pct {
    font-size: 13px;
    font-family: var(--fm);
    font-variant-numeric: tabular-nums;
    color: var(--accent);
    margin-bottom: 2px;
  }
  .ap-count {
    font-size: 11px;
    font-family: var(--fm);
    font-variant-numeric: tabular-nums;
    color: var(--muted);
  }
  .ap-bar {
    position: absolute;
    bottom: 0; left: 0;
    height: 3px;
    background: var(--accent);
    opacity: 0.4;
  }
  .ap-cell.peak .ap-bar { opacity: 1; }
</style>
