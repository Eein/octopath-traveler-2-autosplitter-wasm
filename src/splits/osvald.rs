pub struct OsvaldSplits;
use crate::settings::Settings;
use crate::Vars;

impl OsvaldSplits {
    pub fn split(vars: &mut Vars, settings: &Settings) -> Option<String> {
        if let Some(split) = Self::chapter_split(vars, settings) {
            return Some(split);
        }
        if let Some(split) = Self::main_story_complete(vars, settings) {
            return Some(split);
        }
        if let Some(split) = Self::joins_party(vars, settings) {
            return Some(split);
        }
        None
    }

    fn joins_party(vars: &mut Vars, settings: &Settings) -> Option<String> {
        if vars.osvald_hp.old == 0 && vars.osvald_hp.current != 0 {
            return vars.split("osvald_joins", settings.osvald_joins);
        }
        None
    }

    fn main_story_complete(vars: &mut Vars, settings: &Settings) -> Option<String> {
        if vars.osvald_progress.current == 2500
            && vars.game_state.current == 2
            && vars.event_index.current >= 153
        {
            return vars.split("osvald_story_complete", settings.osvald_story_complete);
        }
        None
    }

    fn chapter_split(vars: &mut Vars, settings: &Settings) -> Option<String> {
        // checks if an old save is lingering, also make sure old zone id isn't 0 later
        if vars.osvald_progress.old != vars.osvald_progress.current {
            match vars.osvald_progress.current {
                p if p == 10 => return vars.split("osvald_10", settings.osvald_10),
                p if p == 20 => return vars.split("osvald_20", settings.osvald_20),
                p if p == 30 => return vars.split("osvald_30", settings.osvald_30),
                p if p == 40 => return vars.split("osvald_40", settings.osvald_40),
                p if p == 50 => return vars.split("osvald_50", settings.osvald_50),
                p if p == 60 => return vars.split("osvald_60", settings.osvald_60),
                p if p == 70 => return vars.split("osvald_70", settings.osvald_70),
                p if p == 80 => return vars.split("osvald_80", settings.osvald_80),
                p if p == 90 => return vars.split("osvald_90", settings.osvald_90),
                p if p == 100 => return vars.split("osvald_100", settings.osvald_100),
                p if p == 120 => return vars.split("osvald_120", settings.osvald_120),
                p if p == 130 => return vars.split("osvald_130", settings.osvald_130),
                p if p == 140 => return vars.split("osvald_140", settings.osvald_140),
                p if p == 160 => return vars.split("osvald_160", settings.osvald_160),
                p if p == 180 => return vars.split("osvald_180", settings.osvald_180),
                p if p == 190 => return vars.split("osvald_190", settings.osvald_190),
                p if p == 200 => return vars.split("osvald_200", settings.osvald_200),
                p if p == 210 => return vars.split("osvald_210", settings.osvald_210),
                p if p == 220 => return vars.split("osvald_220", settings.osvald_220),
                p if p == 500 => return vars.split("osvald_500", settings.osvald_500),
                p if p == 510 => return vars.split("osvald_510", settings.osvald_510),
                p if p == 520 => return vars.split("osvald_520", settings.osvald_520),
                p if p == 530 => return vars.split("osvald_530", settings.osvald_530),
                p if p == 540 => return vars.split("osvald_540", settings.osvald_540),
                p if p == 550 => return vars.split("osvald_550", settings.osvald_550),
                p if p == 560 => return vars.split("osvald_560", settings.osvald_560),
                p if p == 570 => return vars.split("osvald_570", settings.osvald_570),
                p if p == 580 => return vars.split("osvald_580", settings.osvald_580),
                p if p == 590 => return vars.split("osvald_590", settings.osvald_590),
                p if p == 600 => return vars.split("osvald_600", settings.osvald_600),
                p if p == 610 => return vars.split("osvald_610", settings.osvald_610),
                p if p == 620 => return vars.split("osvald_620", settings.osvald_620),
                p if p == 1000 => return vars.split("osvald_1000", settings.osvald_1000),
                p if p == 1010 => return vars.split("osvald_1010", settings.osvald_1010),
                p if p == 1020 => return vars.split("osvald_1020", settings.osvald_1020),
                p if p == 1030 => return vars.split("osvald_1030", settings.osvald_1030),
                p if p == 1040 => return vars.split("osvald_1040", settings.osvald_1040),
                p if p == 1050 => return vars.split("osvald_1050", settings.osvald_1050),
                p if p == 1060 => return vars.split("osvald_1060", settings.osvald_1060),
                p if p == 1070 => return vars.split("osvald_1070", settings.osvald_1070),
                p if p == 1500 => return vars.split("osvald_1500", settings.osvald_1500),
                p if p == 1510 => return vars.split("osvald_1510", settings.osvald_1510),
                p if p == 1520 => return vars.split("osvald_1520", settings.osvald_1520),
                p if p == 1530 => return vars.split("osvald_1530", settings.osvald_1530),
                p if p == 1540 => return vars.split("osvald_1540", settings.osvald_1540),
                p if p == 1560 => return vars.split("osvald_1560", settings.osvald_1560),
                p if p == 1570 => return vars.split("osvald_1570", settings.osvald_1570),
                p if p == 1590 => return vars.split("osvald_1590", settings.osvald_1590),
                p if p == 1600 => return vars.split("osvald_1600", settings.osvald_1600),
                p if p == 1620 => return vars.split("osvald_1620", settings.osvald_1620),
                p if p == 2000 => return vars.split("osvald_2000", settings.osvald_2000),
                p if p == 2010 => return vars.split("osvald_2010", settings.osvald_2010),
                p if p == 2020 => return vars.split("osvald_2020", settings.osvald_2020),
                p if p == 2030 => return vars.split("osvald_2030", settings.osvald_2030),
                p if p == 2040 => return vars.split("osvald_2040", settings.osvald_2040),
                p if p == 2050 => return vars.split("osvald_2050", settings.osvald_2050),
                p if p == 2070 => return vars.split("osvald_2070", settings.osvald_2070),
                p if p == 2080 => return vars.split("osvald_2080", settings.osvald_2080),
                p if p == 2090 => return vars.split("osvald_2090", settings.osvald_2090),
                p if p == 2100 => return vars.split("osvald_2100", settings.osvald_2100),
                p if p == 2110 => return vars.split("osvald_2110", settings.osvald_2110),
                p if p == 2120 => return vars.split("osvald_2120", settings.osvald_2120),
                p if p == 2130 => return vars.split("osvald_2130", settings.osvald_2130),
                p if p == 2500 => return vars.split("osvald_2500", settings.osvald_2500),
                _ => return None,
            }
        } else {
            return None;
        }
    }
}
