use unity::prelude::*;
use unity::system::List;
use crate::gamedata::item::ItemData;
use crate::proc::{desc::ProcDesc, ProcInst, ProcInstFields, Bindable};

pub mod hubsequence;
pub mod access;


pub struct HubUtil {} 
impl HubUtil {
    pub fn get_current_cooking_pid() -> Option<&'static Il2CppString> { 
        unsafe { get_CurrentCookingPid(None) } 
    }
    pub fn get_item_count_with_bonus(item: &ItemData, count: i32) -> i32 { 
        unsafe { Hub_Get_Item_Count_Bonus(item, count, None) }
    }
    pub fn is_hub_sequence() -> bool { unsafe { is_hubsequence(None) } }
    pub fn set_cooking_pid(pid: &Il2CppString) { unsafe { set_CurrentCookingPid(pid, None); }}
    pub fn get_current_scene_name() -> &'static Il2CppString { unsafe {hub_get_currentscenename(None)} }
}
#[unity::from_offset("App", "HubUtil", "IsHubSequence")]
pub fn is_hubsequence(method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubUtil", "set_CurrentCookingPid")]
pub fn set_CurrentCookingPid(value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x02a64450)]
pub fn get_CurrentCookingPid(method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "HubUtil", "GetItemCountWithBonus")]
pub fn Hub_Get_Item_Count_Bonus(item: &ItemData, base_count: i32, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "HubVariable", "GetCurrentSceneName")]
fn hub_get_currentscenename(method_info: OptionalMethod) -> &'static Il2CppString;

