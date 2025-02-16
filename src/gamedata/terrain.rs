use unity::{engine::Color, prelude::*};
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
    pub hp_normal: i32,
    pub hp_hard: i32,
    pub hp_lunatic: i32,
    pub defense: i8,
    pub avoid: i8,
    pub player_defense: i8,
    pub enemy_defense: i8,
    pub player_avoid: i8,
    pub enemy_avoid: i8,
    pub heal: i8,
    pub life: u8,
    pub move_cost: u8,
    pub fly_cost: u8,
    pub move_first: i8,
    pub offset: f32,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    pub color: Color,
    pub change_tid: Option<&'static Il2CppString>,
    pub change_encount: Option<&'static Il2CppString>,
    pub height: f32,
    pub put_effect: Option<&'static Il2CppString>,
    pub minimap: Option<&'static Il2CppString>,
    pub cannon_skill: Option<&'static Il2CppString>,
    pub cannon_shell_normal: u8,
    pub cannon_shell_hard: u8,
    pub cannon_shell_lunatic: u8,
    pub flag: i32,
    pub put_allow: u8,
    pub ascii_name: Option<&'static Il2CppString>,
}
impl Gamedata for TerrainData {}
impl TerrainData {
    pub fn is_not_target(&self) -> bool {
        unsafe { terraindata_is_not_target(self, None) }
    }
    pub fn is_visit(&self) -> bool {
        unsafe { terraindata_is_visit(self, None) }
    }
}

#[unity::from_offset("App", "TerrainData", "IsNotTarget")]
extern "C" fn terraindata_is_not_target(this: &TerrainData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "TerrainData", "IsVisit")]
fn terraindata_is_visit(this: &TerrainData, method_info: OptionalMethod) -> bool;