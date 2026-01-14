use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Button, Label, Box, Scale, Orientation, Frame, ListBox, TreeView, TreeViewColumn, TreeStore, CellRendererText, ScrolledWindow, Grid, Separator, FileChooserNative, FileChooserAction};
use libadwaita::{Application as AdwApplication, HeaderBar, WindowTitle, TabView, TabBar, ActionRow};

use std::path::PathBuf;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use rodio::{Source, Sink, Decoder, OutputStream};
use std::io::BufReader;

#[derive(Clone)]
struct Song {
    title: String,
    artist: String,
    album: String,
    path: PathBuf,
    duration: Option<u64>,
}

struct PlayerState {
    current_song: Option<Song>,
    is_playing: bool,
    sink: Option<Sink>,
    stream_handle: Option<rodio::OutputStreamHandle>,
}

impl PlayerState {
    fn new() -> Self {
        Self {
            current_song: None,
            is_playing: false,
            sink: None,
            stream_handle: None,
        }
    }
}

fn main() {
    let player_state = Arc::new(Mutex::new(PlayerState::new()));
    
    let app = AdwApplication::builder()
        .application_id("com.archlinux.musicplayer")
        .build();

    app.connect_activate(move |app| {
        let player_state = player_state.clone();
        build_ui(app, &player_state);
    });

    app.run();
}

fn build_ui(app: &AdwApplication, player_state: &Arc<Mutex<PlayerState>>) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Arch Music Player")
        .default_width(1200)
        .default_height(800)
        .build();

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();

    // Header bar with functionality
    let header_bar = HeaderBar::builder()
        .title_widget(&WindowTitle::new("Arch Music Player", ""))
        .build();

    // Add button to header
    let add_btn = Button::builder()
        .label("➕ Add Folder")
        .build();
    
    let refresh_btn = Button::builder()
        .label("🔄 Refresh")
        .build();
    
    let settings_btn = Button::builder()
        .label("⚙️ Settings")
        .build();

    // Header box for buttons
    let header_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .build();
    
    header_box.append(&add_btn);
    header_box.append(&refresh_btn);
    header_box.append(&settings_btn);

    // Add to header bar (libadwaita allows adding custom widgets)
    header_bar.pack_start(&add_btn);
    header_bar.pack_start(&refresh_btn);
    header_bar.pack_start(&settings_btn);

    // Status label
    let status_label = Label::builder()
        .label("🎵 Ready - Click 'Add Folder' to scan music")
        .halign(gtk4::Align::Center)
        .build();

    let tab_view = TabView::builder()
        .vexpand(true)
        .build();
    
    let _tab_bar = TabBar::builder()
        .view(&tab_view)
        .autohide(true)
        .build();

    // Create initial content (empty for now)
    let all_songs_content = create_all_songs_list(&Vec::new());
    let albums_content = create_albums_grid(&Vec::new());
    let artists_content = create_artists_list(&Vec::new());
    let folders_content = create_folders_tree(&Vec::new());

    // Add tabs
    let _tab1 = tab_view.append(&all_songs_content);
    let _tab2 = tab_view.append(&albums_content);
    let _tab3 = tab_view.append(&artists_content);
    let _tab4 = tab_view.append(&folders_content);

    // Now Playing bar
    let now_playing = create_now_playing(player_state);

    // Header content
    let header_content = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();
    
    header_content.append(&header_box);
    header_content.append(&status_label);
    header_content.append(&Separator::new(Orientation::Horizontal));

    main_box.append(&header_bar);
    main_box.append(&header_content);
    main_box.append(&tab_view);
    main_box.append(&now_playing);

    // Setup signal handlers
    let status_label_clone = status_label.clone();
    add_btn.connect_clicked(move |_| {
        // Create file chooser dialog
        let dialog = FileChooserNative::new(
            Some("Select Music Folder"),
            Some(&window),
            FileChooserAction::SelectFolder,
            "Select",
            "Cancel",
        );
        
        dialog.add_button("Cancel", gtk4::ResponseType::Cancel);
        dialog.add_button("Select", gtk4::ResponseType::Accept);
        
        dialog.connect_response(move |dialog, response| {
            if response == gtk4::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        status_label_clone.set_label(&format!("🔍 Scanning: {}...", path.display()));
                        // Scan the selected directory
                        thread::spawn(move || {
                            let extensions = ["mp3", "flac", "ogg", "m4a", "wav", "aac", "wma"];
                            let mut songs = Vec::new();
                            scan_directory_recursive(&path, &mut songs, &extensions);
                            println!("🎵 Found {} music files in selected folder!", songs.len());
                        });
                    }
                }
            }
            dialog.close();
        });
        
        dialog.present();
    });

    window.set_title(Some("Arch Music Player"));
    window.set_child(Some(&main_box));
    window.present();

    // Initial scan of default directories
    let status_label = status_label.clone();
    let player_state = player_state.clone();
    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(500));
        let songs = scan_music_library();
        println!("🎵 Found {} music files!", songs.len());
        status_label.set_label(&format!("📚 Found {} music files", songs.len()));
    });
}

