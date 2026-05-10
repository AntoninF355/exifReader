use std::collections::HashMap;
use crate::models::{PhotoMetadata, PhotoStats, BucketEntry, ApertureCell, RankingEntry};

//tuples range iso (label, min, max)
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

pub fn compute_stats(photos: &[PhotoMetadata]) -> PhotoStats {
    let total = photos.len();

    // Compte les photos par camera
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

    // ISO
    let mut iso_counts = vec![0u32; ISO_BUCKETS.len()];

    for photo in photos {
        if let Some(idx) = ISO_BUCKETS.iter().position(|(_, min, max)| {
            photo.iso_speed >= *min && photo.iso_speed <= *max
        }) {
            iso_counts[idx] += 1;
        }
    }

    let iso_buckets: Vec<BucketEntry> = ISO_BUCKETS.iter()
        .zip(iso_counts.iter())
        .map(|((label, _, _), &count)| BucketEntry {
            label: label.to_string(),
            count,
            percentage: if total > 0 { count as f32 / total as f32 * 100.0 } else { 0.0 },
        })
        .collect();

    // 1. Focales
    // 3. Ouvertures
    // 4. Vitesses
    // 6. Objectifs
    // 7. Heures

    PhotoStats {
        total,
        cameras,
        iso_buckets,
        // placeholders
        most_used_focal_length: None,
        median_aperture: None,
        median_iso: None,
        most_active_hour: None,
        focal_length_buckets: vec![],
        aperture_stops: vec![],
        shutter_buckets: vec![],
        lenses: vec![],
        shooting_hours: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PhotoMetadata;

    fn mock_photo(iso: u32, make: &str, model: &str) -> PhotoMetadata {
        PhotoMetadata {
            filename: "test.arw".to_string(),
            iso_speed: iso,
            shutter: 0.004,
            aperture: 2.8,
            focal_length: 85.0,
            make: make.to_string(),
            model: model.to_string(),
            lens: "".to_string(),
            datetime: None,
            thumbnail: None,
        }
    }

    #[test]
    fn test_camera_ranking() {
        let photos = vec![
            mock_photo(400, "Sony", "A1"),
            mock_photo(800, "Sony", "A1"),
            mock_photo(200, "Nikon", "Z9"),
        ];
        let stats = compute_stats(&photos);
        assert_eq!(stats.cameras[0].name, "Sony A1");
        assert_eq!(stats.cameras[0].count, 2);
    }

    #[test]
    fn test_iso_buckets() {
        let photos = vec![
            mock_photo(100, "Sony", "A1"),
            mock_photo(400, "Sony", "A1"),
            mock_photo(400, "Sony", "A1"),
        ];
        let stats = compute_stats(&photos);
        let iso400 = stats.iso_buckets.iter().find(|b| b.label == "400").unwrap();
        assert_eq!(iso400.count, 2);
    }
}
