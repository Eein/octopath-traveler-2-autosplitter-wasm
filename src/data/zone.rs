pub struct Areas;
use crate::settings::Settings;
use crate::Vars;

impl Areas {
    pub fn enter_area(vars: &mut Vars, settings: &Settings) -> Option<String> {
        return match vars.level_id.current {
            5 => return vars.split("fld_mnt_1_1_enter", settings.fld_mnt_1_1_enter),
            6 => return vars.split("fld_mnt_1_2_enter", settings.fld_mnt_1_2_enter),
            7 => return vars.split("fld_mnt_2_1_enter", settings.fld_mnt_2_1_enter),
            8 => return vars.split("fld_mnt_2_2_enter", settings.fld_mnt_2_2_enter),
            9 => return vars.split("fld_mnt_3_1_enter", settings.fld_mnt_3_1_enter),
            10 => return vars.split("fld_mnt_3_2_enter", settings.fld_mnt_3_2_enter),
            11 => return vars.split("fld_mnt_3_3_enter", settings.fld_mnt_3_3_enter),
            12 => return vars.split("dng_mnt_1_1_enter", settings.dng_mnt_1_1_enter),
            13 => return vars.split("dng_mnt_2_1_enter", settings.dng_mnt_2_1_enter),
            14 => return vars.split("dng_mnt_2_2_enter", settings.dng_mnt_2_2_enter),
            15 => return vars.split("dng_mnt_3_3_enter", settings.dng_mnt_3_3_enter),
            16 => return vars.split("dng_mnt_2_job_enter", settings.dng_mnt_2_job_enter),
            17 => return vars.split("dng_mnt_3_1_a_enter", settings.dng_mnt_3_1_a_enter),
            19 => return vars.split("fld_dst_1_1_enter", settings.fld_dst_1_1_enter),
            20 => return vars.split("twn_mnt_1_1_a_enter", settings.twn_mnt_1_1_a_enter),
            21 => return vars.split("twn_mnt_1_2_a_enter", settings.twn_mnt_1_2_a_enter),
            22 => return vars.split("twn_mnt_1_2_b_enter", settings.twn_mnt_1_2_b_enter),
            23 => return vars.split("fld_dst_3_1_a_enter", settings.fld_dst_3_1_a_enter),
            24 => return vars.split("twn_mnt_2_1_a_enter", settings.twn_mnt_2_1_a_enter),
            25 => return vars.split("twn_mnt_2_1_b_enter", settings.twn_mnt_2_1_b_enter),
            26 => return vars.split("twn_mnt_2_1_c_enter", settings.twn_mnt_2_1_c_enter),
            27 => return vars.split("twn_mnt_3_1_a_enter", settings.twn_mnt_3_1_a_enter),
            28 => return vars.split("twn_dst_1_1_a_enter", settings.twn_dst_1_1_a_enter),
            29 => return vars.split("twn_dst_2_1_a_enter", settings.twn_dst_2_1_a_enter),
            30 => return vars.split("twn_dst_3_1_a_enter", settings.twn_dst_3_1_a_enter),
            31 => return vars.split("twn_dst_3_1_b_enter", settings.twn_dst_3_1_b_enter),
            32 => return vars.split("twn_dst_3_1_c_enter", settings.twn_dst_3_1_c_enter),
            33 => return vars.split("twn_mnt_3_1_b_enter", settings.twn_mnt_3_1_b_enter),
            34 => return vars.split("dng_mnt_3_1_b_enter", settings.dng_mnt_3_1_b_enter),
            35 => return vars.split("fld_dst_2_2_enter", settings.fld_dst_2_2_enter),
            36 => return vars.split("fld_dst_2_3_enter", settings.fld_dst_2_3_enter),
            37 => return vars.split("fld_dst_2_4_enter", settings.fld_dst_2_4_enter),
            38 => return vars.split("fld_dst_3_2_enter", settings.fld_dst_3_2_enter),
            39 => return vars.split("dng_dst_2_1_enter", settings.dng_dst_2_1_enter),
            40 => return vars.split("dng_dst_2_2_enter", settings.dng_dst_2_2_enter),
            41 => return vars.split("dng_dst_2_3_enter", settings.dng_dst_2_3_enter),
            42 => return vars.split("dng_dst_2_job_enter", settings.dng_dst_2_job_enter),
            43 => return vars.split("dng_dst_3_1_enter", settings.dng_dst_3_1_enter),
            44 => return vars.split("fld_cty_1_1_enter", settings.fld_cty_1_1_enter),
            45 => return vars.split("fld_cty_1_2_enter", settings.fld_cty_1_2_enter),
            46 => return vars.split("fld_cty_1_3_enter", settings.fld_cty_1_3_enter),
            47 => return vars.split("fld_cty_1_4_enter", settings.fld_cty_1_4_enter),
            48 => return vars.split("fld_cty_2_1_enter", settings.fld_cty_2_1_enter),
            49 => return vars.split("fld_cty_3_1_enter", settings.fld_cty_3_1_enter),
            50 => return vars.split("dng_cty_1_1_enter", settings.dng_cty_1_1_enter),
            51 => return vars.split("dng_cty_1_2_enter", settings.dng_cty_1_2_enter),
            52 => return vars.split("dng_cty_1_3_enter", settings.dng_cty_1_3_enter),
            53 => return vars.split("dng_cty_1_4_enter", settings.dng_cty_1_4_enter),
            54 => return vars.split("dng_cty_2_1_enter", settings.dng_cty_2_1_enter),
            56 => return vars.split("dng_cty_2_job_enter", settings.dng_cty_2_job_enter),
            57 => return vars.split("dng_cty_3_1_enter", settings.dng_cty_3_1_enter),
            58 => return vars.split("dng_cty_3_2_a_enter", settings.dng_cty_3_2_a_enter),
            60 => return vars.split("twn_cty_1_1_a_enter", settings.twn_cty_1_1_a_enter),
            61 => return vars.split("twn_cty_1_1_b_enter", settings.twn_cty_1_1_b_enter),
            62 => return vars.split("twn_cty_1_1_c_enter", settings.twn_cty_1_1_c_enter),
            63 => return vars.split("twn_cty_1_2_a_enter", settings.twn_cty_1_2_a_enter),
            64 => return vars.split("twn_cty_2_1_a_enter", settings.twn_cty_2_1_a_enter),
            65 => return vars.split("twn_cty_2_1_b_enter", settings.twn_cty_2_1_b_enter),
            66 => return vars.split("twn_cty_3_1_a_enter", settings.twn_cty_3_1_a_enter),
            67 => return vars.split("fld_isd_1_1_enter", settings.fld_isd_1_1_enter),
            68 => return vars.split("fld_isd_1_2_enter", settings.fld_isd_1_2_enter),
            69 => return vars.split("fld_isd_1_3_enter", settings.fld_isd_1_3_enter),
            70 => return vars.split("fld_isd_2_1_enter", settings.fld_isd_2_1_enter),
            71 => return vars.split("dng_isd_3_1_b_enter", settings.dng_isd_3_1_b_enter),
            72 => return vars.split("dng_isd_3_2_a_enter", settings.dng_isd_3_2_a_enter),
            73 => return vars.split("fld_isd_3_3_enter", settings.fld_isd_3_3_enter),
            74 => return vars.split("dng_isd_1_1_enter", settings.dng_isd_1_1_enter),
            75 => return vars.split("dng_isd_2_1_enter", settings.dng_isd_2_1_enter),
            76 => return vars.split("dng_isd_3_1_a_enter", settings.dng_isd_3_1_a_enter),
            77 => return vars.split("dng_isd_3_2_b_enter", settings.dng_isd_3_2_b_enter),
            78 => return vars.split("twn_isd_1_1_a_enter", settings.twn_isd_1_1_a_enter),
            79 => return vars.split("twn_isd_2_1_a_enter", settings.twn_isd_2_1_a_enter),
            80 => return vars.split("twn_isd_2_1_b_enter", settings.twn_isd_2_1_b_enter),
            81 => return vars.split("twn_isd_2_1_c_enter", settings.twn_isd_2_1_c_enter),
            82 => return vars.split("twn_isd_3_1_a_enter", settings.twn_isd_3_1_a_enter),
            83 => return vars.split("fld_wld_1_1_a_enter", settings.fld_wld_1_1_a_enter),
            84 => return vars.split("fld_wld_1_1_b_enter", settings.fld_wld_1_1_b_enter),
            85 => return vars.split("fld_wld_1_2_enter", settings.fld_wld_1_2_enter),
            86 => return vars.split("fld_wld_1_3_enter", settings.fld_wld_1_3_enter),
            87 => return vars.split("fld_wld_2_1_enter", settings.fld_wld_2_1_enter),
            88 => return vars.split("fld_wld_2_2_enter", settings.fld_wld_2_2_enter),
            89 => return vars.split("fld_wld_3_1_enter", settings.fld_wld_3_1_enter),
            90 => return vars.split("fld_wld_3_2_enter", settings.fld_wld_3_2_enter),
            91 => return vars.split("dng_wld_1_1_enter", settings.dng_wld_1_1_enter),
            92 => return vars.split("dng_wld_1_2_enter", settings.dng_wld_1_2_enter),
            94 => return vars.split("dng_wld_2_1_a_enter", settings.dng_wld_2_1_a_enter),
            95 => return vars.split("dng_wld_2_job_enter", settings.dng_wld_2_job_enter),
            96 => return vars.split("dng_wld_3_1_a_enter", settings.dng_wld_3_1_a_enter),
            97 => return vars.split("dng_wld_3_2_enter", settings.dng_wld_3_2_enter),
            98 => return vars.split("twn_wld_1_1_a_enter", settings.twn_wld_1_1_a_enter),
            99 => return vars.split("twn_wld_1_1_b_enter", settings.twn_wld_1_1_b_enter),
            100 => return vars.split("twn_wld_1_1_c_enter", settings.twn_wld_1_1_c_enter),
            101 => return vars.split("twn_wld_2_1_a_enter", settings.twn_wld_2_1_a_enter),
            102 => return vars.split("fld_wld_2_3_enter", settings.fld_wld_2_3_enter),
            103 => return vars.split("dng_wld_2_2_enter", settings.dng_wld_2_2_enter),
            104 => return vars.split("twn_wld_3_1_a_enter", settings.twn_wld_3_1_a_enter),
            106 => return vars.split("fld_sea_1_1_enter", settings.fld_sea_1_1_enter),
            107 => return vars.split("fld_sea_1_2_enter", settings.fld_sea_1_2_enter),
            108 => return vars.split("fld_sea_1_3_enter", settings.fld_sea_1_3_enter),
            110 => return vars.split("fld_sea_2_2_enter", settings.fld_sea_2_2_enter),
            111 => return vars.split("fld_sea_2_3_enter", settings.fld_sea_2_3_enter),
            112 => return vars.split("fld_sea_3_1_enter", settings.fld_sea_3_1_enter),
            113 => return vars.split("dng_sea_1_1_enter", settings.dng_sea_1_1_enter),
            114 => return vars.split("dng_sea_1_2_enter", settings.dng_sea_1_2_enter),
            115 => return vars.split("dng_sea_1_3_enter", settings.dng_sea_1_3_enter),
            116 => return vars.split("dng_sea_2_1_enter", settings.dng_sea_2_1_enter),
            117 => return vars.split("dng_sea_2_2_enter", settings.dng_sea_2_2_enter),
            118 => return vars.split("dng_sea_2_3_enter", settings.dng_sea_2_3_enter),
            119 => return vars.split("dng_sea_2_job_enter", settings.dng_sea_2_job_enter),
            120 => return vars.split("fld_dst_3_1_b_enter", settings.fld_dst_3_1_b_enter),
            121 => {
                return vars.split(
                    "twn_dst_3_1_a_fire_enter",
                    settings.twn_dst_3_1_a_fire_enter,
                )
            }
            122 => return vars.split("dng_sea_3_1_a_enter", settings.dng_sea_3_1_a_enter),
            123 => return vars.split("twn_sea_1_1_a_enter", settings.twn_sea_1_1_a_enter),
            124 => return vars.split("twn_sea_2_1_a_enter", settings.twn_sea_2_1_a_enter),
            125 => return vars.split("twn_sea_2_1_b_enter", settings.twn_sea_2_1_b_enter),
            126 => return vars.split("twn_sea_2_1_c_enter", settings.twn_sea_2_1_c_enter),
            127 => return vars.split("twn_sea_3_1_a_enter", settings.twn_sea_3_1_a_enter),
            128 => return vars.split("twn_sea_3_1_b_enter", settings.twn_sea_3_1_b_enter),
            129 => return vars.split("fld_fst_1_1_enter", settings.fld_fst_1_1_enter),
            130 => return vars.split("fld_fst_1_2_enter", settings.fld_fst_1_2_enter),
            131 => return vars.split("fld_fst_1_3_enter", settings.fld_fst_1_3_enter),
            132 => return vars.split("fld_fst_1_4_enter", settings.fld_fst_1_4_enter),
            133 => return vars.split("fld_fst_2_1_enter", settings.fld_fst_2_1_enter),
            134 => return vars.split("fld_fst_2_2_enter", settings.fld_fst_2_2_enter),
            135 => return vars.split("fld_fst_3_1_enter", settings.fld_fst_3_1_enter),
            136 => return vars.split("fld_fst_3_2_enter", settings.fld_fst_3_2_enter),
            137 => return vars.split("dng_fst_1_1_enter", settings.dng_fst_1_1_enter),
            138 => return vars.split("dng_fst_1_2_enter", settings.dng_fst_1_2_enter),
            139 => return vars.split("dng_fst_2_1_enter", settings.dng_fst_2_1_enter),
            140 => return vars.split("dng_fst_2_2_a_enter", settings.dng_fst_2_2_a_enter),
            141 => return vars.split("dng_fst_2_job_enter", settings.dng_fst_2_job_enter),
            142 => return vars.split("dng_fst_3_1_a_enter", settings.dng_fst_3_1_a_enter),
            143 => return vars.split("dng_fst_3_2_enter", settings.dng_fst_3_2_enter),
            144 => return vars.split("dng_fst_3_3_enter", settings.dng_fst_3_3_enter),
            145 => return vars.split("twn_fst_1_1_a_enter", settings.twn_fst_1_1_a_enter),
            146 => return vars.split("twn_fst_2_1_a_enter", settings.twn_fst_2_1_a_enter),
            147 => return vars.split("twn_fst_2_1_b_enter", settings.twn_fst_2_1_b_enter),
            149 => return vars.split("twn_fst_3_1_a_enter", settings.twn_fst_3_1_a_enter),
            150 => return vars.split("twn_fst_3_1_b_enter", settings.twn_fst_3_1_b_enter),
            151 => return vars.split("fld_snw_1_1_a_enter", settings.fld_snw_1_1_a_enter),
            152 => return vars.split("fld_snw_1_2_enter", settings.fld_snw_1_2_enter),
            153 => return vars.split("fld_snw_1_3_enter", settings.fld_snw_1_3_enter),
            154 => return vars.split("twn_snw_2_1_b_enter", settings.twn_snw_2_1_b_enter),
            155 => return vars.split("fld_snw_2_2_enter", settings.fld_snw_2_2_enter),
            156 => return vars.split("fld_snw_3_1_enter", settings.fld_snw_3_1_enter),
            158 => return vars.split("twn_snw_3_1_c_enter", settings.twn_snw_3_1_c_enter),
            159 => return vars.split("dng_snw_1_1_enter", settings.dng_snw_1_1_enter),
            160 => return vars.split("dng_snw_1_2_enter", settings.dng_snw_1_2_enter),
            161 => return vars.split("dng_snw_2_1_enter", settings.dng_snw_2_1_enter),
            162 => return vars.split("dng_snw_3_1_enter", settings.dng_snw_3_1_enter),
            163 => return vars.split("dng_snw_3_2_a_enter", settings.dng_snw_3_2_a_enter),
            164 => return vars.split("dng_snw_3_3_enter", settings.dng_snw_3_3_enter),
            165 => return vars.split("dng_snw_3_4_a_enter", settings.dng_snw_3_4_a_enter),
            166 => return vars.split("twn_snw_1_1_a_enter", settings.twn_snw_1_1_a_enter),
            167 => return vars.split("twn_snw_1_1_b_enter", settings.twn_snw_1_1_b_enter),
            168 => return vars.split("twn_snw_1_1_c_enter", settings.twn_snw_1_1_c_enter),
            169 => return vars.split("twn_snw_1_2_a_enter", settings.twn_snw_1_2_a_enter),
            170 => return vars.split("twn_snw_2_1_a_enter", settings.twn_snw_2_1_a_enter),
            171 => return vars.split("twn_snw_3_1_a_enter", settings.twn_snw_3_1_a_enter),
            172 => return vars.split("twn_snw_3_1_b_enter", settings.twn_snw_3_1_b_enter),
            173 => return vars.split("twn_snw_3_2_a_enter", settings.twn_snw_3_2_a_enter),
            174 => return vars.split("dng_snw_2_job_enter", settings.dng_snw_2_job_enter),
            175 => return vars.split("dng_isd_2_job_enter", settings.dng_isd_2_job_enter),
            176 => return vars.split("dng_isd_1_2_enter", settings.dng_isd_1_2_enter),
            177 => return vars.split("twn_dst_2_1_b_enter", settings.twn_dst_2_1_b_enter),
            179 => return vars.split("fld_dst_2_5_a_enter", settings.fld_dst_2_5_a_enter),
            180 => return vars.split("dng_dst_2_4_enter", settings.dng_dst_2_4_enter),
            181 => return vars.split("fld_cty_2_2_enter", settings.fld_cty_2_2_enter),
            182 => return vars.split("dng_cty_2_2_enter", settings.dng_cty_2_2_enter),
            189 => return vars.split("fld_ocn_1_1_enter", settings.fld_ocn_1_1_enter),
            190 => return vars.split("dng_mnt_2_3_enter", settings.dng_mnt_2_3_enter),
            191 => return vars.split("twn_fst_3_2_a_enter", settings.twn_fst_3_2_a_enter),
            192 => return vars.split("dng_sea_3_1_b_enter", settings.dng_sea_3_1_b_enter),
            193 => return vars.split("twn_wld_1_2_a_enter", settings.twn_wld_1_2_a_enter),
            194 => return vars.split("twn_wld_1_2_b_enter", settings.twn_wld_1_2_b_enter),
            196 => return vars.split("twn_cty_1_2_b_enter", settings.twn_cty_1_2_b_enter),
            197 => return vars.split("fld_dst_2_5_b_enter", settings.fld_dst_2_5_b_enter),
            198 => return vars.split("fld_snw_1_1_b_enter", settings.fld_snw_1_1_b_enter),
            199 => return vars.split("dng_snw_3_2_b_enter", settings.dng_snw_3_2_b_enter),
            200 => return vars.split("dng_wld_2_1_b_enter", settings.dng_wld_2_1_b_enter),
            201 => return vars.split("dng_cty_3_2_b_enter", settings.dng_cty_3_2_b_enter),
            202 => return vars.split("dng_isd_3_1_c_enter", settings.dng_isd_3_1_c_enter),
            203 => return vars.split("dng_isd_3_2_c_enter", settings.dng_isd_3_2_c_enter),
            204 => return vars.split("dng_isd_3_2_d_enter", settings.dng_isd_3_2_d_enter),
            205 => return vars.split("dng_isd_3_2_e_enter", settings.dng_isd_3_2_e_enter),
            206 => return vars.split("dng_wld_3_1_b_enter", settings.dng_wld_3_1_b_enter),
            207 => return vars.split("dng_fst_3_1_b_enter", settings.dng_fst_3_1_b_enter),
            208 => return vars.split("fld_atl_3_1_enter", settings.fld_atl_3_1_enter),
            209 => return vars.split("dng_atl_3_1_enter", settings.dng_atl_3_1_enter),
            214 => return vars.split("dng_dst_3_2_a_enter", settings.dng_dst_3_2_a_enter),
            215 => return vars.split("dng_dst_3_2_b_enter", settings.dng_dst_3_2_b_enter),
            216 => return vars.split("dng_dst_3_2_c_enter", settings.dng_dst_3_2_c_enter),
            217 => return vars.split("dng_dst_3_2_d_enter", settings.dng_dst_3_2_d_enter),
            218 => return vars.split("dng_fst_2_2_b_enter", settings.dng_fst_2_2_b_enter),
            219 => return vars.split("fld_ocn_1_2_enter", settings.fld_ocn_1_2_enter),
            220 => return vars.split("fld_ocn_1_3_enter", settings.fld_ocn_1_3_enter),
            221 => return vars.split("dng_ocn_1_1_enter", settings.dng_ocn_1_1_enter),
            222 => return vars.split("dng_ocn_1_2_enter", settings.dng_ocn_1_2_enter),
            223 => return vars.split("dng_ocn_1_3_enter", settings.dng_ocn_1_3_enter),
            224 => return vars.split("dng_ocn_1_4_enter", settings.dng_ocn_1_4_enter),
            225 => return vars.split("dng_dst_3_2_e_enter", settings.dng_dst_3_2_e_enter),
            227 => return vars.split("dng_cty_2_3_enter", settings.dng_cty_2_3_enter),
            228 => return vars.split("fld_ocn_1_4_enter", settings.fld_ocn_1_4_enter),
            229 => return vars.split("dng_snw_3_4_b_enter", settings.dng_snw_3_4_b_enter),
            _ => None,
        };
    }

