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

use data::zone;

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
    hikari_hp: Watcher<u16>,
    ochette_progress: Watcher<u16>,
    ochette_hp: Watcher<u16>,
    castti_progress: Watcher<u16>,
    castti_hp: Watcher<u16>,
    partitio_progress: Watcher<u16>,
    partitio_hp: Watcher<u16>,
    temenos_progress: Watcher<u16>,
    temenos_hp: Watcher<u16>,
    osvald_progress: Watcher<u16>,
    osvald_hp: Watcher<u16>,
    throne_progress: Watcher<u16>,
    throne_hp: Watcher<u16>,
    agnea_progress: Watcher<u16>,
    agnea_hp: Watcher<u16>,
    settings: Settings,
    loading: Watcher<u16>,
    saving: Watcher<u16>,
    start: Watcher<u16>,
    level_id: Watcher<u16>,
    event_index: Watcher<u16>,
    job_license_inventor: Watcher<u16>,
    job_license_hunter: Watcher<u16>,
    job_license_thief: Watcher<u16>,
    job_license_cleric: Watcher<u16>,
    job_license_scholar: Watcher<u16>,
    job_license_merchant: Watcher<u16>,
    job_license_dancer: Watcher<u16>,
    job_license_warrior: Watcher<u16>,
    dialog: Watcher<u8>,
}

