use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Label, Box, Scale, Adjustment, Orientation, Image, Frame, Separator, EventBox, FlowBox, ListBox, TreeView, TreeViewColumn, TreeStore, CellRendererText, SelectionMode};
use libadwaita::{Application as AdwApplication, HeaderBar, WindowTitle, TabView, TabBar, TabPage};

fn main() {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Arch Music Player")
        .default_width(1200)
        .default_height(800)
        .build();

    // Create main container with proper styling
    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();

    // Header bar with window title
    let header_bar = HeaderBar::builder()
        .title_widget(&WindowTitle::new("Arch Music Player", ""))
        .build();

    // Create tab bar for navigation
    let tab_view = TabView::builder()
        .vexpand(true)
        .build();
    
    let tab_bar = TabBar::builder()
        .view(&tab_view)
        .autohide(true)
        .build();

    // Create tabs with proper content
    let albums_page = create_albums_tab();
    let artists_page = create_artists_tab();
    let folders_page = create_folders_tab();
    let playlists_page = create_playlists_tab();
    
    // Add tabs to view
    let _tab1 = tab_view.append(&albums_page);
    let _tab2 = tab_view.append(&artists_page);
    let _tab3 = tab_view.append(&folders_page);
    let _tab4 = tab_view.append(&playlists_page);

    // Now playing section at bottom
    let now_playing = create_now_playing_section();
    
    // Assemble main layout
    main_box.append(&tab_bar);
    main_box.append(&tab_view);
    main_box.append(&now_playing);

    // Setup window
    window.set_title(Some("Arch Music Player"));
    window.set_child(Some(&main_box));
    window.present();
}

fn create_albums_tab() -> gtk4::Widget {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();

    let flow_box = FlowBox::builder()
        .selection_mode(SelectionMode::Single)
        .column_spacing(20)
        .row_spacing(20)
        .homogeneous(true)
        .build();

    // Create album items with professional look
    let albums = vec![
        ("Queen", "A Night at the Opera", "1975", "🎸"),
        ("Pink Floyd", "The Dark Side of the Moon", "1973", "🌙"),
        ("Led Zeppelin", "IV", "1971", "🏔️"),
        ("The Beatles", "Abbey Road", "1969", "🚶"),
        ("Daft Punk", "Random Access Memories", "2013", "🤖"),
        ("Radiohead", "OK Computer", "1997", "💻"),
        ("Nirvana", "Nevermind", "1991", "👶"),
        ("Arctic Monkeys", "AM", "2013", "🏎️"),
        ("Tame Impala", "Currents", "2015", "🌊"),
        ("Pink Floyd", "Wish You Were Here", "1975", "👋"),
        ("Queen", "Greatest Hits", "1981", "🎯"),
        ("The Beatles", "Sgt. Pepper's", "1967", "🌈"),
    ];

    for (i, (artist, album, year, icon)) in albums.iter().enumerate() {
        let album_item = create_album_card(artist, album, year, icon);
        flow_box.append(&album_item);
    }

    scrolled.set_child(Some(&flow_box));
    scrolled.upcast()
}

fn create_album_card(artist: &str, album: &str, year: &str, icon: &str) -> gtk4::Widget {
    let card = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .css_classes(vec!["album-card"])
        .build();

    // Album art placeholder with proper size
    let art_frame = Frame::builder()
        .css_classes(vec!["album-art"])
        .build();

    let art_icon = Label::builder()
        .label(icon)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .build();
    art_icon.set_size_request(150, 150);
    
    art_frame.set_child(Some(&art_icon));

    // Album info
    let album_label = Label::builder()
        .label(album)
        .halign(gtk4::Align::Center)
        .css_classes(vec!["album-title"])
        .build();

    let artist_label = Label::builder()
        .label(artist)
        .halign(gtk4::Align::Center)
        .css_classes(vec!["album-artist"])
        .build();

    let year_label = Label::builder()
        .label(year)
        .halign(gtk4::Align::Center)
        .css_classes(vec!["album-year"])
        .build();

    card.append(&art_frame);
    card.append(&album_label);
    card.append(&artist_label);
    card.append(&year_label);

    card.upcast()
}

