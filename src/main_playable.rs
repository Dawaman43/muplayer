use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Button, Label, Box, Scale, Orientation, Frame, ListBox, TreeView, TreeViewColumn, TreeStore, CellRendererText, ScrolledWindow, Grid, Separator, FileChooserAction, FileChooserDialog};
use gtk4::glib;
use gtk4::glib::clone;
use libadwaita::{Application as AdwApplication, HeaderBar, WindowTitle, TabView, TabBar};

use std::path::PathBuf;
use std::fs;
use std::sync::{Arc, Mutex};
use rodio::{Sink, Decoder, OutputStream};
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
    current_song_index: Option<usize>,
    is_playing: bool,
    volume: f32,
    sink: Option<Sink>,
    stream_handle: Option<(OutputStream, rodio::OutputStreamHandle)>,
    songs: Vec<Song>,
    start_time: Option<std::time::Instant>,
    duration: Option<u64>,
    seek_position: u64,
}

impl PlayerState {
    fn new() -> Self {
        Self {
            current_song: None,
            current_song_index: None,
            is_playing: false,
            volume: 1.0,
            sink: None,
            stream_handle: None,
            songs: Vec::new(),
            start_time: None,
            duration: None,
            seek_position: 0,
        }
    }
    
    fn ensure_audio_initialized(&mut self) -> Result<(), String> {
        if self.stream_handle.is_none() {
            match OutputStream::try_default() {
                Ok((stream, stream_handle)) => {
                    self.stream_handle = Some((stream, stream_handle));
                    match Sink::try_new(&self.stream_handle.as_ref().unwrap().1) {
                        Ok(sink) => {
                            sink.set_volume(self.volume);
                            self.sink = Some(sink);
                            Ok(())
                        }
                        Err(e) => Err(format!("Failed to create audio sink: {}", e))
                    }
                }
                Err(e) => Err(format!("Failed to open audio stream: {}", e))
            }
        } else {
            Ok(())
        }
    }
    
    fn set_volume(&mut self, volume: f32) {
        self.volume = volume.max(0.0).min(1.0);
        if let Some(sink) = &self.sink {
            sink.set_volume(self.volume);
        }
    }
    
    fn get_volume(&self) -> f32 {
        self.volume
    }
    
    fn play_song_at_index(&mut self, index: usize) -> Result<(), String> {
        if index >= self.songs.len() {
            return Err("Invalid song index".to_string());
        }
        
        let song = self.songs[index].clone();
        self.current_song_index = Some(index);
        self.current_song = Some(song.clone());
        
        self.ensure_audio_initialized()?;
        
        if let Some(sink) = &self.sink {
            sink.stop();
        }
        
        if let Some((_, ref stream_handle)) = self.stream_handle {
            if let Ok(file) = std::fs::File::open(&song.path) {
                let reader = BufReader::new(file);
                if let Ok(decoder) = Decoder::new(reader) {
                    if let Some(sink) = &self.sink {
                        sink.append(decoder);
                        self.is_playing = true;
                        self.start_time = Some(std::time::Instant::now());
                        self.duration = song.duration;
                        self.seek_position = 0;
                        return Ok(());
                    }
                }
            }
            Err(format!("Failed to play: {}", song.path.display()))
        } else {
            Err("No audio stream".to_string())
        }
    }
    
    fn play_next(&mut self) -> Result<(), String> {
        if let Some(current_idx) = self.current_song_index {
            let next_idx = if current_idx + 1 < self.songs.len() { current_idx + 1 } else { 0 };
            self.play_song_at_index(next_idx)
        } else if !self.songs.is_empty() {
            self.play_song_at_index(0)
        } else {
            Err("No songs in library".to_string())
        }
    }
    
    fn play_previous(&mut self) -> Result<(), String> {
        if let Some(current_idx) = self.current_song_index {
            let prev_idx = if current_idx > 0 { current_idx - 1 } else { self.songs.len().saturating_sub(1) };
            if self.songs.is_empty() {
                return Err("No songs in library".to_string());
            }
            self.play_song_at_index(prev_idx)
        } else if !self.songs.is_empty() {
            self.play_song_at_index(0)
        } else {
            Err("No songs in library".to_string())
        }
    }
    