impl Game {
    fn new(process: Process, module: u64) -> Option<Self> {
        let game = Self {
            process,
            module,
            start: Watcher::new(vec![0x51E2190, 0x18, 0x4]),
            level_id: Watcher::new(vec![0x4F7BBE0, 0x470]),
            dialog: Watcher::new(vec![0x5189F00, 0x20, 0xC8, 0x278, 0x10, 0x308]),
            settings: Settings::register(),
            game_state: Watcher::new(vec![0x4F7AB68, 0x234]),
            loading: Watcher::new(vec![0x4F7CDD8, 0x308, 0x2C0]),
            saving: Watcher::new(vec![0x4F7CDD8, 0x310, 0x280, 0x0c]),
            hikari_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x0 + 0xEC]),
            hikari_hp: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x0 + 0xC]),
            ochette_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0xF0 + 0xEC]),
            ochette_hp: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0xF0 + 0xC]),
            castti_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x1E0 + 0xEC]),
            castti_hp: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x1E0 + 0xC]),
            partitio_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x2D0 + 0xEC]),
            partitio_hp: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x2D0 + 0xC]),
            temenos_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x3C0 + 0xEC]),
            temenos_hp: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x3C0 + 0xC]),
            osvald_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x4B0 + 0xEC]),
            osvald_hp: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x4B0 + 0xC]),
            throne_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x5A0 + 0xEC]),
            throne_hp: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x5A0 + 0xC]),
            agnea_progress: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x690 + 0xEC]),
            agnea_hp: Watcher::new(vec![0x4F7AB30, 0x2D8, 0x708, 0x690 + 0xC]),
            job_license_inventor: Watcher::new(vec![0x4F7AB30, 0x2D8, 0xB88, 0x24 + 0x8]),
            job_license_hunter: Watcher::new(vec![0x4F7AB30, 0x2D8, 0xB88, 0x30 + 0x4]),
            job_license_thief: Watcher::new(vec![0x4F7AB30, 0x2D8, 0xB88, 0xc + 0x4]),
            job_license_cleric: Watcher::new(vec![0x4F7AB30, 0x2D8, 0xB88, 0x0 + 0x4]),
            job_license_scholar: Watcher::new(vec![0x4F7AB30, 0x2D8, 0xB88, 0x18 + 0x4]),
            job_license_merchant: Watcher::new(vec![0x4F7AB30, 0x2D8, 0xB88, 0x54 + 0x4]),
            job_license_dancer: Watcher::new(vec![0x4F7AB30, 0x2D8, 0xB88, 0x6c + 0x4]),
            job_license_warrior: Watcher::new(vec![0x4F7AB30, 0x2D8, 0xB88, 0x60 + 0x4]),
            event_index: Watcher::new(vec![0x4F7B1E0, 0x298]),
            splits: HashSet::new(),
        };
        Some(game)
    }

    fn update_vars(&mut self) -> Option<Vars<'_>> {
        Some(Vars {
            start: self.start.update(&self.process, self.module)?,
            level_id: self.level_id.update(&self.process, self.module)?,
            dialog: self.dialog.update(&self.process, self.module)?,
            loading: match self.loading.update(&self.process, self.module) {
                Some(update) => update,
                None => &Pair { old: 0, current: 0 },
            },
            saving: match self.saving.update(&self.process, self.module) {
                Some(update) => update,
                None => &Pair { old: 0, current: 0 },
            },
            game_state: self.game_state.update(&self.process, self.module)?,
            hikari_progress: self.hikari_progress.update(&self.process, self.module)?,
            hikari_hp: self.hikari_hp.update(&self.process, self.module)?,
            ochette_progress: self.ochette_progress.update(&self.process, self.module)?,
            ochette_hp: self.ochette_hp.update(&self.process, self.module)?,
            castti_progress: self.castti_progress.update(&self.process, self.module)?,
            castti_hp: self.castti_hp.update(&self.process, self.module)?,
            partitio_progress: self.partitio_progress.update(&self.process, self.module)?,
            partitio_hp: self.partitio_hp.update(&self.process, self.module)?,
            temenos_progress: self.temenos_progress.update(&self.process, self.module)?,
            temenos_hp: self.temenos_hp.update(&self.process, self.module)?,
            osvald_progress: self.osvald_progress.update(&self.process, self.module)?,
            osvald_hp: self.osvald_hp.update(&self.process, self.module)?,
            throne_progress: self.throne_progress.update(&self.process, self.module)?,
            throne_hp: self.throne_hp.update(&self.process, self.module)?,
            agnea_progress: self.agnea_progress.update(&self.process, self.module)?,
            agnea_hp: self.agnea_hp.update(&self.process, self.module)?,
            event_index: self.event_index.update(&self.process, self.module)?,
            job_license_inventor: self
                .job_license_inventor
                .update(&self.process, self.module)?,
            job_license_hunter: self.job_license_hunter.update(&self.process, self.module)?,
            job_license_thief: self.job_license_thief.update(&self.process, self.module)?,
            job_license_cleric: self.job_license_cleric.update(&self.process, self.module)?,
            job_license_scholar: self
                .job_license_scholar
                .update(&self.process, self.module)?,
            job_license_merchant: self
                .job_license_merchant
                .update(&self.process, self.module)?,
            job_license_dancer: self.job_license_dancer.update(&self.process, self.module)?,
            job_license_warrior: self
                .job_license_warrior
                .update(&self.process, self.module)?,
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

#[allow(unused)]
pub struct Vars<'a> {
    start: &'a Pair<u16>,
    level_id: &'a Pair<u16>,
    dialog: &'a Pair<u8>,
    loading: &'a Pair<u16>,
    saving: &'a Pair<u16>,
    game_state: &'a Pair<u8>,
    hikari_progress: &'a Pair<u16>,
    hikari_hp: &'a Pair<u16>,
    ochette_progress: &'a Pair<u16>,
    ochette_hp: &'a Pair<u16>,
    castti_progress: &'a Pair<u16>,
    castti_hp: &'a Pair<u16>,
    partitio_progress: &'a Pair<u16>,
    partitio_hp: &'a Pair<u16>,
    temenos_progress: &'a Pair<u16>,
    temenos_hp: &'a Pair<u16>,
    osvald_progress: &'a Pair<u16>,
    osvald_hp: &'a Pair<u16>,
    throne_progress: &'a Pair<u16>,
    throne_hp: &'a Pair<u16>,
    agnea_progress: &'a Pair<u16>,
    agnea_hp: &'a Pair<u16>,
    event_index: &'a Pair<u16>,
    job_license_inventor: &'a Pair<u16>,
    job_license_hunter: &'a Pair<u16>,
    job_license_thief: &'a Pair<u16>,
    job_license_cleric: &'a Pair<u16>,
    job_license_scholar: &'a Pair<u16>,
    job_license_merchant: &'a Pair<u16>,
    job_license_dancer: &'a Pair<u16>,
    job_license_warrior: &'a Pair<u16>,
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

#[no_mangle]
pub extern "C" fn update() {
    let mut state = STATE.lock();
    if state.game.is_none() {
        match Process::attach("Octopath_Travel") {
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

    if state.game.is_none() {
        match Process::attach("Octopath_Traveler2") {
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

                    if vars.game_state.current == 1 && vars.start.old < vars.start.current {
                        timer::start()
                    }
                }
                TimerState::Running => {
                    if let Some(reason) = should_split(&mut vars) {
                        asr::print_message(&reason);
                        timer::split();
                    }

                    if vars.level_id.old != vars.level_id.current {
                        // let debug_text = format!("CHANGING ZONES from {} to {}", vars.level_id.old, vars.level_id.current);
                        // asr::print_message(&debug_text);
                        // trigger enter
                        if let Some(_split) = zone::Areas::enter_area(&mut vars) {
                            timer::split();
                        }
                        if let Some(_split) = zone::Areas::exit_area(&mut vars) {
                            timer::split();
                        }
                    }

                    if vars.settings.load_removal {
                        // load/save removal
                        if vars.loading.current == 0 && vars.saving.old > vars.saving.current {
                            timer::resume_game_time()
                        }

                        if (vars.loading.old == 0 && vars.loading.current != 0)
                            || (vars.saving.current != 0 && vars.saving.old < vars.saving.current)
                        {
                            timer::pause_game_time()
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn should_split(vars: &mut Vars) -> Option<String> {
    if vars.job_license_inventor.old == 0 && vars.job_license_inventor.current == 1 {
        return vars.split("job_license_inventor", vars.settings.job_license_inventor);
    }
    if vars.job_license_hunter.old == 0 && vars.job_license_hunter.current == 1 {
        return vars.split("job_license_hunter", vars.settings.job_license_hunter);
    }
    if vars.job_license_thief.old == 0 && vars.job_license_thief.current == 1 {
        return vars.split("job_license_thief", vars.settings.job_license_thief);
    }
    if vars.job_license_cleric.old == 0 && vars.job_license_cleric.current == 1 {
        return vars.split("job_license_cleric", vars.settings.job_license_cleric);
    }
    if vars.job_license_scholar.old == 0 && vars.job_license_scholar.current == 1 {
        return vars.split("job_license_scholar", vars.settings.job_license_scholar);
    }
    if vars.job_license_merchant.old == 0 && vars.job_license_merchant.current == 1 {
        return vars.split("job_license_merchant", vars.settings.job_license_merchant);
    }
    if vars.job_license_dancer.old == 0 && vars.job_license_dancer.current == 1 {
        return vars.split("job_license_dancer", vars.settings.job_license_dancer);
    }
    if vars.job_license_warrior.old == 0 && vars.job_license_warrior.current == 1 {
        return vars.split("job_license_warrior", vars.settings.job_license_warrior);
    }
    if let Some(split) = splits::hikari::HikariSplits::split(vars) {
        return Some(split);
    }
    if let Some(split) = splits::castti::CasttiSplits::split(vars) {
        return Some(split);
    }
    if let Some(split) = splits::agnea::AgneaSplits::split(vars) {
        return Some(split);
    }
    if let Some(split) = splits::ochette::OchetteSplits::split(vars) {
        return Some(split);
    }
    if let Some(split) = splits::osvald::OsvaldSplits::split(vars) {
        return Some(split);
    }
    if let Some(split) = splits::partitio::PartitioSplits::split(vars) {
        return Some(split);
    }
    if let Some(split) = splits::temenos::TemenosSplits::split(vars) {
        return Some(split);
    }
    if let Some(split) = splits::throne::ThroneSplits::split(vars) {
        return Some(split);
    }

    None
}
