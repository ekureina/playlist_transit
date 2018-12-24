extern crate roxmltree;

use std::fmt;
use std::env;
use std::fs;
use std::io::Read;
use std::process;

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

fn main() {
    let args: Vec<_> = env::args().collect();
    let songs: Vec<Song> = get_songs(&args[1]);
    for song in songs {
        println!("{}", song);
    }
}

fn get_songs(xml_path: &str) -> Vec<Song> {
    let xml_text = get_xml_text(&xml_path);
    let playlists_xml = match roxmltree::Document::parse(&xml_text) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Playlist Parse Error: {}.", e);
            process::exit(1);
        },
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

fn get_xml_text(xml_path: &str) -> String {
    let mut file = fs::File::open(&xml_path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}
