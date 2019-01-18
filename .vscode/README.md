## rust-snes_spc

A Rust FFI wrapper for Blargg's excellent emulator for the SNES SPC-700 APU.

## Example

The included example uses SDL2 to turn the emulator's output into actual,
physical vibrations you can process with your earholes, provided of course
that you have an electronical Von Neumann machine equipped with an electro-
magnetic phonograph.

* Ubuntu Linux: `apt install libsdl2-dev`
* Arch Linux: `pacman -S sdl2`
* OSX: `brew install sdl2`
* Windows: shove the .lib and .dll files from https://www.libsdl.org/release/SDL2-devel-2.0.9-VC.zip into your MSVC toolchain's lib directory

Then `cargo run --example sdl` should be sufficient!