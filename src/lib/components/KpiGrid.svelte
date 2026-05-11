<script lang="ts">
  import type { PhotoStats } from '../../types';
  let { stats }: { stats: PhotoStats } = $props();
</script>

<div class="kpi-grid">
  <div class="kpi-card">
    <div class="kpi-label">Focal length</div>
    <div class="kpi-val">{stats.most_used_focal_length != null ? stats.most_used_focal_length + ' mm' : '—'}</div>
    <div class="kpi-sub">Most-used focal</div>
  </div>
  <div class="kpi-card">
    <div class="kpi-label">Aperture</div>
    <div class="kpi-val">{stats.median_aperture != null ? 'ƒ/' + stats.median_aperture : '—'}</div>
    <div class="kpi-sub">Median aperture</div>
  </div>
  <div class="kpi-card">
    <div class="kpi-label">ISO</div>
    <div class="kpi-val">{stats.median_iso != null ? stats.median_iso : '—'}</div>
    <div class="kpi-sub">Median ISO</div>
  </div>
  <div class="kpi-card">
    <div class="kpi-label">Peak hour</div>
    <div class="kpi-val">{stats.most_active_hour != null ? stats.most_active_hour + ' h' : '—'}</div>
    <div class="kpi-sub">Most active hour</div>
  </div>
</div>

<style>
  .kpi-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 14px;
    margin-bottom: 44px;
  }

  .kpi-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--r);
    padding: 18px 20px 16px;
  }

  .kpi-label {
    font-size: 11px;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--muted);
    font-family: var(--fm);
    margin-bottom: 8px;
  }

  .kpi-val {
    font-family: var(--fd);
    font-size: 30px;
    font-weight: 600;
    letter-spacing: -0.02em;
    line-height: 1.1;
    font-variant-numeric: tabular-nums;
  }

  .kpi-sub { font-size: 12px; color: var(--muted); margin-top: 5px; }
</style>
