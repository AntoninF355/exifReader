use std::collections::HashMap;
use crate::models::{PhotoMetadata, PhotoStats, BucketEntry, ApertureCell, RankingEntry};

const ISO_BUCKETS: &[(&str, u32, u32)] = &[
    ("50",     0,    50),
    ("100",    51,   100),
    ("200",    101,  200),
    ("400",    201,  400),
    ("800",    401,  800),
    ("1600",   801,  1600),
    ("3200",   1601, 3200),
    ("6400",   3201, 6400),
    ("12800+", 6401, u32::MAX),
];

const SHUTTER_BUCKETS: &[(&str, f32, f32)] = &[
    ("30s+",   15.0,   f32::MAX),
    ("15s",     7.0,   15.0),
    ("8s",      3.5,    7.0),
    ("4s",      1.75,   3.5),
    ("2s",      0.75,   1.75),
    ("1s",      0.35,   0.75),
    ("1/4",     0.15,   0.35),
    ("1/15",    0.05,   0.15),
    ("1/60",    0.013,  0.05),
    ("1/250",   0.003,  0.013),
    ("1/1000",  0.0007, 0.003),
    ("1/4000",  0.0002, 0.0007),
    ("1/8000",  0.0,    0.0002),
];

const FL_ORIGIN: u32 = 14;
const FL_STEP: u32 = 15;
const FL_COUNT: usize = 26;

const F_STOPS: &[f32]   = &[1.0, 1.4, 2.0, 2.8, 4.0, 5.6, 8.0, 11.0, 16.0, 22.0, 32.0, 45.0];
const F_LABELS: &[&str] = &["ƒ/1", "ƒ/1.4", "ƒ/2", "ƒ/2.8", "ƒ/4", "ƒ/5.6", "ƒ/8", "ƒ/11", "ƒ/16", "ƒ/22", "ƒ/32", "ƒ/45"];

pub fn compute_stats(photos: &[PhotoMetadata]) -> PhotoStats {
    PhotoStats {
        total:                 photos.len(),
        cameras:               compute_cameras(photos),
        lenses:                compute_lenses(photos),
        iso_buckets:           compute_iso_buckets(photos),
        focal_length_buckets:  compute_focal_buckets(photos),
        aperture_stops:        compute_aperture_stops(photos),
        shutter_buckets:       compute_shutter_buckets(photos),
        shooting_hours:        compute_shooting_hours(photos),
        most_used_focal_length: most_used_focal(photos),
        median_iso:            median_iso(photos),
        median_aperture:       median_aperture(photos),
        most_active_hour:      most_active_hour(photos),
    }
}

fn compute_cameras(photos: &[PhotoMetadata]) -> Vec<RankingEntry> {
    let total = photos.len();

    // On compte combien de photos ont été prises avec chaque caméra.
    // La HashMap associe "Sony A1" → 42, "Nikon Z9" → 15, etc.
    // entry().or_insert(0) : si la clé n'existe pas encore, on l'initialise à 0,
    // puis *= déréférence la valeur pour pouvoir faire += 1.
    let mut map: HashMap<String, u32> = HashMap::new();
    for photo in photos {
        let name = format!("{} {}", photo.make, photo.model);
        *map.entry(name).or_insert(0) += 1;
    }

    // On transforme la HashMap en Vec<RankingEntry> avec into_iter().map().collect() :
    // into_iter() consomme la map et produit des tuples (String, u32),
    // map() convertit chaque tuple en RankingEntry avec le pourcentage calculé,
    // collect() matérialise l'itérateur en Vec.
    let mut result: Vec<RankingEntry> = map
        .into_iter()
        .map(|(name, count)| RankingEntry {
            percentage: if total > 0 { count as f32 / total as f32 * 100.0 } else { 0.0 },
            name,
            count,
        })
        .collect();

    // Tri décroissant par count, puis on garde les 5 premiers.
    result.sort_by(|a, b| b.count.cmp(&a.count));
    result.truncate(5);
    result
}

fn compute_iso_buckets(photos: &[PhotoMetadata]) -> Vec<BucketEntry> {
    let total = photos.len();

    // Un compteur par seau, initialisé à 0. L'index correspond à ISO_BUCKETS :
    // counts[0] = nb de photos ISO ≤ 50, counts[1] = ISO 51–100, etc.
    let mut counts = vec![0u32; ISO_BUCKETS.len()];

    // Pour chaque photo, on cherche dans quel seau tombe son ISO avec .position().
    // .position() retourne Option<usize> : l'index du premier seau qui matche, ou None.
    // *min et *max : déréférencement car min/max sont des &u32 dans le closure.
    for photo in photos {
        if let Some(idx) = ISO_BUCKETS.iter().position(|(_, min, max)| {
            photo.iso_speed >= *min && photo.iso_speed <= *max
        }) {
            counts[idx] += 1;
        }
    }

    // .zip() assemble ISO_BUCKETS et counts en paires ((label, min, max), count),
    // puis on convertit chaque paire en BucketEntry.
    // On ignore min et max avec _ car on n'a besoin que du label pour l'affichage.
    ISO_BUCKETS.iter()
        .zip(counts.iter())
        .map(|((label, _, _), &count)| BucketEntry {
            label: label.to_string(),
            count,
            percentage: if total > 0 { count as f32 / total as f32 * 100.0 } else { 0.0 },
        })
        .collect()
}

