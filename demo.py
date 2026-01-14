#!/usr/bin/env python3
"""
Demo script to show the Arch Music Player interface
"""

import gi
gi.require_version('Gtk', '4.0')
gi.require_version('Adw', '1')
from gi.repository import Gtk, Adw, Gio, GdkPixbuf

class MusicPlayerDemo:
    def __init__(self):
        self.app = Adw.Application(
            application_id='com.archlinux.musicplayer.demo',
            flags=Gio.ApplicationFlags.DEFAULT_FLAGS
        )
        self.app.connect('activate', self.on_activate)
    
    def on_activate(self, app):
        # Create main window
        self.window = Adw.ApplicationWindow(
            application=app,
            title="Arch Music Player",
            default_width=1200,
            default_height=800
        )
        
        # Create header bar
        header_bar = Adw.HeaderBar()
        header_bar.set_title_widget(Adw.WindowTitle("Arch Music Player", ""))
        
        # Create main box
        main_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)
        
        # Create search bar
        search_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        search_box.set_margin_top(6)
        search_box.set_margin_bottom(6)
        search_box.set_margin_start(12)
        search_box.set_margin_end(12)
        
        search_entry = Gtk.SearchEntry()
        search_entry.set_placeholder_text("Search music library...")
        search_entry.set_hexpand(True)
        
        search_box.append(search_entry)
        
        # Create stack for different views
        stack = Gtk.Stack()
        stack.set_transition_type(Gtk.StackTransitionType.SLIDE_LEFT_RIGHT)
        
        # Create view switcher
        view_switcher = Adw.ViewSwitcher()
        view_switcher.set_policy(Adw.ViewSwitcherPolicy.WIDE)
        
        # Album view
        album_scrolled = Gtk.ScrolledWindow()
        album_scrolled.set_vexpand(True)
        
        album_grid = Gtk.Grid()
        album_grid.set_column_spacing(12)
        album_grid.set_row_spacing(12)
        album_grid.set_margin_top(12)
        album_grid.set_margin_bottom(12)
        album_grid.set_margin_start(12)
        album_grid.set_margin_end(12)
        
        # Add some sample albums
        for i in range(12):
            album_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=6)
            album_box.set_size_request(150, 150)
            
            # Album art placeholder
            album_art = Gtk.Image.new_from_icon_name("folder-music-symbolic")
            album_art.set_pixel_size(120)
            
            # Album label
            album_name = f"Album {i+1}"
            artist_name = f"Artist {i+1}"
            
            album_label = Gtk.Label(label=f"<b>{album_name}</b>\n{artist_name}")
            album_label.set_use_markup(True)
            album_label.set_halign(Gtk.Align.CENTER)
            
            album_box.append(album_art)
            album_box.append(album_label)
            
            col = i % 4
            row = i // 4
            album_grid.attach(album_box, col, row, 1, 1)
        
        album_scrolled.set_child(album_grid)
        stack.add_titled(album_scrolled, "albums", "Albums")
        
        # Artist view
        artist_scrolled = Gtk.ScrolledWindow()
        artist_scrolled.set_vexpand(True)
        
        artist_list = Gtk.ListBox()
        artist_list.set_selection_mode(Gtk.SelectionMode.SINGLE)
        
        # Add sample artists
        artists = ["The Beatles", "Pink Floyd", "Led Zeppelin", "Queen", 
                  "Nirvana", "Radiohead", "Daft Punk", "Pink Floyd",
                  "Arctic Monkeys", "Tame Impala", "The Strokes", "Foo Fighters"]
        
        for artist in artists:
            artist_row = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=12)
            artist_row.set_margin_top(6)
            artist_row.set_margin_bottom(6)
            artist_row.set_margin_start(12)
            artist_row.set_margin_end(12)
            
            artist_icon = Gtk.Image.new_from_icon_name("avatar-default-symbolic")
            artist_icon.set_pixel_size(48)
            
            artist_label = Gtk.Label(label=artist)
            artist_label.set_hexpand(True)
            
            artist_row.append(artist_icon)
            artist_row.append(artist_label)
            
            artist_list.append(artist_row)
        
        artist_scrolled.set_child(artist_list)
        stack.add_titled(artist_scrolled, "artists", "Artists")
        
        # Folder view
        folder_scrolled = Gtk.ScrolledWindow()
        folder_scrolled.set_vexpand(True)
        
        folder_tree = Gtk.TreeView()
        folder_tree.set_enable_tree_lines(True)
        
        # Create a simple tree model
        liststore = Gtk.ListStore(str, str)
        liststore.append(["📁 Music", "Folder"])
        liststore.append(["📁 Documents/Music", "Folder"])
        liststore.append(["📁 Downloads/Music", "Folder"])
        liststore.append(["📁 /usr/share/music", "System Folder"])
        
        for item in liststore:
            tree_iter = liststore.iter_next(item.iter)
        
        folder_scrolled.set_child(folder_tree)
        stack.add_titled(folder_scrolled, "folders", "Folders")
        
        # Playlist view
        playlist_scrolled = Gtk.ScrolledWindow()
        playlist_scrolled.set_vexpand(True)
        
        playlist_list = Gtk.ListBox()
        playlist_list.set_selection_mode(Gtk.SelectionMode.SINGLE)
        
        playlists = ["Favorites", "Recently Played", "Most Played", 
                   "Workout Mix", "Chill Vibes", "Rock Classics"]
        
        for playlist in playlists:
            playlist_row = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=12)
            playlist_row.set_margin_top(6)
            playlist_row.set_margin_bottom(6)
            playlist_row.set_margin_start(12)
            playlist_row.set_margin_end(12)
            
            playlist_icon = Gtk.Image.new_from_icon_name("playlist-symbolic")
            playlist_icon.set_pixel_size(48)
            
            playlist_label = Gtk.Label(label=playlist)
            playlist_label.set_hexpand(True)
            
            playlist_row.append(playlist_icon)
            playlist_row.append(playlist_label)
            
            playlist_list.append(playlist_row)
        
        playlist_scrolled.set_child(playlist_list)
        stack.add_titled(playlist_scrolled, "playlists", "Playlists")
        
        # Connect view switcher to stack
        view_switcher.set_stack(stack)
        
        # Create player controls
        player_controls = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=12)
        player_controls.set_margin_start(12)
        player_controls.set_margin_end(12)
        player_controls.set_margin_top(6)
        player_controls.set_margin_bottom(6)
        
        # Playback buttons
        prev_btn = Gtk.Button.new_from_icon_name("media-skip-backward-symbolic")
        prev_btn.set_tooltip_text("Previous Track")
        
        play_btn = Gtk.Button.new_from_icon_name("media-playback-start-symbolic")
        play_btn.set_tooltip_text("Play")
        
        pause_btn = Gtk.Button.new_from_icon_name("media-playback-pause-symbolic")
        pause_btn.set_tooltip_text("Pause")
        
        stop_btn = Gtk.Button.new_from_icon_name("media-playback-stop-symbolic")
        stop_btn.set_tooltip_text("Stop")
        
        next_btn = Gtk.Button.new_from_icon_name("media-skip-forward-symbolic")
        next_btn.set_tooltip_text("Next Track")
        
        # Progress bar
        progress_scale = Gtk.Scale(orientation=Gtk.Orientation.HORIZONTAL)
        progress_scale.set_hexpand(True)
        progress_scale.set_draw_value(False)
        
        # Time label
        time_label = Gtk.Label(label="0:00 / 3:45")
        
        # Volume button
        volume_btn = Gtk.VolumeButton()
        volume_btn.set_value(0.8)
        
        player_controls.append(prev_btn)
        player_controls.append(play_btn)
        player_controls.append(pause_btn)
        player_controls.append(stop_btn)
        player_controls.append(next_btn)
        player_controls.append(progress_scale)
        player_controls.append(time_label)
        player_controls.append(volume_btn)
        
        # Assemble UI
        main_box.append(header_bar)
        main_box.append(search_box)
        main_box.append(stack)
        main_box.append(player_controls)
        
        self.window.set_content(main_box)
        self.window.present()
    
    def run(self):
        return self.app.run()

if __name__ == '__main__':
    demo = MusicPlayerDemo()
    demo.run()