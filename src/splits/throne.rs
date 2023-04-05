pub struct ThroneSplits;
use crate::Vars;

impl ThroneSplits {
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
        if vars.throne_progress.current == 3000
            && vars.game_state.current == 2
            && vars.event_index.current >= 122
        {
            return vars.split("throne_story_complete", vars.settings.throne_story_complete);
        }
        None
    }

    fn chapter_split(vars: &mut Vars) -> Option<String> {
        // checks if an old save is lingering, also make sure old zone id isn't 0 later
        if vars.throne_progress.old != vars.throne_progress.current {
            match vars.throne_progress.current {
                p if p == 10 => return vars.split("throne_10", vars.settings.throne_10),
                p if p == 20 => return vars.split("throne_20", vars.settings.throne_20),
                p if p == 30 => return vars.split("throne_30", vars.settings.throne_30),
                p if p == 40 => return vars.split("throne_40", vars.settings.throne_40),
                p if p == 50 => return vars.split("throne_50", vars.settings.throne_50),
                p if p == 60 => return vars.split("throne_60", vars.settings.throne_60),
                p if p == 70 => return vars.split("throne_70", vars.settings.throne_70),
                p if p == 80 => return vars.split("throne_80", vars.settings.throne_80),
                p if p == 90 => return vars.split("throne_90", vars.settings.throne_90),
                p if p == 100 => return vars.split("throne_100", vars.settings.throne_100),
                p if p == 110 => return vars.split("throne_110", vars.settings.throne_110),
                p if p == 120 => return vars.split("throne_120", vars.settings.throne_120),
                p if p == 150 => return vars.split("throne_150", vars.settings.throne_150),
                p if p == 160 => return vars.split("throne_160", vars.settings.throne_160),
                p if p == 180 => return vars.split("throne_180", vars.settings.throne_180),
                p if p == 190 => return vars.split("throne_190", vars.settings.throne_190),
                p if p == 500 => return vars.split("throne_500", vars.settings.throne_500),
                p if p == 510 => return vars.split("throne_510", vars.settings.throne_510),
                p if p == 520 => return vars.split("throne_520", vars.settings.throne_520),
                p if p == 530 => return vars.split("throne_530", vars.settings.throne_530),
                p if p == 540 => return vars.split("throne_540", vars.settings.throne_540),
                p if p == 550 => return vars.split("throne_550", vars.settings.throne_550),
                p if p == 560 => return vars.split("throne_560", vars.settings.throne_560),
                p if p == 570 => return vars.split("throne_570", vars.settings.throne_570),
                p if p == 580 => return vars.split("throne_580", vars.settings.throne_580),
                p if p == 590 => return vars.split("throne_590", vars.settings.throne_590),
                p if p == 610 => return vars.split("throne_610", vars.settings.throne_610),
                p if p == 1000 => return vars.split("throne_1000", vars.settings.throne_1000),
                p if p == 1010 => return vars.split("throne_1010", vars.settings.throne_1010),
                p if p == 1020 => return vars.split("throne_1020", vars.settings.throne_1020),
                p if p == 1030 => return vars.split("throne_1030", vars.settings.throne_1030),
                p if p == 1040 => return vars.split("throne_1040", vars.settings.throne_1040),
                p if p == 1050 => return vars.split("throne_1050", vars.settings.throne_1050),
                p if p == 1060 => return vars.split("throne_1060", vars.settings.throne_1060),
                p if p == 1070 => return vars.split("throne_1070", vars.settings.throne_1070),
                p if p == 1080 => return vars.split("throne_1080", vars.settings.throne_1080),
                p if p == 1090 => return vars.split("throne_1090", vars.settings.throne_1090),
                p if p == 1100 => return vars.split("throne_1100", vars.settings.throne_1100),
                p if p == 1500 => return vars.split("throne_1500", vars.settings.throne_1500),
                p if p == 1510 => return vars.split("throne_1510", vars.settings.throne_1510),
                p if p == 1520 => return vars.split("throne_1520", vars.settings.throne_1520),
                p if p == 1530 => return vars.split("throne_1530", vars.settings.throne_1530),
                p if p == 1540 => return vars.split("throne_1540", vars.settings.throne_1540),
                p if p == 1550 => return vars.split("throne_1550", vars.settings.throne_1550),
                p if p == 1560 => return vars.split("throne_1560", vars.settings.throne_1560),
                p if p == 1570 => return vars.split("throne_1570", vars.settings.throne_1570),
                p if p == 1590 => return vars.split("throne_1590", vars.settings.throne_1590),
                p if p == 1600 => return vars.split("throne_1600", vars.settings.throne_1600),
                p if p == 1610 => return vars.split("throne_1610", vars.settings.throne_1610),
                p if p == 2000 => return vars.split("throne_2000", vars.settings.throne_2000),
                p if p == 2010 => return vars.split("throne_2010", vars.settings.throne_2010),
                p if p == 2020 => return vars.split("throne_2020", vars.settings.throne_2020),
                p if p == 2030 => return vars.split("throne_2030", vars.settings.throne_2030),
                p if p == 2040 => return vars.split("throne_2040", vars.settings.throne_2040),
                p if p == 2050 => return vars.split("throne_2050", vars.settings.throne_2050),
                p if p == 2060 => return vars.split("throne_2060", vars.settings.throne_2060),
                p if p == 2070 => return vars.split("throne_2070", vars.settings.throne_2070),
                p if p == 2080 => return vars.split("throne_2080", vars.settings.throne_2080),
                p if p == 2090 => return vars.split("throne_2090", vars.settings.throne_2090),
                p if p == 2500 => return vars.split("throne_2500", vars.settings.throne_2500),
                p if p == 2510 => return vars.split("throne_2510", vars.settings.throne_2510),
                p if p == 2520 => return vars.split("throne_2520", vars.settings.throne_2520),
                p if p == 2530 => return vars.split("throne_2530", vars.settings.throne_2530),
                p if p == 2540 => return vars.split("throne_2540", vars.settings.throne_2540),
                p if p == 2550 => return vars.split("throne_2550", vars.settings.throne_2550),
                p if p == 2560 => return vars.split("throne_2560", vars.settings.throne_2560),
                p if p == 2580 => return vars.split("throne_2580", vars.settings.throne_2580),
                p if p == 2600 => return vars.split("throne_2600", vars.settings.throne_2600),
                p if p == 2610 => return vars.split("throne_2610", vars.settings.throne_2610),
                p if p == 2620 => return vars.split("throne_2620", vars.settings.throne_2620),
                p if p == 2630 => return vars.split("throne_2630", vars.settings.throne_2630),
                p if p == 2640 => return vars.split("throne_2640", vars.settings.throne_2640),
                p if p == 2650 => return vars.split("throne_2650", vars.settings.throne_2650),
                p if p == 2660 => return vars.split("throne_2660", vars.settings.throne_2660),
                p if p == 2670 => return vars.split("throne_2670", vars.settings.throne_2670),
                p if p == 3000 => return vars.split("throne_3000", vars.settings.throne_3000),
                _ => return None,
            }
        }
        None
    }
}
