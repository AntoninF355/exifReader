<script lang="ts">
  let { open, title, sub, rows, onclose }: {
    open: boolean;
    title: string;
    sub: string;
    rows: [string, string][];
    onclose: () => void;
  } = $props();
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="overlay" onclick={(e) => { if (e.target === e.currentTarget) onclose(); }}>
    <div class="modal">
      <div class="modal-title">{title}</div>
      <div class="modal-sub">{sub}</div>
      {#each rows as [label, val]}
        <div class="modal-row">
          <span class="modal-row-label">{label}</span>
          <span class="modal-row-val">{val}</span>
        </div>
      {/each}
      <button class="btn-ghost" onclick={onclose}>Close</button>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: oklch(0% 0 0 / 0.45);
    z-index: 200;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
  }

  .modal {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 26px 26px 22px;
    max-width: 420px;
    width: 100%;
  }

  .modal-title { font-size: 15px; font-weight: 600; letter-spacing: -0.01em; margin-bottom: 3px; }
  .modal-sub { font-size: 12px; color: var(--muted); margin-bottom: 18px; }

  .modal-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 9px 0;
    border-bottom: 1px solid var(--border);
    font-size: 13px;
  }
  .modal-row:last-of-type { border-bottom: none; }
  .modal-row-label { color: var(--muted); }
  .modal-row-val { font-family: var(--fm); font-variant-numeric: tabular-nums; }

  .btn-ghost {
    margin-top: 18px;
    width: 100%;
    padding: 9px;
    border: 1px solid var(--border);
    background: none;
    border-radius: var(--r);
    font-family: var(--f);
    font-size: 13px;
    color: var(--fg);
    cursor: pointer;
    transition: background var(--ease);
  }
  .btn-ghost:hover { background: var(--bg); }
</style>
