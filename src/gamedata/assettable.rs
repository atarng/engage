pub use unity::prelude::*;
pub use unity::engine::Color;
use super::{*, unit::*};

#[unity::class("App", "AssetTable")]
pub struct AssetTable {
    pub parent: StructBaseFields,
    pub preset_name: Option<&'static Il2CppString>,
    pub mode: i32,
    __: i32,
    pub conditions: Option<&'static mut Array<&'static mut Il2CppString>>,
    pub body_model: Option<&'static Il2CppString>,
    pub dress_model: Option<&'static Il2CppString>,
    pub head_model: Option<&'static Il2CppString>,
    pub hair_model: Option<&'static Il2CppString>,
    pub ride_model: Option<&'static Il2CppString>,
    pub ride_dress_model: Option<&'static Il2CppString>,
    pub left_hand: Option<&'static Il2CppString>,
    pub right_hand: Option<&'static Il2CppString>,
    pub trail: Option<&'static Il2CppString>,
    pub magic: Option<&'static Il2CppString>,
    pub body_anim: Option<&'static Il2CppString>, 
    pub ride_anim: Option<&'static Il2CppString>,
    pub info_anim: Option<&'static Il2CppString>,
    pub talk_anim: Option<&'static Il2CppString>,
    pub demo_anim: Option<&'static Il2CppString>,
    pub hub_anim: Option<&'static Il2CppString>,
    pub hair_r: u8,
    pub hair_g: u8,
    pub hair_b: u8,
    pub grad_r: u8,
    pub grad_g: u8,
    pub grad_b: u8,
    pub skin_r: u8,
    pub skin_g: u8,
    pub skin_b: u8,
    pub toon_r: u8,
    pub toon_g: u8,
    pub toon_b: u8,
    pub mask_color_100_r: u8,
    pub mask_color_100_g: u8,
    pub mask_color_100_b: u8,
    pub mask_color_075_r: u8,
    pub mask_color_075_g: u8,
    pub mask_color_075_b: u8,
    pub mask_color_050_r: u8,
    pub mask_color_050_g: u8,
    pub mask_color_050_b: u8,
    pub mask_color_025_r: u8,
    pub mask_color_025_g: u8,
    pub mask_color_025_b: u8,
    pub unity_colors: [Color; 8],
    pub accessories: [&'static mut AssetTableAccessory; 8],
    pub accessory_list: &'static List<AssetTableAccessory>,
    pub scale_stuff: [f32; 19], 
    ___: i32,
    pub voice: Option<&'static Il2CppString>,
    pub foot_steps: Option<&'static Il2CppString>,
    pub material: Option<&'static Il2CppString>,
    pub comment: Option<&'static Il2CppString>,
    //ConditionIndexes
}
impl Gamedata for AssetTable {}

#[unity::class("App", "AsssetTable.ConditionFlags")]
pub struct AssetTableConditionFlags {}

impl AssetTableConditionFlags {
    pub fn add_by_key<'a>(&self, key: impl Into<&'a Il2CppString>) {
        unsafe { condition_add_by_key(self, key.into(), None);}
    }
    pub fn add_unit(&self, unit: &Unit ) {
        unsafe { condition_add_unit(self, unit, None);}
    }

}

#[repr(C)]
pub struct AssetTableStaticFields { 
    preset_name: &'static List<Il2CppString>,
    pub search_lists: &'static mut Array<&'static mut List<AssetTable>>,
    condition_indexes: *const u8,
    pub condition_flags: &'static AssetTableConditionFlags,
}

pub struct AssetTableSound {
    pub voice: Option<&'static Il2CppString>,
    pub footstep: Option<&'static Il2CppString>,
    pub material: Option<&'static Il2CppString>,
}

#[unity::class("App", "AssetTable.Result")]
pub struct AssetTableResult {
    pub pid: &'static Il2CppString,
    pub jid: &'static Il2CppString,
    pub body_model: &'static Il2CppString,
    pub dress_model: &'static Il2CppString,
    pub head_model: &'static Il2CppString,
    pub hair_model: &'static Il2CppString,
    pub ride_model: &'static Il2CppString,
    pub ride_dress_model: &'static Il2CppString,
    pub left_hand: &'static Il2CppString,
    pub right_hand: &'static Il2CppString,
    pub trail: &'static Il2CppString,
    pub magic: &'static Il2CppString,
    pub body_anim: Option<&'static Il2CppString>,
    pub ride_anim: Option<&'static Il2CppString>,
    pub unity_colors: [Color; 8],
    pub scale_stuff: [f32; 19], 
    __ : i32,
    pub sound: AssetTableSound,
    pub info_anims: Option<&'static Il2CppString>,
    pub talk_anims: Option<&'static Il2CppString>,
    pub demo_anims: Option<&'static Il2CppString>,
    pub hub_anims: Option<&'static Il2CppString>,
    pub force_id: Option<&'static Il2CppString>,
    pub weapon_id: Option<&'static Il2CppString>,
    pub body_anims: &'static mut List<Il2CppString>,
    pub accessory_list: &'static mut List<AssetTableAccessory>,
}


#[unity::class("App", "AssetTable.Accessory")]
pub struct AssetTableAccessory {
    pub locator: Option<&'static Il2CppString>,
    pub model: Option<&'static Il2CppString>, 
}

#[unity::from_offset("App","AssetTable", "set_Conditions")]
fn asset_table_set_conditions(this: &AssetTable, value: &Array<&Il2CppString>, method_info: OptionalMethod);

#[unity::from_offset("App","AssetTable", "set_Conditions")]
fn asset_table_mut_set_conditions(this: &mut AssetTable, value: &Array<&Il2CppString>, method_info: OptionalMethod);

#[unity::from_offset("App","AssetTable", "get_Conditions")]
fn asset_table_get_conditions(this: &AssetTable, method_info: OptionalMethod) -> &'static mut Array<&'static Il2CppString>;

#[unity::from_offset("App","AssetTable", ".ctor")]
fn asset_table_ctor(this: &AssetTable, method_info: OptionalMethod);

#[unity::from_offset("App","AssetTable", ".cctor")]
fn asset_table_cctor( method_info: OptionalMethod);

#[skyline::from_offset(0x01bafdd0)]
fn condition_add_by_key(condition: &AssetTableConditionFlags, key: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x01bb0200)]
fn condition_add_unit(condition: &AssetTableConditionFlags, unit: &Unit, method_info: OptionalMethod);