    pub fn exit_area(vars: &mut Vars, settings: &Settings) -> Option<String> {
        return match vars.level_id.old {
            5 => return vars.split("fld_mnt_1_1_exit", settings.fld_mnt_1_1_exit),
            6 => return vars.split("fld_mnt_1_2_exit", settings.fld_mnt_1_2_exit),
            7 => return vars.split("fld_mnt_2_1_exit", settings.fld_mnt_2_1_exit),
            8 => return vars.split("fld_mnt_2_2_exit", settings.fld_mnt_2_2_exit),
            9 => return vars.split("fld_mnt_3_1_exit", settings.fld_mnt_3_1_exit),
            10 => return vars.split("fld_mnt_3_2_exit", settings.fld_mnt_3_2_exit),
            11 => return vars.split("fld_mnt_3_3_exit", settings.fld_mnt_3_3_exit),
            12 => return vars.split("dng_mnt_1_1_exit", settings.dng_mnt_1_1_exit),
            13 => return vars.split("dng_mnt_2_1_exit", settings.dng_mnt_2_1_exit),
            14 => return vars.split("dng_mnt_2_2_exit", settings.dng_mnt_2_2_exit),
            15 => return vars.split("dng_mnt_3_3_exit", settings.dng_mnt_3_3_exit),
            16 => return vars.split("dng_mnt_2_job_exit", settings.dng_mnt_2_job_exit),
            17 => return vars.split("dng_mnt_3_1_a_exit", settings.dng_mnt_3_1_a_exit),
            19 => return vars.split("fld_dst_1_1_exit", settings.fld_dst_1_1_exit),
            20 => return vars.split("twn_mnt_1_1_a_exit", settings.twn_mnt_1_1_a_exit),
            21 => return vars.split("twn_mnt_1_2_a_exit", settings.twn_mnt_1_2_a_exit),
            22 => return vars.split("twn_mnt_1_2_b_exit", settings.twn_mnt_1_2_b_exit),
            23 => return vars.split("fld_dst_3_1_a_exit", settings.fld_dst_3_1_a_exit),
            24 => return vars.split("twn_mnt_2_1_a_exit", settings.twn_mnt_2_1_a_exit),
            25 => return vars.split("twn_mnt_2_1_b_exit", settings.twn_mnt_2_1_b_exit),
            26 => return vars.split("twn_mnt_2_1_c_exit", settings.twn_mnt_2_1_c_exit),
            27 => return vars.split("twn_mnt_3_1_a_exit", settings.twn_mnt_3_1_a_exit),
            28 => return vars.split("twn_dst_1_1_a_exit", settings.twn_dst_1_1_a_exit),
            29 => return vars.split("twn_dst_2_1_a_exit", settings.twn_dst_2_1_a_exit),
            30 => return vars.split("twn_dst_3_1_a_exit", settings.twn_dst_3_1_a_exit),
            31 => return vars.split("twn_dst_3_1_b_exit", settings.twn_dst_3_1_b_exit),
            32 => return vars.split("twn_dst_3_1_c_exit", settings.twn_dst_3_1_c_exit),
            33 => return vars.split("twn_mnt_3_1_b_exit", settings.twn_mnt_3_1_b_exit),
            34 => return vars.split("dng_mnt_3_1_b_exit", settings.dng_mnt_3_1_b_exit),
            35 => return vars.split("fld_dst_2_2_exit", settings.fld_dst_2_2_exit),
            36 => return vars.split("fld_dst_2_3_exit", settings.fld_dst_2_3_exit),
            37 => return vars.split("fld_dst_2_4_exit", settings.fld_dst_2_4_exit),
            38 => return vars.split("fld_dst_3_2_exit", settings.fld_dst_3_2_exit),
            39 => return vars.split("dng_dst_2_1_exit", settings.dng_dst_2_1_exit),
            40 => return vars.split("dng_dst_2_2_exit", settings.dng_dst_2_2_exit),
            41 => return vars.split("dng_dst_2_3_exit", settings.dng_dst_2_3_exit),
            42 => return vars.split("dng_dst_2_job_exit", settings.dng_dst_2_job_exit),
            43 => return vars.split("dng_dst_3_1_exit", settings.dng_dst_3_1_exit),
            44 => return vars.split("fld_cty_1_1_exit", settings.fld_cty_1_1_exit),
            45 => return vars.split("fld_cty_1_2_exit", settings.fld_cty_1_2_exit),
            46 => return vars.split("fld_cty_1_3_exit", settings.fld_cty_1_3_exit),
            47 => return vars.split("fld_cty_1_4_exit", settings.fld_cty_1_4_exit),
            48 => return vars.split("fld_cty_2_1_exit", settings.fld_cty_2_1_exit),
            49 => return vars.split("fld_cty_3_1_exit", settings.fld_cty_3_1_exit),
            50 => return vars.split("dng_cty_1_1_exit", settings.dng_cty_1_1_exit),
            51 => return vars.split("dng_cty_1_2_exit", settings.dng_cty_1_2_exit),
            52 => return vars.split("dng_cty_1_3_exit", settings.dng_cty_1_3_exit),
            53 => return vars.split("dng_cty_1_4_exit", settings.dng_cty_1_4_exit),
            54 => return vars.split("dng_cty_2_1_exit", settings.dng_cty_2_1_exit),
            56 => return vars.split("dng_cty_2_job_exit", settings.dng_cty_2_job_exit),
            57 => return vars.split("dng_cty_3_1_exit", settings.dng_cty_3_1_exit),
            58 => return vars.split("dng_cty_3_2_a_exit", settings.dng_cty_3_2_a_exit),
            60 => return vars.split("twn_cty_1_1_a_exit", settings.twn_cty_1_1_a_exit),
            61 => return vars.split("twn_cty_1_1_b_exit", settings.twn_cty_1_1_b_exit),
            62 => return vars.split("twn_cty_1_1_c_exit", settings.twn_cty_1_1_c_exit),
            63 => return vars.split("twn_cty_1_2_a_exit", settings.twn_cty_1_2_a_exit),
            64 => return vars.split("twn_cty_2_1_a_exit", settings.twn_cty_2_1_a_exit),
            65 => return vars.split("twn_cty_2_1_b_exit", settings.twn_cty_2_1_b_exit),
            66 => return vars.split("twn_cty_3_1_a_exit", settings.twn_cty_3_1_a_exit),
            67 => return vars.split("fld_isd_1_1_exit", settings.fld_isd_1_1_exit),
            68 => return vars.split("fld_isd_1_2_exit", settings.fld_isd_1_2_exit),
            69 => return vars.split("fld_isd_1_3_exit", settings.fld_isd_1_3_exit),
            70 => return vars.split("fld_isd_2_1_exit", settings.fld_isd_2_1_exit),
            71 => return vars.split("dng_isd_3_1_b_exit", settings.dng_isd_3_1_b_exit),
            72 => return vars.split("dng_isd_3_2_a_exit", settings.dng_isd_3_2_a_exit),
            73 => return vars.split("fld_isd_3_3_exit", settings.fld_isd_3_3_exit),
            74 => return vars.split("dng_isd_1_1_exit", settings.dng_isd_1_1_exit),
            75 => return vars.split("dng_isd_2_1_exit", settings.dng_isd_2_1_exit),
            76 => return vars.split("dng_isd_3_1_a_exit", settings.dng_isd_3_1_a_exit),
            77 => return vars.split("dng_isd_3_2_b_exit", settings.dng_isd_3_2_b_exit),
            78 => return vars.split("twn_isd_1_1_a_exit", settings.twn_isd_1_1_a_exit),
            79 => return vars.split("twn_isd_2_1_a_exit", settings.twn_isd_2_1_a_exit),
            80 => return vars.split("twn_isd_2_1_b_exit", settings.twn_isd_2_1_b_exit),
            81 => return vars.split("twn_isd_2_1_c_exit", settings.twn_isd_2_1_c_exit),
            82 => return vars.split("twn_isd_3_1_a_exit", settings.twn_isd_3_1_a_exit),
            83 => return vars.split("fld_wld_1_1_a_exit", settings.fld_wld_1_1_a_exit),
            84 => return vars.split("fld_wld_1_1_b_exit", settings.fld_wld_1_1_b_exit),
            85 => return vars.split("fld_wld_1_2_exit", settings.fld_wld_1_2_exit),
            86 => return vars.split("fld_wld_1_3_exit", settings.fld_wld_1_3_exit),
            87 => return vars.split("fld_wld_2_1_exit", settings.fld_wld_2_1_exit),
            88 => return vars.split("fld_wld_2_2_exit", settings.fld_wld_2_2_exit),
            89 => return vars.split("fld_wld_3_1_exit", settings.fld_wld_3_1_exit),
            90 => return vars.split("fld_wld_3_2_exit", settings.fld_wld_3_2_exit),
            91 => return vars.split("dng_wld_1_1_exit", settings.dng_wld_1_1_exit),
            92 => return vars.split("dng_wld_1_2_exit", settings.dng_wld_1_2_exit),
            94 => return vars.split("dng_wld_2_1_a_exit", settings.dng_wld_2_1_a_exit),
            95 => return vars.split("dng_wld_2_job_exit", settings.dng_wld_2_job_exit),
            96 => return vars.split("dng_wld_3_1_a_exit", settings.dng_wld_3_1_a_exit),
            97 => return vars.split("dng_wld_3_2_exit", settings.dng_wld_3_2_exit),
            98 => return vars.split("twn_wld_1_1_a_exit", settings.twn_wld_1_1_a_exit),
            99 => return vars.split("twn_wld_1_1_b_exit", settings.twn_wld_1_1_b_exit),
            100 => return vars.split("twn_wld_1_1_c_exit", settings.twn_wld_1_1_c_exit),
            101 => return vars.split("twn_wld_2_1_a_exit", settings.twn_wld_2_1_a_exit),
            102 => return vars.split("fld_wld_2_3_exit", settings.fld_wld_2_3_exit),
            103 => return vars.split("dng_wld_2_2_exit", settings.dng_wld_2_2_exit),
            104 => return vars.split("twn_wld_3_1_a_exit", settings.twn_wld_3_1_a_exit),
            106 => return vars.split("fld_sea_1_1_exit", settings.fld_sea_1_1_exit),
            107 => return vars.split("fld_sea_1_2_exit", settings.fld_sea_1_2_exit),
            108 => return vars.split("fld_sea_1_3_exit", settings.fld_sea_1_3_exit),
            110 => return vars.split("fld_sea_2_2_exit", settings.fld_sea_2_2_exit),
            111 => return vars.split("fld_sea_2_3_exit", settings.fld_sea_2_3_exit),
            112 => return vars.split("fld_sea_3_1_exit", settings.fld_sea_3_1_exit),
            113 => return vars.split("dng_sea_1_1_exit", settings.dng_sea_1_1_exit),
            114 => return vars.split("dng_sea_1_2_exit", settings.dng_sea_1_2_exit),
            115 => return vars.split("dng_sea_1_3_exit", settings.dng_sea_1_3_exit),
            116 => return vars.split("dng_sea_2_1_exit", settings.dng_sea_2_1_exit),
            117 => return vars.split("dng_sea_2_2_exit", settings.dng_sea_2_2_exit),
            118 => return vars.split("dng_sea_2_3_exit", settings.dng_sea_2_3_exit),
            119 => return vars.split("dng_sea_2_job_exit", settings.dng_sea_2_job_exit),
            120 => return vars.split("fld_dst_3_1_b_exit", settings.fld_dst_3_1_b_exit),
            121 => return vars.split("twn_dst_3_1_a_fire_exit", settings.twn_dst_3_1_a_fire_exit),
            122 => return vars.split("dng_sea_3_1_a_exit", settings.dng_sea_3_1_a_exit),
            123 => return vars.split("twn_sea_1_1_a_exit", settings.twn_sea_1_1_a_exit),
            124 => return vars.split("twn_sea_2_1_a_exit", settings.twn_sea_2_1_a_exit),
            125 => return vars.split("twn_sea_2_1_b_exit", settings.twn_sea_2_1_b_exit),
            126 => return vars.split("twn_sea_2_1_c_exit", settings.twn_sea_2_1_c_exit),
            127 => return vars.split("twn_sea_3_1_a_exit", settings.twn_sea_3_1_a_exit),
            128 => return vars.split("twn_sea_3_1_b_exit", settings.twn_sea_3_1_b_exit),
            129 => return vars.split("fld_fst_1_1_exit", settings.fld_fst_1_1_exit),
            130 => return vars.split("fld_fst_1_2_exit", settings.fld_fst_1_2_exit),
            131 => return vars.split("fld_fst_1_3_exit", settings.fld_fst_1_3_exit),
            132 => return vars.split("fld_fst_1_4_exit", settings.fld_fst_1_4_exit),
            133 => return vars.split("fld_fst_2_1_exit", settings.fld_fst_2_1_exit),
            134 => return vars.split("fld_fst_2_2_exit", settings.fld_fst_2_2_exit),
            135 => return vars.split("fld_fst_3_1_exit", settings.fld_fst_3_1_exit),
            136 => return vars.split("fld_fst_3_2_exit", settings.fld_fst_3_2_exit),
            137 => return vars.split("dng_fst_1_1_exit", settings.dng_fst_1_1_exit),
            138 => return vars.split("dng_fst_1_2_exit", settings.dng_fst_1_2_exit),
            139 => return vars.split("dng_fst_2_1_exit", settings.dng_fst_2_1_exit),
            140 => return vars.split("dng_fst_2_2_a_exit", settings.dng_fst_2_2_a_exit),
            141 => return vars.split("dng_fst_2_job_exit", settings.dng_fst_2_job_exit),
            142 => return vars.split("dng_fst_3_1_a_exit", settings.dng_fst_3_1_a_exit),
            143 => return vars.split("dng_fst_3_2_exit", settings.dng_fst_3_2_exit),
            144 => return vars.split("dng_fst_3_3_exit", settings.dng_fst_3_3_exit),
            145 => return vars.split("twn_fst_1_1_a_exit", settings.twn_fst_1_1_a_exit),
            146 => return vars.split("twn_fst_2_1_a_exit", settings.twn_fst_2_1_a_exit),
            147 => return vars.split("twn_fst_2_1_b_exit", settings.twn_fst_2_1_b_exit),
            149 => return vars.split("twn_fst_3_1_a_exit", settings.twn_fst_3_1_a_exit),
            150 => return vars.split("twn_fst_3_1_b_exit", settings.twn_fst_3_1_b_exit),
            151 => return vars.split("fld_snw_1_1_a_exit", settings.fld_snw_1_1_a_exit),
            152 => return vars.split("fld_snw_1_2_exit", settings.fld_snw_1_2_exit),
            153 => return vars.split("fld_snw_1_3_exit", settings.fld_snw_1_3_exit),
            154 => return vars.split("twn_snw_2_1_b_exit", settings.twn_snw_2_1_b_exit),
            155 => return vars.split("fld_snw_2_2_exit", settings.fld_snw_2_2_exit),
            156 => return vars.split("fld_snw_3_1_exit", settings.fld_snw_3_1_exit),
            158 => return vars.split("twn_snw_3_1_c_exit", settings.twn_snw_3_1_c_exit),
            159 => return vars.split("dng_snw_1_1_exit", settings.dng_snw_1_1_exit),
            160 => return vars.split("dng_snw_1_2_exit", settings.dng_snw_1_2_exit),
            161 => return vars.split("dng_snw_2_1_exit", settings.dng_snw_2_1_exit),
            162 => return vars.split("dng_snw_3_1_exit", settings.dng_snw_3_1_exit),
            163 => return vars.split("dng_snw_3_2_a_exit", settings.dng_snw_3_2_a_exit),
            164 => return vars.split("dng_snw_3_3_exit", settings.dng_snw_3_3_exit),
            165 => return vars.split("dng_snw_3_4_a_exit", settings.dng_snw_3_4_a_exit),
            166 => return vars.split("twn_snw_1_1_a_exit", settings.twn_snw_1_1_a_exit),
            167 => return vars.split("twn_snw_1_1_b_exit", settings.twn_snw_1_1_b_exit),
            168 => return vars.split("twn_snw_1_1_c_exit", settings.twn_snw_1_1_c_exit),
            169 => return vars.split("twn_snw_1_2_a_exit", settings.twn_snw_1_2_a_exit),
            170 => return vars.split("twn_snw_2_1_a_exit", settings.twn_snw_2_1_a_exit),
            171 => return vars.split("twn_snw_3_1_a_exit", settings.twn_snw_3_1_a_exit),
            172 => return vars.split("twn_snw_3_1_b_exit", settings.twn_snw_3_1_b_exit),
            173 => return vars.split("twn_snw_3_2_a_exit", settings.twn_snw_3_2_a_exit),
            174 => return vars.split("dng_snw_2_job_exit", settings.dng_snw_2_job_exit),
            175 => return vars.split("dng_isd_2_job_exit", settings.dng_isd_2_job_exit),
            176 => return vars.split("dng_isd_1_2_exit", settings.dng_isd_1_2_exit),
            177 => return vars.split("twn_dst_2_1_b_exit", settings.twn_dst_2_1_b_exit),
            179 => return vars.split("fld_dst_2_5_a_exit", settings.fld_dst_2_5_a_exit),
            180 => return vars.split("dng_dst_2_4_exit", settings.dng_dst_2_4_exit),
            181 => return vars.split("fld_cty_2_2_exit", settings.fld_cty_2_2_exit),
            182 => return vars.split("dng_cty_2_2_exit", settings.dng_cty_2_2_exit),
            189 => return vars.split("fld_ocn_1_1_exit", settings.fld_ocn_1_1_exit),
            190 => return vars.split("dng_mnt_2_3_exit", settings.dng_mnt_2_3_exit),
            191 => return vars.split("twn_fst_3_2_a_exit", settings.twn_fst_3_2_a_exit),
            192 => return vars.split("dng_sea_3_1_b_exit", settings.dng_sea_3_1_b_exit),
            193 => return vars.split("twn_wld_1_2_a_exit", settings.twn_wld_1_2_a_exit),
            194 => return vars.split("twn_wld_1_2_b_exit", settings.twn_wld_1_2_b_exit),
            196 => return vars.split("twn_cty_1_2_b_exit", settings.twn_cty_1_2_b_exit),
            197 => return vars.split("fld_dst_2_5_b_exit", settings.fld_dst_2_5_b_exit),
            198 => return vars.split("fld_snw_1_1_b_exit", settings.fld_snw_1_1_b_exit),
            199 => return vars.split("dng_snw_3_2_b_exit", settings.dng_snw_3_2_b_exit),
            200 => return vars.split("dng_wld_2_1_b_exit", settings.dng_wld_2_1_b_exit),
            201 => return vars.split("dng_cty_3_2_b_exit", settings.dng_cty_3_2_b_exit),
            202 => return vars.split("dng_isd_3_1_c_exit", settings.dng_isd_3_1_c_exit),
            203 => return vars.split("dng_isd_3_2_c_exit", settings.dng_isd_3_2_c_exit),
            204 => return vars.split("dng_isd_3_2_d_exit", settings.dng_isd_3_2_d_exit),
            205 => return vars.split("dng_isd_3_2_e_exit", settings.dng_isd_3_2_e_exit),
            206 => return vars.split("dng_wld_3_1_b_exit", settings.dng_wld_3_1_b_exit),
            207 => return vars.split("dng_fst_3_1_b_exit", settings.dng_fst_3_1_b_exit),
            208 => return vars.split("fld_atl_3_1_exit", settings.fld_atl_3_1_exit),
            209 => return vars.split("dng_atl_3_1_exit", settings.dng_atl_3_1_exit),
            214 => return vars.split("dng_dst_3_2_a_exit", settings.dng_dst_3_2_a_exit),
            215 => return vars.split("dng_dst_3_2_b_exit", settings.dng_dst_3_2_b_exit),
            216 => return vars.split("dng_dst_3_2_c_exit", settings.dng_dst_3_2_c_exit),
            217 => return vars.split("dng_dst_3_2_d_exit", settings.dng_dst_3_2_d_exit),
            218 => return vars.split("dng_fst_2_2_b_exit", settings.dng_fst_2_2_b_exit),
            219 => return vars.split("fld_ocn_1_2_exit", settings.fld_ocn_1_2_exit),
            220 => return vars.split("fld_ocn_1_3_exit", settings.fld_ocn_1_3_exit),
            221 => return vars.split("dng_ocn_1_1_exit", settings.dng_ocn_1_1_exit),
            222 => return vars.split("dng_ocn_1_2_exit", settings.dng_ocn_1_2_exit),
            223 => return vars.split("dng_ocn_1_3_exit", settings.dng_ocn_1_3_exit),
            224 => return vars.split("dng_ocn_1_4_exit", settings.dng_ocn_1_4_exit),
            225 => return vars.split("dng_dst_3_2_e_exit", settings.dng_dst_3_2_e_exit),
            227 => return vars.split("dng_cty_2_3_exit", settings.dng_cty_2_3_exit),
            228 => return vars.split("fld_ocn_1_4_exit", settings.fld_ocn_1_4_exit),
            229 => return vars.split("dng_snw_3_4_b_exit", settings.dng_snw_3_4_b_exit),
            _ => None,
        };
    }
}
