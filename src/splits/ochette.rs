pub struct OchetteSplits;
use crate::Vars;

impl OchetteSplits {
    pub fn split(vars: &mut Vars) -> Option<String> {
        if let Some(split) = Self::chapter_split(vars) {
            return Some(split);
        }
        if let Some(split) = Self::main_story_complete(vars) {
            return Some(split);
        }
        None
    }

    fn main_story_complete(vars: &mut Vars) -> Option<String> {
        if vars.ochette_progress.current == 2500
            && vars.game_state.current == 2
            && vars.event_index.current >= 130
        {
            return vars.split(
                "ochette_story_complete",
                vars.settings.ochette_story_complete,
            );
        }
        None
    }

    fn chapter_split(vars: &mut Vars) -> Option<String> {
        // checks if an old save is lingering, also make sure old zone id isn't 0 later
        if vars.ochette_progress.old != vars.ochette_progress.current {
            match vars.ochette_progress.current {
                p if p == 10 => return vars.split("ochette_10", vars.settings.ochette_10),
                p if p == 30 => return vars.split("ochette_30", vars.settings.ochette_30),
                p if p == 40 => return vars.split("ochette_40", vars.settings.ochette_40),
                p if p == 50 => return vars.split("ochette_50", vars.settings.ochette_50),
                p if p == 60 => return vars.split("ochette_60", vars.settings.ochette_60),
                p if p == 70 => return vars.split("ochette_70", vars.settings.ochette_70),
                p if p == 90 => return vars.split("ochette_90", vars.settings.ochette_90),
                p if p == 100 => return vars.split("ochette_100", vars.settings.ochette_100),
                p if p == 130 => return vars.split("ochette_130", vars.settings.ochette_130),
                p if p == 140 => return vars.split("ochette_140", vars.settings.ochette_140),
                p if p == 150 => return vars.split("ochette_150", vars.settings.ochette_150),
                p if p == 160 => return vars.split("ochette_160", vars.settings.ochette_160),
                p if p == 170 => return vars.split("ochette_170", vars.settings.ochette_170),
                p if p == 180 => return vars.split("ochette_180", vars.settings.ochette_180),
                p if p == 190 => return vars.split("ochette_190", vars.settings.ochette_190),
                p if p == 200 => return vars.split("ochette_200", vars.settings.ochette_200),
                p if p == 500 => return vars.split("ochette_500", vars.settings.ochette_500),
                p if p == 510 => return vars.split("ochette_510", vars.settings.ochette_510),
                p if p == 520 => return vars.split("ochette_520", vars.settings.ochette_520),
                p if p == 530 => return vars.split("ochette_530", vars.settings.ochette_530),
                p if p == 540 => return vars.split("ochette_540", vars.settings.ochette_540),
                p if p == 550 => return vars.split("ochette_550", vars.settings.ochette_550),
                p if p == 560 => return vars.split("ochette_560", vars.settings.ochette_560),
                p if p == 570 => return vars.split("ochette_570", vars.settings.ochette_570),
                p if p == 1000 => return vars.split("ochette_1000", vars.settings.ochette_1000),
                p if p == 1010 => return vars.split("ochette_1010", vars.settings.ochette_1010),
                p if p == 1020 => return vars.split("ochette_1020", vars.settings.ochette_1020),
                p if p == 1030 => return vars.split("ochette_1030", vars.settings.ochette_1030),
                p if p == 1040 => return vars.split("ochette_1040", vars.settings.ochette_1040),
                p if p == 1050 => return vars.split("ochette_1050", vars.settings.ochette_1050),
                p if p == 1060 => return vars.split("ochette_1060", vars.settings.ochette_1060),
                p if p == 1070 => return vars.split("ochette_1070", vars.settings.ochette_1070),
                p if p == 1500 => return vars.split("ochette_1500", vars.settings.ochette_1500),
                p if p == 1510 => return vars.split("ochette_1510", vars.settings.ochette_1510),
                p if p == 1520 => return vars.split("ochette_1520", vars.settings.ochette_1520),
                p if p == 1530 => return vars.split("ochette_1530", vars.settings.ochette_1530),
                p if p == 1550 => return vars.split("ochette_1550", vars.settings.ochette_1550),
                p if p == 1560 => return vars.split("ochette_1560", vars.settings.ochette_1560),
                p if p == 1570 => return vars.split("ochette_1570", vars.settings.ochette_1570),
                p if p == 1580 => return vars.split("ochette_1580", vars.settings.ochette_1580),
                p if p == 1590 => return vars.split("ochette_1590", vars.settings.ochette_1590),
                p if p == 1600 => return vars.split("ochette_1600", vars.settings.ochette_1600),
                p if p == 2000 => return vars.split("ochette_2000", vars.settings.ochette_2000),
                p if p == 2020 => return vars.split("ochette_2020", vars.settings.ochette_2020),
                p if p == 2030 => return vars.split("ochette_2030", vars.settings.ochette_2030),
                p if p == 2040 => return vars.split("ochette_2040", vars.settings.ochette_2040),
                p if p == 2050 => return vars.split("ochette_2050", vars.settings.ochette_2050),
                p if p == 2060 => return vars.split("ochette_2060", vars.settings.ochette_2060),
                p if p == 2070 => return vars.split("ochette_2070", vars.settings.ochette_2070),
                p if p == 2080 => return vars.split("ochette_2080", vars.settings.ochette_2080),
                p if p == 2090 => return vars.split("ochette_2090", vars.settings.ochette_2090),
                p if p == 2100 => return vars.split("ochette_2100", vars.settings.ochette_2100),
                p if p == 2110 => return vars.split("ochette_2110", vars.settings.ochette_2110),
                p if p == 2120 => return vars.split("ochette_2120", vars.settings.ochette_2120),
                p if p == 2130 => return vars.split("ochette_2130", vars.settings.ochette_2130),
                p if p == 2140 => return vars.split("ochette_2140", vars.settings.ochette_2140),
                p if p == 2150 => return vars.split("ochette_2150", vars.settings.ochette_2150),
                p if p == 2160 => return vars.split("ochette_2160", vars.settings.ochette_2160),
                p if p == 2170 => return vars.split("ochette_2170", vars.settings.ochette_2170),
                p if p == 2180 => return vars.split("ochette_2180", vars.settings.ochette_2180),
                p if p == 2190 => return vars.split("ochette_2190", vars.settings.ochette_2190),
                p if p == 2500 => return vars.split("ochette_2500", vars.settings.ochette_2500),
                _ => return None,
            }
        } else {
            return None;
        }
    }
}
