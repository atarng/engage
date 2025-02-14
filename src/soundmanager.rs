use unity::{
    prelude::*,
    system::List,
};
use crate::combat::Character;

// Careful, this is a nested class, do not instantiate this yourself
#[unity::class("App.SoundSystem", "SoundHandle")]
pub struct SoundHandle {}

impl SoundHandle {
    pub fn get_event_name(&self) -> &'static mut Il2CppString {
        self.get_class()
            .get_virtual_method("GetEventName")
            .map(|method| {
                let get_event_name = unsafe {
                    std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> &'static mut Il2CppString>(method.method_info.method_ptr)
                };

                get_event_name(&self, method.method_info)
            })
            .unwrap()
    }
}

#[unity::class("App", "SoundManager")]
pub struct SoundManager {
    padding: [u8; 0x88],
    sound_handle_list: &'static List<SoundHandle>,
    // ...
}

#[unity::class("", "SoundHandle")]
pub struct SoundSystemSoundHandle {}

impl SoundManager {
    pub fn get_instance() -> &'static Self {
        crate::util::get_instance_monobehaviour()
    }

    pub fn post_event(&self, event_name: impl AsRef<str>, character: Option<&Character>, is_get_position: bool) -> &'static SoundSystemSoundHandle {
        unsafe { soundmanager_post_event(self, event_name.as_ref().into(), character, is_get_position, None) }
    }

    pub fn is_event_playing_with_prefix<S: AsRef<str>>(&self, event_name: S) -> bool {
        let event_name = event_name.as_ref();

        self.sound_handle_list
            .iter()
            .find(|handle| handle.get_event_name().to_string().starts_with(event_name))
            .is_some()
    }
}

#[skyline::from_offset(0x24f70b0)]
fn soundmanager_post_event(this: &SoundManager, event_name: &Il2CppString, character: Option<&Character>, is_get_position: bool, method_info: OptionalMethod) -> &'static SoundSystemSoundHandle;
