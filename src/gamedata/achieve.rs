use unity::prelude::*;
use super::{Gamedata, StructBaseFields, StructDataFields};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AchieveDataStatus {
    None = 0,
    Cleared = 1,
    Showed = 2,
    Completed = 3,
}

#[unity::class("App", "AchieveData")]
pub struct AchieveData {
    pub parent: StructBaseFields,
    pub aid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub category: i32,
    pub kind: i32,
    pub count: i32,
    arg: &'static Il2CppString,
    count_unit: &'static Il2CppString,
    pub kizuna_rewards: i32,
    pub chapter: &'static Il2CppString,
    pub flagname: &'static Il2CppString,
}

impl AchieveData {
    /// Increments counter for unit arena battle wins
    pub fn add_count_unit_battle_win() { unsafe { achieve_add_unit_battle_win(None); }  }
    /// Increments counter for arena emblem battle wins
    pub fn add_count_god_battle_win() { unsafe { achieve_add_god_battle_win(None); }  }
    pub fn add_count_unit_battle_count() { unsafe { achieve_add_unit_battle_count(None); }  }
    /// Increments counter for arena emblem battles 
    pub fn add_count_god_battle_count() { unsafe { achieve_add_god_battle_count(None); }}

    pub fn add_count_reliance_a() { unsafe { add_a_reliance_count(None) }}
    pub fn add_count_reliance_b() { unsafe { add_b_reliance_count(None) }}
    pub fn add_count_reliance_s() { { unsafe { add_s_reliance_count(None) }}}

    pub fn set_status(&self, status: AchieveDataStatus) {
        unsafe { achieve_set_status(self, status, None) }
    }
    pub fn get_status(&self) -> AchieveDataStatus {  unsafe { achieve_status(self, None) } }
    pub fn get_reward(&self) -> i32 { unsafe { achieve_reward(self, None)}}
}
impl Gamedata for AchieveData {}

#[skyline::from_offset(0x027cb090)]
fn achieve_add_unit_battle_count(method_info: OptionalMethod);

#[skyline::from_offset(0x027cb0f0)]
fn achieve_add_unit_battle_win(method_info: OptionalMethod);

#[skyline::from_offset(0x027cb150)]
fn achieve_add_god_battle_count(method_info: OptionalMethod);

#[skyline::from_offset(0x027cb1b0)]
fn achieve_add_god_battle_win(method_info: OptionalMethod);

#[skyline::from_offset(0x027c7380)]
fn achieve_status(this: &AchieveData, method_info: OptionalMethod) -> AchieveDataStatus;

#[skyline::from_offset(0x027c74b0)]
fn achieve_set_status(this: &AchieveData, status:  AchieveDataStatus, method_info: OptionalMethod);

#[skyline::from_offset(0x027c6b30)]
fn achieve_reward(this: &AchieveData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "AchieveData","AddCountRelianceA")]
pub fn add_a_reliance_count(method_info: OptionalMethod);

#[unity::from_offset("App", "AchieveData","AddCountRelianceB")]
pub fn add_b_reliance_count(method_info: OptionalMethod);

#[unity::from_offset("App", "AchieveData","AddCountRelianceS")]
pub fn add_s_reliance_count(method_info: OptionalMethod);