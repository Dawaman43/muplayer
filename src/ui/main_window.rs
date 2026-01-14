use std::sync::{Arc, Mutex};
use gtk4::prelude::*;
use gtk4::*;
use libadwaita::prelude::*;
use libadwaita::{Application, ApplicationWindow, HeaderBar};
use gtk4::Box as AdwBox;

use crate::audio::player::AudioPlayer;
use crate::library::database::LibraryDatabase;
use crate::ui::player_controls::PlayerControls;
use crate::ui::library_view::LibraryView;

pub struct MainWindow {
    window: ApplicationWindow,
    player_controls: Arc<PlayerControls>,
    library_view: Arc<LibraryView>,
}

impl MainWindow {
    pub fn new(
        app: &Application,
        audio_player: Arc<Mutex<AudioPlayer>>,
        database: Arc<Mutex<LibraryDatabase>>,
    ) -> Self {
        // Create main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Arch Music Player")
            .default_width(1200)
            .default_height(800)
            .build();
        
        // Create header bar
        let header_bar = HeaderBar::builder()
            .title_widget(&libadwaita::WindowTitle::new("Arch Music Player", ""))
            .build();
        
        // Create main box
        let main_box = AdwBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .build();
        
        // Create player controls
        let player_controls = Arc::new(PlayerControls::new(audio_player.clone()));
        
        // Create library view
        let library_view = Arc::new(LibraryView::new(database.clone()));
        
        // Assemble UI
        main_box.append(&header_bar);
        // Add widgets to main window
        main_box.append(library_view.as_ref());
        main_box.append(player_controls.as_ref());
        
        // Set main content
        window.set_content(Some(&main_box));
        
        Self {
            window,
            player_controls,
            library_view,
        }
    }
    
    pub fn present(&self) {
        self.window.present();
    }
}