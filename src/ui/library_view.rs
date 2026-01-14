use std::sync::{Arc, Mutex};
use gtk4::prelude::*;
use gtk4::*;

// Add Widget implementation for LibraryView
impl AsRef<Widget> for LibraryView {
    fn as_ref(&self) -> &Widget {
        &self.container
    }
}

use crate::library::database::LibraryDatabase;

pub struct LibraryView {
    container: Box,
    stack: Stack,
    album_view: ScrolledWindow,
    artist_view: ScrolledWindow,
    folder_view: ScrolledWindow,
    playlist_view: ScrolledWindow,
    search_entry: SearchEntry,
    database: Arc<Mutex<LibraryDatabase>>,
}

impl LibraryView {
    pub fn new(database: Arc<Mutex<LibraryDatabase>>) -> Self {
        // Create container
        let container = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .build();
        
        // Create header with search
        let header_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(6)
            .margin_start(12)
            .margin_end(12)
            .margin_top(6)
            .margin_bottom(6)
            .build();
        
        let search_entry = SearchEntry::builder()
            .placeholder_text("Search music library...")
            .hexpand(true)
            .build();
        
        // Create a simple tab button bar for now instead of ViewSwitcher
        let tab_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(6)
            .build();
        
        let albums_btn = Button::builder()
            .label("Albums")
            .build();
        let artists_btn = Button::builder()
            .label("Artists")
            .build();
        let folders_btn = Button::builder()
            .label("Folders")
            .build();
        let playlists_btn = Button::builder()
            .label("Playlists")
            .build();
        
        tab_box.append(&albums_btn);
        tab_box.append(&artists_btn);
        tab_box.append(&folders_btn);
        tab_box.append(&playlists_btn);
        
        header_box.append(&search_entry);
        header_box.append(&tab_box);
        
        // Create stack for different views
        let stack = Stack::builder()
            .transition_type(StackTransitionType::SlideLeftRight)
            .build();
        
        // Create individual views
        let album_view = ScrolledWindow::builder()
            .vexpand(true)
            .build();
        
        let artist_view = ScrolledWindow::builder()
            .vexpand(true)
            .build();
        
        let folder_view = ScrolledWindow::builder()
            .vexpand(true)
            .build();
        
        let playlist_view = ScrolledWindow::builder()
            .vexpand(true)
            .build();
        
        // Add views to stack
        stack.add_titled(&album_view, Some("albums"), "Albums");
        stack.add_titled(&artist_view, Some("artists"), "Artists");
        stack.add_titled(&folder_view, Some("folders"), "Folders");
        stack.add_titled(&playlist_view, Some("playlists"), "Playlists");
        
        // Connect view switcher to stack
        // Note: ViewSwitcher expects a ViewStack, not a regular Stack
        // For now, just create a simple switcher without stack
        // view_switcher.set_stack(Some(&stack));
        
        // Assemble UI
        container.append(&header_box);
        container.append(&stack);
        
        let library_view = Self {
            container,
            stack,
            album_view,
            artist_view,
            folder_view,
            playlist_view,
            search_entry,
            database,
        };
        
        // Initialize views
        library_view.initialize_views();
        
        library_view
    }
    
    fn initialize_views(&self) {
        // Setup album view
        self.setup_album_view();
        
        // Setup artist view
        self.setup_artist_view();
        
        // Setup folder view
        self.setup_folder_view();
        
        // Setup playlist view
        self.setup_playlist_view();
        
        // Setup search
        self.setup_search();
    }
    
    fn setup_album_view(&self) {
        let album_grid = Grid::builder()
            .column_spacing(12)
            .row_spacing(12)
            .margin_start(12)
            .margin_end(12)
            .margin_top(12)
            .margin_bottom(12)
            .build();
        
        // Load albums from database
        if let Ok(db) = self.database.lock() {
            if let Ok(albums) = db.get_all_albums() {
                for (i, album) in albums.iter().enumerate() {
                    let album_widget = self.create_album_widget(album);
                    album_grid.attach(&album_widget, (i % 4) as i32, (i / 4) as i32, 1, 1);
                }
            }
        }
        
        self.album_view.set_child(Some(&album_grid));
    }
    
