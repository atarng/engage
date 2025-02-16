use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use crate::singleton::SingletonClass;
use crate::gamedata::unit::Unit;
use crate::menu::BasicMenuSelect;
use crate::util::get_instance;

#[unity::class("App", "SortieSelectionUnitManager")]
pub struct SortieSelectionUnitManager {}

impl SortieSelectionUnitManager {
    pub fn get_instance() -> Option<&'static mut SortieSelectionUnitManager> {
        Some(get_instance::<Self>())
    }
    pub fn is_sortie_mode() -> bool {
        unsafe { sortie_is_sortie_mode( Self::get_instance().unwrap(), None )}
    }
    pub fn get_unit() -> &'static mut Unit {
        let instance = Self::get_instance().unwrap();
        unsafe { sortie_get_unit(instance, None) }
    }
}

#[unity::class("App", "SortieTopMenuManager")]
pub struct SortieTopMenuManager {}

impl SortieTopMenuManager {
    pub fn get_instance() -> Option<&'static mut SortieTopMenuManager> {
        Some(get_instance::<Self>())
    }
    pub fn get_menu_select() -> &'static mut BasicMenuSelect {
        unsafe { sortie_top_menu_get_menu_select(Self::get_instance().unwrap(), None)}
    }
}


#[skyline::from_offset(0x01fe8c00)]
fn sortie_is_sortie_mode(this: &SortieSelectionUnitManager, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01fe8db0)]
fn sortie_get_unit(this: &SortieSelectionUnitManager, method_info: OptionalMethod) -> &'static mut Unit;

#[unity::from_offset("App", "SortieTopMenuManager", "get_MenuSelect")]
fn sortie_top_menu_get_menu_select(this: &SortieTopMenuManager, method_info: OptionalMethod) -> &'static mut BasicMenuSelect;
