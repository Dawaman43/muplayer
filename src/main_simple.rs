use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Label, Box, Entry, Scale, Adjustment, Orientation};
use libadwaita::{Application as AdwApplication, HeaderBar, WindowTitle};

fn main() {
    // Create application
    let app = AdwApplication::builder()
        .application_id("com.archlinux.musicplayer")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &AdwApplication) {
    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Arch Music Player")
        .default_width(800)
        .default_height(600)
        .build();

    // Create header bar
    let header_bar = HeaderBar::builder()
        .title_widget(&WindowTitle::new("Arch Music Player", ""))
        .build();

    // Create main container
    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    // Add welcome label
    let welcome_label = Label::builder()
        .label("🎵 Welcome to Arch Music Player! 🎵")
        .margin_top(20)
        .build();

    // Add search box
    let search_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .margin_top(10)
        .build();

    let search_entry = Entry::builder()
        .placeholder_text("Search your music library...")
        .hexpand(true)
        .build();

    let search_button = Button::builder()
        .label("Search")
        .build();

    search_box.append(&search_entry);
    search_box.append(&search_button);

    // Add player controls
    let controls_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .margin_top(20)
        .halign(gtk4::Align::Center)
        .build();

    let play_button = Button::builder()
        .label("▶ Play")
        .build();

    let pause_button = Button::builder()
        .label("⏸ Pause")
        .build();

    let stop_button = Button::builder()
        .label("⏹ Stop")
        .build();

    let prev_button = Button::builder()
        .label("⏮ Prev")
        .build();

    let next_button = Button::builder()
        .label("⏭ Next")
        .build();

    controls_box.append(&prev_button);
    controls_box.append(&play_button);
    controls_box.append(&pause_button);
    controls_box.append(&stop_button);
    controls_box.append(&next_button);

    // Add volume control
    let volume_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .margin_top(10)
        .build();

    let volume_label = Label::builder()
        .label("🔊 Volume:")
        .build();

    let volume_scale = Scale::builder()
        .orientation(Orientation::Horizontal)
        .adjustment(&Adjustment::new(70.0, 0.0, 100.0, 1.0, 5.0, None))
        .hexpand(true)
        .build();

    volume_box.append(&volume_label);
    volume_box.append(&volume_scale);

    // Add progress bar
    let progress_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .margin_top(10)
        .build();

    let progress_label = Label::builder()
        .label("⏱ Progress:")
        .build();

    let progress_scale = Scale::builder()
        .orientation(Orientation::Horizontal)
        .adjustment(&Adjustment::new(30.0, 0.0, 100.0, 1.0, 10.0, None))
        .hexpand(true)
        .draw_value(false)
        .build();

    progress_box.append(&progress_label);
    progress_box.append(&progress_scale);

    // Add status info
    let status_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .margin_top(20)
        .build();

    let status_label = Label::builder()
        .label("📊 Library Status: Ready")
        .build();

    let info_label = Label::builder()
        .label("🎧 Now Playing: No track selected")
        .build();

    let features_label = Label::builder()
        .label("✅ Features: Rust + GTK4 | Multiple Audio Formats | Auto-Library Scanning")
        .build();

    status_box.append(&status_label);
    status_box.append(&info_label);
    status_box.append(&features_label);

    // Connect signals
    play_button.connect_clicked(|_| {
        println!("▶ Play button clicked!");
    });

    pause_button.connect_clicked(|_| {
        println!("⏸ Pause button clicked!");
    });

    stop_button.connect_clicked(|_| {
        println!("⏹ Stop button clicked!");
    });

    search_button.connect_clicked(move |_| {
        let search_text = search_entry.text();
        println!("🔍 Searching for: {}", search_text);
    });

    // Assemble UI
    main_box.append(&welcome_label);
    main_box.append(&search_box);
    main_box.append(&controls_box);
    main_box.append(&volume_box);
    main_box.append(&progress_box);
    main_box.append(&status_box);

    window.set_title_widget(Some(&header_bar));
    window.set_content(Some(&main_box));
    window.present();
}