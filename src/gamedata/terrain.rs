use unity::prelude::*;
use super::{StructBaseFields, Gamedata};

#[unity::class("App", "TerrainData")]
pub struct TerrainData {
    pub parent: StructBaseFields,
    pub tid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub cost_name: &'static Il2CppString,
    pub cost_type: i32,
    pub layer: i32,
    pub prohibition: i32,
    pub command: i32,
    pub sight: u8,
    pub destroyer: i32,
    pub hp: [i32; 3],
    pub defense: i8,
    pub avoid: i8,
    pub player_defense: i8,
    pub enemy_defense: i8,
}
impl Gamedata for TerrainData {}
impl TerrainData {
    pub fn is_not_target(&self) -> bool {
        unsafe { terraindata_is_not_target(self, None) }
    }
}

#[unity::from_offset("App", "TerrainData", "IsNotTarget")]
extern "C" fn terraindata_is_not_target(this: &TerrainData, method_info: OptionalMethod) -> bool;