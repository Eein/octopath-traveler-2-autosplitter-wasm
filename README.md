# Octopath Traveler 2 WASM Autosplitter

A Windows and Linux autosplitter for Octopath Traveler 2 

(May work on mac if someone asks for it)

# TODO:

- [x] Character Splits
- [ ] Ending splits
- [ ] Character Joins
- [ ] Enter/Exit Zone
- [ ] Get Job License
- [ ] Get Shrine
- [ ] Chapter Ends on Frame (first black frame when fin screen shows... maybe achievement?)
- [x] Load/AutoSave Removal


## Load/Autosave Removal 

To remove loads and autosaves, enable the setting in the options menu for the Autosplitting Runtime

## Install

Since this autosplitter is in prerelease, you'll need to download the following file and add an "Auto Splitting Runtime" to your layout and add this file. WASM files should be supported in mainline livesplit.

### Windows

https://github.com/Eein/octopath-traveler-2-autosplitter-wasm/releases/latest/download/octopath_traveler_2_autosplitter_wasm.wasm

### Linux

https://github.com/Eein/octopath-traveler-2-autosplitter-wasm/releases/latest/download/octopath_traveler_2_autosplitter_linux_wasm.wasm

## build
1. install rustup + stable rust https://rustup.rs/
2. install wasm target
  - `rustup target add wasm32-unknown-unknown`
3. build wasm file (--release optional)
  - `cargo build --target wasm32-unknown-unknown --release`
