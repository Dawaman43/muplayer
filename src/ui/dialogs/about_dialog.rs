use gtk4::prelude::*;
use gtk4::*;
use libadwaita::prelude::*;

pub struct AboutDialog {
    dialog: gtk4::AboutDialog,
}

impl AboutDialog {
    pub fn new() -> Self {
        let dialog = gtk4::AboutDialog::builder()
            .application("Arch Music Player")
            .version("0.1.0")
            .developer_name("Arch Linux Community")
            .license_type(gtk4::License::Gpl30)
            .website("https://github.com/archlinux/music-player")
            .issue_url("https://github.com/archlinux/music-player/issues")
            .build();
        
        // Add developers
        dialog.set_developers(&[
            "Arch Linux Community",
            "Contributors on GitHub",
        ]);
        
        // Add designers
        dialog.set_designers(&[
            "GNOME Design Team",
            "Libadwaita Contributors",
        ]);
        
        // Add translator credits
        dialog.set_translator_credits("Translator credits");
        
        Self { dialog }
    }
    
    pub fn show(&self, parent: &ApplicationWindow) {
        self.dialog.set_transient_for(Some(parent));
        self.dialog.present();
    }
}