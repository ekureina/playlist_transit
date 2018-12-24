extern crate roxmltree;

struct Song {
    title: String,
    artist: String,
    album: String,
}

fn main() {
    let songs: Vec<Song> = vec![Song{
        title: String::from("風の日"),
        artist: String::from("ELLEGARDEN"),
        album: String::from("Figureheads Compilation")}];
}
