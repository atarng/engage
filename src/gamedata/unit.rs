pub use unity::prelude::*;

use super::{JobData, WeaponMask, PersonData, item::ItemData};

#[unity::class("App", "Unit")]
pub struct Unit {
    status: &'static (),
    pub prev: Option<&'static Unit>,
    pub next: Option<&'static Unit>,
    ai: &'static (),
    edit: &'static (),
    pub ident: i32,
    pub person: &'static mut PersonData,
    // ...
}

impl Unit {
    /// Learn the skill for a job regardless of the unit's level.
    pub fn learn_job_skill(&self, job: &JobData) {
        unsafe {
            unit_learnjobskill(self, job, None);
        }
    }

    /// Performs a class change on the unit without playing the sequence
    pub fn class_change(&self, job: &JobData) {
        unsafe {
            unit_classchange(self, job, 0 as _, None);
        }
    }

    /// Add item to the unit's inventory without a notification
    pub fn add_item(&self, item: &ItemData) {
        unsafe {
            unit_itemadd(self, item, None);
        }
    }

    pub fn set_level(&self, level: i32) {
        unsafe {
            unit_set_level(self, level, None);
        }
    }

    pub fn set_selected_weapon(&self, weapon_mask: &WeaponMask) {
        unsafe {
            unit_setselectedweapon(self, weapon_mask, None);
        }
    }

    pub fn get_job(&self) -> &'static JobData {
        unsafe { unit_get_job(self, None) }
    }

    pub fn is_engaging(&self) -> bool {
        unsafe { unit_is_engaging(self, None) }
    }

    pub fn is_engage_owner(&self) -> bool {
        unsafe { unit_is_engage_owner(self, None) }
    }
}

#[skyline::from_offset(0x1a3f400)]
extern "C" fn unit_itemadd(this: &Unit, item: &ItemData, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "ClassChange")]
extern "C" fn unit_classchange(this: &Unit, job: &JobData, item: *const u8, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "LearnJobSkill")]
extern "C" fn unit_learnjobskill(this: &Unit, job: &JobData, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "set_Level")]
extern "C" fn unit_set_level(this: &Unit, level: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "get_Job")]
extern "C" fn unit_get_job(this: &Unit, method_info: OptionalMethod) -> &'static JobData;

#[unity::from_offset("App", "Unit", "SetSelectedWeapon")]
extern "C" fn unit_setselectedweapon(this: &Unit, weapon_mask: &WeaponMask, method_info: OptionalMethod);

// App.Unit$$IsEngaging	7101a265e0	bool App.Unit$$IsEngaging(App_Unit_o * __this, MethodInfo * method)	96
#[skyline::from_offset(0x1a265e0)]
extern "C" fn unit_is_engaging(this: &Unit, method_info: OptionalMethod) -> bool;

// App.Unit$$IsEngageOwner	7101a197a0	bool App.Unit$$IsEngageOwner(App_Unit_o * __this, MethodInfo * method)	112
#[skyline::from_offset(0x1a197a0)]
extern "C" fn unit_is_engage_owner(this: &Unit, method_info: OptionalMethod) -> bool;