fn create_artists_tab() -> gtk4::Widget {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();

    let list_box = ListBox::builder()
        .selection_mode(SelectionMode::Single)
        .build();

    let artists = vec![
        ("🎸 Queen", "12 albums • 84 songs", "British rock band formed in London"),
        ("🌙 Pink Floyd", "15 albums • 92 songs", "English rock band formed in London"),
        ("🏔️ Led Zeppelin", "9 albums • 62 songs", "English rock band formed in London"),
        ("🚶 The Beatles", "13 albums • 213 songs", "English rock band formed in Liverpool"),
        ("🤖 Daft Punk", "4 albums • 48 songs", "French electronic music duo"),
        ("💻 Radiohead", "9 albums • 84 songs", "English rock band formed in Abingdon"),
        ("👶 Nirvana", "3 albums • 56 songs", "American rock band formed in Aberdeen"),
        ("🏎️ Arctic Monkeys", "7 albums • 78 songs", "English indie rock band"),
        ("🌊 Tame Impala", "4 albums • 42 songs", "Australian psychedelic music project"),
    ];

    for (artist, albums, desc) in &artists {
        let row = create_artist_row(artist, albums, desc);
        list_box.append(&row);
    }

    scrolled.set_child(Some(&list_box));
    scrolled.upcast()
}

fn create_artist_row(artist: &str, albums: &str, desc: &str) -> gtk4::Widget {
    let row = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(16)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(16)
        .margin_end(16)
        .css_classes(vec!["artist-row"])
        .build();

    let avatar = Label::builder()
        .label(artist.split_whitespace().next().unwrap_or(""))
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .build();
    avatar.set_size_request(64, 64);
    avatar.add_css_class("artist-avatar");

    let info_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .hexpand(true)
        .build();

    let name_label = Label::builder()
        .label(artist)
        .halign(gtk4::Align::Start)
        .css_classes(vec!["artist-name"])
        .build();

    let albums_label = Label::builder()
        .label(albums)
        .halign(gtk4::Align::Start)
        .css_classes(vec!["artist-albums"])
        .build();

    let desc_label = Label::builder()
        .label(desc)
        .halign(gtk4::Align::Start)
        .css_classes(vec!["artist-desc"])
        .build();

    info_box.append(&name_label);
    info_box.append(&albums_label);
    info_box.append(&desc_label);

    let chevron = Label::builder()
        .label("▶")
        .halign(gtk4::Align::End)
        .build();

    row.append(&avatar);
    row.append(&info_box);
    row.append(&chevron);

    row.upcast()
}

fn create_folders_tab() -> gtk4::Widget {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();

    let tree_view = TreeView::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    // Create tree model
    let tree_store = TreeStore::new(&[String::static_type()]);
    let root = tree_store.append(None);
    tree_store.set(&root, &[0], &[&"📁 Music Library"]);
    
    let music = tree_store.append(Some(&root));
    tree_store.set(&music, &[0], &[&"🎵 ~/Music"]);
    
    let docs = tree_store.append(Some(&root));
    tree_store.set(&docs, &[0], &[&"📁 ~/Documents/Music"]);
    
    let downloads = tree_store.append(Some(&root));
    tree_store.set(&downloads, &[0], &[&"📥 ~/Downloads/Music"]);
    
    let system = tree_store.append(None);
    tree_store.set(&system, &[0], &[&"📂 System Folders"]);
    
    let usr = tree_store.append(Some(&system));
    tree_store.set(&usr, &[0], &[&"💾 /usr/share/music"]);
    
    let var = tree_store.append(Some(&system));
    tree_store.set(&var, &[0], &[&"💾 /var/music"]);

    let col = TreeViewColumn::builder()
        .title("Folders")
        .build();
    
    let renderer = CellRendererText::builder().build();
    col.pack_start(&renderer, true);
    col.add_attribute(&renderer, "text", 0);
    
    tree_view.append_column(&col);
    tree_view.set_model(Some(&tree_store));

    scrolled.set_child(Some(&tree_view));
    scrolled.upcast()
}

fn create_playlists_tab() -> gtk4::Widget {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();

    let list_box = ListBox::builder()
        .selection_mode(SelectionMode::Single)
        .build();

    let playlists = vec![
        ("❤️ Favorites", "128 songs • 8h 32m", "⭐"),
        ("🎸 Rock Classics", "64 songs • 4h 15m", "🎵"),
        ("🎧 Chill Vibes", "42 songs • 2h 48m", "🌙"),
        ("🏃 Workout Mix", "32 songs • 1h 45m", "💪"),
        ("🎤 Sing-Along", "56 songs • 3h 12m", "🎶"),
        ("🎹 Instrumental", "28 songs • 2h 5m", "🎼"),
        ("🎃 Halloween Party", "24 songs • 1h 28m", "🎃"),
        ("🎄 Christmas Hits", "48 songs • 2h 56m", "🎅"),
    ];

    for (name, info, icon) in &playlists {
        let row = create_playlist_row(name, info, icon);
        list_box.append(&row);
    }

    scrolled.set_child(Some(&list_box));
    scrolled.upcast()
}

