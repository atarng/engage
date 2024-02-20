pub use unity::prelude::*;
use super::{Gamedata, GodData, unit::GodUnit, skill::SkillArray};

#[unity::class("App", "UnitItemList")]
pub struct UnitItemList {}

#[unity::class("App", "ItemData")]
pub struct ItemData {
	structbase: [u8; 0x10],
	pub iid: &'static Il2CppString,
	pub name: &'static Il2CppString,
	pub help: &'static Il2CppString,
	pub tutorial: &'static Il2CppString,
	pub aid: &'static Il2CppString,
	pub kind: u32,
	pub attr: u32,
	pub usetype: u32,
	pub weaponattr: u32,
	pub icon: &'static Il2CppString,
	pub endurance: u8,
	pub power: u8,
}
impl Gamedata for ItemData { }

#[unity::class("App", "UnitItem")]
pub struct UnitItem {
    pub m_index: i32,
    pub m_item: &'static ItemData,
    pub m_endurance: u8,
    pub m_RefineLevel: u8,
    pub m_Flags :i32,
    pub m_Engrave: Option<&'static GodData>,
    pub m_GodUnit: Option<&'static GodUnit>,
}

impl UnitItem {
	pub fn ctor(&self, item: &ItemData) { unsafe { unititem_ctor(self, item, None); }}
	pub fn get_equipped_skills(&self) ->  Option<&SkillArray> { unsafe { unititem_get_equip_skills(self, None)}}
	pub fn get_power(&self) -> i32 { unsafe { unititem_get_power(self, None)}}

	pub fn is_weapon(&self) -> bool { { unsafe { unititem_is_weapon(self, None) }}}
	pub fn is_drop(&self) -> bool { { unsafe {  unititem_get_is_drop(self, None)}}}
	pub fn refine_data_exist(&self) -> bool { {unsafe { unititem_is_exsistrefinedata(self, None)}}}

	pub fn set_engrave(&self, engrave: &GodData) -> bool { unsafe { unititem_set_engrave(self, engrave, None)}}
	pub fn set_refine_level(&self, level: i32) { unsafe { unititem_set_refine_level(self, level, None); }}
}

impl UnitItemList {
	pub fn get_count(&self) -> i32 { unsafe { unititemlist_get_count(self, None)}}
	pub fn add(&self, item: &ItemData) { unsafe { unititemlist_add(self, item, None); }}
	pub fn get_item(&self, index: i32) -> Option<&'static mut UnitItem> { unsafe { unititemlist_get_item(self, index, None)}}
}


#[unity::from_offset("App", "UnitItemList", "get_Count")]
pub fn unititemlist_get_count(this: &UnitItemList, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "UnitItemList", "get_Item")]
pub fn unititemlist_get_item(this: &UnitItemList, index: i32, method_info: OptionalMethod) ->  Option<&'static mut UnitItem>;

#[skyline::from_offset(0x01fb3ab0)]
pub fn unititemlist_add(this: &UnitItemList, item: &ItemData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "UnitItem", "set_RefineLevel")]
pub fn unititem_set_refine_level(this: &UnitItem, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitItem", "IsExistRefineData")]
pub fn unititem_is_exsistrefinedata(this: &UnitItem, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "UnitItem", "SetEngrave")]
pub fn unititem_set_engrave(this: &UnitItem, data: &GodData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01fad9e0)]
pub fn unititem_ctor(this: &UnitItem, item: &ItemData, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitItem", "IsWeapon")]
pub fn unititem_is_weapon(this: &UnitItem, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "UnitItem", "get_IsDrop")]
pub fn unititem_get_is_drop(this: &UnitItem, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "UnitItem", "GetPower")]
pub fn unititem_get_power(this: &UnitItem, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "UnitItem", "GetEquipSkills")]
pub fn unititem_get_equip_skills(this: &UnitItem, method_info: OptionalMethod) -> Option<&SkillArray>;