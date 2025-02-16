use unity::prelude::*;
use unity::il2cpp::object::Array;
use crate::{gamedata::{unit::Unit, PersonData}, force::Force};


#[unity::class("App", "UnitPool")]
pub struct UnitPool { }

pub struct UnitPoolStaticFields {
    pub s_unit: &'static Array<&'static Unit>,
    pub forces: &'static Array<&'static Force>,
}

impl UnitPool {
    pub fn get_force(index: i32) -> &'static Force {
        unsafe { unitpool_getforce(index, None) }
    }
    pub fn get_by_index(index: i32) -> Option<&'static mut Unit> {
        unsafe { unit_pool_get(index, None)}
    }
    pub fn get_from_person_mut(pid: &Il2CppString, relay: bool) -> Option<&'static mut Unit> {
        unsafe { unitpool_get_unit_from_pid(pid.into(), relay, None)}
    }
    pub fn get_from_person_force_mask(person: &PersonData, mask: i32) -> Option<&'static mut Unit> {
        unsafe { unit_pool_get_force_mask(person, mask, None)}
    }
}


#[unity::from_offset("App", "UnitPool", "GetForce")]
fn unitpool_getforce(index: i32, method_info: OptionalMethod) -> &'static Force;

#[skyline::from_offset(0x01c54fa0)]
pub fn unitpool_get_unit_from_pid(pid: &Il2CppString, relay: bool, method_info: OptionalMethod) -> Option<&'static mut Unit>;

#[skyline::from_offset(0x01c53f80)]
fn unit_pool_get(index:i32, method_info: OptionalMethod) -> Option<&'static mut Unit>;

#[skyline::from_offset(0x01c55030)]
fn unit_pool_get_force_mask(person: &PersonData, force_mask: i32, method_info: OptionalMethod) -> Option<&'static mut Unit>;