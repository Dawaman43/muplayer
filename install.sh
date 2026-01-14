#!/bin/bash
# Arch Music Player Installation Script
# Installs the application system-wide and adds to application menu

set -e  # Exit on error

echo "🎵 Arch Music Player - Installation Script"
echo "=========================================="

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "⚠️  This script requires root privileges."
    echo "Run with: sudo $0"
    exit 1
fi

echo "📁 Setting up installation directories..."

# Create necessary directories
mkdir -p /usr/bin
mkdir -p /usr/share/applications
mkdir -p /usr/share/icons/hicolor/scalable/apps

echo "📦 Copying binary..."
# Copy the binary
if [ -f "./target/release/arch-music-player" ]; then
    cp ./target/release/arch-music-player /usr/bin/arch-music-player
    chmod +x /usr/bin/arch-music-player
    echo "✅ Binary installed to /usr/bin/arch-music-player"
elif [ -f "./target/debug/arch-music-player" ]; then
    cp ./target/debug/arch-music-player /usr/bin/arch-music-player
    chmod +x /usr/bin/arch-music-player
    echo "✅ Debug binary installed to /usr/bin/arch-music-player"
else
    echo "❌ Error: No binary found. Build the project first!"
    echo "   Run: cargo build --release"
    exit 1
fi

echo "📋 Installing desktop entry..."
# Copy desktop file
cp arch-music-player.desktop /usr/share/applications/arch-music-player.desktop
chmod 644 /usr/share/applications/arch-music-player.desktop

# Update desktop database
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database /usr/share/applications/
    echo "✅ Desktop database updated"
fi

# Create simple icon if none exists
if [ ! -f /usr/share/icons/hicolor/scalable/apps/arch-music-player.svg ]; then
    cat > /usr/share/icons/hicolor/scalable/apps/arch-music-player.svg << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 48 48">
  <rect width="48" height="48" rx="8" fill="#4a90d9"/>
  <text x="24" y="32" text-anchor="middle" font-size="24" fill="white">🎵</text>
</svg>
EOF
    echo "✅ Icon installed"
fi

# Refresh icon cache
if command -v gtk-update-icon-cache &> /dev/null; then
    gtk-update-icon-cache -f /usr/share/icons/hicolor/ 2>/dev/null || true
    echo "✅ Icon cache updated"
fi

echo ""
echo "🎉 Installation Complete!"
echo "========================"
echo ""
echo "✅ Arch Music Player has been installed successfully!"
echo ""
echo "📍 Installation Summary:"
echo "   • Binary: /usr/bin/arch-music-player"
echo "   • Desktop file: /usr/share/applications/arch-music-player.desktop"
echo "   • Icon: /usr/share/icons/hicolor/scalable/apps/arch-music-player.svg"
echo ""
echo "🎵 You can now:"
echo "   • Find 'Arch Music Player' in your application menu"
echo "   • Run 'arch-music-player' from terminal"
echo "   • Right-click desktop file for actions"
echo ""
echo "🎧 Enjoy your music!"

exit 0