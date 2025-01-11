pub use unity::prelude::*;

use crate::{gamedata::unit::Unit, proc::{Bindable, ProcInstFields}};

#[repr(C)]
#[unity::class("App", "MapSequenceHuman")]
pub struct MapSequenceHuman {
    pub proc: ProcInstFields,
    is_resume: bool,
    is_loaded: bool,
    job_intro_unit: Option<&'static Unit>,
    job_intro_keyhelp_type: i32,
    return_label: i32,
    old_unit_x: i32,
    old_unit_z: i32,
    old_cursor_x: i32,
    old_cursor_z: i32,
    old_pickup_x: i32,
    old_pickup_z: i32,
    engage_x: i32,
    engage_z: i32,
    enter_x: i32,
    enter_z: i32,
    is_enemy_attack_range: bool,
    is_update_support_skill: bool,
    update_support_skill_unit: Option<&'static Unit>,
    operate_mode: i32,
}

impl Bindable for MapSequenceHuman { }

impl AsRef<ProcInstFields> for MapSequenceHuman {
    fn as_ref(&self) -> &ProcInstFields {
        &self.proc
    }
}

impl AsMut<ProcInstFields> for MapSequenceHuman {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}
