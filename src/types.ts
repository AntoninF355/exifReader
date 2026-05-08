export interface PhotoMetadata {
  filename: string;
  iso_speed: number;
  shutter: number;
  aperture: number;
  focal_length: number;
  make: string;
  model: string;
  datetime?: string | null;
}
