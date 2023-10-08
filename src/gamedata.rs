//! Structures representing a singular entry from the gamedata files in memory.

use unity::{prelude::*, system::List};

pub mod unit;
pub mod item;

#[unity::class("App", "JobData")]
pub struct JobData {
    pub parent: StructBaseFields,
    pub jid: &'static Il2CppString,
    // ...
}

#[unity::class("App", "PersonData")]
pub struct PersonData {
    pub parent: StructBaseFields,
    pub pid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    // ...
}

#[unity::class("App", "StructData`1")]
pub struct StructDataGeneric { }

#[unity::class("App", "StructData`1")]
pub struct StructData { }

// pub static_fields: &'static StructDataStaticFields<T>,

#[derive(Clone, Copy)]
pub struct StructDataStaticFields<T: 'static> {
    pub s_list: &'static StructList<T>,
    pub loaded: bool,
}

#[unity::class("App", "StructList<`1>")]
pub struct StructList<T: 'static> {
    pub list: List<T>,
}

#[unity::class("App", "StructBase")]
pub struct StructBase {
    index: i32,
    hash: i32,
    key: &'static Il2CppString,
}

#[unity::class("App", "WeaponMask")]
pub struct WeaponMask {
    pub value: i32,
}