    fn set_songs(&mut self, songs: Vec<Song>) {
        self.songs = songs;
    }
    
    fn get_songs(&self) -> Vec<Song> {
        self.songs.clone()
    }
    
    fn add_songs(&mut self, new_songs: Vec<Song>) {
        self.songs.extend(new_songs);
    }
    
    fn get_elapsed(&self) -> u64 {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed().as_secs();
            if let Some(dur) = self.duration {
                if elapsed >= dur as u64 {
                    return dur;
                }
            }
            return elapsed;
        }
        0
    }
    
    fn get_duration(&self) -> u64 {
        self.duration.unwrap_or(0)
    }
    
    fn is_playing(&self) -> bool {
        self.is_playing
    }
    
    fn set_seek_position(&mut self, position: u64) {
        self.seek_position = position;
        self.start_time = Some(std::time::Instant::now() - std::time::Duration::from_secs(position));
    }
    
    fn get_seek_position(&self) -> u64 {
        self.seek_position
    }
}

fn format_time(seconds: u64) -> String {
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{}:{:02}", mins, secs)
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

    let header_bar = HeaderBar::builder()
        .title_widget(&WindowTitle::new("Arch Music Player", ""))
        .build();

    let add_btn = Button::builder()
        .label("➕ Add Folder")
        .build();
    
    let refresh_btn = Button::builder()
        .label("🔄 Refresh")
        .build();
    
    let settings_btn = Button::builder()
        .label("⚙️ Settings")
        .build();

    // Don't add buttons to header_box first - just pack directly into header bar
    header_bar.pack_start(&add_btn);
    header_bar.pack_start(&refresh_btn);
    header_bar.pack_start(&settings_btn);

    let status_label = Label::builder()
        .label("🎵 Ready - Scanning music library...")
        .halign(gtk4::Align::Center)
        .build();

    let tab_view = TabView::builder()
        .vexpand(true)
        .build();
    
    let _tab_bar = TabBar::builder()
        .view(&tab_view)
        .autohide(true)
        .build();

    let track_name_ref = Arc::new(Mutex::new(None::<gtk4::Label>));
    let track_artist_ref = Arc::new(Mutex::new(None::<gtk4::Label>));
    let play_btn_ref = Arc::new(Mutex::new(None::<gtk4::Button>));

    let all_songs_box = Box::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .build();
    
    let all_songs_scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();
    all_songs_scrolled.set_child(Some(&all_songs_box));
    
    let albums_box = Box::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .build();
    
    let albums_scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();
    albums_scrolled.set_child(Some(&albums_box));
    
    let artists_box = Box::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .build();
    
    let artists_scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();
    artists_scrolled.set_child(Some(&artists_box));
    
    let folders_box = Box::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .build();
    
    let folders_scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();
    folders_scrolled.set_child(Some(&folders_box));

    let _tab1 = tab_view.append(&all_songs_scrolled);
    let _tab2 = tab_view.append(&albums_scrolled);
    let _tab3 = tab_view.append(&artists_scrolled);
    let _tab4 = tab_view.append(&folders_scrolled);

    let (now_playing_box, np_track_name, np_track_artist, np_play_btn, np_prog, np_time1, np_time2, np_prev, np_next, np_vol_scale) = create_now_playing();

    let header_content = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();
    
    // Only append status_label since we removed header_box
    header_content.append(&status_label);
    header_content.append(&Separator::new(Orientation::Horizontal));

    main_box.append(&header_bar);
    main_box.append(&header_content);
    main_box.append(&tab_view);
    main_box.append(&now_playing_box);

    *track_name_ref.lock().unwrap() = Some(np_track_name.clone());
    *track_artist_ref.lock().unwrap() = Some(np_track_artist.clone());
    *play_btn_ref.lock().unwrap() = Some(np_play_btn.clone());
    
    // Progress update source ID for timer management - make it accessible
    let progress_source_id = Arc::new(std::cell::RefCell::new(None::<gtk4::glib::SourceId>));
    
    // Helper function to start progress timer
    let start_progress_timer = {
        let player_state = player_state.clone();
        let np_prog = np_prog.clone();
        let np_time1 = np_time1.clone();
        let np_time2 = np_time2.clone();
        let progress_source_id = progress_source_id.clone();
        
        move || {
            // Remove existing timer
            if let Some(source_id) = progress_source_id.borrow_mut().take() {
                source_id.remove();
            }
            
            let player_state = player_state.clone();
            let np_prog = np_prog.clone();
            let np_time1 = np_time1.clone();
            let np_time2 = np_time2.clone();
            
            let update_progress = move || -> glib::ControlFlow {
                let state = player_state.lock().unwrap();
                if state.is_playing() {
                    let elapsed = state.get_elapsed();
                    let duration = state.get_duration();
                    
                    drop(state);
                    
                    // Update labels
                    np_time1.set_text(&format_time(elapsed));
                    np_time2.set_text(&format_time(duration));
                    
                    // Update progress bar if we have duration
                    if duration > 0 {
                        let pct = (elapsed as f64 / duration as f64 * 100.0) as f64;
                        np_prog.set_value(pct);
                    }
                    
                    glib::ControlFlow::Continue
                } else {
                    glib::ControlFlow::Break
                }
            };
            
            let source_id = glib::timeout_add_local(std::time::Duration::from_millis(250), update_progress);
            *progress_source_id.borrow_mut() = Some(source_id);
        }
    };
    
    // Play/Pause button handler
    let start_progress_timer_for_play = start_progress_timer.clone();
    let player_state_for_play = player_state.clone();
    
    np_play_btn.connect_clicked(clone!(@weak player_state, @weak np_play_btn, @weak np_prog, @weak np_time1, @weak np_time2 => move |button| {
        let mut state = player_state.lock().unwrap();
        
        if state.is_playing {
            // Pause
            if let Some(sink) = &state.sink {
                sink.pause();
            }
            state.is_playing = false;
            button.set_label("▶");
        } else {
            // Play
            if let Some(sink) = &state.sink {
                if state.current_song.is_some() {
                    sink.play();
                    state.is_playing = true;
                    button.set_label("⏸");
                    
                    // Reset seek position if starting fresh
                    if state.seek_position == 0 {
                        state.start_time = Some(std::time::Instant::now());
                    } else {
                        state.start_time = Some(std::time::Instant::now() - std::time::Duration::from_secs(state.seek_position));
                    }
                    
                    // Start progress update timer
                    start_progress_timer_for_play();
                }
            }
        }
    }));
    
    // Progress bar seek handler - update time while dragging
    np_prog.connect_value_changed(clone!(@weak player_state, @weak np_time1, @weak np_time2 => move |scale| {
        let pct = scale.value();
        let duration = player_state.lock().unwrap().get_duration();
        
        // Calculate time based on position
        let total_seconds = if duration > 0 {
            (pct / 100.0 * duration as f64) as u64
        } else {
            0
        };
        
        np_time1.set_text(&format_time(total_seconds));
        np_time2.set_text(&format_time(duration));
        
        // Update seek position
        player_state.lock().unwrap().set_seek_position(total_seconds);
    }));
    
    // Previous button handler
    np_prev.connect_clicked(clone!(@weak player_state, @weak np_track_name, @weak np_track_artist, @weak np_play_btn => move |_| {
        let mut state = player_state.lock().unwrap();
        
        if let Err(e) = state.play_previous() {
            eprintln!("Failed to play previous: {}", e);
            return;
        }
        
        if let Some(ref song) = state.current_song {
            np_track_name.set_text(&song.title);
            np_track_artist.set_text(&song.artist);
            np_play_btn.set_label("⏸");
        }
    }));
    
    // Next button handler
    np_next.connect_clicked(clone!(@weak player_state, @weak np_track_name, @weak np_track_artist, @weak np_play_btn => move |_| {
        let mut state = player_state.lock().unwrap();
        
        if let Err(e) = state.play_next() {
            eprintln!("Failed to play next: {}", e);
            return;
        }
        
        if let Some(ref song) = state.current_song {
            np_track_name.set_text(&song.title);
            np_track_artist.set_text(&song.artist);
            np_play_btn.set_label("⏸");
        }
    }));
    
    // Volume handler
    let player_state_for_vol = player_state.clone();
    np_vol_scale.connect_value_changed(clone!(@weak np_vol_scale => move |scale| {
        let volume = scale.value() as f32;
        if let Ok(mut state) = player_state_for_vol.lock() {
            state.set_volume(volume);
        }
        
        // Update volume icon
        let icon = if volume == 0.0 {
            "🔇"
        } else if volume < 0.5 {
            "🔉"
        } else {
            "🔊"
        };
        // Note: We'd need to store the icon label to update it
    }));
    
    // Progress bar seek handler
    let player_state_for_seek = player_state.clone();
    np_prog.connect_value_changed(clone!(@weak np_prog, @weak np_time1, @weak np_time2 => move |scale| {
        let pct = scale.value();
        let duration = player_state_for_seek.lock().unwrap().get_duration();
        if duration > 0 {
            let elapsed = (pct / 100.0 * duration as f64) as u64;
            np_time1.set_text(&format_time(elapsed));
            np_time2.set_text(&format_time(duration));
        }
    }));

    // Add Folder handler
    add_btn.connect_clicked(clone!(@weak status_label, @weak window, @weak track_name_ref, @weak track_artist_ref, @weak play_btn_ref, @weak player_state, @weak all_songs_box, @weak albums_box, @weak artists_box, @weak folders_box => move |_| {
        let dialog = FileChooserDialog::new(
            Some("Select Music Folder"),
            Some(&window),
            FileChooserAction::SelectFolder,
            &[
                ("Cancel", gtk4::ResponseType::Cancel),
                ("Select", gtk4::ResponseType::Accept),
            ],
        );
        
        dialog.add_button("Cancel", gtk4::ResponseType::Cancel);
        dialog.add_button("Select", gtk4::ResponseType::Accept);
        
        let status_label = status_label.clone();
        let track_name_ref = track_name_ref.clone();
        let track_artist_ref = track_artist_ref.clone();
        let play_btn_ref = play_btn_ref.clone();
        let player_state = player_state.clone();
        
        dialog.connect_response(move |dialog, response| {
            if response == gtk4::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        status_label.set_label(&format!("🔍 Scanning: {}...", path.display()));
                        
                        let status_label = status_label.clone();
                        let scan_path = path.clone();
                        let track_name_ref = track_name_ref.clone();
                        let track_artist_ref = track_artist_ref.clone();
                        let play_btn_ref = play_btn_ref.clone();
                        let player_state = player_state.clone();
                        let all_songs_box = all_songs_box.clone();
                        let albums_box = albums_box.clone();
                        let artists_box = artists_box.clone();
                        let folders_box = folders_box.clone();
                        
                        glib::MainContext::ref_thread_default().spawn_local(async move {
                            let extensions = ["mp3", "flac", "ogg", "m4a", "wav", "aac", "wma"];
                            let mut new_songs = Vec::new();
                            scan_directory_recursive(&scan_path, &mut new_songs, &extensions);
                            let count = new_songs.len();
                            println!("🎵 Found {} music files in selected folder!", count);
                            
                            {
                                let mut state = player_state.lock().unwrap();
                                state.add_songs(new_songs.clone());
                            }
                            
                            status_label.set_label(&format!("📁 Added {} songs from selected folder", count));
                            
                            let songs = player_state.lock().unwrap().get_songs();
                            
                            let track_name_clone = track_name_ref.lock().unwrap().clone();
                            let track_artist_clone = track_artist_ref.lock().unwrap().clone();
                            let play_btn_clone = play_btn_ref.lock().unwrap().clone();
                            
                            if let (Some(track_name_label), Some(track_artist_label), Some(play_btn_inner)) = (track_name_clone, track_artist_clone, play_btn_clone) {
                                populate_all_tabs(&all_songs_box, &albums_box, &artists_box, &folders_box, &player_state, &songs, &track_name_label, &track_artist_label, &play_btn_inner);
                            }
                        });
                    }
                }
            }
            dialog.close();
        });
        
        dialog.present();
    }));

    // Refresh handler
    let all_songs_box_for_refresh = all_songs_box.clone();
    let albums_box_for_refresh = albums_box.clone();
    let artists_box_for_refresh = artists_box.clone();
    let folders_box_for_refresh = folders_box.clone();
    let player_state_for_refresh = player_state.clone();
    let track_name_for_refresh = track_name_ref.clone();
    let track_artist_for_refresh = track_artist_ref.clone();
    let play_btn_for_refresh = play_btn_ref.clone();
    
    refresh_btn.connect_clicked(clone!(@weak status_label, @weak track_name_ref, @weak track_artist_ref, @weak play_btn_ref, @weak player_state, @weak all_songs_box, @weak albums_box, @weak artists_box, @weak folders_box => move |_| {
        status_label.set_label("🔄 Scanning music library...");
        
        let status_label = status_label.clone();
        let track_name_ref = track_name_ref.clone();
        let track_artist_ref = track_artist_ref.clone();
        let play_btn_ref = play_btn_ref.clone();
        let player_state = player_state.clone();
        let all_songs_box = all_songs_box.clone();
        let albums_box = albums_box.clone();
        let artists_box = artists_box.clone();
        let folders_box = folders_box.clone();
        
        glib::MainContext::ref_thread_default().spawn_local(async move {
            let songs = scan_music_library();
            println!("🎵 Found {} music files!", songs.len());
            
            let count = songs.len();
            
            {
                let mut state = player_state.lock().unwrap();
                state.set_songs(songs.clone());
            }
            
            status_label.set_label(&format!("📚 Found {} music files", count));
            
            let track_name_inner = track_name_ref.lock().unwrap().clone();
            let track_artist_inner = track_artist_ref.lock().unwrap().clone();
            
            if let (Some(track_name_label), Some(track_artist_label)) = (track_name_inner, track_artist_inner) {
                let play_btn_inner_opt = play_btn_ref.lock().unwrap().clone();
                if let Some(play_btn_inner) = play_btn_inner_opt {
                    populate_all_tabs(&all_songs_box, &albums_box, &artists_box, &folders_box, &player_state, &songs, &track_name_label, &track_artist_label, &play_btn_inner);
                }
            }
        });
    }));

    settings_btn.connect_clicked(clone!(@weak window => move |_| {
        let dialog = gtk4::MessageDialog::new(
            Some(&window),
            gtk4::DialogFlags::MODAL,
            gtk4::MessageType::Info,
            gtk4::ButtonsType::Close,
            "Settings\n\nConfigure your music player settings here.\n\n- Music directories\n- Audio playback\n- Theme preferences",
        );
        dialog.present();
    }));

    window.set_title(Some("Arch Music Player"));
    window.set_child(Some(&main_box));
    window.present();

    let status_label = status_label.clone();
    let player_state = player_state.clone();
    let play_btn_ref = play_btn_ref.clone();
    let all_songs_box = all_songs_box.clone();
    let albums_box = albums_box.clone();
    let artists_box = artists_box.clone();
    let folders_box = folders_box.clone();
    
    // Get the actual labels from the Arc<Mutex> before spawning
    let track_name_for_init = track_name_ref.lock().unwrap().clone();
    let track_artist_for_init = track_artist_ref.lock().unwrap().clone();
    let play_btn_for_init = play_btn_ref.lock().unwrap().clone();
    
    eprintln!("DEBUG: Labels extracted from Arc<Mutex>");
    
    if let (Some(track_name), Some(track_artist), Some(play_btn)) = (track_name_for_init, track_artist_for_init, play_btn_for_init) {
        let track_name_clone = track_name.clone();
        let track_artist_clone = track_artist.clone();
        let play_btn_clone = play_btn.clone();
        
        eprintln!("DEBUG: All labels present, spawning async task");
        
        glib::MainContext::ref_thread_default().spawn_local(clone!(@weak status_label, @weak player_state, @weak all_songs_box, @weak albums_box, @weak artists_box, @weak folders_box => async move {
            eprintln!("DEBUG: Async task started");
            
            let songs = scan_music_library();
            eprintln!("DEBUG: Scan complete, found {} songs", songs.len());
            
            {
                let mut state = player_state.lock().unwrap();
                state.set_songs(songs.clone());
            }
            
            status_label.set_label(&format!("📚 Found {} music files", songs.len()));
            eprintln!("DEBUG: Calling populate_all_tabs");
            
            populate_all_tabs(&all_songs_box, &albums_box, &artists_box, &folders_box, &player_state, &songs, &track_name_clone, &track_artist_clone, &play_btn_clone);
            eprintln!("DEBUG: populate_all_tabs complete");
        }));
    } else {
        eprintln!("DEBUG: ERROR - Labels were None!");
    }
}

