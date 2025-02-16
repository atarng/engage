pub use unity::prelude::*;
use unity::il2cpp::object::Array;
use super::{*, GodData, unit::GodUnit, skill::SkillArray};
use crate::gamedata::StructBaseFields;

#[unity::class("App", "ItemData")]
pub struct ItemData {
	pub parent: StructBaseFields,
	pub iid: &'static Il2CppString,
	pub name: &'static Il2CppString,
	pub help: &'static Il2CppString,
	pub tutorial: &'static Il2CppString,
	pub aid: &'static Il2CppString,
	pub kind: u32,
	pub attr: u32,
	pub usetype: u32,
	pub weapon_attr: u32,
	pub icon: Option<&'static Il2CppString>,
	pub endurance: u8,
	pub power: u8,
	pub weight: u8,
	pub range_i: u8,
	pub range_o: u8,
	pub disnace: u8, 
	pub hit: i16,
	pub critical: i16,
	pub avoid: i16,
	pub secure: i16,
	__: i16,	//
	pub price: i32, 
	pub weapon_level: &'static Il2CppString,
	pub rod_type: i32, 
	pub rod_exp: u8,
	pub rate_arena: u8,
	pub shoot_effect: Option<&'static Il2CppString>,
	pub hit_effect: Option<&'static Il2CppString>, 
	pub cannon_effect: Option<&'static Il2CppString>,
	pub overlap_terrain: Option<&'static Il2CppString>,
	pub flag: &'static mut ItemDataFlag,
	pub enchance: &'static CapabilitySbyte,
	pub grow_ratio: &'static CapabilitySbyte,
	pub equip_condition: Option<&'static Il2CppString>,
}
impl Gamedata for ItemData { }

#[unity::class("App", "ItemDataFlag")]
pub struct ItemDataFlag {
    pub value: i32,
}

#[unity::class("App", "UnitItem")]
pub struct UnitItem {
    pub index: i32,
    pub item: &'static ItemData,
    pub endurance: u8,
    pub refine_level: u8,
    pub flags :i32,
    pub engrave: Option<&'static GodData>,
    pub god_unit: Option<&'static GodUnit>,
}

#[unity::class("App", "UnitItemList")]
pub struct UnitItemList {
	pub unit_items: &'static mut Array<Option<&'static mut UnitItem>>
}

#[unity::class("App", "RewardData")]
pub struct RewardData {
	pub parent: StructDataArrayFields,
	pub iid: &'static Il2CppString,
	pub ratio: f32,
	pub factor: f32,
	pub min: f32,
	pub max: f32, 
	pub is_show: bool,
}

impl GamedataArray for RewardData {}

#[unity::class("App", "ItemEvolveData")]
pub struct ItemEvolveData {
    pub parent: StructDataArrayFields,
    pub iid: &'static Il2CppString,
    pub iron: u16,
    pub steel: u16,
    pub silver: u16,
    pub price: u16,
    pub refine_level: u8,
}
impl GamedataArray for ItemEvolveData  {}

impl RewardData {
	pub fn ctor(&self) { unsafe { rewarddata_ctor(self, None); }}
	pub fn set_iid(&self, iid: &Il2CppString) { unsafe { rewarddata_set_iid(self, iid, None); }}
	pub fn calc_rewards(name: &Il2CppString) -> Option<&'static mut List<ItemData>> { unsafe { rewarddata_calc_reward(name, None)}}
}

