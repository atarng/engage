use unity::prelude::*;
use crate::{
    proc::{ProcInstFields, Bindable},
    gamedata::{item::ItemData, JobData, WeaponMask, unit::Unit},
};

#[unity::class("App", "UnitGrowSequence")]
pub struct UnitGrowSequence {
    pub proc: ProcInstFields,
    pub camera_mode: i32,
    pub unit: &'static Unit,
    pub exp: i32,
    pub old_level: i32,
    pub is_talk: bool,
    pub skill_point: bool,
    pub class_change_job: Option<&'static JobData>,
    pub class_change_item: Option<&'static ItemData>,
    pub class_change_weapon_mask: Option<&'static WeaponMask>,
    pub class_change_weapon: Option<&'static ItemData>,
}
impl Bindable for UnitGrowSequence {}

impl UnitGrowSequence {
    pub fn create_bind<P: Bindable>(proc: &P) -> &'static UnitGrowSequence { unsafe { unitgrowsequence_create_bind(proc, None) } }
    pub fn set_unit_grow_data(&self, unit: &Unit, exp: i32, sp: i32, is_talk: bool) {
        unsafe { unitgrowsequence_set_unit_grow_data(self, unit, exp, sp, is_talk, None) }
    }
}



#[unity::from_offset("App", "UnitGrowSequence", "CreateBind")]
fn unitgrowsequence_create_bind<P: Bindable>(proc: &P, method_info: OptionalMethod) -> &'static UnitGrowSequence;

#[skyline::from_offset(0x01f7ea40)]
fn unitgrowsequence_set_unit_grow_data(this: &UnitGrowSequence, unit: &Unit, exp: i32, sp: i32, is_talk: bool, method_info: OptionalMethod);