fn populate_all_tabs(
    all_songs_box: &gtk4::Box,
    albums_box: &gtk4::Box,
    artists_box: &gtk4::Box,
    folders_box: &gtk4::Box,
    player_state: &Arc<Mutex<PlayerState>>,
    songs: &[Song],
    track_name: &gtk4::Label,
    track_artist: &gtk4::Label,
    play_btn_ref: &gtk4::Button,
) {
    eprintln!("DEBUG populate_all_tabs: Called with {} songs", songs.len());
    
    if songs.is_empty() {
        eprintln!("DEBUG populate_all_tabs: Songs is empty, returning early");
        return;
    }
    
    eprintln!("DEBUG populate_all_tabs: Processing {} songs", songs.len());
    
    while let Some(child) = all_songs_box.first_child() {
        all_songs_box.remove(&child);
    }
    while let Some(child) = albums_box.first_child() {
        albums_box.remove(&child);
    }
    while let Some(child) = artists_box.first_child() {
        artists_box.remove(&child);
    }
    while let Some(child) = folders_box.first_child() {
        folders_box.remove(&child);
    }
    
    for song in songs {
        let play_btn_for_row = play_btn_ref.clone();
        let track_name_for_row = track_name.clone();
        let track_artist_for_row = track_artist.clone();
        let progress_source_id_for_row = progress_source_id.clone();
        let player_state_for_row = player_state.clone();
        
        // Get UI elements for timer from the passed references
        let np_prog_for_row = track_name.clone(); // These are just placeholders for the clone macro
        let np_time1_for_row = track_name.clone();
        let np_time2_for_row = track_name.clone();
        
        let row = create_song_row(song, player_state, &track_name_for_row, &track_artist_for_row, &play_btn_for_row, &progress_source_id_for_row, &np_prog_for_row, &np_time1_for_row, &np_time2_for_row);
        all_songs_box.append(&row);
    }
    
    let mut albums: std::collections::HashMap<(String, String), String> = std::collections::HashMap::new();
    for song in songs {
        let key = (song.album.clone(), song.artist.clone());
        if !albums.contains_key(&key) {
            albums.insert(key.clone(), song.title.clone());
        }
    }
    
    let albums_grid = Grid::builder()
        .column_spacing(15)
        .row_spacing(15)
        .build();
    
    let albums_vec: Vec<_> = albums.into_iter().collect();
    for (i, ((album, artist), sample_title)) in albums_vec.iter().enumerate().take(24) {
        let card = create_album_card(album, artist, "🎸", sample_title);
        let row = (i / 4) as i32;
        let col = (i % 4) as i32;
        albums_grid.attach(&card, col, row, 1, 1);
    }
    
    let albums_scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .build();
    albums_scrolled.set_child(Some(&albums_grid));
    albums_box.append(&albums_scrolled);
    
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

        artists_box.append(&row);
    }
    
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
    
    let folders_scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .build();
    folders_scrolled.set_child(Some(&tree));
    folders_box.append(&folders_scrolled);
}

