#!/bin/bash

# PKGBUILD for Arch Linux

pkgname=arch-music-player
pkgver=0.1.0
pkgrel=1
pkgdesc="A comprehensive music player for Arch Linux"
arch=('x86_64')
url="https://github.com/archlinux/music-player"
license=('GPL3')
depends=('gtk4' 'libadwaita' 'gstreamer' 'gst-plugins-base' 'gst-plugins-good' 'gst-plugins-bad' 'gst-plugins-ugly' 'sqlite' 'alsa-lib' 'pulseaudio')
makedepends=('rust' 'pkgconfig' 'git')
optdepends=('gst-libav: Additional codec support')
provides=('arch-music-player')
conflicts=('arch-music-player-git')
source=("git+$url.git#tag=v$pkgver")
sha256sums=('SKIP')

build() {
    cd "$pkgname"
    cargo build --release --features gtk4
}

package() {
    cd "$pkgname"
    
    # Install binary
    install -Dm755 "target/release/arch-music-player" "$pkgdir/usr/bin/arch-music-player"
    
    # Install desktop file
    install -Dm644 "com.archlinux.MusicPlayer.desktop" "$pkgdir/usr/share/applications/com.archlinux.MusicPlayer.desktop"
    
    # Install metainfo
    install -Dm644 "com.archlinux.MusicPlayer.metainfo.xml" "$pkgdir/usr/share/metainfo/com.archlinux.MusicPlayer.metainfo.xml"
    
    # Install icon (if available)
    if [ -f "data/icons/hicolor/scalable/apps/com.archlinux.MusicPlayer.svg" ]; then
        install -Dm644 "data/icons/hicolor/scalable/apps/com.archlinux.MusicPlayer.svg" "$pkgdir/usr/share/icons/hicolor/scalable/apps/com.archlinux.MusicPlayer.svg"
    fi
}