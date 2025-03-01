//! Types and methods used in the zoomed-out map view.
use unity::prelude::*;

use crate::{gamedata::unit::Unit, util::get_instance};

#[repr(C)]
#[unity::class("App", "MapMind")]
pub struct MapMind {
    pub sup: [u8;0x9],
    pub unit_index: u8,
    pub first_unit_index: u8,
    pub first_x: i8,
    pub first_z: i8,
    pub unit_show_x: i8,
    pub unit_show_z: i8,
    pub x: i8,
    pub z: i8,
    pub mind: i32,
    pub attack_x: i8,
    pub attack_z: i8,
    pub item_index: i8,
    pub target_unit_index: u8,
    pub target_x: i8,
    pub target_z: i8,
    pub focus_x: i8,
    pub focus_z: i8,
    pub target_argument: i16,
    pub trade_unit_index: u8,
    pub event_unit_index: u8,
}

impl MapMind {
    pub fn get_instance() -> &'static mut MapMind {
        get_instance::<MapMind>()
    }

    /// Seems to get the current unit that is selected by the player. Needs more experimentation.
    pub fn get_unit() -> &'static mut Unit {
        let instance = Self::get_instance();
        unsafe { get_unit(instance, None) }
    }

    pub fn set_trade_unit_index(&mut self, value: i32,) {
        self.focus_x = value as i8;
    }
}

// App.MapMind$$get_Unit	7101dee2b0	App_Unit_o * App.MapMind$$get_Unit(App_MapMind_o * __this, MethodInfo * method)	12
#[unity::from_offset("App", "MapMind", "get_Unit")]
fn get_unit(this: &MapMind, method_info: OptionalMethod) -> &mut Unit;
