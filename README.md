# spacker
A CLI tool to bundle all files found in an fxmanifest.lua file into a zip file, for easy distribution.
# Installation
- Download the latest release from the Releases section
- Add the spacker binary path to your PATH enviromental variable

# Usage
- Invoke the program like: `spacker resource/fxmanifest.lua`, or just `spacker` if the manifest file is in the current directory.


# Building from source
- You have to have the [Rust toolchain](https://rustup.rs/) installed 
- In the `spacker` directory, run `cargo build --release`
