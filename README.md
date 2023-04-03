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


## Install

Since this autosplitter is in prerelease, you'll need to download the following file and add an autosplitter component to your splits and add this file. WASM files should be supported in mainline livesplit.

Unfortunately, I have OS detection in this, so it may or may not work until its added to the autosplitting runtime in LiveSplit.exe

https://github.com/Eein/octopath-traveler-2-autosplitter-wasm/releases/latest/download/octopath_traveler_2_autosplitter_wasm.wasm

## build
1. install rustup + stable rust https://rustup.rs/
2. install wasm target
  - `rustup target add wasm32-unknown-unknown`
3. build wasm file (--release optional)
  - `cargo build --target wasm32-unknown-unknown --release`
