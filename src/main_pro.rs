use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Button, Label, Box, Scale, Orientation, Frame, ListBox, TreeView, TreeViewColumn, TreeStore, CellRendererText, ScrolledWindow, Grid, Separator, SelectionMode};
use libadwaita::{Application as AdwApplication, HeaderBar, WindowTitle, TabView, TabBar};

fn main() {
    let app = AdwApplication::builder()
        .application_id("com.archlinux.musicplayer")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Arch Music Player")
            .default_width(1100)
            .default_height(750)
            .build();

        // Main container
        let main_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .build();

        // Header
        let header_bar = HeaderBar::builder()
            .title_widget(&WindowTitle::new("Arch Music Player", ""))
            .build();

        // Tab navigation
        let tab_view = TabView::builder()
            .vexpand(true)
            .build();
        
        let _tab_bar = TabBar::builder()
            .view(&tab_view)
            .autohide(true)
            .build();

        // Create tab content
        let albums_content = create_albums_grid();
        let artists_content = create_artists_list();
        let folders_content = create_folders_tree();
        let playlists_content = create_playlists_list();

        // Add tabs
        let _tab1 = tab_view.append(&albums_content);
        let _tab2 = tab_view.append(&artists_content);
        let _tab3 = tab_view.append(&folders_content);
        let _tab4 = tab_view.append(&playlists_content);

        // Now Playing bar at bottom
        let now_playing = create_now_playing();

        // Assemble
        main_box.append(&header_bar);
        main_box.append(&tab_view);
        main_box.append(&now_playing);

        window.set_title(Some("Arch Music Player"));
        window.set_child(Some(&main_box));
        window.present();
    });

    app.run();
}

fn create_albums_grid() -> gtk4::Widget {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();

    let grid = Grid::builder()
        .column_spacing(12)
        .row_spacing(12)
        .build();

    let albums = vec![
        ("A Night at the Opera", "Queen", "1975", "🎸"),
        ("The Dark Side of the Moon", "Pink Floyd", "1973", "🌙"),
        ("Led Zeppelin IV", "Led Zeppelin", "1971", "🏔️"),
        ("Abbey Road", "The Beatles", "1969", "🚶"),
        ("Random Access Memories", "Daft Punk", "2013", "🤖"),
        ("OK Computer", "Radiohead", "1997", "💻"),
        ("Nevermind", "Nirvana", "1991", "👶"),
        ("AM", "Arctic Monkeys", "2013", "🏎️"),
        ("Currents", "Tame Impala", "2015", "🌊"),
        ("Wish You Were Here", "Pink Floyd", "1975", "👋"),
        ("Greatest Hits", "Queen", "1981", "🎯"),
        ("Sgt. Pepper's", "The Beatles", "1967", "🌈"),
    ];

    for (i, (album, artist, year, icon)) in albums.iter().enumerate() {
        let card = create_album_card(album, artist, year, icon);
        let row = (i / 4) as i32;
        let col = (i % 4) as i32;
        grid.attach(&card, col, row, 1, 1);
    }

    scrolled.set_child(Some(&grid));
    scrolled.upcast()
}

fn create_album_card(album: &str, artist: &str, year: &str, icon: &str) -> gtk4::Widget {
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

    let year_lbl = Label::new(Some(year));
    year_lbl.set_halign(gtk4::Align::Center);
    year_lbl.add_css_class("album-year");

    card.append(&art_frame);
    card.append(&album_lbl);
    card.append(&artist_lbl);
    card.append(&year_lbl);

    card.upcast()
}

fn create_artists_list() -> gtk4::Widget {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();

    let list = ListBox::builder()
        .selection_mode(SelectionMode::Single)
        .build();

    let artists = vec![
        ("🎸 Queen", "12 albums • 84 songs", "British rock legends"),
        ("🌙 Pink Floyd", "15 albums • 92 songs", "Psychedelic pioneers"),
        ("🏔️ Led Zeppelin", "9 albums • 62 songs", "Hard rock pioneers"),
        ("🚶 The Beatles", "13 albums • 213 songs", "The Fab Four"),
        ("🤖 Daft Punk", "4 albums • 48 songs", "French electronic duo"),
        ("💻 Radiohead", "9 albums • 84 songs", "Alternative rock icons"),
        ("👶 Nirvana", "3 albums • 56 songs", "Grunge revolution"),
        ("🏎️ Arctic Monkeys", "7 albums • 78 songs", "Modern indie rock"),
        ("🌊 Tame Impala", "4 albums • 42 songs", "Psychedelic master"),
    ];

    for (name, albums, desc) in artists {
        let row = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(12)
            .margin_end(12)
            .build();

        let avatar = Label::new(Some(name.split_whitespace().next().unwrap_or("")));
        avatar.set_size_request(56, 56);
        avatar.add_css_class("artist-avatar");

        let info = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .build();

        let name_lbl = Label::new(Some(name));
        name_lbl.set_halign(gtk4::Align::Start);
        name_lbl.add_css_class("artist-name");

        let albums_lbl = Label::new(Some(albums));
        albums_lbl.set_halign(gtk4::Align::Start);
        albums_lbl.add_css_class("artist-albums");

        let desc_lbl = Label::new(Some(desc));
        desc_lbl.set_halign(gtk4::Align::Start);
        desc_lbl.add_css_class("artist-desc");

        info.append(&name_lbl);
        info.append(&albums_lbl);
        info.append(&desc_lbl);

        row.append(&avatar);
        row.append(&info);
        row.append(&Label::new(Some("▶")));

        list.append(&row);
    }

    scrolled.set_child(Some(&list));
    scrolled.upcast()
}

