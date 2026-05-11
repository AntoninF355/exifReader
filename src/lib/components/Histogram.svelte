<script lang="ts">
  import type { HistoBar } from '../helpers';

  let { title, sub, bars, mode, modeOptions, onmodechange }: {
    title: string;
    sub: string;
    bars: HistoBar[];
    mode: string;
    modeOptions: { value: string; label: string }[];
    onmodechange: (v: string) => void;
  } = $props();
</script>

<div class="section">
  <div class="section-head">
    <div>
      <div class="s-title">{title}</div>
      <div class="s-sub">{sub}</div>
    </div>
    <div class="tog">
      {#each modeOptions as opt}
        <button class="tog-btn" class:on={mode === opt.value} onclick={() => onmodechange(opt.value)}>
          {opt.label}
        </button>
      {/each}
    </div>
  </div>
  <div class="histo">
    <div class="histo-bars">
      {#each bars as bar}
        <div class="bar-wrap">
          <div class="tip"><span>{bar.tip1}</span><span>{bar.tip2}</span></div>
          <div class="bar" class:peak={bar.isPeak} style="height:{bar.h}%"></div>
        </div>
      {/each}
    </div>
    <div class="histo-xlabels">
      {#each bars as bar}
        <div class="histo-xl">{bar.label}</div>
      {/each}
    </div>
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

  .tog {
    display: flex;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
    flex-shrink: 0;
  }
  .tog-btn {
    background: none;
    border: none;
    padding: 5px 11px;
    font-family: var(--f);
    font-size: 12px;
    color: var(--muted);
    cursor: pointer;
    transition: background var(--ease), color var(--ease);
  }
  .tog-btn.on { background: var(--accent); color: #fff; }

  .histo {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--r);
    padding: 20px 20px 0;
  }

  .histo-bars {
    display: flex;
    align-items: flex-end;
    gap: 2px;
    height: 110px;
  }

  .bar-wrap {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 100%;
    justify-content: flex-end;
    position: relative;
    cursor: default;
  }

  .bar {
    width: 100%;
    background: var(--accent);
    border-radius: 2px 2px 0 0;
    opacity: 0.45;
    transition: opacity var(--ease);
    min-height: 2px;
  }
  .bar.peak { opacity: 1; }
  .bar-wrap:hover .bar { opacity: 0.75; }
  .bar-wrap:hover .bar.peak { opacity: 1; }

  .tip {
    position: absolute;
    bottom: calc(100% + 6px);
    left: 50%;
    transform: translateX(-50%);
    background: var(--fg);
    color: var(--bg);
    font-family: var(--fm);
    font-size: 11px;
    padding: 5px 8px;
    border-radius: 4px;
    white-space: nowrap;
    pointer-events: none;
    opacity: 0;
    z-index: 20;
    transition: opacity 0.1s;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1px;
  }
  .bar-wrap:hover .tip { opacity: 1; }

  .histo-xlabels {
    display: flex;
    gap: 2px;
    padding: 7px 0 14px;
  }
  .histo-xl {
    flex: 1;
    font-size: 10px;
    font-family: var(--fm);
    color: var(--muted);
    text-align: center;
    white-space: nowrap;
    overflow: hidden;
  }
</style>
