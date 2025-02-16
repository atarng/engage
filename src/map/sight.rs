use unity::prelude::*;

use crate::util::get_instance;
// Fog of War 

#[unity::class("App", "MapSight")]
pub struct MapSight {
   junk: [u8; 0x28],
   pub usable: bool,   // true = Fog of War
}

impl MapSight {
    pub fn get_instance() -> &'static mut MapSight { get_instance::<Self>() }
    pub fn update_all(&self) { unsafe {  update_map_sight(self, None); } }
}


#[skyline::from_offset(0x01f35220)]
pub fn update_map_sight(this: &MapSight, method_info: OptionalMethod);