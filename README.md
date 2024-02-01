# CHIP-8 emulator written in Rust

This is a simple emulator of the [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
system written in Rust,
using SDL for the graphics and sound.

## Running

To run the emulator with a game use it like this:

```shell
$ cargo run -- roms/PONG2
```

Or with the `--release` option to enable optimizations:

```shell
$ cargo run --release -- roms/PONG2
```

The emulator uses the keys 1/2/3/4, Q/W/E/R, A/S/D/F, and Z/X/C/V for input.
