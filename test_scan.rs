use std::path::PathBuf;
use std::fs;

#[derive(Clone)]
struct Song {
    title: String,
    artist: String,
    album: String,
    path: PathBuf,
}

fn scan_directory_recursive(dir: &PathBuf, songs: &mut Vec<Song>, extensions: &[&str]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if extensions.contains(&ext.to_lowercase().as_str()) {
                        if let Some(song) = create_song_from_path(&path) {
                            songs.push(song);
                            println!("Found: {:?}", path);
                        }
                    }
                }
            } else if path.is_dir() {
                scan_directory_recursive(&path, songs, extensions);
            }
        }
    }
}

fn create_song_from_path(path: &PathBuf) -> Option<Song> {
    let file_name = path.file_stem()?.to_string_lossy().to_string();
    
    let title = if file_name.contains(" - ") {
        file_name.split(" - ").nth(0)?.to_string()
    } else {
        file_name.clone()
    };

    let artist = if file_name.contains(" - ") && file_name.split(" - ").nth(1).is_some() {
        file_name.split(" - ").nth(1)?.to_string()
    } else {
        "Unknown Artist".to_string()
    };

    let album = path.parent()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown Album".to_string());

    Some(Song {
        title,
        artist,
        album,
        path: path.clone(),
    })
}

fn main() {
    let mut songs = Vec::new();
    let extensions = ["mp3", "flac", "ogg", "m4a", "wav", "aac", "wma"];
    
    let search_paths = vec![
        "/home/dave/Music",
        "/home/dave/Downloads",
        "/home/dave/Documents/Music", 
        "/home/dave/.local/share/music",
        "/home/dave/.config/music",
    ];

    for dir in &search_paths {
        let path = PathBuf::from(dir);
        println!("Checking: {}", path.display());
        if path.exists() {
            scan_directory_recursive(&path, &mut songs, &extensions);
        }
    }
    
    println!("\n=== Total songs found: {} ===", songs.len());
    for song in &songs {
        println!("  {} - {} ({})", song.artist, song.title, song.album);
    }
}
