use unity::prelude::*;

use super::Gamedata;

#[unity::class("App", "TerrainData")]
pub struct TerrainData {}

impl Gamedata for TerrainData { }

impl TerrainData {
    pub fn is_not_target(&self) -> bool {
        unsafe { terraindata_is_not_target(self, None) }
    }
}

#[unity::from_offset("App", "TerrainData", "IsNotTarget")]
extern "C" fn terraindata_is_not_target(this: &TerrainData, method_info: OptionalMethod) -> bool;
