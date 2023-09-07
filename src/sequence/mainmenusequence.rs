use unity::prelude::*;

use crate::{proc::ProcInst, singleton::SingletonProcInst};

#[repr(C)]
#[unity::class("App", "MainMenuSequence")]
pub struct MainMenuSequence {
    pub proc: ProcInst,
    is_resume: bool,
    is_loaded: bool,
    pub prev_sequence: i32,
    pub now_sequence: i32,
    pub next_sequence: i32,
}

impl MainMenuSequence {
    pub fn get() -> &'static Il2CppObject<MainMenuSequence> {
        let idk = get_generic_class!(SingletonProcInst<MainMenuSequence>).unwrap();

        let get_instance = unsafe {
            std::mem::transmute::<_, extern "C" fn(OptionalMethod) -> &'static mut Il2CppObject<MainMenuSequence>>(idk.rgctx_data.get_instance.method_ptr)
        };

        get_instance(Some(idk.rgctx_data.get_instance))
    }

    pub fn get_mut() -> &'static mut Il2CppObject<MainMenuSequence> {
        let idk = get_generic_class!(SingletonProcInst<MainMenuSequence>).unwrap();

        let get_instance = unsafe {
            std::mem::transmute::<_, extern "C" fn(OptionalMethod) -> &'static mut Il2CppObject<MainMenuSequence>>(idk.rgctx_data.get_instance.method_ptr)
        };

        get_instance(Some(idk.rgctx_data.get_instance))
    }

    pub fn jump_to_next_sequence() {
        let instance = Self::get();
        unsafe { mainmenusequence_jumptonextsequence(instance) };
    }
}

impl AsRef<ProcInst> for MainMenuSequence {
    fn as_ref(&self) -> &ProcInst {
        &self.proc
    }
}

impl AsMut<ProcInst> for MainMenuSequence {
    fn as_mut(&mut self) -> &mut ProcInst {
        &mut self.proc
    }
}

#[unity::from_offset("App", "MainMenuSequence", "JumpToNextSequence")]
fn mainmenusequence_jumptonextsequence(this: &Il2CppObject<MainMenuSequence>);