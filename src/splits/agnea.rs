pub struct AgneaSplits;
use crate::Vars;

impl AgneaSplits {
    pub fn split(vars: &mut Vars) -> Option<String> {
        if let Some(split) = Self::chapter_split(vars) {
            return Some(split);
        }
        if let Some(split) = Self::main_story_complete(vars) {
            return Some(split);
        }
        if let Some(split) = Self::joins_party(vars) {
            return Some(split);
        }
        None
    }

    fn joins_party(vars: &mut Vars) -> Option<String> {
        if vars.agnea_hp.old == 0 && vars.agnea_hp.current != 0 {
            return vars.split("agnea_joins", vars.settings.agnea_joins);
        }
        None
    }

    fn main_story_complete(vars: &mut Vars) -> Option<String> {
        if vars.agnea_progress.current == 2500
            && vars.game_state.current == 2
            && vars.event_index.current >= 131
        {
            return vars.split("agnea_story_complete", vars.settings.agnea_story_complete);
        }
        None
    }

    fn chapter_split(vars: &mut Vars) -> Option<String> {
        // checks if an old save is lingering, also make sure old zone id isn't 0 later
        if vars.agnea_progress.old != vars.agnea_progress.current {
            match vars.agnea_progress.current {
                p if p == 10 => return vars.split("agnea_10", vars.settings.agnea_10),
                p if p == 20 => return vars.split("agnea_20", vars.settings.agnea_20),
                p if p == 30 => return vars.split("agnea_30", vars.settings.agnea_30),
                p if p == 40 => return vars.split("agnea_40", vars.settings.agnea_40),
                p if p == 50 => return vars.split("agnea_50", vars.settings.agnea_50),
                p if p == 60 => return vars.split("agnea_60", vars.settings.agnea_60),
                p if p == 70 => return vars.split("agnea_70", vars.settings.agnea_70),
                p if p == 80 => return vars.split("agnea_80", vars.settings.agnea_80),
                p if p == 90 => return vars.split("agnea_90", vars.settings.agnea_90),
                p if p == 100 => return vars.split("agnea_100", vars.settings.agnea_100),
                p if p == 110 => return vars.split("agnea_110", vars.settings.agnea_110),
                p if p == 120 => return vars.split("agnea_120", vars.settings.agnea_120),
                p if p == 130 => return vars.split("agnea_130", vars.settings.agnea_130),
                p if p == 140 => return vars.split("agnea_140", vars.settings.agnea_140),
                p if p == 150 => return vars.split("agnea_150", vars.settings.agnea_150),
                p if p == 160 => return vars.split("agnea_160", vars.settings.agnea_160),
                p if p == 170 => return vars.split("agnea_170", vars.settings.agnea_170),
                p if p == 500 => return vars.split("agnea_500", vars.settings.agnea_500),
                p if p == 510 => return vars.split("agnea_510", vars.settings.agnea_510),
                p if p == 520 => return vars.split("agnea_520", vars.settings.agnea_520),
                p if p == 550 => return vars.split("agnea_550", vars.settings.agnea_550),
                p if p == 560 => return vars.split("agnea_560", vars.settings.agnea_560),
                p if p == 570 => return vars.split("agnea_570", vars.settings.agnea_570),
                p if p == 580 => return vars.split("agnea_580", vars.settings.agnea_580),
                p if p == 590 => return vars.split("agnea_590", vars.settings.agnea_590),
                p if p == 600 => return vars.split("agnea_600", vars.settings.agnea_600),
                p if p == 610 => return vars.split("agnea_610", vars.settings.agnea_610),
                p if p == 620 => return vars.split("agnea_620", vars.settings.agnea_620),
                p if p == 630 => return vars.split("agnea_630", vars.settings.agnea_630),
                p if p == 640 => return vars.split("agnea_640", vars.settings.agnea_640),
                p if p == 650 => return vars.split("agnea_650", vars.settings.agnea_650),
                p if p == 660 => return vars.split("agnea_660", vars.settings.agnea_660),
                p if p == 670 => return vars.split("agnea_670", vars.settings.agnea_670),
                p if p == 680 => return vars.split("agnea_680", vars.settings.agnea_680),
                p if p == 690 => return vars.split("agnea_690", vars.settings.agnea_690),
                p if p == 700 => return vars.split("agnea_700", vars.settings.agnea_700),
                p if p == 1000 => return vars.split("agnea_1000", vars.settings.agnea_1000),
                p if p == 1010 => return vars.split("agnea_1010", vars.settings.agnea_1010),
                p if p == 1030 => return vars.split("agnea_1030", vars.settings.agnea_1030),
                p if p == 1040 => return vars.split("agnea_1040", vars.settings.agnea_1040),
                p if p == 1050 => return vars.split("agnea_1050", vars.settings.agnea_1050),
                p if p == 1060 => return vars.split("agnea_1060", vars.settings.agnea_1060),
                p if p == 1070 => return vars.split("agnea_1070", vars.settings.agnea_1070),
                p if p == 1080 => return vars.split("agnea_1080", vars.settings.agnea_1080),
                p if p == 1500 => return vars.split("agnea_1500", vars.settings.agnea_1500),
                p if p == 1510 => return vars.split("agnea_1510", vars.settings.agnea_1510),
                p if p == 1520 => return vars.split("agnea_1520", vars.settings.agnea_1520),
                p if p == 1530 => return vars.split("agnea_1530", vars.settings.agnea_1530),
                p if p == 1540 => return vars.split("agnea_1540", vars.settings.agnea_1540),
                p if p == 1550 => return vars.split("agnea_1550", vars.settings.agnea_1550),
                p if p == 1580 => return vars.split("agnea_1580", vars.settings.agnea_1580),
                p if p == 1590 => return vars.split("agnea_1590", vars.settings.agnea_1590),
                p if p == 1600 => return vars.split("agnea_1600", vars.settings.agnea_1600),
                p if p == 1610 => return vars.split("agnea_1610", vars.settings.agnea_1610),
                p if p == 1620 => return vars.split("agnea_1620", vars.settings.agnea_1620),
                p if p == 1630 => return vars.split("agnea_1630", vars.settings.agnea_1630),
                p if p == 1640 => return vars.split("agnea_1640", vars.settings.agnea_1640),
                p if p == 1650 => return vars.split("agnea_1650", vars.settings.agnea_1650),
                p if p == 1660 => return vars.split("agnea_1660", vars.settings.agnea_1660),
                p if p == 2000 => return vars.split("agnea_2000", vars.settings.agnea_2000),
                p if p == 2010 => return vars.split("agnea_2010", vars.settings.agnea_2010),
                p if p == 2020 => return vars.split("agnea_2020", vars.settings.agnea_2020),
                p if p == 2030 => return vars.split("agnea_2030", vars.settings.agnea_2030),
                p if p == 2040 => return vars.split("agnea_2040", vars.settings.agnea_2040),
                p if p == 2050 => return vars.split("agnea_2050", vars.settings.agnea_2050),
                p if p == 2060 => return vars.split("agnea_2060", vars.settings.agnea_2060),
                p if p == 2080 => return vars.split("agnea_2080", vars.settings.agnea_2080),
                p if p == 2090 => return vars.split("agnea_2090", vars.settings.agnea_2090),
                p if p == 2100 => return vars.split("agnea_2100", vars.settings.agnea_2100),
                p if p == 2110 => return vars.split("agnea_2110", vars.settings.agnea_2110),
                p if p == 2120 => return vars.split("agnea_2120", vars.settings.agnea_2120),
                p if p == 2130 => return vars.split("agnea_2130", vars.settings.agnea_2130),
                p if p == 2140 => return vars.split("agnea_2140", vars.settings.agnea_2140),
                p if p == 2150 => return vars.split("agnea_2150", vars.settings.agnea_2150),
                p if p == 2160 => return vars.split("agnea_2160", vars.settings.agnea_2160),
                p if p == 2170 => return vars.split("agnea_2170", vars.settings.agnea_2170),
                p if p == 2500 => return vars.split("agnea_2500", vars.settings.agnea_2500),
                _ => return None,
            }
        } else {
            return None;
        }
    }
}
