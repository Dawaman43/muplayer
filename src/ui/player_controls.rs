use std::sync::{Arc, Mutex};
use gtk4::prelude::*;
use gtk4::*;

// Add Widget implementation for PlayerControls
impl AsRef<Widget> for PlayerControls {
    fn as_ref(&self) -> &Widget {
        &self.container
    }
}

use crate::audio::player::AudioPlayer;

pub struct PlayerControls {
    container: Box,
    play_button: Button,
    pause_button: Button,
    stop_button: Button,
    previous_button: Button,
    next_button: Button,
    progress_bar: Scale,
    volume_button: VolumeButton,
    time_label: Label,
    audio_player: Arc<Mutex<AudioPlayer>>,
}

impl PlayerControls {
    pub fn new(audio_player: Arc<Mutex<AudioPlayer>>) -> Self {
        // Create container
        let container = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .margin_start(12)
            .margin_end(12)
            .margin_top(6)
            .margin_bottom(6)
            .build();
        
        // Create playback controls
        let previous_button = Button::builder()
            .icon_name("media-skip-backward-symbolic")
            .tooltip_text("Previous Track")
            .build();
        
        let play_button = Button::builder()
            .icon_name("media-playback-start-symbolic")
            .tooltip_text("Play")
            .build();
        
        let pause_button = Button::builder()
            .icon_name("media-playback-pause-symbolic")
            .tooltip_text("Pause")
            .build();
        
        let stop_button = Button::builder()
            .icon_name("media-playback-stop-symbolic")
            .tooltip_text("Stop")
            .build();
        
        let next_button = Button::builder()
            .icon_name("media-skip-forward-symbolic")
            .tooltip_text("Next Track")
            .build();
        
        // Create progress bar
        let progress_bar = Scale::builder()
            .orientation(Orientation::Horizontal)
            .adjustment(&Adjustment::new(0.0, 0.0, 100.0, 1.0, 10.0, 0.0))
            .hexpand(true)
            .build();
        progress_bar.set_draw_value(false);
        
        // Create time label
        let time_label = Label::builder()
            .label("0:00 / 0:00")
            .build();
        
        // Create volume button
        let volume_button = VolumeButton::builder()
            .build();
        volume_button.set_value(1.0);
        
        // Assemble controls
        container.append(&previous_button);
        container.append(&play_button);
        container.append(&pause_button);
        container.append(&stop_button);
        container.append(&next_button);
        container.append(&progress_bar);
        container.append(&time_label);
        container.append(&volume_button);
        
        let controls = Self {
            container,
            play_button,
            pause_button,
            stop_button,
            previous_button,
            next_button,
            progress_bar,
            volume_button,
            time_label,
            audio_player,
        };
        
        // Connect signals
        controls.setup_signals();
        
        controls
    }
    
    fn setup_signals(&self) {
        let audio_player = self.audio_player.clone();
        
        // Play button
        self.play_button.connect_clicked(move |_| {
            if let Ok(mut player) = audio_player.lock() {
                let _ = player.play();
            }
        });
        
        // Pause button
        let audio_player = self.audio_player.clone();
        self.pause_button.connect_clicked(move |_| {
            if let Ok(mut player) = audio_player.lock() {
                let _ = player.pause();
            }
        });
        
        // Stop button
        let audio_player = self.audio_player.clone();
        self.stop_button.connect_clicked(move |_| {
            if let Ok(mut player) = audio_player.lock() {
                let _ = player.stop();
            }
        });
        
        // Previous button
        let audio_player = self.audio_player.clone();
        self.previous_button.connect_clicked(move |_| {
            if let Ok(mut player) = audio_player.lock() {
                let _ = player.previous();
            }
        });
        
        // Next button
        let audio_player = self.audio_player.clone();
        self.next_button.connect_clicked(move |_| {
            if let Ok(mut player) = audio_player.lock() {
                let _ = player.next();
            }
        });
        
        // Volume control
        let audio_player = self.audio_player.clone();
        self.volume_button.connect_value_changed(move |_, value| {
            if let Ok(mut player) = audio_player.lock() {
                let _ = player.set_volume(value as f32);
            }
        });
    }
    
    pub fn update_time_display(&self, current: u64, total: u64) {
        let current_str = format_duration(current);
        let total_str = format_duration(total);
        self.time_label.set_label(&format!("{} / {}", current_str, total_str));
    }
    
    pub fn update_progress(&self, progress: f64) {
        self.progress_bar.set_value(progress);
    }
}

impl AsRef<Widget> for PlayerControls {
    fn as_ref(&self) -> &Widget {
        self.container.as_ref()
    }
}

fn format_duration(seconds: u64) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{}:{:02}", minutes, seconds)
}