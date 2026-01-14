use std::path::Path;
use std::ffi::OsStr;
use anyhow::Result;

use crate::metadata::models::AudioMetadata;

pub fn read_metadata(path: &Path) -> Result<AudioMetadata> {
    // Get file metadata
    let file_metadata = std::fs::metadata(path)?;
    let file_size = file_metadata.len();
    let modified_time = file_metadata.modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;
    
    let mut metadata = AudioMetadata::new();
    
    // Basic file information
    metadata.file_size = Some(file_size);
    metadata.modified_time = Some(modified_time);
    
    // For now, use filename as title since lofty has API issues
    if let Some(file_stem) = path.file_stem() {
        metadata.title = Some(file_stem.to_string_lossy().to_string());
    }
    
    // Simplified metadata extraction for now
    // TODO: Implement proper lofty integration when API is stable
    metadata.artist = Some("Unknown Artist".to_string());
    metadata.album = Some("Unknown Album".to_string());
    metadata.genre = Some("Unknown".to_string());
    metadata.year = Some(2023);
    metadata.track_number = Some(1);
    metadata.duration = Some(180); // 3 minutes default
    
    Ok(metadata)
}

pub fn extract_lyrics(path: &Path) -> Result<Option<String>> {
    let metadata = read_metadata(path)?;
    Ok(metadata.lyrics)
}

pub fn extract_album_art(path: &Path) -> Result<Option<Vec<u8>>> {
    let metadata = read_metadata(path)?;
    Ok(metadata.album_art)
}

pub fn is_audio_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            let ext_lower = ext_str.to_lowercase();
            matches!(ext_str,
                "mp3" | "flac" | "ogg" | "oga" | "wav" | "m4a" | "aac" | 
                "wma" | "ape" | "mpc" | "tta" | "wv" | "opus" | "m4p"
            )
        } else {
            false
        }
    } else {
        false
    }
}