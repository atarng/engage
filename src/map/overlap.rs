use unity::prelude::*;
use crate::gamedata::{unit::Unit, terrain::TerrainData};

/// Contains tiles created/placed over the map terrains
#[unity::class("App", "MapOverlap")]
pub struct MapOverlap {}

impl MapOverlap {
    pub fn can_create(attacker: Option<&Unit>, x: i32, z: i32, terrain: &TerrainData) -> bool{
        unsafe { mapoverlap_can_create(attacker, x, z, terrain, None) }
    }
    
    pub fn get_terrain_by_index(index: i32) -> Option<&'static TerrainData> {
        unsafe { mapoverlap_get_terrain_index(index, None) }
    }
    pub fn get_terrain(x: i32, z: i32) -> Option<&'static TerrainData> {
        unsafe { mapoverlap_get_terrain(x, z, None) }
    }

    pub fn set(x: i32, z: i32, tid: &Il2CppString, turn: i32, phase: i32) -> bool {
        unsafe { mapoverlap_set(x, z, tid, turn, phase, None) }
    }
}

#[skyline::from_offset(0x01dfe300)]
fn mapoverlap_can_create(attacker: Option<&Unit>, x: i32, y: i32, terrain: &TerrainData, method_info: OptionalMethod) -> bool;


#[skyline::from_offset(0x01dfd9b0)]
fn mapoverlap_get_terrain_index(index: i32, method_info: OptionalMethod) -> Option<&'static TerrainData>;

#[skyline::from_offset(0x01dfda90)]
fn mapoverlap_get_terrain(x: i32, z: i32, method_info: OptionalMethod) -> Option<&'static TerrainData>;

#[skyline::from_offset(0x01decd40)]
fn mapoverlap_set(x: i32, z: i32, tid: &Il2CppString, turn: i32, phase: i32,  method_info: OptionalMethod) -> bool;