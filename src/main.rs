use gtk4::prelude::*;
use libadwaita::{prelude::*, Application};

mod app;
mod ui;
mod audio;
mod library;
mod metadata;

use app::application::MusicPlayerApplication;

fn main() {
    // Initialize GTK
    gtk4::init().expect("Failed to initialize GTK");
    
    // Create application
    let app = Application::new(
        Some("com.archlinux.musicplayer"),
        libadwaita::gio::ApplicationFlags::HANDLES_OPEN
    );
    
    // Connect to activate signal
    app.connect_activate(|app| {
        let mut application = MusicPlayerApplication::new(app);
        application.activate();
    });
    
    // Connect to open signal for file handling
    app.connect_open(|app, files, _| {
        let application = MusicPlayerApplication::new(app);
        application.open_files(files);
    });
    
    // Run the application
    app.run();
}