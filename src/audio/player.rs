use std::path::PathBuf;
use anyhow::Result;
use rodio::{OutputStream, Sink, Decoder, Source};
use std::fs::File;
use std::io::BufReader;

pub struct AudioPlayer {
    stream: Option<OutputStream>,
    sink: Option<Sink>,
    current_track: Option<PathBuf>,
    volume: f32,
    is_playing: bool,
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self {
            stream: None,
            sink: None,
            current_track: None,
            volume: 1.0,
            is_playing: false,
        }
    }
    
    pub fn play(&mut self) -> Result<()> {
        if let Some(ref sink) = self.sink {
            sink.play();
            self.is_playing = true;
        }
        Ok(())
    }
    
    pub fn pause(&mut self) -> Result<()> {
        if let Some(ref sink) = self.sink {
            sink.pause();
            self.is_playing = false;
        }
        Ok(())
    }
    
    pub fn stop(&mut self) -> Result<()> {
        if let Some(ref sink) = self.sink {
            sink.stop();
            self.is_playing = false;
        }
        Ok(())
    }
    
    pub fn load_track(&mut self, path: PathBuf) -> Result<()> {
        // Initialize audio stream if not already done
        if self.stream.is_none() {
            let (stream, handle) = OutputStream::try_default()?;
            self.stream = Some(stream);
            self.sink = Some(Sink::try_new(&handle)?);
        }
        
        // Load the audio file
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)?;
        
        // Clear previous track and load new one
        if let Some(ref sink) = self.sink {
            sink.stop();
            sink.append(source);
            self.current_track = Some(path);
        }
        
        Ok(())
    }
    
    pub fn next(&mut self) -> Result<()> {
        // This would load the next track in the queue
        // For now, just stop current playback
        self.stop()
    }
    
    pub fn previous(&mut self) -> Result<()> {
        // This would load the previous track in the queue
        // For now, just stop current playback
        self.stop()
    }
    
    pub fn set_volume(&mut self, volume: f32) -> Result<()> {
        self.volume = volume.clamp(0.0, 1.0);
        if let Some(ref sink) = self.sink {
            sink.set_volume(self.volume);
        }
        Ok(())
    }
    
    pub fn get_volume(&self) -> f32 {
        self.volume
    }
    
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
    
    pub fn get_current_track(&self) -> Option<&PathBuf> {
        self.current_track.as_ref()
    }
    
    pub fn get_position(&self) -> Option<u64> {
        // Rodio doesn't provide easy position tracking
        // This would need to be implemented with custom timing
        None
    }
    
    pub fn get_duration(&self) -> Option<u64> {
        // This would need to be implemented with metadata reading
        None
    }
}