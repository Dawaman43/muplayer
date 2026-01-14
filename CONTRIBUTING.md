# Contributing to Arch Music Player

We welcome contributions to the Arch Music Player! This document provides guidelines for contributing.

## Development Setup

1. **Prerequisites**:
   ```bash
   sudo pacman -S rust gtk4 libadwaita pkg-config sqlite
   ```

2. **Clone and build**:
   ```bash
   git clone https://github.com/archlinux/music-player.git
   cd music-player
   cargo build
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

## Code Style

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow Rust idioms and conventions
- Document public APIs with `rustdoc`

## Submitting Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## Bug Reports

- Use the issue tracker for bug reports
- Include system information and steps to reproduce
- Provide logs if possible

## Feature Requests

- Open an issue with the "enhancement" label
- Describe the use case and expected behavior
- Consider if it fits the project goals

## Areas for Contribution

- **Audio Engine**: GStreamer integration, crossfade, equalizer
- **UI/UX**: Additional themes, visualizations, keyboard shortcuts
- **Library Management**: Playlist import/export, smart playlists
- **Metadata**: Additional tag formats, automatic tagging
- **Platform Integration**: MPRIS support, system tray

## Testing

- Write unit tests for new functionality
- Test on different audio formats
- Verify UI doesn't break on different screen sizes

Thank you for contributing!