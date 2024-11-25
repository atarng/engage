use unity::prelude::*;

use crate::soundmanager::SoundSystemSoundHandle;

#[unity::class("App", "GameSound")]
pub struct GameSound {}

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

#[skyline::from_offset(0x1e6db20)]
fn gamesound_handle_ctor(this: &GameSoundHandle, sound_handle: &SoundSystemSoundHandle, method_info: OptionalMethod);