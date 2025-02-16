use access::HubAccess;
use unity::{prelude::*, system::List};
use crate::gamedata::item::ItemData;

pub mod hubsequence;
pub mod access;


#[unity::class("App", "HubMiniMap")]
pub struct HubMiniMap {}

#[unity::class("App", "HubPlayerController")]
pub struct HubPlayerController {}

#[unity::class("App", "HubLocatorGroup")]
pub struct HubLocatorGroup {
    active_group: u64,
    unit_list: u64,
    pub access_list: &'static mut List<HubAccess>,
}
pub struct HubUtil {} 
impl HubUtil {
    pub fn get_current_cooking_pid() -> Option<&'static Il2CppString> { 
        unsafe { get_current_cooking_pid(None) } 
    }
    pub fn get_item_count_with_bonus(item: &ItemData, count: i32) -> i32 { 
        unsafe { hub_get_item_count_bonus(item, count, None) }
    }
    pub fn is_hub_sequence() -> bool { unsafe { is_hubsequence(None) } }
    pub fn set_cooking_pid(pid: &Il2CppString) { unsafe { set_current_cooking_pid(pid, None); }}
    pub fn get_current_scene_name() -> &'static Il2CppString { unsafe {hub_get_currentscenename(None)} }
    pub fn is_comeplete(cid: &Il2CppString) -> bool { unsafe { hub_util_is_comeplet(cid, None)}}
    pub fn found_mascot() -> bool { unsafe { mascot_is_found(None)}}
    pub fn get_animal_item_flag(index: i32) -> &'static Il2CppString { unsafe { hub_animal_item_flagname(index, None)}}
}

impl HubMiniMap {
    pub fn hide(&self) { unsafe { hub_mini_map_hide(self, None); }}
    pub fn hide_system_menu(&self) { unsafe { hub_mini_map_system_hide(self, None); } }
    pub fn is_show(&self) -> bool { unsafe { hub_mini_map_is_show(self, None)}}
    pub fn set_mode(&self, mode: i32) { unsafe { hub_mini_map_set_mode(self, mode, None); }}
}

impl HubPlayerController {
    pub fn get_access(&self) -> Option<&'static HubAccess> {  unsafe { controller_get_access(self, None) } }
    pub fn get_last_access(&self) -> Option<&'static HubAccess> { unsafe { controller_get_last_access(self, None) }}
    pub fn update_access(&self, force: bool) { unsafe { controller_update_access(self, force, None); }}
}
pub struct HubVariableMascot;
impl HubVariableMascot {
    pub fn is_found() -> bool { unsafe { mascot_is_found(None)}}
    pub fn done_strok() -> bool { unsafe {mascot_done_strok(None) } }
    pub fn add_point(pt: i32) { unsafe { mascot_add_point(pt, None); } }
    pub fn is_done_food() -> bool { unsafe { mascot_is_done_eat_food(None) }}
    pub fn done_food() { unsafe { mascot_done_food(None);}}
    pub fn found() { unsafe { mascot_found(None); }}
}


#[unity::from_offset("App", "HubUtil", "IsHubSequence")]
fn is_hubsequence(method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02aee390)]
fn mascot_is_found(method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02aef8f0)]
fn mascot_done_strok(method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02aeed20)]
fn mascot_add_point(pt: i32, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02aefc10)]
fn mascot_is_done_eat_food(method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02aee4b0)]
fn mascot_found(method_info: OptionalMethod);

#[skyline::from_offset(0x02aefa10)]
fn mascot_done_food(method_info: OptionalMethod);

#[skyline::from_offset(0x02a6a410)]
fn hub_animal_item_flagname(index: i32, method_info: OptionalMethod) -> &'static Il2CppString;
#[unity::from_offset("App", "HubUtil", "set_CurrentCookingPid")]
pub fn set_current_cooking_pid(value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x02a64450)]
pub fn get_current_cooking_pid(method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "HubUtil", "GetItemCountWithBonus")]
pub fn hub_get_item_count_bonus(item: &ItemData, base_count: i32, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "HubUtil", "IsComplete")]
pub fn hub_util_is_comeplet(cid: &Il2CppString, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubVariable", "GetCurrentSceneName")]
fn hub_get_currentscenename(method_info: OptionalMethod) -> &'static Il2CppString;


#[unity::from_offset("App", "HubMiniMap", "HideSystemMenu")]
fn hub_mini_map_system_hide(this: &HubMiniMap, method_info: OptionalMethod);

#[unity::from_offset("App", "HubMiniMap", "SetMode")]
fn hub_mini_map_set_mode(this: &HubMiniMap, mode: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "HubMiniMap", "Hide")]
fn hub_mini_map_hide(this: &HubMiniMap, method_info: OptionalMethod);

#[unity::from_offset("App", "HubMiniMap", "IsShow")]
fn hub_mini_map_is_show(this: &HubMiniMap, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubPlayerController", "TryGetNowAccess")]
fn controller_get_access(this: &HubPlayerController, method_info: OptionalMethod) -> Option<&'static HubAccess>;

#[unity::from_offset("App", "HubPlayerController", "TryGetLastAccess")]
fn controller_get_last_access(this: &HubPlayerController, method_info: OptionalMethod) -> Option<&'static HubAccess>;

#[unity::from_offset("App", "HubPlayerController", "UpdateAccess")]
fn controller_update_access(this: &HubPlayerController, force: bool, method_info: OptionalMethod);

