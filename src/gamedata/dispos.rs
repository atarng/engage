use unity::prelude::*;
use super::{PersonData, StructBaseFields};
use crate::gamedata::*;
// Contains DisposData and ChapterData Impls
#[unity::class("App", "DisposData")]
pub struct DisposData {
    pub parent: StructDataArrayFields,
    pub _group: &'static Il2CppString,
    pub pid: &'static Il2CppString,
    tid: &'static Il2CppString,
    pub flag: &'static DisposDataFlag,
    jid: &'static Il2CppString,
    sid: &'static Il2CppString,
    bid: &'static Il2CppString,
    pub appear_x: i8,
    pub appear_y: i8,
    pub dispos_x: i8,
    pub dispos_y: i8,
    pub direction: i32,
    pub rotation: i8,
    pub level_n: u8,
    pub level_h: u8,
    pub level_l: u8,
    __: i32,
    items: u64,
    item1: u64,
    item2: u64,
    item3: u64,
    item4: u64,
    item5: u64,
    item6: u64,
    pub gid: Option<&'static Il2CppString>,
    pub hp_stock_count: u32,
    state0: i32,
    state1: i32,
    state2: i32,
    state3: i32,
    state4: i32,
    state5: i32,
    ___: i32,
    pub ai_action_name: &'static Il2CppString,
    pub ai_action_value: Option<&'static Il2CppString>,
    pub ai_mind_name: &'static Il2CppString,
    pub ai_mind_value: Option<&'static Il2CppString>,
    pub ai_attack_name: &'static Il2CppString,
    pub ai_attack_value: Option<&'static Il2CppString>,
    pub ai_move_name: &'static Il2CppString,
    pub ai_move_value: Option<&'static Il2CppString>,
}
#[unity::class("App", "DisposDataFlag")]
pub struct DisposDataFlag {
    pub value: i32,
}
impl GamedataArray for DisposData {}

impl DisposData {
    pub fn get_flag(&self) -> &'static mut DisposDataFlag { unsafe { disposdata_get_flag(self, None)}}
    pub fn get_force(&self) -> i8 { unsafe {disposdata_get_force(self, None)}}
    pub fn get_gid(&self) -> &'static Il2CppString { unsafe { disposdata_get_gid(self, None)} }
    pub fn get_hp_stock_count(&self) -> u8 { unsafe {disposdata_get_hp_stock_count(self, None)}}
    pub fn get_person(&self) -> Option<&PersonData> { unsafe { disposdata_get_person(self, None)}}
    pub fn get_pid(&self) -> Option<&'static Il2CppString> { unsafe { disposdata_get_pid(self, None)}}
    pub fn get_sid(&self) -> Option<&'static Il2CppString> { unsafe { disposdata_get_sid(self, None)}}

    pub fn set_ai_attack_name(&self, name: &Il2CppString) { unsafe { disposdata_set_ai_attack_name(self, name, None); }}
    pub fn set_ai_attack_value(&self, value: &Il2CppString) { unsafe { disposdata_set_ai_attack_value(self, value, None);}}
    pub fn set_flag(&self, flag: &mut DisposDataFlag) { unsafe {disposdata_set_flag(self, flag, None);}} 
    pub fn set_gid(&self, gid: &Il2CppString) { unsafe { disposdata_set_gid(self, gid, None); }}
    pub fn set_hp_stock_count(&self, value: u8) { unsafe { disposdata_set_hp_stock_count(self, value, None);}}
    pub fn set_pid(&self, pid: &Il2CppString) { unsafe { disposdata_set_pid(self, pid, None); }}
    pub fn set_sid(&self, sid: &Il2CppString) { unsafe { disposdata_set_sid(self, sid, None);}}

}

#[unity::class("App", "ChapterData")]
pub struct ChapterData {
    pub parent: StructBaseFields, 
    pub cid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    //
}

impl Gamedata for ChapterData {}

impl ChapterData {
    pub fn try_set_spot_state(&self, state: i32) { unsafe { chapter_data_try_set_spot_state(self, state, None); } }
    pub fn get_flag(&self) -> i32 { unsafe { get_chapter_flag(self, None)}}
    pub fn get_cleared_flag_name(&self) -> &'static Il2CppString { unsafe { get_cleared_flagname(self, None) }}
    pub fn get_gmap_spot_flag_name(&self) -> &'static Il2CppString { unsafe { get_gmap_spot_flagname(self, None) }}
    pub fn get_gmap_open_condition(&self) -> &'static Il2CppString { unsafe { chapter_get_gmapspotopencondition(self, None)} }
    pub fn get_recommended_level(&self) -> u8 { unsafe { chapter_get_recommended_level(self, None)}}
    pub fn get_prefixless_cid(&self) -> &'static Il2CppString { unsafe { chapter_get_prefixless_cid(self, None)}}
    pub fn is_evil(&self) -> bool { unsafe {chapter_is_dlc_evil(self, None)} }
    pub fn is_god(&self) -> bool { unsafe {chapter_is_dlc_god(self, None)}}
    
