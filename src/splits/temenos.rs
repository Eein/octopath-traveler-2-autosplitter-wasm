pub struct TemenosSplits;
use crate::Vars;

impl TemenosSplits {
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
        if vars.temenos_progress.current == 2500
            && vars.game_state.current == 2
            && vars.event_index.current >= 142
        {
            return vars.split(
                "temenos_story_complete",
                vars.settings.temenos_story_complete,
            );
        }
        None
    }

    fn chapter_split(vars: &mut Vars) -> Option<String> {
        // checks if an old save is lingering, also make sure old zone id isn't 0 later
        if vars.temenos_progress.old != vars.temenos_progress.current {
            match vars.temenos_progress.current {
                p if p == 10 => return vars.split("temenos_10", vars.settings.temenos_10),
                p if p == 20 => return vars.split("temenos_20", vars.settings.temenos_20),
                p if p == 30 => return vars.split("temenos_30", vars.settings.temenos_30),
                p if p == 40 => return vars.split("temenos_40", vars.settings.temenos_40),
                p if p == 60 => return vars.split("temenos_60", vars.settings.temenos_60),
                p if p == 90 => return vars.split("temenos_90", vars.settings.temenos_90),
                p if p == 100 => return vars.split("temenos_100", vars.settings.temenos_100),
                p if p == 110 => return vars.split("temenos_110", vars.settings.temenos_110),
                p if p == 120 => return vars.split("temenos_120", vars.settings.temenos_120),
                p if p == 130 => return vars.split("temenos_130", vars.settings.temenos_130),
                p if p == 140 => return vars.split("temenos_140", vars.settings.temenos_140),
                p if p == 150 => return vars.split("temenos_150", vars.settings.temenos_150),
                p if p == 160 => return vars.split("temenos_160", vars.settings.temenos_160),
                p if p == 170 => return vars.split("temenos_170", vars.settings.temenos_170),
                p if p == 500 => return vars.split("temenos_500", vars.settings.temenos_500),
                p if p == 510 => return vars.split("temenos_510", vars.settings.temenos_510),
                p if p == 520 => return vars.split("temenos_520", vars.settings.temenos_520),
                p if p == 530 => return vars.split("temenos_530", vars.settings.temenos_530),
                p if p == 540 => return vars.split("temenos_540", vars.settings.temenos_540),
                p if p == 550 => return vars.split("temenos_550", vars.settings.temenos_550),
                p if p == 560 => return vars.split("temenos_560", vars.settings.temenos_560),
                p if p == 570 => return vars.split("temenos_570", vars.settings.temenos_570),
                p if p == 580 => return vars.split("temenos_580", vars.settings.temenos_580),
                p if p == 590 => return vars.split("temenos_590", vars.settings.temenos_590),
                p if p == 600 => return vars.split("temenos_600", vars.settings.temenos_600),
                p if p == 610 => return vars.split("temenos_610", vars.settings.temenos_610),
                p if p == 620 => return vars.split("temenos_620", vars.settings.temenos_620),
                p if p == 630 => return vars.split("temenos_630", vars.settings.temenos_630),
                p if p == 640 => return vars.split("temenos_640", vars.settings.temenos_640),
                p if p == 650 => return vars.split("temenos_650", vars.settings.temenos_650),
                p if p == 660 => return vars.split("temenos_660", vars.settings.temenos_660),
                p if p == 670 => return vars.split("temenos_670", vars.settings.temenos_670),
                p if p == 680 => return vars.split("temenos_680", vars.settings.temenos_680),
                p if p == 690 => return vars.split("temenos_690", vars.settings.temenos_690),
                p if p == 700 => return vars.split("temenos_700", vars.settings.temenos_700),
                p if p == 1000 => return vars.split("temenos_1000", vars.settings.temenos_1000),
                p if p == 1010 => return vars.split("temenos_1010", vars.settings.temenos_1010),
                p if p == 1020 => return vars.split("temenos_1020", vars.settings.temenos_1020),
                p if p == 1030 => return vars.split("temenos_1030", vars.settings.temenos_1030),
                p if p == 1040 => return vars.split("temenos_1040", vars.settings.temenos_1040),
                p if p == 1050 => return vars.split("temenos_1050", vars.settings.temenos_1050),
                p if p == 1060 => return vars.split("temenos_1060", vars.settings.temenos_1060),
                p if p == 1070 => return vars.split("temenos_1070", vars.settings.temenos_1070),
                p if p == 1080 => return vars.split("temenos_1080", vars.settings.temenos_1080),
                p if p == 1100 => return vars.split("temenos_1100", vars.settings.temenos_1100),
                p if p == 1110 => return vars.split("temenos_1110", vars.settings.temenos_1110),
                p if p == 1120 => return vars.split("temenos_1120", vars.settings.temenos_1120),
                p if p == 1130 => return vars.split("temenos_1130", vars.settings.temenos_1130),
                p if p == 1140 => return vars.split("temenos_1140", vars.settings.temenos_1140),
                p if p == 1170 => return vars.split("temenos_1170", vars.settings.temenos_1170),
                p if p == 1190 => return vars.split("temenos_1190", vars.settings.temenos_1190),
                p if p == 1200 => return vars.split("temenos_1200", vars.settings.temenos_1200),
                p if p == 1210 => return vars.split("temenos_1210", vars.settings.temenos_1210),
                p if p == 1220 => return vars.split("temenos_1220", vars.settings.temenos_1220),
                p if p == 1230 => return vars.split("temenos_1230", vars.settings.temenos_1230),
                p if p == 1240 => return vars.split("temenos_1240", vars.settings.temenos_1240),
                p if p == 1250 => return vars.split("temenos_1250", vars.settings.temenos_1250),
                p if p == 1500 => return vars.split("temenos_1500", vars.settings.temenos_1500),
                p if p == 1510 => return vars.split("temenos_1510", vars.settings.temenos_1510),
                p if p == 1530 => return vars.split("temenos_1530", vars.settings.temenos_1530),
                p if p == 1540 => return vars.split("temenos_1540", vars.settings.temenos_1540),
                p if p == 1550 => return vars.split("temenos_1550", vars.settings.temenos_1550),
                p if p == 1560 => return vars.split("temenos_1560", vars.settings.temenos_1560),
                p if p == 1580 => return vars.split("temenos_1580", vars.settings.temenos_1580),
                p if p == 1590 => return vars.split("temenos_1590", vars.settings.temenos_1590),
                p if p == 1600 => return vars.split("temenos_1600", vars.settings.temenos_1600),
                p if p == 1610 => return vars.split("temenos_1610", vars.settings.temenos_1610),
                p if p == 1620 => return vars.split("temenos_1620", vars.settings.temenos_1620),
                p if p == 1630 => return vars.split("temenos_1630", vars.settings.temenos_1630),
                p if p == 2000 => return vars.split("temenos_2000", vars.settings.temenos_2000),
                p if p == 2010 => return vars.split("temenos_2010", vars.settings.temenos_2010),
                p if p == 2020 => return vars.split("temenos_2020", vars.settings.temenos_2020),
                p if p == 2030 => return vars.split("temenos_2030", vars.settings.temenos_2030),
                p if p == 2040 => return vars.split("temenos_2040", vars.settings.temenos_2040),
                p if p == 2050 => return vars.split("temenos_2050", vars.settings.temenos_2050),
                p if p == 2060 => return vars.split("temenos_2060", vars.settings.temenos_2060),
                p if p == 2070 => return vars.split("temenos_2070", vars.settings.temenos_2070),
                p if p == 2080 => return vars.split("temenos_2080", vars.settings.temenos_2080),
                p if p == 2090 => return vars.split("temenos_2090", vars.settings.temenos_2090),
                p if p == 2100 => return vars.split("temenos_2100", vars.settings.temenos_2100),
                p if p == 2110 => return vars.split("temenos_2110", vars.settings.temenos_2110),
                p if p == 2500 => return vars.split("temenos_2500", vars.settings.temenos_2500),
                _ => return None,
            }
        }
        None
    }
}
