use std::collections::HashMap;
use crate::models::{PhotoMetadata, PhotoStats, BucketEntry, ApertureCell, RankingEntry};

//tuples range iso (label, min, max)
const ISO_BUCKETS: &[(&str, u32, u32)] = &[
    ("50", 0, 50),
    ("100", 51, 100),
    ("200", 101, 200),
    ("400", 201, 400),
    ("800", 401, 800),
    ("1600", 801, 1600),
    ("3200", 1601, 3200),
    ("6400", 3201, 6400),
    ("12800+", 6401, u32::MAX),
];

pub fn compute_stats(photos: &[PhotoMetadata]) -> PhotoStats {
    let total = photos.len();

    //Compte les photos par camera
    let mut camera_map: HashMap<String, u32> = HashMap::new();
    for photo in photos {
        let name = format!("{} {}", photo.make, photo.model);
        *camera_map.entry(name).or_insert(0) += 1;
    }

    let mut cameras: Vec<RankingEntry> = camera_map
      .into_iter()
      .map(|(name, count)| RankingEntry {
          percentage: if total > 0 { count as f32 / total as f32 * 100.0 } else { 0.0 },
          name,
          count,
      })
      .collect();
    cameras.sort_by(|a, b| b.count.cmp(&a.count));
    cameras.truncate(5);


    // 1. Focales
    // 2. ISO
    // 3. Ouvertures
    // 4. Vitesses
    // 5. Caméras
    // 6. Objectifs
    // 7. Heures

    PhotoStats {
      total,
      cameras,
      // tout le reste en placeholder pour l'instant
      most_used_focal_length: None,
      median_aperture: None,
      median_iso: None,
      most_active_hour: None,
      focal_length_buckets: vec![],
      aperture_stops: vec![],
      iso_buckets: vec![],
      shutter_buckets: vec![],
      lenses: vec![],
      shooting_hours: vec![],
    }

}