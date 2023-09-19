//! Structures representing a singular entry from the gamedata files in memory.

use unity::{prelude::*, system::List};

pub mod unit;

#[repr(C)]
#[unity::class("App", "ItemData")]
pub struct ItemData;

#[repr(C)]
#[unity::class("App", "JobData")]
pub struct JobData {
    structbase: [u8; 0x10],
    pub jid: &'static Il2CppString,
    // ...
}

#[repr(C)]
#[unity::class("App", "PersonData")]
pub struct PersonData {
    structbase: [u8; 0x10],
    pub pid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    // ...
}

#[repr(C)]
#[unity::class("App", "StructData`1")]
pub struct StructDataGeneric;

#[repr(C)]
pub struct StructData;

// pub static_fields: &'static StructDataStaticFields<T>,

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StructDataStaticFields<T: 'static> {
    pub s_list: &'static Il2CppObject<StructList<Il2CppObject<T>>>,
    pub loaded: bool,
}

#[repr(C)]
pub struct StructList<T: 'static> {
    pub list: List<T>,
}

#[repr(C)]
#[unity::class("App", "WeaponMask")]
pub struct WeaponMask {
    pub value: i32,
}
