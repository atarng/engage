//! Methods and wrappers to manipulate the settings of the player.

use crate::{singleton::SingletonClass, gamevariable::GameVariable};
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};

#[repr(C)]
#[unity::class("App", "GameUserData")]
pub struct GameUserData { }

#[unity::from_offset("App", "GameUserData", "get_Variable")]
fn get_variable(this: &GameUserData, method_info: OptionalMethod) -> &'static mut GameVariable;

#[unity::from_offset("App", "GameUserData", "SetGameMode")]
fn set_game_mode(this: &GameUserData, game_mode: GameMode, method_info: OptionalMethod);

#[unity::from_offset("App", "GameUserData", "GetGameMode")]
fn get_game_mode(this: &GameUserData, method_info: OptionalMethod) -> GameMode;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Casual,
    Classic,
    Phoenix
}

impl From<i32> for GameMode {
    fn from(value: i32) -> Self {
        match value {
            0 => GameMode::Casual,
            1 => GameMode::Classic,
            2 => GameMode::Phoenix,
            _ => GameMode::Casual,
        }
    }
}

impl GameUserData {
    pub fn get_instance() -> &'static mut GameUserData {
        let idk = get_generic_class!(SingletonClass<GameUserData>).unwrap();
        let pointer = unsafe { &*(idk.rgctx_data as *const Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 6]) };
        let get_instance =
            unsafe { std::mem::transmute::<_, extern "C" fn(OptionalMethod) -> &'static mut GameUserData>(pointer[5].method_ptr) };
            
        get_instance(Some(&pointer[5]))
    }

    pub fn get_variable() -> &'static mut GameVariable {
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
