extern crate roxmltree;

use std::fmt;
use std::env;
use std::fs;
use std::io;
use std::io::{Read, Write};

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
    let argc = args.len();
    let mut song_path: String = String::new();
    if argc < 2 {
        println!("Input the file to read from: ");
        match io::stdin().read_line(&mut song_path) {
            Ok(_) => (),
            Err(x) => panic!("IO read Error: {}", x),
        }
    } else {
        song_path = String::from(args[1].as_str());
    }
    let songs: Vec<Song> = get_songs(&song_path);
    for song in songs {
        println!("{}", song);
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
