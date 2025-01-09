use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use crate::singleton::SingletonClass;
use crate::gamedata::unit::Unit;
use crate::util::get_instance;

#[unity::class("App", "SortieSelectionUnitManager")]
pub struct SortieSelectionUnitManager {}

impl SortieSelectionUnitManager {
    pub fn get_instance() -> &'static mut SortieSelectionUnitManager {
        let idk = get_generic_class!( SingletonClass<SortieSelectionUnitManager>).unwrap();
        let pointer = unsafe { &*(idk.rgctx_data as *const unity::il2cpp::class::Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 6]) };
        let get_instance =
            unsafe { std::mem::transmute::<_, extern "C" fn(OptionalMethod) -> &'static mut SortieSelectionUnitManager>(pointer[5].method_ptr) };
            
        get_instance(Some(&pointer[5]))
    }
    pub fn is_sortie_mode() -> bool {
        unsafe { sortie_is_sortie_mode( &SortieSelectionUnitManager::get_instance(), None )}
    }
    pub fn get_unit() -> &'static mut Unit {
        let instance = Self::get_instance();
        unsafe { sortie_get_unit(instance, None) }
    }
}

#[skyline::from_offset(0x01fe8c00)]
fn sortie_is_sortie_mode(this: &SortieSelectionUnitManager, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01fe8db0)]
fn sortie_get_unit(this: &SortieSelectionUnitManager, method_info: OptionalMethod) -> &'static mut Unit;