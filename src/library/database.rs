use std::path::Path;
use anyhow::Result;
use rusqlite::{Connection, params};
use std::sync::Arc;
use parking_lot::Mutex;

use crate::library::models::{Song, Playlist, Album, Artist};

pub struct LibraryDatabase {
    connection: Arc<Mutex<Connection>>,
}

impl LibraryDatabase {
    pub fn new() -> Self {
        let db_path = dirs::data_dir()
            .unwrap_or_else(|| Path::new(".").to_path_buf())
            .join("arch-music-player")
            .join("library.db");
        
        // Create directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        
        let connection = Connection::open(db_path).expect("Failed to open database");
        
        let db = Self {
            connection: Arc::new(Mutex::new(connection)),
        };
        
        db.initialize_schema().expect("Failed to initialize database schema");
        db
    }
    
    fn initialize_schema(&self) -> Result<()> {
        let conn = self.connection.lock();
        
        // Songs table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS songs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT UNIQUE NOT NULL,
                title TEXT,
                artist TEXT,
                album TEXT,
                album_artist TEXT,
                genre TEXT,
                year INTEGER,
                track_number INTEGER,
                duration INTEGER,
                file_size INTEGER,
                modified_time INTEGER,
                rating INTEGER DEFAULT 0,
                play_count INTEGER DEFAULT 0,
                last_played INTEGER,
                is_favorite BOOLEAN DEFAULT FALSE,
                created_at INTEGER DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER DEFAULT (strftime('%s', 'now'))
            )",
            [],
        )?;
        
        // Playlists table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS playlists (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                created_at INTEGER DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER DEFAULT (strftime('%s', 'now'))
            )",
            [],
        )?;
        
        // Playlist songs junction table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS playlist_songs (
                playlist_id INTEGER,
                song_id INTEGER,
                position INTEGER,
                added_at INTEGER DEFAULT (strftime('%s', 'now')),
                PRIMARY KEY (playlist_id, song_id),
                FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
                FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE
            )",
            [],
        )?;
        
        // Create indexes for better performance
        conn.execute("CREATE INDEX IF NOT EXISTS idx_songs_artist ON songs(artist)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_songs_album ON songs(album)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_songs_genre ON songs(genre)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_songs_year ON songs(year)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_songs_favorite ON songs(is_favorite)", [])?;
        
        Ok(())
    }
    
    pub fn add_song(&self, path: &Path, metadata: &crate::metadata::models::AudioMetadata) -> Result<i64> {
        let conn = self.connection.lock();
        let path_str = path.to_string_lossy();
        
        let mut stmt = conn.prepare(
            "INSERT OR REPLACE INTO songs (
                path, title, artist, album, album_artist, genre, year,
                track_number, duration, file_size, modified_time
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"
        )?;
        
        let song_id = stmt.insert(params![
            path_str,
            metadata.title,
            metadata.artist,
            metadata.album,
            metadata.album_artist,
            metadata.genre,
            metadata.year,
            metadata.track_number,
            metadata.duration,
            metadata.file_size,
            metadata.modified_time
        ])?;
        
        Ok(song_id)
    }
    
    pub fn get_all_songs(&self) -> Result<Vec<Song>> {
        let conn = self.connection.lock();
        let mut stmt = conn.prepare(
            "SELECT id, path, title, artist, album, album_artist, genre, year,
                    track_number, duration, file_size, modified_time, rating,
                    play_count, last_played, is_favorite
             FROM songs ORDER BY artist, album, track_number"
        )?;
        
        let songs = stmt.query_map([], |row| {
            Ok(Song {
                id: Some(row.get(0)?),
                path: row.get(1)?,
                title: row.get(2)?,
                artist: row.get(3)?,
                album: row.get(4)?,
                album_artist: row.get(5)?,
                genre: row.get(6)?,
                year: row.get(7)?,
                track_number: row.get(8)?,
                duration: row.get(9)?,
                file_size: row.get(10)?,
                modified_time: row.get(11)?,
                rating: row.get(12)?,
                play_count: row.get(13)?,
                last_played: row.get(14)?,
                is_favorite: row.get(15)?,
            })
        })?;
        
        let mut result = Vec::new();
        for song in songs {
            result.push(song?);
        }
        
        Ok(result)
    }
    
    pub fn get_all_albums(&self) -> Result<Vec<String>> {
        let conn = self.connection.lock();
        let mut stmt = conn.prepare(
            "SELECT DISTINCT album FROM songs WHERE album IS NOT NULL ORDER BY album"
        )?;
        
        let albums = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;
        
        let mut result = Vec::new();
        for album in albums {
            result.push(album?);
        }
        
        Ok(result)
    }
    
    pub fn get_all_artists(&self) -> Result<Vec<String>> {
        let conn = self.connection.lock();
        let mut stmt = conn.prepare(
            "SELECT DISTINCT artist FROM songs WHERE artist IS NOT NULL ORDER BY artist"
        )?;
        
        let artists = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;
        
        let mut result = Vec::new();
        for artist in artists {
            result.push(artist?);
        }
        
        Ok(result)
    }
    
    pub fn get_all_playlists(&self) -> Result<Vec<String>> {
        let conn = self.connection.lock();
        let mut stmt = conn.prepare(
            "SELECT name FROM playlists ORDER BY name"
        )?;
        
        let playlists = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;
        
        let mut result = Vec::new();
        for playlist in playlists {
            result.push(playlist?);
        }
        
        Ok(result)
    }
    
    pub fn search(&self, query: &str) -> Result<Vec<Song>> {
        let conn = self.connection.lock();
        let search_pattern = format!("%{}%", query);
        
        let mut stmt = conn.prepare(
            "SELECT id, path, title, artist, album, album_artist, genre, year,
                    track_number, duration, file_size, modified_time, rating,
                    play_count, last_played, is_favorite
             FROM songs 
             WHERE title LIKE ?1 OR artist LIKE ?1 OR album LIKE ?1
             ORDER BY artist, album, track_number"
        )?;
        
        let songs = stmt.query_map([&search_pattern], |row| {
            Ok(Song {
                id: Some(row.get(0)?),
                path: row.get(1)?,
                title: row.get(2)?,
                artist: row.get(3)?,
                album: row.get(4)?,
                album_artist: row.get(5)?,
                genre: row.get(6)?,
                year: row.get(7)?,
                track_number: row.get(8)?,
                duration: row.get(9)?,
                file_size: row.get(10)?,
                modified_time: row.get(11)?,
                rating: row.get(12)?,
                play_count: row.get(13)?,
                last_played: row.get(14)?,
                is_favorite: row.get(15)?,
            })
        })?;
        
        let mut result = Vec::new();
        for song in songs {
            result.push(song?);
        }
        
        Ok(result)
    }
    
    pub fn create_playlist(&self, name: &str) -> Result<i64> {
        let conn = self.connection.lock();
        
        let mut stmt = conn.prepare(
            "INSERT INTO playlists (name) VALUES (?)"
        )?;
        
        let playlist_id = stmt.insert(params![name])?;
        Ok(playlist_id)
    }
    
    pub fn add_song_to_playlist(&self, playlist_id: i64, song_id: i64, position: i32) -> Result<()> {
        let conn = self.connection.lock();
        
        conn.execute(
            "INSERT OR REPLACE INTO playlist_songs (playlist_id, song_id, position) VALUES (?1, ?2, ?3)",
            params![playlist_id, song_id, position],
        )?;
        
        Ok(())
    }
    
    pub fn update_play_count(&self, song_id: i64) -> Result<()> {
        let conn = self.connection.lock();
        
        conn.execute(
            "UPDATE songs SET play_count = play_count + 1, last_played = strftime('%s', 'now') WHERE id = ?1",
            params![song_id],
        )?;
        
        Ok(())
    }
    
    pub fn toggle_favorite(&self, song_id: i64) -> Result<bool> {
        let conn = self.connection.lock();
        
        // Get current favorite status
        let mut stmt = conn.prepare("SELECT is_favorite FROM songs WHERE id = ?1")?;
        let current_favorite: bool = stmt.query_row(params![song_id], |row| row.get(0))?;
        
        // Toggle favorite status
        let new_favorite = !current_favorite;
        conn.execute(
            "UPDATE songs SET is_favorite = ?1 WHERE id = ?2",
            params![new_favorite, song_id],
        )?;
        
        Ok(new_favorite)
    }
}