<script lang="ts">
  let { count }: { count: number } = $props();
</script>

<div class="spinner-screen">
  <div class="spinner"></div>
  <div class="spinner-label">Analysing <span class="spinner-n">{count}</span> photos…</div>
</div>

<style>
  .spinner-screen {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 20px;
  }

  .spinner {
    width: 32px; height: 32px;
    border: 2px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.75s linear infinite;
  }
  @keyframes spin { to { transform: rotate(360deg); } }

  .spinner-label { font-size: 14px; color: var(--muted); }
  .spinner-n { font-family: var(--fm); color: var(--fg); font-variant-numeric: tabular-nums; }
</style>
