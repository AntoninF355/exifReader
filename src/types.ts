export interface PhotoMetadata {
  filename: string;
  iso_speed: number;
  lens: string;
  shutter: number;
  aperture: number;
  focal_length: number;
  make: string;
  model: string;
  datetime?: string | null;
  thumbnail?: string | null;
}

export interface BucketEntry { //ligne d'un histogramme
  label: string,
  count: number,
  percentage: number
}

export interface ApertureCell { //bucketEntry avec un peak pour aperture
  label: string,
  count: number,
  percentage: number,
  is_peak: boolean
}

export interface RankingEntry { //classement camera, lens
  name: string,
  count: number,
  percentage: number
}

export interface PhotoStats {
  total: number,

  //key stats
  most_used_focal_length: number | null,
  median_aperture: number | null,
  median_iso: number | null,
  most_active_hour: number | null,

  //histograms
  focal_length_buckets: BucketEntry[],
  aperture_stops: ApertureCell[],
  iso_buckets: BucketEntry[],
  shutter_buckets: BucketEntry[],

  //Classements
  cameras: RankingEntry[],
  lenses: RankingEntry[],

  shooting_hours: number[]
}

export interface FolderAnalysis {
  photos: PhotoMetadata[],
  stats: PhotoStats
}