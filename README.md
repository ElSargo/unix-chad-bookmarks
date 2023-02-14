# Unix chad bookmarks
Simple universal bookmarking utllity inspired by a [video make by luke smith](https://www.youtube.com/watch?v=d_11QaTlf1I).
It feeds a list of options into dmenu and runs the command associated with it.  
The syntax is very simple, for each entry make a new line, then the name, a pipe and the command
```
NAME | COMMAND
```
For example
```
El_Sargo gihub | firefox https://github.com/El_Sargo
```
Anything else will be treated as a comment, see the bookmarks file for examples


## Building and running
Dmenu must be installed  
Install with cargo
Build
```fish
git clone https://github.com/ElSargo/new-terminal-hyprland
cd new-termainal-hyprland
```
Try it
```fish
cargo run
```

Install
```fish
cargo install --path ./
```

The bookmark file should be located at in ~/.config/boomarks
You can of course invoke it from the command line but I have it bound to a key