fn compute_lenses(photos: &[PhotoMetadata]) -> Vec<RankingEntry> {
    let total = photos.len();

    let mut map: HashMap<String, u32> = HashMap::new();
    for photo in photos {
        if photo.lens.is_empty() { continue; }  // ← skip si pas de donnée
        let name = photo.lens.clone();
        *map.entry(name).or_insert(0) += 1;
    }

    let mut result: Vec<RankingEntry> = map
        .into_iter()
        .map(|(name, count)| RankingEntry {
            percentage: if total > 0 { count as f32 / total as f32 * 100.0 } else { 0.0 },
            name,
            count,
        })
        .collect();

    // Tri décroissant par count, puis on garde les 5 premiers.
    result.sort_by(|a, b| b.count.cmp(&a.count));
    result.truncate(5);
    result
}

fn compute_focal_buckets(photos: &[PhotoMetadata]) -> Vec<BucketEntry> {
    let total = photos.len();
      let mut counts = [0u32; FL_COUNT];

      for photo in photos {
          let fl = photo.focal_length.round() as u32;
          let idx = if fl < FL_ORIGIN {
              0
          } else {
              ((fl - FL_ORIGIN) / FL_STEP).min(FL_COUNT as u32 - 1) as usize
          };
          counts[idx] += 1;
      }

      counts.iter().enumerate().map(|(i, &count)| {
          let start = FL_ORIGIN + i as u32 * FL_STEP;
          BucketEntry {
              label: format!("{}mm", start),
              count,
              percentage: if total > 0 { count as f32 / total as f32 * 100.0 } else { 0.0 },
          }
      }).collect()
}

