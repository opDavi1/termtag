# termtag

A simple audio file tag editor for the terminal.


## How to run
No releases yet, just use
```console
$ cargo run -- --help
```
to try it out.


## A small disclaimer

The program can't actually edit metadata yet. The command line options are there just so I remember which ones I want to add, but they don't do anything as of now.

and YES, I know that It's easier, more maintainable, and safer to use a FLAC library but I wanted to make my own implementation for education. All other audio formats I add in the future will use their respective libraries.


## Compilation

to compile, first make sure that cargo is installed on your system and then run
```console
$ cargo build --release
```

to make the binary.
