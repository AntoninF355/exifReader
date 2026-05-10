use std::fs;
use base64::Engine;
use rsraw::RawImage;
use crate::models::PhotoMetadata;

pub fn read_exif_from_file(filepath: &str) -> Result<PhotoMetadata, String> {

    let buf = fs::read(filepath).map_err(|e| e.to_string())?;

    let mut image = RawImage::open(&buf).map_err(|e| e.to_string())?;

    let thumb64 = image.extract_thumbs().ok()
      .and_then(|thumbs| thumbs.into_iter().find(|t| t.format == rsraw::ThumbFormat::Jpeg))
      .map(|t| format!("data:image/jpeg;base64,{}", base64::engine::general_purpose::STANDARD.encode(&t.data)));

    let info = image.full_info();
    
    let filename = std::path::Path::new(filepath)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    Ok(PhotoMetadata { 
        filename,
        iso_speed: info.iso_speed,
        shutter: info.shutter,
        aperture: info.aperture,
        focal_length: info.focal_len,
        lens: info.lens_info.lens_name,
        make: info.make,
        model: info.model,
        datetime: info.datetime.map(|d| d.to_string()), 
        thumbnail: thumb64
    })
}