fn compute_aperture_stops(photos: &[PhotoMetadata]) -> Vec<ApertureCell> {
    let total = photos.len();
    let mut counts = [0u32; 12];

    for photo in photos {
        if photo.aperture <= 0.0 { continue; }
        let idx = F_STOPS.iter().enumerate()
            .min_by(|(_, &a), (_, &b)| {
                (photo.aperture - a).abs()
                    .partial_cmp(&(photo.aperture - b).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0);
        counts[idx] += 1;
    }

    let peak = counts.iter().copied().max().unwrap_or(0);

    F_LABELS.iter().zip(counts.iter()).map(|(&label, &count)| {
        ApertureCell {
            label: label.to_string(),
            count,
            percentage: if total > 0 { count as f32 / total as f32 * 100.0 } else { 0.0 },
            is_peak: peak > 0 && count == peak,
        }
    }).collect()
}

fn compute_shutter_buckets(photos: &[PhotoMetadata]) -> Vec<BucketEntry> {
    let total = photos.len();

    let mut counts = vec![0u32; SHUTTER_BUCKETS.len()];

    for photo in photos {
        if let Some(idx) = SHUTTER_BUCKETS.iter().position(|(_, min, max)| {
            photo.shutter >= *min && photo.shutter <= *max
        }) {
            counts[idx] += 1;
        }
    }

    SHUTTER_BUCKETS.iter()
        .zip(counts.iter())
        .map(|((label, _, _), &count)| BucketEntry {
            label: label.to_string(),
            count,
            percentage: if total > 0 { count as f32 / total as f32 * 100.0 } else { 0.0 },
        })
        .collect()
}

fn compute_shooting_hours(photos: &[PhotoMetadata]) -> Vec<u32> {
    let mut hours = vec![0u32; 24];
    for photo in photos {
    if let Some(dt) = &photo.datetime {
        // Coupe "2024:03:15 14:32:07" → ["2024:03:15", "14:32:07"]
        if let Some(time_part) = dt.splitn(2, ' ').nth(1) {
            // Prend "14" depuis "14:32:07" et parse en u8
            if let Ok(hour) = time_part[..2].parse::<u8>() {
                if (hour as usize) < 24 {
                    hours[hour as usize] += 1;
                }
            }
        }
    }
    }
    hours
}

fn most_used_focal(photos: &[PhotoMetadata]) -> Option<u32> {
      let buckets = compute_focal_buckets(photos);
      buckets.iter()
          .enumerate()
          .max_by_key(|(_, b)| b.count)
          .filter(|(_, b)| b.count > 0)
          .map(|(i, _)| FL_ORIGIN + i as u32 * FL_STEP)
  }

fn median_iso(photos: &[PhotoMetadata]) -> Option<u32> {
    let mut values: Vec<u32> = photos.iter().map(|p| p.iso_speed).collect();
    values.sort_unstable();
    values.get(values.len() / 2).copied()
}

fn median_aperture(photos: &[PhotoMetadata]) -> Option<f32> {
    let mut values: Vec<f32> = photos.iter().map(|p|p.aperture).collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    values.get(values.len() / 2).copied()
}

fn most_active_hour(photos: &[PhotoMetadata]) -> Option<u8> {
    let hours = compute_shooting_hours(photos);
      hours.iter()
          .enumerate()
          .max_by_key(|(_, &count)| count)
          .filter(|(_, &count)| count > 0)  // None si aucune photo avec date
          .map(|(hour, _)| hour as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PhotoMetadata;

    fn mock_photo(iso: u32, make: &str, model: &str, datetime: Option<&str>) -> PhotoMetadata {
        PhotoMetadata {
            filename: "test.arw".to_string(),
            iso_speed: iso,
            shutter: 0.004,
            aperture: 2.8,
            focal_length: 85.0,
            make: make.to_string(),
            model: model.to_string(),
            lens: "".to_string(),
            datetime: datetime.map(|s| s.to_string()),
            thumbnail: None,
        }
    }

    #[test]
    fn test_camera_ranking() {
        let photos = vec![
            mock_photo(400, "Sony", "A1", None),
            mock_photo(800, "Sony", "A1", None),
            mock_photo(200, "Nikon", "Z9", None),
        ];
        let stats = compute_stats(&photos);
        assert_eq!(stats.cameras[0].name, "Sony A1");
        assert_eq!(stats.cameras[0].count, 2);
    }

    #[test]
    fn test_iso_buckets() {
        let photos = vec![
            mock_photo(100, "Sony", "A1", None),
            mock_photo(400, "Sony", "A1", None),
            mock_photo(400, "Sony", "A1", None),
        ];
        let stats = compute_stats(&photos);
        let iso400 = stats.iso_buckets.iter().find(|b| b.label == "400").unwrap();
        assert_eq!(iso400.count, 2);
    }

    #[test]
    fn test_median_iso() {
        let photos = vec![
            mock_photo(100, "Sony", "A1", None),
            mock_photo(400, "Sony", "A1", None),
            mock_photo(800, "Sony", "A1", None),
        ];
        let stats = compute_stats(&photos);
        assert_eq!(stats.median_iso, Some(400));
    }

    #[test]
    fn test_median_aperture() {
        let photos = vec![
            mock_photo(100, "Sony", "A1", None), // aperture = 2.8 pour tous
            mock_photo(100, "Sony", "A1", None),
            mock_photo(100, "Sony", "A1", None),
        ];
        let stats = compute_stats(&photos);
        assert_eq!(stats.median_aperture, Some(2.8));
    }

    #[test]
    fn test_lenses_ranking() {
        let mut p1 = mock_photo(400, "Sony", "A1", None);
        p1.lens = "FE 85mm F1.4".to_string();
        let mut p2 = mock_photo(400, "Sony", "A1", None);
        p2.lens = "FE 85mm F1.4".to_string();
        let mut p3 = mock_photo(400, "Sony", "A1", None);
        p3.lens = "FE 24mm F1.4".to_string();
        let stats = compute_stats(&[p1, p2, p3]);
        assert_eq!(stats.lenses[0].name, "FE 85mm F1.4");
        assert_eq!(stats.lenses[0].count, 2);
    }

    #[test]
    fn test_shooting_hours() {
        let photos = vec![
            mock_photo(400, "Sony", "A1", Some("2024:03:15 14:32:07")),
            mock_photo(400, "Sony", "A1", Some("2024:03:15 14:45:00")),
            mock_photo(400, "Sony", "A1", Some("2024:03:15 09:10:00")),
        ];
        let stats = compute_stats(&photos);
        assert_eq!(stats.shooting_hours[14], 2);
        assert_eq!(stats.shooting_hours[9], 1);
    }

    #[test]
    fn test_most_active_hour() {
        let photos = vec![
            mock_photo(400, "Sony", "A1", Some("2024:03:15 14:32:07")),
            mock_photo(400, "Sony", "A1", Some("2024:03:15 14:45:00")),
            mock_photo(400, "Sony", "A1", Some("2024:03:15 09:10:00")),
        ];
        let stats = compute_stats(&photos);
        assert_eq!(stats.most_active_hour, Some(14));
    }

    #[test]
    fn test_no_datetime() {
        let photos = vec![mock_photo(400, "Sony", "A1", None)];
        let stats = compute_stats(&photos);
        assert_eq!(stats.most_active_hour, None);
        assert!(stats.shooting_hours.iter().all(|&h| h == 0));
    }
}
