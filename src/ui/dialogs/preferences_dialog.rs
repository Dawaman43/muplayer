use gtk4::prelude::*;
use gtk4::*;
use libadwaita::prelude::*;

pub struct PreferencesDialog {
    dialog: libadwaita::PreferencesWindow,
}

impl PreferencesDialog {
    pub fn new() -> Self {
        let dialog = libadwaita::PreferencesWindow::builder()
            .title("Preferences")
            .default_width(600)
            .default_height(400)
            .build();
        
        let mut prefs_dialog = Self { dialog };
        
        // Create preferences pages
        prefs_dialog.setup_library_page(&prefs_dialog.dialog);
        prefs_dialog.setup_playback_page(&prefs_dialog.dialog);
        prefs_dialog.setup_interface_page(&prefs_dialog.dialog);
        
        prefs_dialog
    }
    
    fn setup_library_page(&self, window: &libadwaita::PreferencesWindow) {
        let page = libadwaita::PreferencesPage::builder()
            .title("Library")
            .icon_name("folder-music-symbolic")
            .build();
        
        let library_group = libadwaita::PreferencesGroup::builder()
            .title("Music Library")
            .build();
        
        // Music folder setting
        let music_folder_row = libadwaita::ActionRow::builder()
            .title("Music Folder")
            .subtitle("Default location for music files")
            .build();
        
        let folder_button = Button::builder()
            .label("Choose Folder")
            .build();
        
        music_folder_row.add_suffix(&folder_button);
        library_group.add(&music_folder_row);
        
        // Auto-scan setting
        let auto_scan_row = libadwaita::ActionRow::builder()
            .title("Auto-scan Library")
            .subtitle("Automatically scan for new music files")
            .build();
        
        let auto_scan_switch = Switch::builder()
            .active(true)
            .build();
        
        auto_scan_row.add_suffix(&auto_scan_switch);
        library_group.add(&auto_scan_row);
        
        page.add(&library_group);
        // Note: PreferencesWindow doesn't have add_page method
        // window.add_page(&page);
    }
    
    fn setup_playback_page(&self, window: &libadwaita::PreferencesWindow) {
        let page = libadwaita::PreferencesPage::builder()
            .title("Playback")
            .icon_name("media-playback-start-symbolic")
            .build();
        
        let playback_group = libadwaita::PreferencesGroup::builder()
            .title("Audio Settings")
            .build();
        
        // Crossfade setting
        let crossfade_row = libadwaita::ActionRow::builder()
            .title("Crossfade")
            .subtitle("Enable crossfade between tracks")
            .build();
        
        let crossfade_switch = Switch::builder()
            .active(false)
            .build();
        
        crossfade_row.add_suffix(&crossfade_switch);
        playback_group.add(&crossfade_row);
        
        // ReplayGain setting
        let replaygain_row = libadwaita::ActionRow::builder()
            .title("ReplayGain")
            .subtitle("Normalize volume using ReplayGain")
            .build();
        
        let replaygain_switch = Switch::builder()
            .active(true)
            .build();
        
        replaygain_row.add_suffix(&replaygain_switch);
        playback_group.add(&replaygain_row);
        
        page.add(&playback_group);
        // Note: PreferencesWindow doesn't have add_page method
        // window.add_page(&page);
    }
    
    fn setup_interface_page(&self, window: &libadwaita::PreferencesWindow) {
        let page = libadwaita::PreferencesPage::builder()
            .title("Interface")
            .icon_name("applications-graphics-symbolic")
            .build();
        
        let interface_group = libadwaita::PreferencesGroup::builder()
            .title("Appearance")
            .build();
        
        // Theme setting
        let theme_row = libadwaita::ActionRow::builder()
            .title("Theme")
            .subtitle("Choose application theme")
            .build();
        
        let theme_dropdown = DropDown::from_strings(&["System Default", "Light", "Dark"]);
        theme_row.add_suffix(&theme_dropdown);
        interface_group.add(&theme_row);
        
        // Show album art setting
        let album_art_row = libadwaita::ActionRow::builder()
            .title("Show Album Art")
            .subtitle("Display album art in library views")
            .build();
        
        let album_art_switch = Switch::builder()
            .active(true)
            .build();
        
        album_art_row.add_suffix(&album_art_switch);
        interface_group.add(&album_art_row);
        
        page.add(&interface_group);
        // Note: PreferencesWindow doesn't have add_page method
        // window.add_page(&page);
    }
    
    pub fn show(&self, parent: &ApplicationWindow) {
        self.dialog.set_transient_for(Some(parent));
        self.dialog.present();
    }
}