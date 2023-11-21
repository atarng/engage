//! Methods to interact with the savedata variables.

use unity::prelude::*;

use crate::gameuserdata::*;

#[repr(C)]
#[unity::class("App", "GameVariable")]
pub struct GameVariable { }

#[unity::from_offset("App", "GameVariable", "GetBool")]
pub fn get_bool(this: &GameVariable, key: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x251ed10)]
pub fn set_bool(this: &GameVariable, key: &Il2CppString, enable: bool, method_info: OptionalMethod);

#[skyline::from_offset(0x2512870)]
pub fn entry(this: &GameVariable, key: &Il2CppString, num: i32, method_info: OptionalMethod) -> bool;
// bool App.GameVariable$$Entry
// (App_GameVariable_o *__this,System_String_o *key,int32_t num,MethodInfo *method)

// App.GameVariable$$EntryNoRewind	710251e560	bool App.GameVariable$$EntryNoRewind(App_GameVariable_o * __this, System_String_o * key, int32_t num, MethodInfo * method)	248
#[skyline::from_offset(0x251e560)]
pub fn entry_no_rewind(this: &GameVariable, key: &Il2CppString, num: i32, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "GameVariable", "GetNumber")]
pub fn get_number(this: &GameVariable, key: &Il2CppString, num: i32, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x251efb0)]
pub fn set_number(this: &GameVariable, key: &Il2CppString, num: i32, method_info: OptionalMethod);

pub struct GameVariableManager;

impl GameVariableManager {
    /// Create an entry in the game variable dictionary. Returns true if the entry was created.
    /// Returns false if the entry already exists.
    /// Sets the value of the entry but then we can just set it to whatever type we want, it seems.
    pub fn make_entry(key: &str, num: i32) -> bool {
        let game_variable = GameUserData::get_variable();

        unsafe { entry(game_variable, key.into(), num, None) }
    }

    pub fn make_entry_norewind(key: &str, num: i32) -> bool {
        let game_variable = GameUserData::get_variable();

        unsafe { entry_no_rewind(game_variable, key.into(), num, None) }
    }

    /// Get a game variable as a boolean value.
    /// If the entry doesn't exist, this will always return false.
    pub fn get_bool(key: &str) -> bool {
        let game_variable = GameUserData::get_variable();

        unsafe { get_bool(game_variable, key.into(), None) }
    }

    /// Set a game variable to the provided boolean value.
    /// This will NOT create a new entry if it deosn't exist.
    pub fn set_bool(key: &str, value: bool) {
        let game_variable = GameUserData::get_variable();

        unsafe {
            set_bool(game_variable, key.into(), value, None);
        }
    }
}
