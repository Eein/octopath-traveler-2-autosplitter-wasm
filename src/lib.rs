// #![no_std]
use spinning_top::{const_spinlock, Spinlock};
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};
use std::env;

use bytemuck::Pod;

use asr::{
    timer::{self, TimerState},
    watcher::Pair,
    Address, Process,
    get_os
};

// mod data;
// use data::zone::{ADVANCED_JOB_FIGHTS, AREAS, SHRINES};

static STATE: Spinlock<State> = const_spinlock(State { game: None });

struct Watcher<T> {
    watcher: asr::watcher::Watcher<T>,
    address: Vec<u64>,
}

impl<T: Pod> Watcher<T> {
    fn new(address: Vec<u64>) -> Self {
        Self {
            watcher: asr::watcher::Watcher::new(),
            address,
        }
    }

    fn update(&mut self, process: &Process, module: u64) -> Option<&Pair<T>> {
        let value = process.read_pointer_path64::<T>(module, &self.address);
        self.watcher.update(value.ok())
    }
}

struct Game {
    process: Process,
    module: u64,
    splits: HashSet<String>,
    game_state: Watcher<u8>,
    start: Watcher<u8>,
}

impl Game {
    fn new(process: Process, module: u64) -> Option<Self> {
        let game = Self {
            process,
            module,
            start: Watcher::new(vec![0x5219628, 0xA8]),
            game_state: Watcher::new(vec![0x4F7AB68, 0x234]),
            splits: HashSet::new(),
        };
        Some(game)
    }

    fn update_vars(&mut self) -> Option<Vars<'_>> {
        Some(Vars {
            start: self.start.update(&self.process, self.module)?,
            game_state: self.game_state.update(&self.process, self.module)?,
            splits: &mut self.splits,
        })
    }
}

pub struct State {
    game: Option<Game>,
}

// This enum maps to the SavePlayerCharacterData 
#[derive(Default, PartialEq)]
pub enum Character {
    #[default]
    NoCharacter = -1,
    Hikari = 0,
    Ochette = 1,
    Castti = 2,
    Partitio = 3,
    Temenos = 4,
    Osvald = 5,
    Throne = 6,
    Agnea = 7,
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Character::NoCharacter => write!(f, "none"),
            Character::Hikari => write!(f, "hikari"),
            Character::Ochette => write!(f, "ochette"),
            Character::Castti => write!(f, "castti"),
            Character::Partitio => write!(f, "partitio"),
            Character::Temenos => write!(f, "temenos"),
            Character::Osvald => write!(f, "osvald"),
            Character::Throne => write!(f, "throne"),
            Character::Agnea => write!(f, "agnea"),
        }
    }
}

#[derive(Default)]
pub struct Flags {
    char_chapter_ending: Character,
}

#[allow(unused)]
struct Vars<'a> {
    start: &'a Pair<u8>,
    game_state: &'a Pair<u8>,
    splits: &'a mut HashSet<String>,
}

impl Vars<'_> {
    fn split(&mut self, key: &str) -> Option<String> {
        if self.splits.contains(key) {
            return None;
        }
        self.splits.insert(key.to_string());

        Some(key.to_string())
    }
}

pub struct Splits(HashSet<String>);

#[no_mangle]
pub extern "C" fn update() {
    let mut state = STATE.lock();
    if state.game.is_none() {
        let os = get_os().unwrap();
        let process_for_os = match os.as_str() {
            "windows" => "Octopath_Traveler2",
            "linux" => "Octopath_Travel",
            _ => "Octopath_Traveler2"
        };
        match Process::attach(process_for_os) {
            Some(process) => {
                match process.get_module_address("Octopath_Traveler2-Win64-Shipping.exe") {
                    Ok(Address(module)) => {
                        asr::print_message("attached to process");

                        state.game = Game::new(process, module)
                    }
                    _ => (),
                };
            }
            None => (),
        }
    }

    if let Some(game) = &mut state.game {
        if !game.process.is_open() {
            state.game = None;
            return;
        }
                if let Some(vars) = game.update_vars() {
            // timer::set_variable_int("Encounters", *vars.encounters);
            // timer::set_variable_int("Deaths", *vars.deaths);
            match timer::state() {
                TimerState::NotRunning => {
                    if vars.game_state.current == 1 && vars.start.old == 0 && vars.start.current == 1 {
                        // *vars.deaths = 0;
                        // *vars.encounters = 0;
                        // *vars.splits = Default::default();
                        // *vars.flags = Default::default();
                        timer::start()
                    }
                }
                TimerState::Running => {
                    // if let Some(reason) = should_split(&mut vars) {
                    //     asr::print_message(&reason);
                    //     timer::split();
                    // }
                    // if vars.game_state.current == 6 && vars.game_state.old == 2 { 
                    //     *vars.encounters = *vars.encounters + 1;
                    // } 
                    // if vars.game_state.current == 7 && vars.game_state.old == 6 { 
                    //     *vars.deaths = *vars.deaths + 1;
                    // }
                }
                _ => {}
            }
        }

    }
}

fn should_split(vars: &mut Vars) -> Option<String> {
    None
}
