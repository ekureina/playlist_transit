extern crate roxmltree;
#[cfg(feature = "gui")]
extern crate gtk;
#[cfg(feature = "gui")]
extern crate gio;

use std::fmt;
use std::env;
use std::fs;
use std::io;
use std::io::{Read, Write};

#[cfg(feature = "gui")]
use gtk::prelude::*;
#[cfg(feature = "gui")]
use gio::prelude::*;
#[cfg(feature = "gui")]
use gtk::Application;

struct Song {
    title: String,
    artist: String,
    album: String,
}

impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {} - {}", self.artist, self.title, self.album)
    }
}

#[cfg(feature = "gui")]
fn gui_path() {
    let application = Application::new("playlist.transit", Default::default()).expect("Application::new failed");
    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Playlist Transit");
        window.set_default_size(200, 400);
        let chooser_frame = gtk::Frame::new("Playlist File");
        let playlist_popup = gtk::FileChooserDialog::with_buttons(
            "Select Playlist File",
            Some(&window),
            gtk::FileChooserAction::Open,
            &[("Open", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel)]);
        let xspf_filter = gtk::FileFilter::new();
        xspf_filter.set_name("XSPF Playlist file (.xspf)");
        xspf_filter.add_mime_type("application/xspf+xml");
        playlist_popup.add_filter(&xspf_filter);

        let chooser_button = gtk::FileChooserButton::new_with_dialog(
            &playlist_popup);
        chooser_button.connect_file_set(|chooser_button| {if let Some(file) = chooser_button.get_filename() {
            if let Ok(filename) = file.into_os_string().into_string() {
                for song in get_songs(&filename) {
                    println!("{}", song);
                }
            }
        }});
        chooser_frame.add(&chooser_button);
        window.add(&chooser_frame);
        window.show_all();
    });

    application.run(&[]);
}

#[cfg(not(feature = "gui"))]
fn run_no_gui() {
    for song in get_songs(&get_song_path()) {
        println!("{}", song);
    }
}

fn run_auto(args: &mut env::Args) {
    let song_path = String::from(args.nth(1).unwrap().as_str());
    let songs: Vec<Song> = get_songs(&song_path);
    for song in songs {
        println!("{}", song);
    }
}

#[cfg(not(feature = "gui"))]
fn get_song_path() -> String {
    let mut song_path = String::new();
    loop {
        println!("Input the file to read from: ");
        match io::stdin().read_line(&mut song_path) {
            Ok(_) => return song_path,
            Err(x) => panic!("IO read Error: {}", x),
        }
    }
}

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        #[cfg(not(feature = "gui"))]
        run_no_gui();
        #[cfg(feature = "gui")]
        gui_path();
    } else {
        run_auto(&mut args);
    }
}

fn get_songs(xml_path: &str) -> Vec<Song> {
    let xml_text = get_xml_text(&xml_path);
    let playlists_xml = match roxmltree::Document::parse(&xml_text) {
        Ok(doc) => doc,
        Err(e) => panic!("Playlist Parse Error: {}.", e),
    };
    let mut songs = Vec::new();
    for song in playlists_xml.descendants() {
        if song.tag_name().name() == "track" {
            songs.push(Song{
                title: String::from(song.descendants().
                    find(|n| n.has_tag_name("title")).unwrap().text().unwrap()),
                artist: String::from(song.descendants().
                    find(|n| n.has_tag_name("creator")).unwrap().text().unwrap()),
                album: String::from(song.descendants().
                    find(|n| n.has_tag_name("album")).unwrap().text().unwrap())
            });
        }
    }
    songs
}

fn get_xml_file(xml_path_start: &str) -> fs::File {
    let mut xml_path = String::from(xml_path_start);
    let mut file = fs::File::open(&xml_path);
    while file.is_err() {
        print!("Invalid File: {}Enter valid file to read from: ", xml_path);
        match io::stdout().flush() {
            Ok(_) => (),
            Err(x) => panic!("IO flush Error: {}", x),
        }
        xml_path = String::new();
        match io::stdin().read_line(&mut xml_path) {
            Ok(_) => (),
            Err(x) => panic!("IO read Error: {}", x),
        }
        file = fs::File::open(&xml_path);
    }
    file.unwrap()
}

fn get_xml_text(xml_path: &str) -> String {
    let mut text = String::new();
    if xml_path != "--stdin" {
        let mut file = get_xml_file(xml_path);
        file.read_to_string(&mut text).unwrap();
    } else {
        io::stdin().read_to_string(&mut text).unwrap();
    }
    text
}