    fn setup_artist_view(&self) {
        let artist_list = ListBox::builder()
            .selection_mode(SelectionMode::Single)
            .build();
        
        // Load artists from database
        if let Ok(db) = self.database.lock() {
            if let Ok(artists) = db.get_all_artists() {
                for artist in artists {
                    let artist_row = self.create_artist_row(&artist);
                    artist_list.append(&artist_row);
                }
            }
        }
        
        let scrolled = ScrolledWindow::builder()
            .child(&artist_list)
            .vexpand(true)
            .build();
        
        self.artist_view.set_child(Some(&scrolled));
    }
    
    fn setup_folder_view(&self) {
        let folder_tree = TreeView::builder()
            .enable_tree_lines(true)
            .build();
        
        // Setup folder tree structure
        self.setup_folder_tree(&folder_tree);
        
        self.folder_view.set_child(Some(&folder_tree));
    }
    
    fn setup_playlist_view(&self) {
        let playlist_list = ListBox::builder()
            .selection_mode(SelectionMode::Single)
            .build();
        
        // Load playlists from database
        if let Ok(db) = self.database.lock() {
            if let Ok(playlists) = db.get_all_playlists() {
                for playlist in playlists {
                    let playlist_row = self.create_playlist_row(&playlist);
                    playlist_list.append(&playlist_row);
                }
            }
        }
        
        let scrolled = ScrolledWindow::builder()
            .child(&playlist_list)
            .vexpand(true)
            .build();
        
        self.playlist_view.set_child(Some(&scrolled));
    }
    
    fn setup_search(&self) {
        let database = self.database.clone();
        let stack = self.stack.clone();
        
        self.search_entry.connect_search_changed(move |entry| {
            let search_text: String = entry.text().to_string();
            if !search_text.is_empty() {
                // Perform search and update current view
                if let Ok(db) = database.lock() {
                    if let Ok(results) = db.search(&search_text) {
                        // Update current view with search results
                        // This would update the visible view based on search
                    }
                }
            }
        });
    }
    
    fn create_album_widget(&self, album: &str) -> Widget {
        let container = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .build();
        
        // Album art placeholder
        let album_art = Image::builder()
            .icon_name("folder-music-symbolic")
            .pixel_size(120)
            .build();
        
        // Album name
        let album_label = Label::builder()
            .label(album)
            .halign(Align::Center)
            .build();
        
        container.append(&album_art);
        container.append(&album_label);
        
        container.upcast()
    }
    
    fn create_artist_row(&self, artist: &str) -> Widget {
        let box_row = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .margin_start(12)
            .margin_end(12)
            .margin_top(6)
            .margin_bottom(6)
            .build();
        
        let artist_icon = Image::builder()
            .icon_name("avatar-default-symbolic")
            .pixel_size(48)
            .build();
        
        let artist_label = Label::builder()
            .label(artist)
            .hexpand(true)
            .build();
        
        box_row.append(&artist_icon);
        box_row.append(&artist_label);
        
        box_row.upcast()
    }
    
    fn create_playlist_row(&self, playlist: &str) -> Widget {
        let box_row = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .margin_start(12)
            .margin_end(12)
            .margin_top(6)
            .margin_bottom(6)
            .build();
        
        let playlist_icon = Image::builder()
            .icon_name("playlist-symbolic")
            .pixel_size(48)
            .build();
        
        let playlist_label = Label::builder()
            .label(playlist)
            .hexpand(true)
            .build();
        
        box_row.append(&playlist_icon);
        box_row.append(&playlist_label);
        
        box_row.upcast()
    }
    
    fn setup_folder_tree(&self, tree_view: &TreeView) {
        // This would setup the folder tree structure
        // Implementation would use TreeStore and TreeViewColumn
    }
}

impl AsRef<Widget> for LibraryView {
    fn as_ref(&self) -> &Widget {
        self.container.as_ref()
    }
}