fn scan_music_library() -> Vec<Song> {
    let mut songs = Vec::new();
    
    let extensions = ["mp3", "flac", "ogg", "m4a", "wav", "aac", "wma"];
    
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
        let path = PathBuf::from(dir);
        println!("Checking directory: {}", path.display());
        if path.exists() {
            println!("  Directory exists, scanning...");
            scan_directory_recursive(&path, &mut songs, &extensions);
        } else {
            println!("  Directory does not exist");
        }
    }

    let system_dirs = ["/usr/share/music", "/var/music"];
    for dir in &system_dirs {
        let path = PathBuf::from(dir);
        if path.exists() {
            scan_directory_recursive(&path, &mut songs, &extensions);
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

fn create_song_row(song: &Song, player_state: &Arc<Mutex<PlayerState>>, track_name: &gtk4::Label, track_artist: &gtk4::Label, play_btn: &gtk4::Button, progress_source_id: &Arc<std::cell::RefCell<Option<gtk4::glib::SourceId>>>) -> gtk4::Widget {
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

    info.append(&title);
    info.append(&artist_album);

    let play_icon = Label::new(Some("▶"));
    
    row.append(&icon);
    row.append(&info);
    row.append(&play_icon);

    let song_clone = song.clone();
    let player_state = player_state.clone();
    let track_name = track_name.clone();
    let track_artist = track_artist.clone();
    let play_btn = play_btn.clone();
    
    let event_controller = gtk4::GestureClick::new();
    let start_progress_timer = start_progress_timer.clone();
    event_controller.connect_pressed(clone!(@weak player_state, @weak track_name, @weak track_artist, @weak play_btn => move |_, _, _, _| {
        println!("🎵 Playing: {} - {}", song_clone.title, song_clone.artist);
        
        track_name.set_text(&song_clone.title);
        track_artist.set_text(&song_clone.artist);
        
        let mut state = player_state.lock().unwrap();
        
        if let Err(e) = state.ensure_audio_initialized() {
            eprintln!("Failed to initialize audio: {}", e);
            return;
        }
        
        if let Some(sink) = &state.sink {
            sink.stop();
        }
        
        if let Some((_, ref _stream_handle)) = state.stream_handle {
            if let Ok(file) = std::fs::File::open(&song_clone.path) {
                let reader = BufReader::new(file);
                if let Ok(decoder) = Decoder::new(reader) {
                    if let Some(sink) = &state.sink {
                        sink.append(decoder);
                        state.current_song = Some(song_clone.clone());
                        state.is_playing = true;
                        state.current_song_index = Some(song_clone.title.parse().unwrap_or(0));
                        state.start_time = Some(std::time::Instant::now());
                        state.seek_position = 0;
                        
                        play_btn.set_label("⏸");
                        
                        // Start progress update timer
                        drop(state);
                        start_progress_timer();
                        
                        println!("✅ Now playing: {}", song_clone.title);
                    }
                } else {
                    eprintln!("Failed to decode audio file: {}", song_clone.path.display());
                }
            } else {
                eprintln!("Failed to open audio file: {}", song_clone.path.display());
            }
        }
    }));
    
    row.add_controller(event_controller);

    row.upcast()
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

fn create_now_playing() -> (gtk4::Box, gtk4::Label, gtk4::Label, gtk4::Button, gtk4::Scale, gtk4::Label, gtk4::Label, gtk4::Button, gtk4::Button, gtk4::Scale) {
    let container = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();

    let progress_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .margin_start(16)
        .margin_end(16)
        .margin_top(8)
        .build();

    let time1 = Label::new(Some("0:00"));
    time1.add_css_class("time-label");
    time1.set_width_request(50);

    let prog = Scale::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .draw_value(false)
        .build();
    prog.set_range(0.0, 100.0);

    let time2 = Label::new(Some("0:00"));
    time2.add_css_class("time-label");
    time2.set_width_request(50);

    progress_box.append(&time1);
    progress_box.append(&prog);
    progress_box.append(&time2);

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
    track_name.set_halign(gtk4::Align::Start);

    let track_artist = Label::new(Some("Select a song to play"));
    track_artist.add_css_class("track-artist");
    track_artist.set_halign(gtk4::Align::Start);

    track_info.append(&track_name);
    track_info.append(&track_artist);

    let playback = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .build();

    let prev = Button::builder().label("⏮").build();
    prev.add_css_class("control-btn");

    let play_btn = Button::builder().label("▶").build();
    play_btn.add_css_class("control-btn");

    let next = Button::builder().label("⏭").build();
    next.add_css_class("control-btn");

    playback.append(&prev);
    playback.append(&play_btn);
    playback.append(&next);

    let volume = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .build();

    let vol_icon = Label::new(Some("🔊"));
    
    let vol_scale = Scale::builder()
        .orientation(Orientation::Horizontal)
        .width_request(100)
        .draw_value(false)
        .build();
    vol_scale.set_range(0.0, 1.0);
    vol_scale.set_value(1.0);

    volume.append(&vol_icon);
    volume.append(&vol_scale);

    let main = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(20)
        .vexpand(true)
        .valign(gtk4::Align::Center)
        .build();

    main.append(&art);
    main.append(&track_info);
    main.append(&playback);
    main.append(&volume);

    container.append(&progress_box);
    container.append(&main);

    let sep = Separator::new(Orientation::Horizontal);
    container.prepend(&sep);

    (container, track_name, track_artist, play_btn, prog, time1, time2, prev, next, vol_scale)
}