fn create_playlist_row(name: &str, info: &str, icon: &str) -> gtk4::Widget {
    let row = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(16)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(16)
        .margin_end(16)
        .build();

    let icon_label = Label::builder()
        .label(icon)
        .halign(gtk4::Align::Center)
        .build();
    icon_label.set_size_request(48, 48);

    let info_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .hexpand(true)
        .build();

    let name_label = Label::builder()
        .label(name)
        .halign(gtk4::Align::Start)
        .css_classes(vec!["playlist-name"])
        .build();

    let info_label = Label::builder()
        .label(info)
        .halign(gtk4::Align::Start)
        .css_classes(vec!["playlist-info"])
        .build();

    info_box.append(&name_label);
    info_box.append(&info_label);

    let menu_btn = Button::builder()
        .label("⋮")
        .halign(gtk4::Align::End)
        .build();

    row.append(&icon_label);
    row.append(&info_box);
    row.append(&menu_btn);

    row.upcast()
}

fn create_now_playing_section() -> gtk4::Widget {
    let container = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .css_classes(vec!["now-playing"])
        .build();

    // Progress bar section
    let progress_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .margin_start(16)
        .margin_end(16)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let time_current = Label::builder()
        .label("2:15")
        .css_classes(vec!["time-label"])
        .build();

    let progress = Scale::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .draw_value(false)
        .adjustment(&Adjustment::new(35.0, 0.0, 100.0, 1.0, None))
        .build();
    progress.add_css_class("progress-bar");

    let time_total = Label::builder()
        .label("5:55")
        .css_classes(vec!["time-label"])
        .build();

    progress_box.append(&time_current);
    progress_box.append(&progress);
    progress_box.append(&time_total);

    // Main controls section
    let controls_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(16)
        .margin_start(16)
        .margin_end(16)
        .margin_top(8)
        .margin_bottom(16)
        .halign(gtk4::Align::Center)
        .build();

    // Album art in now playing
    let art_display = Label::builder()
        .label("🎸")
        .halign(gtk4::Align::Start)
        .build();
    art_display.set_size_request(80, 80);
    art_display.add_css_class("now-playing-art");

    // Track info
    let track_info = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .hexpand(true)
        .build();

    let track_name = Label::builder()
        .label("Bohemian Rhapsody")
        .halign(gtk4::Align::Start)
        .css_classes(vec!["track-name"])
        .build();

    let track_artist = Label::builder()
        .label("Queen")
        .halign(gtk4::Align::Start)
        .css_classes(vec!["track-artist"])
        .build();

    track_info.append(&track_name);
    track_info.append(&track_artist);

    // Playback controls
    let playback_controls = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .build();

    let btn_style = "control-button";

    let prev_btn = Button::builder()
        .label("⏮")
        .css_classes(vec![btn_style])
        .build();

    let play_btn = Button::builder()
        .label("▶")
        .css_classes(vec![btn_style, "play-button"])
        .build();

    let next_btn = Button::builder()
        .label("⏭")
        .css_classes(vec![btn_style])
        .build();

    playback_controls.append(&prev_btn);
    playback_controls.append(&play_btn);
    playback_controls.append(&next_btn);

    // Volume control
    let volume_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .build();

    let vol_icon = Label::builder()
        .label("🔊")
        .build();

    let volume = Scale::builder()
        .orientation(Orientation::Horizontal)
        .width_request(100)
        .draw_value(false)
        .adjustment(&Adjustment::new(70.0, 0.0, 100.0, 1.0, None))
        .build();
    volume.add_css_class("volume-slider");

    volume_box.append(&vol_icon);
    volume_box.append(&volume);

    // Assemble controls
    let main_controls = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(24)
        .hexpand(true)
        .build();

    main_controls.append(&art_display);
    main_controls.append(&track_info);
    main_controls.append(&playback_controls);
    main_controls.append(&volume_box);

    container.append(&progress_box);
    container.append(&main_controls);

    // Add separator at top
    let sep = Separator::new(Orientation::Horizontal);
    container.prepend(&sep);

    container.upcast()
}