fn scan_music_library() -> Vec<Song> {
    let mut songs = Vec::new();
    
    let extensions = ["mp3", "flac", "ogg", "m4a", "wav", "aac", "wma"];
    
    // Scan all configured directories
    let search_paths = vec![
        "/home/dave/Music",
        "/home/dave/Downloads",
        "/home/dave/Documents/Music", 
        "/home/dave/.local/share/music",
        "/home/dave/.config/music",
        "/home/dave/Music/Unsorted",
        "/home/dave/Music/Playlists",
    ];

    for dir in &search_paths {
        if PathBuf::from(dir).exists() {
            scan_directory_recursive(&PathBuf::from(dir), &mut songs, &extensions);
        }
    }

    // System directories
    let system_dirs = ["/usr/share/music", "/var/music"];
    for dir in &system_dirs {
        if PathBuf::from(dir).exists() {
            scan_directory_recursive(&PathBuf::from(dir), &mut songs, &extensions);
        }
    }

    songs
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
        duration: None,
    })
}

fn create_all_songs_list(songs: &[Song]) -> gtk4::Widget {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();

    let list = ListBox::builder()
        .build();

    for song in songs {
        let row = create_song_row(song);
        list.append(&row);
    }

    scrolled.set_child(Some(&list));
    scrolled.upcast()
}

fn create_song_row(song: &Song) -> gtk4::Widget {
    let row = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .margin_top(4)
        .margin_bottom(4)
        .margin_start(8)
        .margin_end(8)
        .build();

    let icon = Label::new(Some("🎵"));
    
    let info = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(2)
        .hexpand(true)
        .build();

    let title = Label::new(Some(&song.title));
    title.set_halign(gtk4::Align::Start);
    title.add_css_class("song-title");

    let artist_album = Label::new(Some(&format!("{} • {}", song.artist, song.album)));
    artist_album.set_halign(gtk4::Align::Start);
    artist_album.add_css_class("song-artist");

    let path_label = Label::new(Some(&song.path.display().to_string()));
    path_label.set_halign(gtk4::Align::Start);
    path_label.add_css_class("song-path");

    info.append(&title);
    info.append(&artist_album);
    info.append(&path_label);

    row.append(&icon);
    row.append(&info);
    row.append(&Label::new(Some("▶")));

    row.upcast()
}

fn create_albums_grid(songs: &[Song]) -> gtk4::Widget {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();

    let grid = Grid::builder()
        .column_spacing(15)
        .row_spacing(15)
        .build();

    let mut albums: std::collections::HashMap<(String, String), String> = std::collections::HashMap::new();
    
    for song in songs {
        let key = (song.album.clone(), song.artist.clone());
        if !albums.contains_key(&key) {
            albums.insert(key.clone(), song.title.clone());
        }
    }

    let albums_vec: Vec<_> = albums.into_iter().collect();
    
    for (i, ((album, artist), sample_title)) in albums_vec.iter().enumerate().take(24) {
        let card = create_album_card(album, artist, "🎸", sample_title);
        let row = (i / 4) as i32;
        let col = (i % 4) as i32;
        grid.attach(&card, col, row, 1, 1);
    }

    scrolled.set_child(Some(&grid));
    scrolled.upcast()
}

fn create_album_card(album: &str, artist: &str, icon: &str, _sample_title: &str) -> gtk4::Widget {
    let card = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .build();

    let art_frame = Frame::builder().build();
    let art_icon = Label::new(Some(icon));
    art_icon.set_size_request(140, 140);
    art_frame.set_child(Some(&art_icon));

    let album_lbl = Label::new(Some(album));
    album_lbl.set_halign(gtk4::Align::Center);
    album_lbl.add_css_class("album-title");

    let artist_lbl = Label::new(Some(artist));
    artist_lbl.set_halign(gtk4::Align::Center);
    artist_lbl.add_css_class("album-artist");

    card.append(&art_frame);
    card.append(&album_lbl);
    card.append(&artist_lbl);

    card.upcast()
}

fn create_artists_list(songs: &[Song]) -> gtk4::Widget {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();

    let list = ListBox::builder()
        .build();

    let mut artists: std::collections::HashMap<String, (u32, String)> = std::collections::HashMap::new();

    for song in songs {
        let entry = artists.entry(song.artist.clone()).or_insert((0, song.album.clone()));
        entry.0 += 1;
    }

    for (artist, (count, sample_album)) in &artists {
        let row = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(12)
            .margin_end(12)
            .build();

        let avatar = Label::new(Some("🎤"));
        avatar.set_size_request(56, 56);

        let info = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .build();

        let name_lbl = Label::new(Some(artist));
        name_lbl.set_halign(gtk4::Align::Start);
        name_lbl.add_css_class("artist-name");

        let count_lbl = Label::new(Some(&format!("{} songs • {}", count, sample_album)));
        count_lbl.set_halign(gtk4::Align::Start);
        count_lbl.add_css_class("artist-albums");

        info.append(&name_lbl);
        info.append(&count_lbl);

        row.append(&avatar);
        row.append(&info);
        row.append(&Label::new(Some("▶")));

        list.append(&row);
    }

    scrolled.set_child(Some(&list));
    scrolled.upcast()
}

