use rsraw::RawImage;
use std::fs;
use serde::Serialize;

#[derive(Serialize)]
struct PhotoMetadata {
    pub iso_speed: u32,
    pub shutter: f32,
    pub aperture: f32,
    pub focal_length: f32,
    pub make: String,
    pub model: String,
    pub datetime: Option<String>
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![read_exif_from_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn read_exif_from_file(filepath: &str) -> Result<PhotoMetadata, String> {

    let buf = fs::read(&filepath).map_err(|e| e.to_string())?;

    let image = RawImage::open(&buf).map_err(|e| e.to_string())?;

    let info = image.full_info();
    
    
    Ok(PhotoMetadata { 
        iso_speed: info.iso_speed,
        shutter: info.shutter,
        aperture: info.aperture,
        focal_length: info.focal_len,
        make: info.make,
        model: info.model,
        datetime: info.datetime.map(|d| d.to_string()) 
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
        fn read_exif_from_file_test() {
            let result = read_exif_from_file("tests/Sony-a1-raw-00001.arw");

            assert!(result.is_ok(), "Échec lecture EXIF : {:?}", result.err());

            let meta = result.unwrap();

            assert!(meta.iso_speed > 0, "ISO devrait être > 0");
            assert!(meta.aperture > 0.0, "Ouverture devrait être > 0");
            assert!(meta.focal_length > 0.0, "Focale devrait être > 0");
            assert!(!meta.make.is_empty(), "Make ne devrait pas être vide");
            assert!(!meta.model.is_empty(), "Model ne devrait pas être vide");

            // Affiche les valeurs pour vérification visuelle
            println!("ISO       : {}", meta.iso_speed);
            println!("Ouverture : f/{}", meta.aperture);
            println!("Focale    : {}mm", meta.focal_length);
            println!("Vitesse   : 1/{}s", meta.shutter);
            println!("Appareil  : {} {}", meta.make, meta.model);
            println!("Date      : {:?}", meta.datetime);
        }
}