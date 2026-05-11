<script lang="ts">
  let { error, onselect }: { error: string | null; onselect: () => void } = $props();
</script>

<div class="upload-screen">
  <span class="wordmark">Frames</span>
  <div class="drop-zone">
    <svg class="drop-icon" viewBox="0 0 44 44" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
      <path d="M4 9a2 2 0 0 1 2-2h10l4 4h18a2 2 0 0 1 2 2v22a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V9z"/>
      <path d="M22 19v8M18 23l4-4 4 4"/>
    </svg>
    <div class="drop-title">Select a folder to analyse</div>
    <div class="drop-sub">Supports JPEG, PNG, ARW, CR3, NEF, DNG</div>
    <button class="btn-primary" onclick={onselect}>
      <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"><path d="M2 12a1 1 0 0 0 1 1h10a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1h-2.5L8.5 4h-3L3 6H3a1 1 0 0 0-1 1v5z"/></svg>
      Select folder
    </button>
    {#if error}
      <div class="drop-error">{error}</div>
    {/if}
  </div>
</div>

<style>
  .upload-screen {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px 24px;
  }

  .wordmark {
    font-family: var(--fd);
    font-size: 20px;
    font-weight: 600;
    letter-spacing: -0.02em;
    margin-bottom: 56px;
  }

  .drop-zone {
    width: 100%;
    max-width: 500px;
    border: 1.5px dashed var(--border);
    border-radius: 12px;
    padding: 52px 40px 44px;
    text-align: center;
    background: var(--surface);
    transition: border-color var(--ease), background var(--ease);
  }
  .drop-zone:hover { border-color: var(--accent); background: var(--accent-a); }

  .drop-icon {
    width: 44px; height: 44px;
    margin: 0 auto 18px;
    color: var(--muted);
    display: block;
  }

  .drop-title {
    font-family: var(--fd);
    font-size: 17px;
    font-weight: 600;
    letter-spacing: -0.01em;
    margin-bottom: 6px;
  }

  .drop-sub {
    font-size: 13px;
    color: var(--muted);
    margin-bottom: 24px;
  }

  .drop-error {
    margin-top: 14px;
    font-size: 12px;
    color: oklch(55% 0.18 25);
    font-family: var(--fm);
  }

  .btn-primary {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    background: var(--accent);
    color: #fff;
    border: none;
    border-radius: var(--r);
    padding: 9px 18px;
    font-family: var(--f);
    font-size: 14px;
    font-weight: 500;
    letter-spacing: 0.01em;
    cursor: pointer;
    transition: opacity var(--ease);
  }
  .btn-primary:hover { opacity: 0.86; }
</style>
