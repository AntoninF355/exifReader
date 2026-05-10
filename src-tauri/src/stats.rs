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

fn compute_focal_buckets(_photos: &[PhotoMetadata]) -> Vec<BucketEntry> {
    vec![]
}

fn compute_aperture_stops(_photos: &[PhotoMetadata]) -> Vec<ApertureCell> {
    vec![]
}

fn compute_shutter_buckets(_photos: &[PhotoMetadata]) -> Vec<BucketEntry> {
    vec![]
}

fn compute_shooting_hours(_photos: &[PhotoMetadata]) -> Vec<u32> {
    vec![0; 24]
}

fn most_used_focal(_photos: &[PhotoMetadata]) -> Option<u32> {
    None
}

fn median_iso(_photos: &[PhotoMetadata]) -> Option<u32> {
    None
}

fn median_aperture(_photos: &[PhotoMetadata]) -> Option<f32> {
    None
}

fn most_active_hour(_photos: &[PhotoMetadata]) -> Option<u8> {
    None
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
