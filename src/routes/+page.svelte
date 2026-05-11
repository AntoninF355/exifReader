<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import type { PhotoStats, FolderAnalysis } from '../types';
  import { histoBars, shutterBars, apertureBars, rankBars } from '../lib/helpers';
  import type { RankingEntryExt } from '../lib/helpers';

  import UploadScreen  from '../lib/components/UploadScreen.svelte';
  import SpinnerScreen from '../lib/components/SpinnerScreen.svelte';
  import StatsHeader   from '../lib/components/StatsHeader.svelte';
  import KpiGrid       from '../lib/components/KpiGrid.svelte';
  import Histogram     from '../lib/components/Histogram.svelte';
  import ApertureGrid  from '../lib/components/ApertureGrid.svelte';
  import RankingList   from '../lib/components/RankingList.svelte';
  import Modal         from '../lib/components/Modal.svelte';

  // ── App state ────────────────────────────────────────────────────────────────
  let stats:      PhotoStats | null = $state(null);
  let loading:    boolean           = $state(false);
  let folderPath: string            = $state('');
  let error:      string | null     = $state(null);
  let theme:      'light' | 'dark' = $state('light');
  let spinCount:  number            = $state(0);

  let flMode:  'count' | 'pct'  = $state('count');
  let isoMode: 'count' | 'pct'  = $state('count');
  let shMode:  'linear' | 'log' = $state('linear');

  let modalOpen:  boolean            = $state(false);
  let modalTitle: string             = $state('');
  let modalSub:   string             = $state('');
  let modalRows:  [string, string][] = $state([]);

  // ── Derived bar data ─────────────────────────────────────────────────────────
  const flBars     = $derived(histoBars(stats?.focal_length_buckets ?? [], flMode));
  const isoBars    = $derived(histoBars(stats?.iso_buckets ?? [], isoMode));
  const shBars     = $derived(shutterBars(stats?.shutter_buckets ?? [], shMode));
  const apStops    = $derived(apertureBars(stats?.aperture_stops ?? []));
  const camEntries = $derived(rankBars(stats?.cameras ?? []));
  const lensEntries = $derived(rankBars(stats?.lenses ?? []));

  // ── Theme sync ───────────────────────────────────────────────────────────────
  $effect(() => {
    document.documentElement.setAttribute('data-theme', theme);
  });

  // ── Folder selection + analysis ───────────────────────────────────────────────
  async function selectFolder() {
    error = null;
    const selected = await open({ directory: true, multiple: false });
    if (!selected || typeof selected !== 'string') return;
    folderPath = selected;
    await runAnalysis();
  }

  async function runAnalysis() {
    loading   = true;
    spinCount = 0;

    const tick = setInterval(() => {
      spinCount = Math.min(spinCount + Math.ceil(500 / 35) + Math.floor(Math.random() * 8), 99999);
    }, 35);

    try {
      const result: FolderAnalysis = await invoke('analyze_folder', { folderPath });
      clearInterval(tick);
      spinCount = result.stats.total;
      await new Promise(r => setTimeout(r, 350));
      stats = result.stats;
    } catch (e) {
      clearInterval(tick);
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function goBack() {
    stats = null;
    error = null;
  }

  function toggleTheme() {
    theme = theme === 'light' ? 'dark' : 'light';
  }

  // ── Modal ─────────────────────────────────────────────────────────────────────
  function openModal(entry: RankingEntryExt, type: 'camera' | 'lens') {
    modalTitle = entry.name;
    modalSub   = `${type === 'camera' ? 'Camera' : 'Lens'} · ${entry.count.toLocaleString()} photos`;
    modalRows  = type === 'camera'
      ? [
          ['Photos',          entry.count.toLocaleString()],
          ['Share',           entry.percentage.toFixed(1) + '%'],
          ['Most-used ISO',   stats?.median_iso != null ? 'ISO ' + stats.median_iso : '—'],
          ['Median aperture', stats?.median_aperture != null ? 'ƒ/' + stats.median_aperture : '—'],
          ['Top focal',       stats?.most_used_focal_length != null ? stats.most_used_focal_length + ' mm' : '—'],
          ['Peak hour',       stats?.most_active_hour != null ? stats.most_active_hour + ' h' : '—'],
        ]
      : [
          ['Photos', entry.count.toLocaleString()],
          ['Share',  entry.percentage.toFixed(1) + '%'],
          ['Type',   entry.name.match(/\d+-\d+/) ? 'Zoom lens' : 'Prime lens'],
        ];
    modalOpen = true;
  }
</script>

<!-- ── Upload ──────────────────────────────────────────────────────────────── -->
{#if !loading && !stats}
  <UploadScreen {error} onselect={selectFolder} />
{/if}

<!-- ── Spinner ─────────────────────────────────────────────────────────────── -->
{#if loading}
  <SpinnerScreen count={spinCount} />
{/if}

<!-- ── Stats ───────────────────────────────────────────────────────────────── -->
{#if stats}
  <StatsHeader
    {folderPath}
    total={stats.total}
    {theme}
    onback={goBack}
    onthemetoggle={toggleTheme}
  />

  <div class="s-content">
    <KpiGrid {stats} />

    <Histogram
      title="Focal length"
      sub="15 mm buckets · 14 mm → 400 mm"
      bars={flBars}
      mode={flMode}
      modeOptions={[{ value: 'count', label: 'Count' }, { value: 'pct', label: '%' }]}
      onmodechange={(v) => (flMode = v as 'count' | 'pct')}
    />

    <ApertureGrid stops={apStops} />

    <Histogram
      title="ISO sensitivity"
      sub="9 buckets · 50 → 12800+"
      bars={isoBars}
      mode={isoMode}
      modeOptions={[{ value: 'count', label: 'Count' }, { value: 'pct', label: '%' }]}
      onmodechange={(v) => (isoMode = v as 'count' | 'pct')}
    />

    <Histogram
      title="Shutter speed"
      sub="13 buckets · 30 s → 1/8000"
      bars={shBars}
      mode={shMode}
      modeOptions={[{ value: 'linear', label: 'Linear' }, { value: 'log', label: 'Log' }]}
      onmodechange={(v) => (shMode = v as 'linear' | 'log')}
    />

    <RankingList
      title="Cameras"
      sub="Top 5 · click for detail"
      entries={camEntries}
      onrowclick={(e) => openModal(e, 'camera')}
    />

    <RankingList
      title="Lenses"
      sub="Top 5 · click for detail"
      entries={lensEntries}
      onrowclick={(e) => openModal(e, 'lens')}
    />
  </div>

  <Modal
    open={modalOpen}
    title={modalTitle}
    sub={modalSub}
    rows={modalRows}
    onclose={() => (modalOpen = false)}
  />
{/if}

<style>
  :global(*, *::before, *::after) { box-sizing: border-box; margin: 0; padding: 0; }

  :global(:root) {
    --bg:      oklch(99% 0.002 240);
    --surface: oklch(100% 0 0);
    --fg:      oklch(18% 0.012 250);
    --muted:   oklch(54% 0.012 250);
    --border:  oklch(92% 0.005 250);
    --accent:  oklch(58% 0.18 255);
    --accent-a:oklch(58% 0.18 255 / 0.12);
    --f:  -apple-system, BlinkMacSystemFont, 'SF Pro Text', 'Inter', system-ui, sans-serif;
    --fd: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Inter', system-ui, sans-serif;
    --fm: ui-monospace, 'JetBrains Mono', 'SF Mono', Menlo, monospace;
    --r:    8px;
    --ease: 0.15s ease;
  }

  :global([data-theme="dark"]) {
    --bg:      oklch(13% 0.01 250);
    --surface: oklch(18% 0.012 250);
    --fg:      oklch(96% 0.005 250);
    --muted:   oklch(58% 0.012 250);
    --border:  oklch(28% 0.012 250);
    --accent:  oklch(65% 0.18 255);
    --accent-a:oklch(65% 0.18 255 / 0.15);
  }

  :global(html, body) {
    height: 100%;
    background: var(--bg);
    color: var(--fg);
    font-family: var(--f);
    font-size: 14px;
    line-height: 1.5;
    -webkit-font-smoothing: antialiased;
    transition: background var(--ease), color var(--ease);
  }

  .s-content {
    max-width: 1080px;
    margin: 0 auto;
    padding: 36px 28px 80px;
  }
</style>
