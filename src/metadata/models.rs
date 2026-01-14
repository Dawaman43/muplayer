#[derive(Debug, Clone)]
pub struct AudioMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub track_number: Option<i32>,
    pub total_tracks: Option<i32>,
    pub disc_number: Option<i32>,
    pub total_discs: Option<i32>,
    pub duration: Option<u64>,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub file_size: Option<u64>,
    pub modified_time: Option<i64>,
    pub lyrics: Option<String>,
    pub album_art: Option<Vec<u8>>,
}

impl AudioMetadata {
    pub fn new() -> Self {
        Self {
            title: None,
            artist: None,
            album: None,
            album_artist: None,
            genre: None,
            year: None,
            track_number: None,
            total_tracks: None,
            disc_number: None,
            total_discs: None,
            duration: None,
            bitrate: None,
            sample_rate: None,
            file_size: None,
            modified_time: None,
            lyrics: None,
            album_art: None,
        }
    }
}

impl Default for AudioMetadata {
    fn default() -> Self {
        Self::new()
    }
}