// #![no_std]
use spinning_top::{const_spinlock, Spinlock};
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};
mod data;
mod settings;
mod splits;
use settings::Settings;

use bytemuck::Pod;

use asr::{
    timer::{self, TimerState},
    watcher::Pair,
    Address, Process,
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
    hikari_progress: Watcher<u16>,
    ochette_progress: Watcher<u16>,
    castti_progress: Watcher<u16>,
    partitio_progress: Watcher<u16>,
    temenos_progress: Watcher<u16>,
    osvald_progress: Watcher<u16>,
    throne_progress: Watcher<u16>,
    agnea_progress: Watcher<u16>,
    settings: Settings,
    loading: Watcher<u8>,
    saving: Watcher<u8>,
    start: Watcher<u8>,
}

impl Game {
    fn new(process: Process, module: u64) -> Option<Self> {
        let game = Self {
            process,
            module,
            start: Watcher::new(vec![0x5219628, 0xA8]),
            settings: Settings::register(),
            game_state: Watcher::new(vec![0x4F7AB68, 0x234]),
            loading: Watcher::new(vec![0x4F7CDD8, 0x308, 0x1A8]),
            saving: Watcher::new(vec![0x4F7CDD8, 0x310, 0x1A8]),
            hikari_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x0 + 0xEC]),
            ochette_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0xF0 + 0xEC]),
            castti_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x1E0 + 0xEC]),
            partitio_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x2D0 + 0xEC]),
            temenos_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x3C0 + 0xEC]),
            osvald_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x4B0 + 0xEC]),
            throne_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x5A0 + 0xEC]),
            agnea_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x690 + 0xEC]),
            splits: HashSet::new(),
        };
        Some(game)
    }

    fn update_vars(&mut self) -> Option<Vars<'_>> {
        Some(Vars {
            start: self.start.update(&self.process, self.module)?,
            loading: self.loading.update(&self.process, self.module)?,
            saving: self.saving.update(&self.process, self.module)?,
            game_state: self.game_state.update(&self.process, self.module)?,
            hikari_progress: self.hikari_progress.update(&self.process, self.module)?,
            ochette_progress: self.ochette_progress.update(&self.process, self.module)?,
            castti_progress: self.castti_progress.update(&self.process, self.module)?,
            partitio_progress: self.partitio_progress.update(&self.process, self.module)?,
            temenos_progress: self.temenos_progress.update(&self.process, self.module)?,
            osvald_progress: self.osvald_progress.update(&self.process, self.module)?,
            throne_progress: self.throne_progress.update(&self.process, self.module)?,
            agnea_progress: self.agnea_progress.update(&self.process, self.module)?,
            settings: &self.settings,
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

// #[derive(Default)]
// pub struct Flags {
//     char_chapter_ending: Character,
// }

#[allow(unused)]
pub struct Vars<'a> {
    start: &'a Pair<u8>,
    loading: &'a Pair<u8>,
    saving: &'a Pair<u8>,
    game_state: &'a Pair<u8>,
    hikari_progress: &'a Pair<u16>,
    ochette_progress: &'a Pair<u16>,
    castti_progress: &'a Pair<u16>,
    partitio_progress: &'a Pair<u16>,
    temenos_progress: &'a Pair<u16>,
    osvald_progress: &'a Pair<u16>,
    throne_progress: &'a Pair<u16>,
    agnea_progress: &'a Pair<u16>,
    settings: &'a Settings,
    splits: &'a mut HashSet<String>,
}

impl Vars<'_> {
    fn split(&mut self, key: &str, settings_field: bool) -> Option<String> {
        if self.splits.contains(key) {
            return None;
        }

        self.splits.insert(key.to_string());

        // only split if in settings
        if settings_field {
            return Some(key.to_string());
        }

        None
    }
    fn clear_splits(&mut self) {
        self.splits.clear()
    }
}

pub struct Splits(HashSet<String>);

#[cfg_attr(feature = "linux", path = "linux_process.rs")]
#[cfg_attr(feature = "windows", path = "windows_process.rs")]
mod os_process;

#[no_mangle]
pub extern "C" fn update() {
    let mut state = STATE.lock();
    if state.game.is_none() {
        match Process::attach(&os_process::get_process_name()) {
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

        if let Some(mut vars) = game.update_vars() {
            match timer::state() {
                TimerState::NotRunning => {
                    vars.clear_splits();
                    if vars.game_state.current == 1
                        && vars.start.old == 0
                        && vars.start.current == 1
                    {
                        timer::start()
                    }
                }
                TimerState::Running => {
                    if let Some(reason) = should_split(&mut vars) {
                        asr::print_message(&reason);
                        timer::split();
                    }
                    // load/save removal
                    if (vars.loading.old == 0 && vars.loading.current != 0) || (vars.saving.old == 0 && vars.saving.current != 0)  {
                        timer::pause_game_time()
                    }

                    if vars.loading.current == 0 && vars.saving.current == 0  {
                        timer::resume_game_time()
                    }

                }
                _ => {}
            }
        }
    }
}

fn should_split(vars: &mut Vars) -> Option<String> {
    // Hikari
    if let Some(split) = splits::hikari::HikariSplits::chapter_split(vars) {
        return Some(split);
    }

    // Castti
    if let Some(split) = splits::castti::CasttiSplits::chapter_split(vars) {
        return Some(split);
    }

    // Agnea
    if let Some(split) = splits::agnea::AgneaSplits::chapter_split(vars) {
        return Some(split);
    }

    // Ochette
    if let Some(split) = splits::ochette::OchetteSplits::chapter_split(vars) {
        return Some(split);
    }

    // Osvald
    if let Some(split) = splits::osvald::OsvaldSplits::chapter_split(vars) {
        return Some(split);
    }

    // Partitio
    if let Some(split) = splits::partitio::PartitioSplits::chapter_split(vars) {
        return Some(split);
    }

    // Temenos
    if let Some(split) = splits::temenos::TemenosSplits::chapter_split(vars) {
        return Some(split);
    }

    // Throne
    if let Some(split) = splits::throne::ThroneSplits::chapter_split(vars) {
        return Some(split);
    }

    None
}
