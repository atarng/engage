use crate::{gmapspotmanager::gmap_spot_manager_open_next_chapters, proc::Bindable};
use crate::gamedata::dispos::ChapterData;
use unity::{system::List, prelude::*};

#[repr(C)]
#[unity::class("App", "GmapSequence")]
pub struct GmapSequence {
    parent: [u8; 0x78], //ProcSceneSequence
    pub now_spot: &'static GmapSpot,
    pub changing_spot: &'static GmapSpot,
    changing_path: *const u8,
    pub dispos_spot: &'static GmapSpot,
    gmap_camera: *const u8,
    path_controller: *const u8,
    virtual_sphere: *const u8,
    whole_map: *const u8,
    pub map_info: &'static GmapMapInfoContent,

}
impl Bindable for GmapSequence {}

#[unity::class("App", "GmapMapInfoContent")]
pub struct GmapMapInfoContent {}

impl GmapMapInfoContent {
    pub fn close(&self) { unsafe { mapinfo_close(self, None); }}
}

#[unity::class("App", "GmapSpot")]
pub struct GmapSpot {
    pub global_flag_name: &'static Il2CppString,
    pub chapters: &'static List<ChapterData>,
}

impl GmapSpot {
    pub fn get_chapter(&self) -> &'static ChapterData { unsafe { gmapspot_get_chapter(self, None) }  }
    pub fn is_completed(&self) -> bool { unsafe { gmapspot_is_completed(self, None) }}
}

#[unity::from_offset("App", "GmapSpot", "get_Chapter")]
fn gmapspot_get_chapter(this: &GmapSpot, method_info: OptionalMethod) -> &'static ChapterData;

#[unity::from_offset("App", "GmapSpot", "IsCompleted")]
fn gmapspot_is_completed(this: &GmapSpot, method_info: OptionalMethod) -> bool; 

#[skyline::from_offset(0x0252a640)]
pub fn mapinfo_close(this: &GmapMapInfoContent, method_info: OptionalMethod); 