fn create_folders_tree(songs: &[Song]) -> gtk4::Widget {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();

    let tree = TreeView::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    let store = TreeStore::new(&[String::static_type()]);
    
    let root = store.append(None);
    store.set(&root, &[(0, &"📁 Music Library")]);
    
    let home = store.append(Some(&root));
    store.set(&home, &[(0, &"🏠 Home Folders")]);
    
    let system = store.append(Some(&root));
    store.set(&system, &[(0, &"💾 System Folders")]);

    let mut home_folders: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    let mut system_folders: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    for song in songs {
        if let Some(parent) = song.path.parent() {
            let path_str = parent.to_string_lossy().to_string();
            
            if path_str.contains("/home/dave") {
                let main_dir = if path_str.contains("/Downloads") {
                    "📥 Downloads"
                } else if path_str.contains("/Music") {
                    "🎵 Music"
                } else if path_str.contains("/Documents") {
                    "📄 Documents"
                } else {
                    "🏠 Other Home"
                };
                
                home_folders.entry(main_dir.to_string())
                    .or_insert_with(Vec::new)
                    .push(path_str);
            } else if path_str.starts_with("/usr") || path_str.starts_with("/var") {
                let main_dir = if path_str.starts_with("/usr") {
                    "💾 /usr/share"
                } else {
                    "💾 /var"
                };
                
                system_folders.entry(main_dir.to_string())
                    .or_insert_with(Vec::new)
                    .push(path_str);
            }
        }
    }

    for (main_dir, paths) in &home_folders {
        let main_node = store.append(Some(&home));
        store.set(&main_node, &[(0, &main_dir)]);
        
        for path in paths.iter().take(10) {
            let parts: Vec<&str> = path.split('/').collect();
            let name = parts.last().map(|s| format!("📁 {}", s)).unwrap_or_else(|| path.clone());
            
            let sub_node = store.append(Some(&main_node));
            store.set(&sub_node, &[(0, &name)]);
        }
    }

    for (main_dir, paths) in &system_folders {
        let main_node = store.append(Some(&system));
        store.set(&main_node, &[(0, &main_dir)]);
        
        for path in paths.iter().take(5) {
            let parts: Vec<&str> = path.split('/').collect();
            let name = parts.last().map(|s| format!("📁 {}", s)).unwrap_or_else(|| path.clone());
            
            let sub_node = store.append(Some(&main_node));
            store.set(&sub_node, &[(0, &name)]);
        }
    }

    let col = TreeViewColumn::builder().title("Folders").build();
    let renderer = CellRendererText::builder().build();
    col.pack_start(&renderer, true);
    col.add_attribute(&renderer, "text", 0);
    
    tree.append_column(&col);
    tree.set_model(Some(&store));

    scrolled.set_child(Some(&tree));
    scrolled.upcast()
}

fn create_now_playing(player_state: &Arc<Mutex<PlayerState>>) -> gtk4::Widget {
    let container = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();

    // Progress bar
    let progress_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .margin_start(16)
        .margin_end(16)
        .margin_top(8)
        .build();

    let time1 = Label::new(Some("0:00"));
    time1.add_css_class("time-label");

    let prog = Scale::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .draw_value(false)
        .build();

    let time2 = Label::new(Some("0:00"));
    time2.add_css_class("time-label");

    progress_box.append(&time1);
    progress_box.append(&prog);
    progress_box.append(&time2);

    // Controls
    let controls_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(20)
        .margin_start(16)
        .margin_end(16)
        .margin_top(12)
        .margin_bottom(12)
        .hexpand(true)
        .build();

    let art = Label::new(Some("🎵"));
    art.set_size_request(70, 70);
    art.add_css_class("now-playing-art");

    let track_info = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(3)
        .hexpand(true)
        .build();

    let track_name = Label::new(Some("No track selected"));
    track_name.add_css_class("track-name");

    let track_artist = Label::new(Some("Select a song to play"));
    track_artist.add_css_class("track-artist");

    track_info.append(&track_name);
    track_info.append(&track_artist);

    let playback = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .build();

    let prev = Button::builder().label("⏮").build();
    prev.add_css_class("control-btn");

    let play = Button::builder().label("▶").build();
    play.add_css_class("control-btn");

    let next = Button::builder().label("⏭").build();
    next.add_css_class("control-btn");

    playback.append(&prev);
    playback.append(&play);
    playback.append(&next);

    let volume = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .build();

    let vol_icon = Label::new(Some("🔊"));
    let vol_scale = Scale::builder()
        .orientation(Orientation::Horizontal)
        .width_request(80)
        .draw_value(false)
        .build();

    volume.append(&vol_icon);
    volume.append(&vol_scale);

    let main = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(20)
        .build();

    main.append(&art);
    main.append(&track_info);
    main.append(&playback);
    main.append(&volume);

    container.append(&progress_box);
    container.append(&main);

    let sep = Separator::new(Orientation::Horizontal);
    container.prepend(&sep);

    container.upcast()
}