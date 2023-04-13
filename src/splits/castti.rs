pub struct CasttiSplits;
use crate::settings::Settings;
use crate::Vars;

impl CasttiSplits {
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
        if vars.castti_hp.old == 0 && vars.castti_hp.current != 0 {
            return vars.split("castti_joins", settings.castti_joins);
        }
        None
    }

    fn main_story_complete(vars: &mut Vars, settings: &Settings) -> Option<String> {
        if vars.castti_progress.current == 2500
            && vars.game_state.current == 2
            && vars.event_index.current >= 128
        {
            return vars.split("castti_story_complete", settings.castti_story_complete);
        }
        None
    }

    fn chapter_split(vars: &mut Vars, settings: &Settings) -> Option<String> {
        // checks if an old save is lingering, also make sure old zone id isn't 0 later
        if vars.castti_progress.old != vars.castti_progress.current {
            match vars.castti_progress.current {
                p if p == 10 => return vars.split("castti_10", settings.castti_10),
                p if p == 20 => return vars.split("castti_20", settings.castti_20),
                p if p == 30 => return vars.split("castti_30", settings.castti_30),
                p if p == 40 => return vars.split("castti_40", settings.castti_40),
                p if p == 50 => return vars.split("castti_50", settings.castti_50),
                p if p == 60 => return vars.split("castti_60", settings.castti_60),
                p if p == 70 => return vars.split("castti_70", settings.castti_70),
                p if p == 80 => return vars.split("castti_80", settings.castti_80),
                p if p == 90 => return vars.split("castti_90", settings.castti_90),
                p if p == 100 => return vars.split("castti_100", settings.castti_100),
                p if p == 110 => return vars.split("castti_110", settings.castti_110),
                p if p == 120 => return vars.split("castti_120", settings.castti_120),
                p if p == 140 => return vars.split("castti_140", settings.castti_140),
                p if p == 150 => return vars.split("castti_150", settings.castti_150),
                p if p == 160 => return vars.split("castti_160", settings.castti_160),
                p if p == 170 => return vars.split("castti_170", settings.castti_170),
                p if p == 180 => return vars.split("castti_180", settings.castti_180),
                p if p == 500 => return vars.split("castti_500", settings.castti_500),
                p if p == 510 => return vars.split("castti_510", settings.castti_510),
                p if p == 520 => return vars.split("castti_520", settings.castti_520),
                p if p == 530 => return vars.split("castti_530", settings.castti_530),
                p if p == 540 => return vars.split("castti_540", settings.castti_540),
                p if p == 550 => return vars.split("castti_550", settings.castti_550),
                p if p == 560 => return vars.split("castti_560", settings.castti_560),
                p if p == 570 => return vars.split("castti_570", settings.castti_570),
                p if p == 580 => return vars.split("castti_580", settings.castti_580),
                p if p == 590 => return vars.split("castti_590", settings.castti_590),
                p if p == 600 => return vars.split("castti_600", settings.castti_600),
                p if p == 610 => return vars.split("castti_610", settings.castti_610),
                p if p == 620 => return vars.split("castti_620", settings.castti_620),
                p if p == 630 => return vars.split("castti_630", settings.castti_630),
                p if p == 640 => return vars.split("castti_640", settings.castti_640),
                p if p == 650 => return vars.split("castti_650", settings.castti_650),
                p if p == 660 => return vars.split("castti_660", settings.castti_660),
                p if p == 1000 => return vars.split("castti_1000", settings.castti_1000),
                p if p == 1010 => return vars.split("castti_1010", settings.castti_1010),
                p if p == 1020 => return vars.split("castti_1020", settings.castti_1020),
                p if p == 1030 => return vars.split("castti_1030", settings.castti_1030),
                p if p == 1040 => return vars.split("castti_1040", settings.castti_1040),
                p if p == 1050 => return vars.split("castti_1050", settings.castti_1050),
                p if p == 1070 => return vars.split("castti_1070", settings.castti_1070),
                p if p == 1080 => return vars.split("castti_1080", settings.castti_1080),
                p if p == 1090 => return vars.split("castti_1090", settings.castti_1090),
                p if p == 1100 => return vars.split("castti_1100", settings.castti_1100),
                p if p == 1110 => return vars.split("castti_1110", settings.castti_1110),
                p if p == 1120 => return vars.split("castti_1120", settings.castti_1120),
                p if p == 1130 => return vars.split("castti_1130", settings.castti_1130),
                p if p == 1140 => return vars.split("castti_1140", settings.castti_1140),
                p if p == 1150 => return vars.split("castti_1150", settings.castti_1150),
                p if p == 1160 => return vars.split("castti_1160", settings.castti_1160),
                p if p == 1170 => return vars.split("castti_1170", settings.castti_1170),
                p if p == 1500 => return vars.split("castti_1500", settings.castti_1500),
                p if p == 1510 => return vars.split("castti_1510", settings.castti_1510),
                p if p == 1520 => return vars.split("castti_1520", settings.castti_1520),
                p if p == 1530 => return vars.split("castti_1530", settings.castti_1530),
                p if p == 1540 => return vars.split("castti_1540", settings.castti_1540),
                p if p == 1550 => return vars.split("castti_1550", settings.castti_1550),
                p if p == 1560 => return vars.split("castti_1560", settings.castti_1560),
                p if p == 1570 => return vars.split("castti_1570", settings.castti_1570),
                p if p == 1580 => return vars.split("castti_1580", settings.castti_1580),
                p if p == 1590 => return vars.split("castti_1590", settings.castti_1590),
                p if p == 1600 => return vars.split("castti_1600", settings.castti_1600),
                p if p == 1610 => return vars.split("castti_1610", settings.castti_1610),
                p if p == 1620 => return vars.split("castti_1620", settings.castti_1620),
                p if p == 1630 => return vars.split("castti_1630", settings.castti_1630),
                p if p == 1640 => return vars.split("castti_1640", settings.castti_1640),
                p if p == 1650 => return vars.split("castti_1650", settings.castti_1650),
                p if p == 1660 => return vars.split("castti_1660", settings.castti_1660),
                p if p == 2000 => return vars.split("castti_2000", settings.castti_2000),
                p if p == 2010 => return vars.split("castti_2010", settings.castti_2010),
                p if p == 2020 => return vars.split("castti_2020", settings.castti_2020),
                p if p == 2030 => return vars.split("castti_2030", settings.castti_2030),
                p if p == 2040 => return vars.split("castti_2040", settings.castti_2040),
                p if p == 2050 => return vars.split("castti_2050", settings.castti_2050),
                p if p == 2060 => return vars.split("castti_2060", settings.castti_2060),
                p if p == 2070 => return vars.split("castti_2070", settings.castti_2070),
                p if p == 2080 => return vars.split("castti_2080", settings.castti_2080),
                p if p == 2090 => return vars.split("castti_2090", settings.castti_2090),
                p if p == 2100 => return vars.split("castti_2100", settings.castti_2100),
                p if p == 2110 => return vars.split("castti_2110", settings.castti_2110),
                p if p == 2120 => return vars.split("castti_2120", settings.castti_2120),
                p if p == 2500 => return vars.split("castti_2500", settings.castti_2500),
                _ => return None,
            }
        } else {
            return None;
        }
    }
}
