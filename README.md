# ExcertBerk!

Definitely not a cheap ExciteBike clone!

## For the developments.

This project is written in [Rust], using [ggez].

For working with Rust, I recommend installing the toolchain via the 
[rustup] tool. As far as I know right now, we'll be targeting the stable 
1.21.0 release of Rust.

The ggez framework is built on top of SDL2, a native library, which requires 
some development headers at build time. The SDL2 rust bindings
[document instructions][SDL2 Install Instructions]
for this, so _please follow the instructions carefully_.

I'm not sure what we will need for distribution at this point. It should be 
possible to compile a static binary for mac/linux, but on Windows they 
recommend distributing a pre-compiled DLL, which will be checked in at the 
root of the repo.

With your toolchain installed, you may also enjoy installing some cli tools 
such as:

name         | description
-------------|------------
`watchexec`  | a simple tool to re-run a command when a directory files change
`clippy`     | a linter for rust projects
`rustfmt`    | a tool for conforming your source to a style guide

These can all be installed using `cargo install <name>`.


[ggez]: http://ggez.rs/
[rustup]: https://rustup.rs
[Rust]: https://www.rust-lang.org
[SDL2 Install Instructions]: https://github.com/Rust-SDL2/rust-sdl2#user-content-requirements
