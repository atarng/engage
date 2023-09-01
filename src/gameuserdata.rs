use crate::{singleton::SingletonClass, gamevariable::GameVariable};
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};

#[repr(C)]
#[unity::class("App", "GameUserData")]
pub struct GameUserData;

#[unity::from_offset("App", "GameUserData", "get_Variable")]
fn get_variable(this: &Il2CppObject<GameUserData>, method_info: OptionalMethod) -> &'static mut Il2CppObject<GameVariable>;

#[unity::from_offset("App", "GameUserData", "SetGameMode")]
fn set_game_mode(this: &Il2CppObject<GameUserData>, game_mode: GameMode, method_info: OptionalMethod);

#[unity::from_offset("App", "GameUserData", "GetGameMode")]
fn get_game_mode(this: &Il2CppObject<GameUserData>, method_info: OptionalMethod) -> GameMode;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Casual,
    Classic,
    Phoenix
}

impl GameUserData {
    pub fn get_instance() -> &'static mut Il2CppObject<GameUserData> {
        let idk = get_generic_class!(SingletonClass<GameUserData>).unwrap();
        let pointer = unsafe { &*(idk.rgctx_data as *const Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 6]) };
        let get_instance =
            unsafe { std::mem::transmute::<_, extern "C" fn(OptionalMethod) -> &'static mut Il2CppObject<GameUserData>>(pointer[5].method_ptr) };
            
        get_instance(Some(&pointer[5]))
    }

    pub fn get_variable() -> &'static mut Il2CppObject<GameVariable> {
        let instance = Self::get_instance();
        unsafe { get_variable(instance, None) }
    }

    pub fn get_game_mode() -> GameMode {
        let instance = Self::get_instance();
        unsafe { get_game_mode(instance, None) }
    }

    pub fn set_game_mode(game_mode: GameMode) {
        let instance = Self::get_instance();
        unsafe { set_game_mode(instance, game_mode, None) }
    }
}
