use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Song {
    pub id: Option<i64>,
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub track_number: Option<i32>,
    pub duration: Option<i64>,
    pub file_size: Option<i64>,
    pub modified_time: Option<i64>,
    pub rating: i32,
    pub play_count: i32,
    pub last_played: Option<i64>,
    pub is_favorite: bool,
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: Option<i64>,
    pub name: String,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct Album {
    pub name: String,
    pub artist: Option<String>,
    pub year: Option<i32>,
    pub song_count: i32,
}

#[derive(Debug, Clone)]
pub struct Artist {
    pub name: String,
    pub album_count: i32,
    pub song_count: i32,
}

impl Song {
    pub fn new(path: &str) -> Self {
        Self {
            id: None,
            path: path.to_string(),
            title: None,
            artist: None,
            album: None,
            album_artist: None,
            genre: None,
            year: None,
            track_number: None,
            duration: None,
            file_size: None,
            modified_time: None,
            rating: 0,
            play_count: 0,
            last_played: None,
            is_favorite: false,
        }
    }
    
    pub fn from_metadata(path: &str, metadata: &crate::metadata::models::AudioMetadata) -> Self {
        Self {
            id: None,
            path: path.to_string(),
            title: metadata.title.clone(),
            artist: metadata.artist.clone(),
            album: metadata.album.clone(),
            album_artist: metadata.album_artist.clone(),
            genre: metadata.genre.clone(),
            year: metadata.year,
            track_number: metadata.track_number,
            duration: metadata.duration.map(|d| d as i64),
            file_size: metadata.file_size.map(|s| s as i64),
            modified_time: None,
            rating: 0,
            play_count: 0,
            last_played: None,
            is_favorite: false,
        }
    }
}

impl Playlist {
    pub fn new(name: &str) -> Self {
        Self {
            id: None,
            name: name.to_string(),
            created_at: chrono::Utc::now().timestamp(),
        }
    }
}