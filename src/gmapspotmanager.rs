use unity::prelude::*;
use crate::gamedata::dispos::ChapterData;

pub struct GmapSpotManager;

impl GmapSpotManager {
    pub fn set_state(cid: impl AsRef<str>, state: i32) {
        unsafe { gmap_spot_manager_set_state(cid.as_ref().into(), state, None) }
    }

    pub fn open_next_chapters(chapter: &ChapterData) {
        unsafe { gmap_spot_manager_open_next_chapters(chapter, None) }
    }
}

#[unity::from_offset("App", "GmapSpotManager", "SetState")]
fn gmap_spot_manager_set_state(cid: &Il2CppString, state: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "GmapSpotManager", "OpenNextChapters")]
fn gmap_spot_manager_open_next_chapters(chapter: &ChapterData, method_info: OptionalMethod);
