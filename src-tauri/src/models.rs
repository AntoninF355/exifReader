use serde::Serialize;

#[derive(Serialize)]
pub struct PhotoMetadata {
    pub filename: String,
    pub iso_speed: u32,
    pub lens: String,
    pub shutter: f32,
    pub aperture: f32,
    pub focal_length: f32,
    pub make: String,
    pub model: String,
    pub datetime: Option<String>,
    pub thumbnail: Option<String>
}

#[derive(Serialize)]
pub struct BucketEntry { //ligne d'un histogramme
    pub label: String,
    pub count: u32,
    pub percentage: f32
}

#[derive(Serialize)]

pub struct ApertureCell { //bucketEntry avec un peak pour aperture
    pub label: String,
    pub count: u32,
    pub percentage: f32,
    pub is_peak: bool
}

#[derive(Serialize)]

pub struct RankingEntry { //classement camera, lens
    pub name: String,
    pub count: u32,
    pub percentage: f32
}

#[derive(Serialize)]

pub struct PhotoStats {
    pub total: usize,

    //key stats
    pub most_used_focal_length: Option<u32>,
    pub median_aperture: Option<f32>,
    pub median_iso: Option<u32>,
    pub most_active_hour: Option<u8>,

    //histograms
    pub focal_length_buckets: Vec<BucketEntry>,
    pub aperture_stops: Vec<ApertureCell>,
    pub iso_buckets: Vec<BucketEntry>,
    pub shutter_buckets: Vec<BucketEntry>,

    //Classements
    pub cameras: Vec<RankingEntry>,
    pub lenses: Vec<RankingEntry>,

    pub shooting_hours: Vec<u32>
    
}

#[derive(Serialize)]

pub struct FolderAnalysis {
    pub photos: Vec<PhotoMetadata>,
    pub stats: PhotoStats
}