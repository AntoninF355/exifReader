<script lang="ts">
  let { folderPath, total, theme, onback, onthemetoggle }: {
    folderPath: string;
    total: number;
    theme: 'light' | 'dark';
    onback: () => void;
    onthemetoggle: () => void;
  } = $props();
</script>

<header class="s-header">
  <button class="btn-back" onclick={onback}>
    <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"><path d="M10 3L5 8l5 5"/></svg>
    Back
  </button>
  <div class="h-sep"></div>
  <span class="h-folder">{folderPath}</span>
  <span class="h-count">{total.toLocaleString()} photos</span>
  <div class="h-right">
    <button class="btn-icon" title="Toggle dark mode" onclick={onthemetoggle}>
      {#if theme === 'light'}
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
          <circle cx="8" cy="8" r="3.5"/>
          <path d="M8 1v2M8 13v2M1 8h2M13 8h2M3.05 3.05l1.42 1.42M11.54 11.54l1.41 1.41M3.05 12.95l1.42-1.41M11.54 4.46l1.41-1.41"/>
        </svg>
      {:else}
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
          <path d="M12.5 8A5.5 5.5 0 1 1 5 2.8 6.5 6.5 0 1 0 12.5 8z" fill="currentColor" stroke="none"/>
        </svg>
      {/if}
    </button>
  </div>
</header>

<style>
  .s-header {
    position: sticky;
    top: 0;
    z-index: 100;
    height: 52px;
    padding: 0 28px;
    display: flex;
    align-items: center;
    gap: 14px;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    backdrop-filter: blur(12px);
  }

  .btn-back {
    display: flex;
    align-items: center;
    gap: 5px;
    background: none;
    border: none;
    color: var(--muted);
    font-size: 13px;
    font-family: var(--f);
    cursor: pointer;
    padding: 4px 0;
    transition: color var(--ease);
  }
  .btn-back:hover { color: var(--fg); }

  .h-sep { width: 1px; height: 18px; background: var(--border); }

  .h-folder {
    font-size: 13px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 360px;
  }

  .h-count {
    font-size: 12px;
    color: var(--muted);
    font-family: var(--fm);
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
  }

  .h-right { margin-left: auto; display: flex; align-items: center; gap: 8px; }

  .btn-icon {
    background: none;
    border: 1px solid var(--border);
    border-radius: 6px;
    width: 30px; height: 30px;
    display: flex; align-items: center; justify-content: center;
    cursor: pointer;
    color: var(--muted);
    transition: color var(--ease), border-color var(--ease);
  }
  .btn-icon:hover { color: var(--fg); border-color: var(--muted); }
</style>