impl ItemData {
	pub fn get_kind(&self) -> i32 { unsafe { itemdata_get_kind(self, None)}}
	pub fn get_weapon_level(&self) -> i32 { unsafe { itemdata_get_weapon_level(self, None)}}
	pub fn get_equip_skills(&self) -> &'static SkillArray { unsafe { item_get_equip_skills(self, None)}}
	pub fn get_give_skills(&self) -> &'static SkillArray { unsafe { item_get_give_skills(self, None)}}
	pub fn on_complete(&self) { unsafe { item_on_complete(self, None); }}
	pub fn set_cannon_effect(&self, value: &Il2CppString) { unsafe { item_set_cannon_effect(self, value, None); }}
	pub fn set_hit_effect(&self, value: &Il2CppString) { unsafe { item_set_hit_effect(self, value, None); }}
	pub fn get_flag(&self) -> &'static ItemDataFlag { unsafe { item_data_flag(self, None)}}
	pub fn add_inventory(&self, count: i32) { unsafe { item_data_add_inventory(self, count, None);}}
	pub fn is_inventory(&self) -> bool  {unsafe { item_data_is_inventory(self, None) } }
	pub fn is_material(&self) -> bool { unsafe { item_data_is_material(self, None)}}
	pub fn is_unknown(&self) -> bool { unsafe { item_data_is_unknown(self, None)}}	
	pub fn is_weapon(&self) -> bool { unsafe { item_data_is_weapon(self, None)}}
	pub fn get_inventory(&self) -> i32 { unsafe { item_data_get_inventory(self, None)}}
}
impl UnitItem {
	pub fn ctor(&self, item: &ItemData) { unsafe { unititem_ctor(self, item, None); }}
	pub fn ctor_str(&self, key: &str) {
		let item = ItemData::get(key);
		if item.is_some() {
			self.ctor(item.unwrap());
		}
	}
	pub fn dispose(&self) { unsafe { unititem_dispose(self, None); }}
	pub fn get_equipped_skills(&self) ->  Option<&SkillArray> { unsafe { unititem_get_equip_skills(self, None)}}
	pub fn get_power(&self) -> i32 { unsafe { unititem_get_power(self, None)}}

	pub fn is_equip(&self) -> bool { unsafe { unititem_is_equip(self, None)}}
	pub fn is_empty(&self) -> bool { unsafe { unititem_is_empty(self, None) } }
	pub fn is_weapon(&self) -> bool {  unsafe { unititem_is_weapon(self, None) } }
	pub fn is_drop(&self) -> bool { unsafe {  unititem_get_is_drop(self, None) } }
	
	pub fn refine_data_exist(&self) -> bool { unsafe { unititem_is_exsistrefinedata(self, None)} }

	pub fn set_engrave(&self, engrave: &GodData) -> bool { unsafe { unititem_set_engrave(self, engrave, None)}}
	pub fn set_refine_level(&self, level: i32) { unsafe { unititem_set_refine_level(self, level, None); }}
	pub fn set_flags(&self, value: i32) { unsafe { unititem_set_flags(self, value, None);}}
	pub fn set_endurance(&self, value: i32) { unsafe { unititem_set_endurance(self, value, None) } }
}

impl UnitItemList {
	pub fn add(&self, item: &ItemData) { unsafe { unititemlist_add(self, item, None); }}
	pub fn get_count(&self) -> i32 { unsafe { unititemlist_get_count(self, None)}}
	pub fn get_item(&self, index: i32) -> Option<&'static mut UnitItem> { unsafe { unititemlist_get_item(self, index, None)}}
	pub fn has_item(&self, item: &ItemData) -> bool { unsafe { unititemlist_has_item(self, item, None)}}
	pub fn has_item_iid(&self, iid: &str) -> bool { 
		let item = ItemData::get(iid);
		if item.is_some() {
			unsafe { return unititemlist_has_item(self, item.unwrap(), None)}
		}
		return false;
	}
	pub fn add_item_no_duplicate(&self, item: &ItemData){
		if !self.has_item(item) { self.add(item); }
	}
	pub fn add_iid_no_duplicate(&self, iid: &str){
		let item = ItemData::get(iid);
		if !self.has_item_iid(iid) && item.is_some() { self.add(item.unwrap()); }
	}
	pub fn move_item(&self, from: i32, to: i32) { unsafe { unititemlist_move(self, from, to, None) } }
	pub fn put_off_all_item(&self) { unsafe { unititemlist_putoffall(self, None); } }
}
impl ItemEvolveData {
	pub fn register() { unsafe { regist_evolve_flags(None);} }
}

#[skyline::from_offset(0x0203dfd0)]
fn regist_evolve_flags(method_info: OptionalMethod);

#[unity::from_offset("App", "ItemData", "get_Flag")]
pub fn item_data_flag(this: &ItemData, method_info: OptionalMethod) -> &'static ItemDataFlag;

