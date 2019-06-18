# Playlist Transit

A converter from XML playlist files (in the [XSPF](http://www.xspf.org) to the KUCI Playlisting format we use in the station.
I wrote this as a utility to convert from an exported XSPF playlist created in VLC, but it should theoretically work on any valid XSPF playlist created.

## Compilation

Playlist Transit is written in rust, and is meant to be compiled using cargo. Run ```cargo build``` or ```cargo build --release``` in the git directory to compile for your environment.

## Call Syntax

Playlist transit expects either the path to the xspf file to read from, the flag ```--stdin```, which directs the program to read the input from standard input.
If neither of these options are provided, and the program is compiled without gui support, the program will ask for the file to read from on the command line (it will also detect if the file asked for is inaccessable, and ask for a new file instead).
If the program is compiled with gui support (the default), it will still support the command line arguments mentioned above. However, if it does not recieve any arguments, it will open a GUI with a button to choose the playlist file to use, and will append to a textbox below said button.
