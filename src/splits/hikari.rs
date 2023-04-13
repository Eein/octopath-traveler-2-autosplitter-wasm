pub struct HikariSplits;
use crate::settings::Settings;
use crate::Vars;

impl HikariSplits {
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
        if vars.hikari_hp.old == 0 && vars.hikari_hp.current != 0 {
            return vars.split("hikari_joins", settings.hikari_joins);
        }
        None
    }

    fn main_story_complete(vars: &mut Vars, settings: &Settings) -> Option<String> {
        if vars.hikari_progress.current == 2500
            && vars.game_state.current == 2
            && vars.event_index.current >= 145
        {
            return vars.split("hikari_story_complete", settings.hikari_story_complete);
        }
        None
    }

    pub fn chapter_split(vars: &mut Vars, settings: &Settings) -> Option<String> {
        // checks if an old save is lingering, also make sure old zone id isn't 0 later
        if vars.hikari_progress.old != vars.hikari_progress.current {
            match vars.hikari_progress.current {
                p if p == 10 => return vars.split("hikari_10", settings.hikari_10),
                p if p == 30 => return vars.split("hikari_30", settings.hikari_30),
                p if p == 50 => return vars.split("hikari_50", settings.hikari_50),
                p if p == 60 => return vars.split("hikari_60", settings.hikari_60),
                p if p == 70 => return vars.split("hikari_70", settings.hikari_70),
                p if p == 80 => return vars.split("hikari_80", settings.hikari_80),
                p if p == 90 => return vars.split("hikari_90", settings.hikari_90),
                p if p == 100 => return vars.split("hikari_100", settings.hikari_100),
                p if p == 120 => return vars.split("hikari_120", settings.hikari_120),
                p if p == 130 => return vars.split("hikari_130", settings.hikari_130),
                p if p == 140 => return vars.split("hikari_140", settings.hikari_140),
                p if p == 150 => return vars.split("hikari_150", settings.hikari_150),
                p if p == 170 => return vars.split("hikari_170", settings.hikari_170),
                p if p == 190 => return vars.split("hikari_190", settings.hikari_190),
                p if p == 200 => return vars.split("hikari_200", settings.hikari_200),
                p if p == 210 => return vars.split("hikari_210", settings.hikari_210),
                p if p == 220 => return vars.split("hikari_220", settings.hikari_220),
                p if p == 230 => return vars.split("hikari_230", settings.hikari_230),
                p if p == 240 => return vars.split("hikari_240", settings.hikari_240),
                p if p == 250 => return vars.split("hikari_250", settings.hikari_250),
                p if p == 500 => return vars.split("hikari_500", settings.hikari_500),
                p if p == 510 => return vars.split("hikari_510", settings.hikari_510),
                p if p == 520 => return vars.split("hikari_520", settings.hikari_520),
                p if p == 530 => return vars.split("hikari_530", settings.hikari_530),
                p if p == 540 => return vars.split("hikari_540", settings.hikari_540),
                p if p == 550 => return vars.split("hikari_550", settings.hikari_550),
                p if p == 560 => return vars.split("hikari_560", settings.hikari_560),
                p if p == 580 => return vars.split("hikari_580", settings.hikari_580),
                p if p == 590 => return vars.split("hikari_590", settings.hikari_590),
                p if p == 600 => return vars.split("hikari_600", settings.hikari_600),
                p if p == 630 => return vars.split("hikari_630", settings.hikari_630),
                p if p == 640 => return vars.split("hikari_640", settings.hikari_640),
                p if p == 650 => return vars.split("hikari_650", settings.hikari_650),
                p if p == 660 => return vars.split("hikari_660", settings.hikari_660),
                p if p == 670 => return vars.split("hikari_670", settings.hikari_670),
                p if p == 690 => return vars.split("hikari_690", settings.hikari_690),
                p if p == 700 => return vars.split("hikari_700", settings.hikari_700),
                p if p == 710 => return vars.split("hikari_710", settings.hikari_710),
                p if p == 730 => return vars.split("hikari_730", settings.hikari_730),
                p if p == 1000 => return vars.split("hikari_1000", settings.hikari_1000),
                p if p == 1010 => return vars.split("hikari_1010", settings.hikari_1010),
                p if p == 1020 => return vars.split("hikari_1020", settings.hikari_1020),
                p if p == 1030 => return vars.split("hikari_1030", settings.hikari_1030),
                p if p == 1040 => return vars.split("hikari_1040", settings.hikari_1040),
                p if p == 1050 => return vars.split("hikari_1050", settings.hikari_1050),
                p if p == 1060 => return vars.split("hikari_1060", settings.hikari_1060),
                p if p == 1070 => return vars.split("hikari_1070", settings.hikari_1070),
                p if p == 1080 => return vars.split("hikari_1080", settings.hikari_1080),
                p if p == 1090 => return vars.split("hikari_1090", settings.hikari_1090),
                p if p == 1100 => return vars.split("hikari_1100", settings.hikari_1100),
                p if p == 1110 => return vars.split("hikari_1110", settings.hikari_1110),
                p if p == 1500 => return vars.split("hikari_1500", settings.hikari_1500),
                p if p == 1510 => return vars.split("hikari_1510", settings.hikari_1510),
                p if p == 1520 => return vars.split("hikari_1520", settings.hikari_1520),
                p if p == 1530 => return vars.split("hikari_1530", settings.hikari_1530),
                p if p == 1540 => return vars.split("hikari_1540", settings.hikari_1540),
                p if p == 1550 => return vars.split("hikari_1550", settings.hikari_1550),
                p if p == 1560 => return vars.split("hikari_1560", settings.hikari_1560),
                p if p == 1570 => return vars.split("hikari_1570", settings.hikari_1570),
                p if p == 1580 => return vars.split("hikari_1580", settings.hikari_1580),
                p if p == 1590 => return vars.split("hikari_1590", settings.hikari_1590),
                p if p == 1600 => return vars.split("hikari_1600", settings.hikari_1600),
                p if p == 1610 => return vars.split("hikari_1610", settings.hikari_1610),
                p if p == 1620 => return vars.split("hikari_1620", settings.hikari_1620),
                p if p == 1630 => return vars.split("hikari_1630", settings.hikari_1630),
                p if p == 1640 => return vars.split("hikari_1640", settings.hikari_1640),
                p if p == 1650 => return vars.split("hikari_1650", settings.hikari_1650),
                p if p == 1660 => return vars.split("hikari_1660", settings.hikari_1660),
                p if p == 1670 => return vars.split("hikari_1670", settings.hikari_1670),
                p if p == 1680 => return vars.split("hikari_1680", settings.hikari_1680),
                p if p == 2000 => return vars.split("hikari_2000", settings.hikari_2000),
                p if p == 2010 => return vars.split("hikari_2010", settings.hikari_2010),
                p if p == 2020 => return vars.split("hikari_2020", settings.hikari_2020),
                p if p == 2030 => return vars.split("hikari_2030", settings.hikari_2030),
                p if p == 2040 => return vars.split("hikari_2040", settings.hikari_2040),
                p if p == 2050 => return vars.split("hikari_2050", settings.hikari_2050),
                p if p == 2060 => return vars.split("hikari_2060", settings.hikari_2060),
                p if p == 2070 => return vars.split("hikari_2070", settings.hikari_2070),
                p if p == 2080 => return vars.split("hikari_2080", settings.hikari_2080),
                p if p == 2090 => return vars.split("hikari_2090", settings.hikari_2090),
                p if p == 2100 => return vars.split("hikari_2100", settings.hikari_2100),
                p if p == 2110 => return vars.split("hikari_2110", settings.hikari_2110),
                p if p == 2120 => return vars.split("hikari_2120", settings.hikari_2120),
                p if p == 2130 => return vars.split("hikari_2130", settings.hikari_2130),
                p if p == 2150 => return vars.split("hikari_2150", settings.hikari_2150),
                p if p == 2160 => return vars.split("hikari_2160", settings.hikari_2160),
                p if p == 2170 => return vars.split("hikari_2170", settings.hikari_2170),
                p if p == 2180 => return vars.split("hikari_2180", settings.hikari_2180),
                p if p == 2190 => return vars.split("hikari_2190", settings.hikari_2190),
                p if p == 2200 => return vars.split("hikari_2200", settings.hikari_2200),
                p if p == 2210 => return vars.split("hikari_2210", settings.hikari_2210),
                p if p == 2220 => return vars.split("hikari_2220", settings.hikari_2220),
                p if p == 2500 => return vars.split("hikari_2500", settings.hikari_2500),
                _ => return None,
            }
        } else {
            return None;
        }
    }
}
