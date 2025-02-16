use unity::prelude::*;

use super::{Gamedata, StructBaseFields, unit::Unit, GodData};

#[unity::class("App", "AccessoryData")]
pub struct AccessoryData {
    pub parent: StructBaseFields,
    pub aid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub help: &'static Il2CppString,
    pub name_m: &'static Il2CppString,
    pub help_m: &'static Il2CppString,
    pub name_f: &'static Il2CppString,
    pub help_f: &'static Il2CppString,
    pub first: bool,
    pub amiibo: bool,
    pub condition_cid: &'static Il2CppString,
    pub condition_gender: i32,
    pub condition_skills: &'static [Il2CppString; 0],
    pub gid: &'static Il2CppString,
    pub asset: &'static Il2CppString,
    pub price: i32,
    pub iron: i32,
    pub steel: i32,
    pub silver: i32,
    pub mask: i32,
    pub kind: i32,
    pub god_data: Option<&'static GodData>,
    pub flag_name: &'static Il2CppString,
    // ...
}

impl Gamedata for AccessoryData { }

impl AccessoryData {
    pub fn can_equip(&self, unit: &Unit) -> bool {
        unsafe { accessory_can_equip(self, unit, None )}
    }
    pub fn get_num(&self) -> i32 {
        unsafe { accessory_get_num(self, None) }
    }
}


#[unity::from_offset("App", "AccessoryData", "CanEquip")]
fn accessory_can_equip(this: &AccessoryData, unit: &Unit, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "AccessoryData", "GetNum")]
fn accessory_get_num(data: &AccessoryData, method_info: OptionalMethod) -> i32;