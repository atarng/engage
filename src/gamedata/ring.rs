use unity::prelude::*;

use crate::gamedata::{Gamedata, StructBaseFields, skill::SkillArray};
use crate::gamedata::unit::{Unit, UnitRing};

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
    pub fn get_pool_ring_stock(&self) -> i32 { unsafe { unit_ring_pool_stock_count(self, None)}}
}

pub struct UnitRingPool;
impl UnitRingPool {
    pub fn get_ring_stock(ring: &RingData) -> i32 { unsafe { unit_ring_pool_stock_count(ring, None)}}
    pub fn add_ring(rnid: &Il2CppString, owner: Option<&Unit>, count: i32) {
        unsafe { add_ring_to_pool(rnid, owner, count, None); }
    }
    pub fn sub_ring(rnid: &Il2CppString, owner: Option<&Unit>, count: i32 ) {
        unsafe { sub_ring_to_pool(rnid, owner, count, None); }
    }
}

//Ring
#[skyline::from_offset(0x024246f0)]
fn ringdata_get_skill_array(this: &RingData, method_info: OptionalMethod) -> &'static SkillArray;

#[skyline::from_offset(0x2424700)]
fn ringdata_set_skill_array(this: &RingData, value: &SkillArray, method_info: OptionalMethod);

//UnitRingPool
#[skyline::from_offset(0x01c5cf40)]
fn unit_ring_pool_stock_count(data: &RingData, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01c5d420)]
fn add_ring_to_pool(rnid: &Il2CppString, owner: Option<&Unit>, count: i32, method_info: OptionalMethod) -> &'static UnitRing;

#[skyline::from_offset(0x01c5d5b0)]
fn sub_ring_to_pool(rnid: &Il2CppString, owner: Option<&Unit>, count: i32, method_info: OptionalMethod);
