use unity::prelude::*;
use unity::il2cpp::object::Array;

#[unity::class("App", "MapTerrain")]
 pub struct MapTerrain {
     _super: u64,
     pub x: i32,
     pub z: i32,
     pub width: i32,
     pub height: i32,
     layers: u64,
     overlaps: u64,
     pub terrains: &'static Array<&'static Il2CppString>, 
 }
impl MapTerrain {
    pub fn get_tid(&self, x: i32, z: i32) -> Option<&'static Il2CppString> { unsafe { get_map_terrain_tid(self, x, z, None) }}
    pub fn get_instance() -> Option<&'static MapTerrain> { unsafe {  get_map_terrain(None) }}
    pub fn set_tid(&self, x: i32, z:i32, tid: &Il2CppString) { unsafe { set_map_terrain_tid(self, x, z, tid, None); }}
    pub fn update_image(&self) { unsafe { update_image_map_terrain(self, None) } }
}

#[unity::from_offset("App", "MapTerrain", "GetTid")]
fn get_map_terrain_tid(this: &MapTerrain, x: i32, z: i32, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "MapTerrain", "SetTid")]
fn set_map_terrain_tid(this: &MapTerrain, x: i32, z: i32, tid: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "MapSetting", "get_MapTerrain")]
fn get_map_terrain(method_info: OptionalMethod) -> Option<&'static MapTerrain>;

#[unity::from_offset("App", "MapTerrain", "UpdateMapImage")]
fn update_image_map_terrain(this: &MapTerrain, method_info: OptionalMethod);
