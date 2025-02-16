use unity::prelude::*;
use unity::system::List;
use crate::proc::{desc::ProcDesc, ProcInst, ProcInstFields, Bindable};
use crate::random::Random;
use crate::gamedata::item::ItemData;

#[unity::class("App", "WellSequence")]
pub struct WellSequence {
    proc: ProcInstFields,
}
impl Bindable for WellSequence {}

impl WellSequence {
    pub fn new() -> &'static mut Self {
        let item = Self::instantiate().unwrap();
        item
    }
    pub fn create_desc(&self) -> &'static mut Il2CppArray<&'static mut ProcDesc> { unsafe { well_create_desc(self, None) } }
    pub fn set_use_flag(flag: i32) { unsafe { set_well_use_flag(flag, None);}}
    pub fn set_seed(seed: i32) { unsafe { set_well_seed(seed, None);}}
    pub fn set_evil_weapon_state(value: i32) { unsafe { wellsequence_set_evilstate(value, None)}}
    pub fn get_use_flag() -> i32 { unsafe { get_well_use_flag(None) }}
    pub fn get_exchange_level() -> i32 { unsafe { well_exchange_level(None)}}
    pub fn get_seed() -> i32 { unsafe { get_well_seed(None) } }
    pub fn calc_item_exchange(level: i32, rng: &Random) -> &'static mut List<ItemData> { unsafe { well_calc_item_exchange(None, level, rng, None) } }
}


#[unity::from_offset("App", "WellSequence", "CreateDesc")]
fn well_create_desc<P: Bindable>( this: &P,  method_info: OptionalMethod) -> &'static mut Il2CppArray<&'static mut ProcDesc>;

#[unity::from_offset("App", "WellSequence", "set_UseFlag")]
fn set_well_use_flag(flag: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "WellSequence", "set_EvilWeaponEventState")]
fn wellsequence_set_evilstate(state: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "WellSequence", "set_Seed")]
fn set_well_seed(seed: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "WellSequence", "get_Seed")]
fn get_well_seed(method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "WellSequence", "get_UseFlag")]
fn get_well_use_flag(method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "WellSequence", "CalcItemExchange")] 
fn well_calc_item_exchange(this: Option<&WellSequence>, level: i32, random: &Random, method_info: OptionalMethod) -> &'static mut List<ItemData>;

#[unity::from_offset("App", "WellSequence", "get_ExchangeLevel")]
fn well_exchange_level(method_info: OptionalMethod) -> i32; 