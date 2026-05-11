use crate::reader::read_exif_from_file;
use crate::stats::compute_stats;
use crate::models::FolderAnalysis;

#[tauri::command]
pub fn analyze_folder(folder_path: String) -> Result<FolderAnalysis, String> {
    let extensions = ["jpg", "jpeg", "png", "arw", "cr3", "nef", "dng"];

    let entries = std::fs::read_dir(&folder_path).map_err(|e| e.to_string())?;

    let photos = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if !path.is_file() { return None; }
            let ext = path.extension()?.to_str()?.to_lowercase();
            if !extensions.contains(&ext.as_str()) { return None; }
            read_exif_from_file(&path.to_string_lossy()).ok()
        })
        .collect::<Vec<_>>();

    let stats = compute_stats(&photos);

    Ok(FolderAnalysis { photos, stats })
}