use unity::prelude::*;

use crate::{combat::Character, soundmanager::SoundSystemSoundHandle};

#[unity::class("App", "GameSound")]
pub struct GameSound {}

impl GameSound {
    pub fn post_event<'a>(event_name: impl Into<&'a Il2CppString>, character: Option<&Character>) -> *const u8 {
        unsafe { gamesound_postevent(event_name.into(), character, None) }
    }

    pub fn is_event_loaded<'a>(event_name: impl Into<&'a Il2CppString>) -> bool {
        unsafe { gamesound_iseventloaded(event_name.into(), None) }
    }
}

#[unity::class("", "Handle")]
pub struct GameSoundHandle {}

impl GameSoundHandle {
    pub fn new(sound_handle: &SoundSystemSoundHandle) -> &'static Self {
        let handle_class = GameSound::class().get_nested_types().iter().find(|class| class.get_name() == "Handle").unwrap();
        let handle = Il2CppObject::<GameSoundHandle>::from_class(&handle_class).unwrap();
        handle.ctor(sound_handle);
        handle
    }
    
    fn ctor(&self, sound_handle: &SoundSystemSoundHandle) {
        unsafe { gamesound_handle_ctor(self, sound_handle, None) }
    }
}

#[skyline::from_offset(0x2272fd0)]
extern "C" fn gamesound_postevent(event_name: &Il2CppString, character: Option<&Character>, method_info: OptionalMethod) -> *const u8;

#[skyline::from_offset(0x1e6db20)]
extern "C" fn gamesound_handle_ctor(this: &GameSoundHandle, sound_handle: &SoundSystemSoundHandle, method_info: OptionalMethod);

#[unity::from_offset("App", "GameSound", "IsEventLoaded")]
extern "C" fn gamesound_iseventloaded(event_name: &Il2CppString, method_info: OptionalMethod) -> bool;
