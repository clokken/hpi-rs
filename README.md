# HPI-rs

HPI-rs is a work-in-progress Rust library and command line program to manipulate .hpi files for the game **Total Annihilation Kingdoms**.

## Features

Currently, only the ability to READ hpi files is supported.

In the future, it will also support the ability to create new hpi files and modify existing ones.

## Installation (as a command line program)

Make sure you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed. Then run the following command on a terminal:
```sh
cargo install --git https://github.com/clokken/hpi-rs
```

Then check out the [wiki](https://github.com/clokken/hpi-rs/wiki/Command-line) to see how to use the commands.

## Installation (as a Rust library)

To use this in a Rust program, simply add this to your project's Cargo.toml dependencies:
```toml
[dependencies]
hpi = { git="https://github.com/clokken/hpi-rs" }
```

Then see this repository (*TODO*) for some examples on how to use the library in a Rust program.

## Notes

This is my first Rust project and I'm using it as a way to learn Rust so the codebase will probably change a lot from version to version.
