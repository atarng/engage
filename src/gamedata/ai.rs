use unity::prelude::*;
use unity::il2cpp::object::Array;
use super::{GamedataArray, unit::Unit, StructDataArrayFields, WeaponMask};

#[unity::class("App", "AIData")]
pub struct AIData {
    pub parent: StructDataArrayFields,
    pub code: i8,
    pub mind: i8,
    pub active: i8,
    pub trans: i8,
    __: i32,
    pub str_value1: &'static Il2CppString,
    pub str_value2: &'static Il2CppString,
}
impl GamedataArray for AIData {}

#[unity::class("App", "UnitAI")]
pub struct UnitAI {
    pub flag: &'static mut WeaponMask,
    pub band: u8,
    pub active: u8,
    pub priority: u8,
    pub heal_rate_a: u8,
    pub heal_rate_b: u8,
    pub battle_rate_type: u8,
    pub prohibit_engage_attack: u8,
    pub prohibit_rod: u8,
    pub prohibit_overlap: u8,
    pub rerewarp_count: u8,
    pub rerewarp_count_max: u8,
    pub rerewarp_last_x: u8,
    pub rerewarp_last_z: u8,
    pub rerwarp_event: Option<&'static Il2CppString>,
    pub random_flag: &'static mut WeaponMask,
    pub move_limit: &'static MoveLimitRange,
    pub vs_type: u8,
    pub bullet_pattern: u8,
    pub sequence: &'static Array<&'static Il2CppString>,
    pub value: &'static Array<&'static AIValue>,
    pub unit: &'static Unit,
    pub vs_think: i32,
}

#[unity::class("App", "UnitAI.MoveLimitRange")]
pub struct MoveLimitRange {
    pub m_type: u8,
    pub x: i8,
    pub z: i8,
    pub w: i8,
    pub h: i8,
}

#[unity::class("App", "AIValue")]
pub struct AIValue {
    pub v8_1: i8,
    pub v8_2: i8,
    pub v16: i16,
}


impl UnitAI {
    pub fn setup_versus_ai(&self) { unsafe { set_up_versus_ai(self, None) }  }
    pub fn set_versus_ai_type(&self, value: i32) { unsafe { set_versus_type(self, value, None) } }
    pub fn set_active(&self, value: u8) { unsafe { unitai_set_active(self, value, None); }}
}

#[skyline::from_offset(0x01f60810)]
pub fn set_up_versus_ai(this: &UnitAI, method_info: OptionalMethod);

#[skyline::from_offset(0x01f5f4c0)]
pub fn set_versus_type(this: &UnitAI, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitAI", "set_Active")]
fn unitai_set_active(this: &UnitAI, value: u8, method_info: OptionalMethod);