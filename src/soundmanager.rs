use unity::prelude::*;
use crate::combat::Character;

#[unity::class("App", "SoundManager")]
pub struct SoundManager {}

#[unity::class("", "SoundHandle")]
pub struct SoundSystemSoundHandle {}

impl SoundManager {
    pub fn get_instance() -> &'static Self {
        crate::util::get_instance_monobehaviour()
    }

    pub fn post_event(&self, event_name: impl AsRef<str>, character: Option<&Character>, is_get_position: bool) -> &'static SoundSystemSoundHandle {
        unsafe { soundmanager_post_event(self, event_name.as_ref().into(), character, is_get_position, None) }
    }
}

#[skyline::from_offset(0x24f70b0)]
fn soundmanager_post_event(this: &SoundManager, event_name: &Il2CppString, character: Option<&Character>, is_get_position: bool, method_info: OptionalMethod) -> &'static SoundSystemSoundHandle;