#[unity::from_offset("App", "ItemData", "IsWeapon")]
pub fn item_data_is_weapon(this: &ItemData, method_info: OptionalMethod) -> bool;
//ItemData
#[unity::from_offset("App", "ItemData", "get_Kind")]
fn itemdata_get_kind(this: &ItemData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "ItemData", "GetWeaponLevel")]
fn itemdata_get_weapon_level(this: &ItemData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "ItemData", "set_CannonEffect")]
fn item_set_cannon_effect(this: &ItemData, value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "ItemData", "set_HitEffect")]
fn item_set_hit_effect(this: &ItemData, value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "ItemData", "OnCompleted")]
fn item_on_complete(this: &ItemData, method_info: OptionalMethod);

#[unity::from_offset("App", "ItemData", "get_EquipSkills")]
fn item_get_equip_skills(this: &ItemData, method_info: OptionalMethod) -> &'static SkillArray;

#[unity::from_offset("App", "ItemData", "get_GiveSkills")]
fn item_get_give_skills(this: &ItemData, method_info: OptionalMethod) -> &'static SkillArray;

#[unity::from_offset("App", "ItemData", "get_Help")]
fn item_data_get_name(this: &ItemData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "ItemData", "IsInventory")]
fn item_data_is_inventory(this: &ItemData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "ItemData", "IsMaterial")]
fn item_data_is_material(this: &ItemData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "ItemData", "IsUnknown")]
fn item_data_is_unknown(this: &ItemData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x027b1920)]
fn item_data_get_inventory(this: &ItemData, method_info: OptionalMethod) -> i32;


#[skyline::from_offset(0x027b1c30)]
fn item_data_add_inventory(this: &ItemData, count: i32, method_info: OptionalMethod);

//UnitItemList
#[unity::from_offset("App", "UnitItemList", "get_Count")]
pub fn unititemlist_get_count(this: &UnitItemList, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "UnitItemList", "get_Item")]
pub fn unititemlist_get_item(this: &UnitItemList, index: i32, method_info: OptionalMethod) ->  Option<&'static mut UnitItem>;

#[unity::from_offset("App", "UnitItemList", "HasItem")]
pub fn unititemlist_has_item(this: &UnitItemList, item: &ItemData, method_info: OptionalMethod) ->  bool;

#[skyline::from_offset(0x01fb3ab0)]
pub fn unititemlist_add(this: &UnitItemList, item: &ItemData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "UnitItemList", "Move")]
pub fn unititemlist_move(this: &UnitItemList, from: i32, to: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitItemList", "PutOffAll")]
pub fn unititemlist_putoffall(this: &UnitItemList, method_info: OptionalMethod);

//UnitItem
#[unity::from_offset("App", "UnitItem", "Dispose")]
pub fn unititem_dispose(this: &UnitItem, method_info: OptionalMethod);

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

#[skyline::from_offset(0x01fb2df0)]
fn unititem_set_endurance(this: &UnitItem, value: i32, method_info: OptionalMethod);
#[unity::from_offset("App", "UnitItem", "GetPower")]
pub fn unititem_get_power(this: &UnitItem, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "UnitItem", "GetEquipSkills")]
pub fn unititem_get_equip_skills(this: &UnitItem, method_info: OptionalMethod) -> Option<&SkillArray>;

#[unity::from_offset("App", "UnitItem", "IsEmpty")]
pub fn unititem_is_empty(this: &UnitItem, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "UnitItem", "get_IsEquipped")]
pub fn unititem_is_equip(this: &UnitItem, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "UnitItem", "SetFlags")]
pub fn unititem_set_flags(this: &UnitItem, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x02018ab0)]
fn rewarddata_set_iid(this: &RewardData, value: &Il2CppString, method_info: OptionalMethod); 

#[skyline::from_offset(0x02019720)]
fn rewarddata_ctor(this: &RewardData, method_info: OptionalMethod); 

#[skyline::from_offset(0x020193e0)]
fn rewarddata_calc_reward(name: &Il2CppString, method_info: OptionalMethod) -> Option<&'static mut List<ItemData>>;