    pub fn set_gmap_open_condition(&self, value: &str) { unsafe { chapter_set_gmap_open_condition(self, value.into(), None); } }
    pub fn set_flag(&self, value: i32) { unsafe {chapter_set_flag(self, value, None); }}
    pub fn set_next_chapter(&self, value: &str) { unsafe {chapter_set_next_chapter(self, value.into(), None); }}
    pub fn set_hold_level(&self, value: u8) { unsafe { chapter_set_hold_level(self, value, None); }}
    pub fn set_recommended_level(&self, level: u8 ) { unsafe { chapter_set_recommended_level(self, level, None );}}
}

// Dispos Data
#[unity::from_offset("App", "DisposData", "set_Gid")]
fn disposdata_set_gid(this: &DisposData, value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "DisposData", "get_Flag")]
fn disposdata_get_flag(this: &DisposData, method_info: OptionalMethod) -> &'static mut DisposDataFlag;

#[unity::from_offset("App", "DisposData", "get_Gid")]
fn disposdata_get_gid(this: &DisposData, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::from_offset("App", "DisposData", "get_Force")]
fn disposdata_get_force(this: &DisposData, method_info: OptionalMethod) -> i8;

#[skyline::from_offset(0x01cfa9b0)]
fn disposdata_set_ai_attack_name(this: &DisposData, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x01cfa840)]
fn disposdata_get_hp_stock_count(this: &DisposData, method_info: OptionalMethod) -> u8;

#[skyline::from_offset(0x01cfa850)]
fn disposdata_set_hp_stock_count(this: &DisposData, value: u8, method_info: OptionalMethod);

#[skyline::from_offset(0x01cfa9d0)]
fn disposdata_set_ai_attack_value(this: &DisposData, value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "DisposData", "get_Pid")]
fn disposdata_get_pid(this: &DisposData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "DisposData", "get_Sid")]
fn disposdata_get_sid(this: &DisposData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "DisposData", "set_Pid")]
fn disposdata_set_pid(this: &DisposData, value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "DisposData", "set_Flag")]
fn disposdata_set_flag(this: &DisposData, value: &mut DisposDataFlag, method_info: OptionalMethod);

#[unity::from_offset("App", "DisposData", "set_Sid")]
fn disposdata_set_sid(this: &DisposData, value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App","DisposData", "GetPerson")]
fn disposdata_get_person(this: &DisposData, method_info: OptionalMethod) -> Option<&PersonData>;

// Chapter Data
#[unity::from_offset("App", "ChapterData", "TrySetSpotState")]
fn chapter_data_try_set_spot_state(this: &ChapterData, state: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "ChapterData", "set_RecommendedLevel")]
fn chapter_set_recommended_level(this: &ChapterData, value: u8, method_info: OptionalMethod);

#[unity::from_offset("App", "ChapterData", "set_HoldLevel")]
fn chapter_set_hold_level(this: &ChapterData, value: u8, method_info: OptionalMethod);

#[unity::from_offset("App", "ChapterData", "get_RecommendedLevel")]
fn chapter_get_recommended_level(this: &ChapterData, method_info: OptionalMethod) -> u8;

#[skyline::from_offset(0x02af9b40)]
fn get_cleared_flagname(this: &ChapterData, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::from_offset("App", "ChapterData", "GetGmapSpotFlagName")]
fn get_gmap_spot_flagname(this: &ChapterData, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::from_offset("App", "ChapterData", "IsDlcGod")]
fn chapter_is_dlc_god(this: &ChapterData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "ChapterData", "IsDlcEvil")]
fn chapter_is_dlc_evil(this: &ChapterData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "ChapterData", "get_GmapSpotOpenCondition")]
fn chapter_get_gmapspotopencondition(this:& ChapterData, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x02af9850)]
fn chapter_set_flag(this: &ChapterData, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "ChapterData", "set_GmapSpotOpenCondition")]
fn chapter_set_gmap_open_condition(this: &ChapterData, value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "ChapterData", "GetPrefixlessCid")]
fn chapter_get_prefixless_cid(this: &ChapterData, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::from_offset("App", "ChapterData", "set_NextChapter")]
fn chapter_set_next_chapter(this: &ChapterData, value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "ChapterData", "get_Flag")]
fn get_chapter_flag(this: &ChapterData, method_info: OptionalMethod) -> i32;
