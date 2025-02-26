use unity::{prelude::*, system::{List, Stack}};

use crate::gamedata::{skill::{SkillArray, SkillData}, unit::Unit};

#[unity::class("App", "MapSequenceTargetSelect")]
pub struct MapSequenceTargetSelect {
    sup: [u8;0x68],
    pub target_data: Option<&'static MapTargetData>,
    item_index: i32,
    battle_info: &'static (),
    battle_calc: &'static (),
    engage_link_info: &'static (),
    mask_skill: &'static SkillArray,
}

impl MapSequenceTargetSelect {
    pub fn can_select_target(&self) -> bool {
        unsafe { mapsequencetargetselect_canselecttarget(self, None) }
    }
    pub fn set_mapmind(&self) -> bool {
        unsafe { mapsequencetargetselect_setmapmind(self, None) }
    }
}

#[unity::class("App", "MapTarget")]
pub struct MapTarget {
  junk: [u8;0x10],
  pub unit: Option<&'static Unit>,
  pub x: i8,
  pub z: i8,
  pub m_mind: u32,
  pub m_action_mask: u32,
  pub m_action_temp: u32,
  pub m_dataset: Option<&'static mut MapTargetDataSet>,
  pub m_buffer_a: Option<&'static MapTargetDataSet>,
  pub m_buffer_b: Option<&'static MapTargetDataSet>,
  pub m_select_unit: Option<&'static Unit>,
  pub m_select_x: i8,
  pub m_select_z: i8,
  pub m_select_item_index: u32,
  pub m_command_skill: Option<&'static SkillData>,
  pub m_enumerate_attack_unit_items: &'static(),
  pub m_enumerate_attack_specified_item: &'static(),
  pub m_enumerate_rod_unit_items: &'static(),
  pub m_enumerate_rod_specified_item: &'static(),
}

#[unity::class("", "MapTarget.Data")]
pub struct MapTargetData {
  pub m_index: i8,
  pub m_unit: &'static Unit,
  pub m_x: i8,
  pub m_z: i8,
  pub m_x1: i8,
  pub m_z1: i8,
  pub m_x2: i8,
  pub m_z2: i8,
  pub m_item_mask: i32,
  pub m_select_item_index: i8,
}

impl MapTargetData {
  pub fn set(&self, unit: &Unit, x: i32, z: i32, item_mask: i32, select_item_mask: i32) -> &'static Self {
    unsafe { maptargetdata_set(self, unit, x, z, item_mask, select_item_mask, None) }
  }
}

#[unity::class("", "MapTarget.DataSet")]
pub struct MapTargetDataSet {
  pub m_list: &'static mut List<MapTargetData>,
  pub m_stack: &'static mut Stack<MapTargetData>,
  pub _item_mask: i32,
}

impl MapTargetDataSet {
  pub fn clear(&self) {
    unsafe { maptargetdataset_clear(self, None) }
  }
}

#[skyline::from_offset(0x1f372e0)]
extern "C" fn mapsequencetargetselect_canselecttarget(this: &MapSequenceTargetSelect, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x1e41ba0)]
extern "C" fn maptargetdata_set(this: &MapTargetData, unit: &Unit, x: i32, z: i32, item_mask: i32, select_item_mask: i32, _method_info: OptionalMethod) -> &'static MapTargetData;

#[skyline::from_offset(0x1e42a60)]
extern "C" fn maptargetdataset_clear(this: &MapTargetDataSet, _method_info: OptionalMethod);

#[skyline::from_offset(0x1f3ba70)]
extern "C" fn mapsequencetargetselect_setmapmind(this: &MapSequenceTargetSelect, method_info: OptionalMethod);
