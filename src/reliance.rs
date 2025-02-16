use unity::prelude::*;

use crate::gamedata::unit::Unit;
#[unity::class("App", "UnitReliance")]
pub struct UnitReliance {}  // Empty Structure

impl UnitReliance {
    pub fn can_level_up( unit_a: &Unit, unit_b: &Unit) -> bool { unsafe { reliance_can_level_up(unit_a, unit_b, None)}}
    pub fn can_be_level_a_plus(unit_a: &Unit, unit_b: &Unit) -> bool { unsafe { can_be_a_plus_support(unit_a, unit_b,  None) } }
    pub fn level_up(unit_a: &Unit, unit_b: &Unit) { unsafe { level_up_support(unit_a, unit_b, None); } }
    pub fn set_level_a_plus(unit_a: &Unit, unit_b: &Unit) { unsafe {  set_level_a_plus(unit_a, unit_b, None); } }
    pub fn try_get(unit_a: &Unit, unit_b: &Unit) -> Option<&'static mut UnitRelianceData> { unsafe { unit_reliance_try_get(unit_a, unit_b, None)}}
}

#[unity::class("App", "UnitRelianceData")]
pub struct UnitRelianceData {
    reliance_exp: *const u8,
    pub level: i32,
    pub exp: i8,
    pub score: i8,
}

impl UnitRelianceData {
    pub fn get_next_level_exp(&self, current_level: i32) -> i32 { unsafe { unit_get_exp_next_level(self, current_level, None) } }
}

#[skyline::from_offset(0x01c5a930)]
pub fn reliance_can_level_up( unit_a: &Unit, unit_b: &Unit, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01c5a040)]
pub fn unit_reliance_try_get(unit_a: &Unit, unit_b: &Unit, method_info: OptionalMethod) -> Option<&'static mut UnitRelianceData>;

#[skyline::from_offset(0x01c5c450)]
pub fn unit_get_exp_next_level(this: & UnitRelianceData, current_level: i32, method_info: OptionalMethod ) -> i32;

#[skyline::from_offset(0x01c5ae10)]
fn can_be_a_plus_support(unit_a: &Unit, unit_b: &Unit, method_info: OptionalMethod ) -> bool;

#[skyline::from_offset(0x01c5abf0)]
fn level_up_support(unit_a: &Unit, unit_b: &Unit, method_info: OptionalMethod );

#[skyline::from_offset(0x01c5b020)]
fn set_level_a_plus(unit_a: &Unit, unit_b: &Unit, method_info: OptionalMethod );