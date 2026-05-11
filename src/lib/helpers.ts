import type { BucketEntry, ApertureCell, RankingEntry } from '../types';

export type HistoBar = {
  label: string;
  h: number;
  isPeak: boolean;
  tip1: string;
  tip2: string;
};

export type ApertureCellExt = ApertureCell & { bw: string };
export type RankingEntryExt = RankingEntry & { bw: string };

export function histoBars(buckets: BucketEntry[], mode: 'count' | 'pct'): HistoBar[] {
  if (!buckets.length) return [];
  const vals = buckets.map(b => mode === 'pct' ? b.percentage : b.count);
  const maxV = Math.max(...vals, 1);
  const peakV = Math.max(...vals);
  return buckets.map((b, i) => ({
    label: b.label,
    h: vals[i] > 0 ? Math.max((vals[i] / maxV) * 100, 2) : 0,
    isPeak: vals[i] === peakV && vals[i] > 0,
    tip1: b.label,
    tip2: mode === 'pct' ? b.percentage.toFixed(1) + '%' : b.count + ' photos',
  }));
}

export function shutterBars(buckets: BucketEntry[], mode: 'linear' | 'log'): HistoBar[] {
  if (!buckets.length) return [];
  const vals = mode === 'log'
    ? buckets.map(b => b.count > 0 ? Math.log2(b.count + 1) : 0)
    : buckets.map(b => b.count);
  const maxV = Math.max(...vals, 1);
  const peakC = Math.max(...buckets.map(b => b.count));
  return buckets.map((b, i) => ({
    label: b.label,
    h: vals[i] > 0 ? Math.max((vals[i] / maxV) * 100, 2) : 0,
    isPeak: b.count === peakC && b.count > 0,
    tip1: b.label,
    tip2: b.count + ' · ' + b.percentage.toFixed(1) + '%',
  }));
}

export function apertureBars(stops: ApertureCell[]): ApertureCellExt[] {
  if (!stops.length) return [];
  const maxC = Math.max(...stops.map(s => s.count), 1);
  return stops.map(s => ({ ...s, bw: s.count > 0 ? (s.count / maxC * 100).toFixed(1) : '0' }));
}

export function rankBars(entries: RankingEntry[]): RankingEntryExt[] {
  if (!entries.length) return [];
  const maxPct = entries[0]?.percentage || 100;
  return entries.map(e => ({ ...e, bw: (e.percentage / maxPct * 100).toFixed(1) }));
}
