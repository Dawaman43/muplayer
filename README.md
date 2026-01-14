# Arch Music Player

A comprehensive, native music player for Arch Linux built with Rust and GTK4.

## Features

- **Extensive Format Support**: MP3, FLAC, OGG, WAV, M4A, AAC, WMA, APE, and more
- **Automatic Library Scanning**: Monitors multiple directories including:
  - `~/Music`
  - `~/Documents/Music`
  - `~/Downloads/Music`
  - `/usr/share/music`
  - `/var/music`
  - Custom user directories
- **Rich Metadata Support**: Read/write ID3v1, ID3v2, Vorbis Comments, MP4 tags
- **Multiple Views**: Album, Artist, Folder, and Playlist views
- **Playback Controls**: Play, pause, stop, next, previous, volume control
- **Smart Search**: Search across title, artist, album, and genre
- **Playlist Management**: Create and manage custom playlists
- **Favorites System**: Mark and quickly access favorite tracks
- **Modern UI**: GTK4 with libadwaita for a beautiful native interface

## Installation

### From AUR (recommended)

```bash
paru -S arch-music-player
```

### Building from source

#### Prerequisites

Install the required dependencies:

```bash
sudo pacman -S rust gtk4 libadwaita pkg-config sqlite \
                alsa-lib pulseaudio gstreamer gst-plugins-base \
                gst-plugins-good gst-plugins-bad gst-plugins-ugly
```

#### Build

```bash
git clone https://github.com/archlinux/music-player.git
cd music-player
cargo build --release
```

The binary will be available at `target/release/arch-music-player`.

### Flatpak

```bash
flatpak install flathub com.archlinux.MusicPlayer
```

## Usage

1. **First Launch**: The app will automatically scan common music directories
2. **Add Music**: Go to Preferences → Library to add custom directories
3. **Play Music**: Click any song or use the playback controls
4. **Create Playlists**: Right-click songs or use the playlist view
5. **Customize**: Adjust themes, audio settings, and library options in Preferences

## Keyboard Shortcuts

- `Space` - Play/Pause
- `Ctrl+Left` - Previous Track
- `Ctrl+Right` - Next Track
- `Ctrl+F` - Search
- `Ctrl+P` - Preferences
- `Ctrl+Q` - Quit

## Configuration

Configuration files are stored in:
- `~/.local/share/arch-music-player/library.db` - Music library database
- `~/.config/arch-music-player/` - User preferences

## Contributing

Contributions are welcome! Please see the [Contributing Guide](CONTRIBUTING.md) for details.

## License

GPL-3.0 - see the [LICENSE](LICENSE) file for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/archlinux/music-player/issues)
- **Discussions**: [GitHub Discussions](https://github.com/archlinux/music-player/discussions)
- **Wiki**: [Project Wiki](https://github.com/archlinux/music-player/wiki)