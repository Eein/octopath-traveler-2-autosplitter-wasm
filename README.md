# Octopath Traveler 2 WASM Autosplitter

A Windows and Linux autosplitter for Octopath Traveler 2 

(May work on mac if someone asks for it)

# TODO:

- [x] Character Splits
- [x] Ending splits
- [x] Character Joins
- [x] Enter/Exit Zone
- [x] Get Job License
  - Advanced Jobs do not split until after initial dialog is complete - this is due to the only flags being set are bitflags - another alternative which is very non-performant would be to scan the inventory, not really inclinded to do that to avoid being unusable on older machines

- [ ] Get Shrine
- [x] Chapter Ends on Frame - uses cue card json
- [x] Load/AutoSave Removal


## NOTE

Before this can be merged into Autosplitters.xml it MUST have settings saving

## Load/Autosave Removal 

To remove loads and autosaves, enable the setting in the options menu for the Autosplitting Runtime 

You may add a second timer and configure it for only game time, if you want to track load/save-less runs.

## Install

Since this autosplitter is in prerelease, you'll need to download the following file and add an "Auto Splitting Runtime" to your layout and add this file. WASM files should be supported in mainline livesplit.

### Windows

https://github.com/Eein/octopath-traveler-2-autosplitter-wasm/releases/latest/download/octopath_traveler_2_autosplitter_wasm.wasm

## build
1. install rustup + stable rust https://rustup.rs/
2. install wasm target
  - `rustup target add wasm32-unknown-unknown`
3. build wasm file (--release optional)
  - `cargo build --target wasm32-unknown-unknown --release`
