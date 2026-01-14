use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use anyhow::Result;
use notify::{Watcher, RecursiveMode, Event, EventKind, RecommendedWatcher};
use tokio::sync::mpsc;
use std::thread;

use crate::library::database::LibraryDatabase;
use crate::metadata::tag_reader::read_metadata;

pub struct LibraryScanner {
    database: Arc<LibraryDatabase>,
    watcher: Option<RecommendedWatcher>,
    scan_paths: Vec<PathBuf>,
}

impl LibraryScanner {
    pub fn new(database: Arc<LibraryDatabase>) -> Self {
        Self {
            database,
            watcher: None,
            scan_paths: Vec::new(),
        }
    }
    
    pub fn initialize_default_paths(&mut self) -> Result<()> {
        let mut paths = Vec::new();
        
        // User music directory
        if let Some(music_dir) = dirs::audio_dir() {
            paths.push(music_dir);
        }
        
        // Additional common paths
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        
        paths.push(home.join("Music"));
        paths.push(home.join("Documents").join("Music"));
        paths.push(home.join("Downloads").join("Music"));
        
        // System-wide paths (if they exist and are readable)
        let system_paths = vec![
            PathBuf::from("/usr/share/music"),
            PathBuf::from("/var/music"),
            PathBuf::from("/usr/local/share/music"),
        ];
        
        for path in system_paths {
            if path.exists() && path.is_dir() {
                if let Ok(metadata) = std::fs::metadata(&path) {
                    // Check if we have read permissions
                    if metadata.permissions().readonly() == false {
                        paths.push(path);
                    }
                }
            }
        }
        
        self.scan_paths = paths;
        Ok(())
    }
    
    pub fn add_scan_path(&mut self, path: PathBuf) {
        if path.exists() && path.is_dir() {
            self.scan_paths.push(path);
        }
    }
    
    pub fn scan_library(&self) -> Result<usize> {
        let mut scanned_count = 0;
        
        for scan_path in &self.scan_paths {
            scanned_count += self.scan_directory(scan_path)?;
        }
        
        Ok(scanned_count)
    }
    
    fn scan_directory(&self, dir_path: &Path) -> Result<usize> {
        let mut scanned_count = 0;
        
        println!("Scanning directory: {}", dir_path.display());
        
        let entries = std::fs::read_dir(dir_path)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Recursively scan subdirectories
                scanned_count += self.scan_directory(&path)?;
            } else if self.is_audio_file(&path) {
                // Process audio file
                if let Ok(metadata) = read_metadata(&path) {
                    if let Err(e) = self.database.add_song(&path, &metadata) {
                        eprintln!("Error adding song {}: {}", path.display(), e);
                    } else {
                        scanned_count += 1;
                    }
                }
            }
        }
        
        Ok(scanned_count)
    }
    
    pub fn start_file_watcher(&mut self) -> Result<()> {
        use notify::Config;
        
        let (tx, mut rx) = mpsc::channel::<Event>(100);
        let database = self.database.clone();
        
        // Create file watcher
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    let _ = tx.blocking_send(event);
                }
            },
            Config::default(),
        )?;
        
        // Watch all scan paths
        for scan_path in &self.scan_paths {
            if let Err(e) = watcher.watch(scan_path, RecursiveMode::Recursive) {
                eprintln!("Error watching path {}: {}", scan_path.display(), e);
            }
        }
        
        self.watcher = Some(watcher);
        
        // Spawn task to handle file system events
        let database_clone = database.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                Self::handle_filesystem_event(event, &database_clone);
            }
        });
        
        Ok(())
    }
    
    fn handle_filesystem_event(event: Event, database: &LibraryDatabase) {
        match event.kind {
            EventKind::Create(_) => {
                for path in event.paths {
                    if path.is_file() && Self::is_audio_file_static(&path) {
                        println!("New audio file detected: {}", path.display());
                        if let Ok(metadata) = read_metadata(&path) {
                            if let Err(e) = database.add_song(&path, &metadata) {
                                eprintln!("Error adding new file {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
            EventKind::Modify(_) => {
                for path in event.paths {
                    if path.is_file() && Self::is_audio_file_static(&path) {
                        println!("Audio file modified: {}", path.display());
                        // Re-read metadata and update database
                        if let Ok(metadata) = read_metadata(&path) {
                            if let Err(e) = database.add_song(&path, &metadata) {
                                eprintln!("Error updating file {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
            EventKind::Remove(_) => {
                for path in event.paths {
                    println!("File removed: {}", path.display());
                    // Remove from database
                    // This would need a remove_song method in the database
                }
            }
            _ => {}
        }
    }
    
    fn is_audio_file(&self, path: &Path) -> bool {
        Self::is_audio_file_static(path)
    }
    
    fn is_audio_file_static(path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                let ext_lower = ext_str.to_lowercase();
                matches!(ext_lower.as_str(),
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
    
    pub fn get_scan_paths(&self) -> &[PathBuf] {
        &self.scan_paths
    }
    
    pub fn remove_scan_path(&mut self, index: usize) -> Result<()> {
        if index < self.scan_paths.len() {
            self.scan_paths.remove(index);
            
            // Restart file watcher if it's running
            if self.watcher.is_some() {
                self.stop_file_watcher()?;
                self.start_file_watcher()?;
            }
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid scan path index"))
        }
    }
    
    pub fn stop_file_watcher(&mut self) -> Result<()> {
        if let Some(mut watcher) = self.watcher.take() {
            // Unwatch all paths
            for scan_path in &self.scan_paths {
                let _ = watcher.unwatch(scan_path);
            }
        }
        Ok(())
    }
}