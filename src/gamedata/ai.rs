use unity::prelude::*;
use super::{StructDataArrayFields, GamedataArray};

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