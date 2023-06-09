pub struct PartitioSplits;
use crate::settings::Settings;
use crate::Vars;

impl PartitioSplits {
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
        if vars.partitio_hp.old == 0 && vars.partitio_hp.current != 0 {
            return vars.split("partitio_joins", settings.partitio_joins);
        }
        None
    }

    fn main_story_complete(vars: &mut Vars, settings: &Settings) -> Option<String> {
        if vars.partitio_progress.current == 2000
            && vars.game_state.current == 2
            && vars.event_index.current >= 125
        {
            return vars.split("partitio_story_complete", settings.partitio_story_complete);
        }
        None
    }

    fn chapter_split(vars: &mut Vars, settings: &Settings) -> Option<String> {
        // checks if an old save is lingering, also make sure old zone id isn't 0 later
        if vars.partitio_progress.old != vars.partitio_progress.current {
            match vars.partitio_progress.current {
                p if p == 10 => return vars.split("partitio_10", settings.partitio_10),
                p if p == 40 => return vars.split("partitio_40", settings.partitio_40),
                p if p == 60 => return vars.split("partitio_60", settings.partitio_60),
                p if p == 90 => return vars.split("partitio_90", settings.partitio_90),
                p if p == 100 => return vars.split("partitio_100", settings.partitio_100),
                p if p == 110 => return vars.split("partitio_110", settings.partitio_110),
                p if p == 120 => return vars.split("partitio_120", settings.partitio_120),
                p if p == 130 => return vars.split("partitio_130", settings.partitio_130),
                p if p == 150 => return vars.split("partitio_150", settings.partitio_150),
                p if p == 170 => return vars.split("partitio_170", settings.partitio_170),
                p if p == 180 => return vars.split("partitio_180", settings.partitio_180),
                p if p == 190 => return vars.split("partitio_190", settings.partitio_190),
                p if p == 200 => return vars.split("partitio_200", settings.partitio_200),
                p if p == 500 => return vars.split("partitio_500", settings.partitio_500),
                p if p == 510 => return vars.split("partitio_510", settings.partitio_510),
                p if p == 520 => return vars.split("partitio_520", settings.partitio_520),
                p if p == 530 => return vars.split("partitio_530", settings.partitio_530),
                p if p == 540 => return vars.split("partitio_540", settings.partitio_540),
                p if p == 550 => return vars.split("partitio_550", settings.partitio_550),
                p if p == 560 => return vars.split("partitio_560", settings.partitio_560),
                p if p == 580 => return vars.split("partitio_580", settings.partitio_580),
                p if p == 600 => return vars.split("partitio_600", settings.partitio_600),
                p if p == 610 => return vars.split("partitio_610", settings.partitio_610),
                p if p == 620 => return vars.split("partitio_620", settings.partitio_620),
                p if p == 630 => return vars.split("partitio_630", settings.partitio_630),
                p if p == 640 => return vars.split("partitio_640", settings.partitio_640),
                p if p == 650 => return vars.split("partitio_650", settings.partitio_650),
                p if p == 670 => return vars.split("partitio_670", settings.partitio_670),
                p if p == 680 => return vars.split("partitio_680", settings.partitio_680),
                p if p == 690 => return vars.split("partitio_690", settings.partitio_690),
                p if p == 1000 => return vars.split("partitio_1000", settings.partitio_1000),
                p if p == 1010 => return vars.split("partitio_1010", settings.partitio_1010),
                p if p == 1020 => return vars.split("partitio_1020", settings.partitio_1020),
                p if p == 1030 => return vars.split("partitio_1030", settings.partitio_1030),
                p if p == 1040 => return vars.split("partitio_1040", settings.partitio_1040),
                p if p == 1050 => return vars.split("partitio_1050", settings.partitio_1050),
                p if p == 1060 => return vars.split("partitio_1060", settings.partitio_1060),
                p if p == 1070 => return vars.split("partitio_1070", settings.partitio_1070),
                p if p == 1080 => return vars.split("partitio_1080", settings.partitio_1080),
                p if p == 1090 => return vars.split("partitio_1090", settings.partitio_1090),
                p if p == 1130 => return vars.split("partitio_1130", settings.partitio_1130),
                p if p == 1140 => return vars.split("partitio_1140", settings.partitio_1140),
                p if p == 1150 => return vars.split("partitio_1150", settings.partitio_1150),
                p if p == 1160 => return vars.split("partitio_1160", settings.partitio_1160),
                p if p == 1170 => return vars.split("partitio_1170", settings.partitio_1170),
                p if p == 1500 => return vars.split("partitio_1500", settings.partitio_1500),
                p if p == 1510 => return vars.split("partitio_1510", settings.partitio_1510),
                p if p == 1520 => return vars.split("partitio_1520", settings.partitio_1520),
                p if p == 1530 => return vars.split("partitio_1530", settings.partitio_1530),
                p if p == 1540 => return vars.split("partitio_1540", settings.partitio_1540),
                p if p == 1550 => return vars.split("partitio_1550", settings.partitio_1550),
                p if p == 1560 => return vars.split("partitio_1560", settings.partitio_1560),
                p if p == 1580 => return vars.split("partitio_1580", settings.partitio_1580),
                p if p == 1590 => return vars.split("partitio_1590", settings.partitio_1590),
                p if p == 1600 => return vars.split("partitio_1600", settings.partitio_1600),
                p if p == 1610 => return vars.split("partitio_1610", settings.partitio_1610),
                p if p == 1620 => return vars.split("partitio_1620", settings.partitio_1620),
                p if p == 1630 => return vars.split("partitio_1630", settings.partitio_1630),
                p if p == 1640 => return vars.split("partitio_1640", settings.partitio_1640),
                p if p == 1650 => return vars.split("partitio_1650", settings.partitio_1650),
                p if p == 2000 => return vars.split("partitio_2000", settings.partitio_2000),
                _ => return None,
            }
        } else {
            return None;
        }
    }
}
