use unity::prelude::*;

use crate::gamedata::{Gamedata, StructBaseFields, skill::SkillArray};

#[unity::class("App", "RingData")]
pub struct RingData {
    pub parent: StructBaseFields,
    pub rid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub help: &'static Il2CppString,
    pub gid: Option<&'static Il2CppString>,
    ring_model: &'static Il2CppString,
    pub kind: i32,
    pub rank: i32,
    pub icon: &'static Il2CppString,
}

impl Gamedata for RingData {}

impl RingData {
    pub fn get_equip_skills(&self) -> &'static SkillArray { unsafe { ringdata_get_skill_array(self, None)} }
    pub fn set_equip_skills(&self, value: &SkillArray) { unsafe { ringdata_set_skill_array(self, value, None); } }
}

//Ring
#[skyline::from_offset(0x024246f0)]
fn ringdata_get_skill_array(this: &RingData, method_info: OptionalMethod) -> &'static SkillArray;

#[skyline::from_offset(0x2424700)]
fn ringdata_set_skill_array(this: &RingData, value: &SkillArray, method_info: OptionalMethod);
