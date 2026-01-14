use gtk4::prelude::*;
use gtk4::*;
use libadwaita::prelude::*;

pub struct PlaylistDialog {
    dialog: gtk4::MessageDialog,
    name_entry: Entry,
}

impl PlaylistDialog {
    pub fn new() -> Self {
        let dialog = gtk4::MessageDialog::builder()
            .text("Create New Playlist")
            .body("Enter a name for your new playlist")
            .build();
        
        // Add entry for playlist name
        let name_entry = Entry::builder()
            .placeholder_text("Playlist name...")
            .build();
        
        let content_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_bottom(12)
            .build();
        
        content_box.append(&name_entry);
        
        dialog.set_extra_child(Some(&content_box));
        
        // Add response buttons
        dialog.add_response("cancel", "Cancel");
        dialog.add_response("create", "Create");
        dialog.set_default_response(Some("create"));
        
        Self {
            dialog,
            name_entry,
        }
    }
    
    pub fn show(&self, parent: &ApplicationWindow) -> Option<String> {
        self.dialog.set_transient_for(Some(parent));
        
        // Show dialog and return simplified response
        self.dialog.show();
        "create" // Simplified response for now
        
        if response == "create" {
            Some(self.name_entry.text().to_string())
        } else {
            None
        }
    }
}