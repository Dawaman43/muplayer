use std::sync::{Arc, Mutex};
use gtk4::prelude::*;
use gtk4::{gio, Application as GtkApplication};
use libadwaita::{prelude::*, Application, ApplicationWindow};
use anyhow::Result;

use crate::ui::main_window::MainWindow;
use crate::audio::player::AudioPlayer;
use crate::library::database::LibraryDatabase;

pub struct MusicPlayerApplication {
    app: Application,
    main_window: Option<Arc<MainWindow>>,
    audio_player: Arc<Mutex<AudioPlayer>>,
    database: Arc<Mutex<LibraryDatabase>>,
}

impl MusicPlayerApplication {
    pub fn new(app: &Application) -> Self {
        let audio_player = Arc::new(Mutex::new(AudioPlayer::new()));
        let database = Arc::new(Mutex::new(LibraryDatabase::new()));
        
        Self {
            app: app.clone(),
            main_window: None,
            audio_player,
            database,
        }
    }
    
    pub fn activate(&mut self) {
        // Create main window
        let main_window = Arc::new(MainWindow::new(&self.app, self.audio_player.clone(), self.database.clone()));
        self.main_window = Some(main_window);
        
        // Show the window
        if let Some(ref window) = self.main_window {
            window.present();
        }
    }
    
    pub fn open_files(&self, files: &[gio::File]) {
        // Handle opening of audio files
        for file in files {
            if let Some(path) = file.path() {
                if let Err(e) = self.add_file_to_library(&path) {
                    eprintln!("Error adding file to library: {}", e);
                }
            }
        }
    }
    
    fn add_file_to_library(&self, path: &std::path::Path) -> Result<()> {
        // Add file to database and library
        let metadata = crate::metadata::tag_reader::read_metadata(path)?;
        let database = self.database.lock().unwrap();
        database.add_song(path, &metadata)?;
        Ok(())
    }
}