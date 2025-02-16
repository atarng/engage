use unity::prelude::*;
use crate::util::get_instance;

// Structure that control map win / lose conditions

#[unity::class("App", "MapSituation")]
pub struct MapSituation {
    junk: [u8; 0x28],
    pub current_force: i32,
    pub human_force: i32,
    force_cursor: *const u8,
    pub turn: i32,
    pub sub_phase: i32,
    pub win_rule_enemy_less_than_equal: i32, 
    pub win_rule_turn: i32,
    pub win_lose_result: i32,
    pub entrust: i32,
    pub win_rule_mid: &'static Il2CppString,
    pub win_rule_mid_arg: Option<&'static Il2CppString>,
    pub lose_rule_mid: &'static Il2CppString,
    pub lose_rule_arg: Option<&'static Il2CppString>,
    pub average_level: i32,
}
impl MapSituation {
    pub fn get_instance() -> &'static mut MapSituation { get_instance::<Self>() }
    pub fn set_status(&self, status: i32) { unsafe { mapsituation_set_status(self, status, None); } }
    pub fn check_status(&self, status: i32) -> bool { unsafe { mapsituation_check_status(self, status, None) } }
}

#[unity::from_offset("App", "MapSituation", "SetStatus")]
fn mapsituation_set_status(this: &MapSituation, status: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "MapSituation", "CheckStatus")]
fn mapsituation_check_status(this: &MapSituation, status: i32, method_info: OptionalMethod) -> bool;