fn create_folders_tree() -> gtk4::Widget {
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
    
    let music = store.append(Some(&root));
    store.set(&music, &[(0, &"🎵 ~/Music")]);
    
    let docs = store.append(Some(&root));
    store.set(&docs, &[(0, &"📁 ~/Documents/Music")]);
    
    let downloads = store.append(Some(&root));
    store.set(&downloads, &[(0, &"📥 ~/Downloads/Music")]);
    
    let sys = store.append(None);
    store.set(&sys, &[(0, &"💾 System Folders")]);
    
    let usr = store.append(Some(&sys));
    store.set(&usr, &[(0, &"📂 /usr/share/music")]);
    
    let var = store.append(Some(&sys));
    store.set(&var, &[(0, &"📂 /var/music")]);

    let col = TreeViewColumn::builder().title("Folders").build();
    let renderer = CellRendererText::builder().build();
    col.pack_start(&renderer, true);
    col.add_attribute(&renderer, "text", 0);
    
    tree.append_column(&col);
    tree.set_model(Some(&store));

    scrolled.set_child(Some(&tree));
    scrolled.upcast()
}

fn create_playlists_list() -> gtk4::Widget {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .build();

    let list = ListBox::builder()
        .selection_mode(SelectionMode::Single)
        .build();

    let playlists = vec![
        ("❤️ Favorites", "128 songs • 8h 32m", "⭐"),
        ("🎸 Rock Classics", "64 songs • 4h 15m", "🎵"),
        ("🌙 Chill Vibes", "42 songs • 2h 48m", "🌙"),
        ("💪 Workout Mix", "32 songs • 1h 45m", "💪"),
        ("🎤 Sing-Along", "56 songs • 3h 12m", "🎶"),
        ("🎹 Instrumental", "28 songs • 2h 05m", "🎼"),
    ];

    for (name, info, icon) in playlists {
        let row = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(12)
            .margin_end(12)
            .build();

        let icon_lbl = Label::new(Some(icon));
        icon_lbl.set_size_request(44, 44);

        let info_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .build();

        let name_lbl = Label::new(Some(name));
        name_lbl.set_halign(gtk4::Align::Start);
        name_lbl.add_css_class("playlist-name");

        let info_lbl = Label::new(Some(info));
        info_lbl.set_halign(gtk4::Align::Start);
        info_lbl.add_css_class("playlist-info");

        info_box.append(&name_lbl);
        info_box.append(&info_lbl);

        let menu = Button::builder().label("⋮").build();

        row.append(&icon_lbl);
        row.append(&info_box);
        row.append(&menu);

        list.append(&row);
    }

    scrolled.set_child(Some(&list));
    scrolled.upcast()
}

fn create_now_playing() -> gtk4::Widget {
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

    let time1 = Label::new(Some("2:15"));
    time1.add_css_class("time-label");

    let prog = Scale::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .draw_value(false)
        .build();

    let time2 = Label::new(Some("5:55"));
    time2.add_css_class("time-label");

    progress_box.append(&time1);
    progress_box.append(&prog);
    progress_box.append(&time2);

    // Controls row
    let controls_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(20)
        .margin_start(16)
        .margin_end(16)
        .margin_top(12)
        .margin_bottom(12)
        .hexpand(true)
        .build();

    // Album art
    let art = Label::new(Some("🎸"));
    art.set_size_request(70, 70);
    art.add_css_class("now-playing-art");

    // Track info
    let track_info = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(3)
        .hexpand(true)
        .build();

    let track_name = Label::new(Some("Bohemian Rhapsody"));
    track_name.add_css_class("track-name");

    let track_artist = Label::new(Some("Queen"));
    track_artist.add_css_class("track-artist");

    track_info.append(&track_name);
    track_info.append(&track_artist);

    // Playback buttons
    let playback = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .build();

    let btn_style = "control-btn";

    let prev = Button::builder().label("⏮").build();
    prev.add_css_class("control-btn");

    let play = Button::builder().label("▶").build();
    play.add_css_class("control-btn");

    let next = Button::builder().label("⏭").build();
    next.add_css_class("control-btn");

    playback.append(&prev);
    playback.append(&play);
    playback.append(&next);

    // Volume
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

    // Assemble controls
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

    // Top separator
    let sep = Separator::new(Orientation::Horizontal);
    container.prepend(&sep);

    container.upcast()
}