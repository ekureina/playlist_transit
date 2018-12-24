extern crate roxmltree;

use std::fmt;

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
    let songs: Vec<Song> = vec![Song{
        title: String::from("風の日"),
        artist: String::from("ELLEGARDEN"),
        album: String::from("Figureheads Compilation")}];
    print!("{}", songs[0]);
}
