# Octopath Traveler WASM Autosplitter

# TODO:
- [ ] Correctly handle reset states (if they are even implemented yet)
- [ ] Waiting on settings to be added to implement user configuration
- [x] Waiting on tie-breaker PR to be merged https://github.com/LiveSplit/livesplit-core/pull/589
- [ ] Add XBOX and Windows Steam checks
- [ ] Optimize data structures further (mostly a cut and paste job for now)
- [ ] Add additional features requested (Golden Axe)
- [ ] Figure out better way to split on shrines

## Install

https://github.com/Eein/octopath-traveler-autosplitter-wasm/releases/latest/download/octopath_traveler_autosplitter_wasm.wasm

## build
1. install rustup + stable rust https://rustup.rs/
2. install wasm target
  - `rustup target add wasm32-unknown-unknown`
3. build wasm file (--release optional)
  - `cargo build --target wasm32-unknown-unknown --release`
