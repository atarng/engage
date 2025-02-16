use unity::prelude::*;
use crate::proc::{Bindable, ProcInst, ProcInstFields};
use super::{*, access::{HubAccess, HubAccessManager}, HubLocatorGroup};

#[unity::class("App", "HubSequence")]
pub struct HubSequence {
    proc: ProcInstFields,
    is_resume: bool,
    m_is_loaded: bool,
    //
}
impl Bindable for HubSequence {}

impl HubSequence {
    pub fn get_instance() -> &'static Self {
        let method = HubSequence::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("get_Instance")));
        let get_instance = unsafe { std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> &'static HubSequence>( method.unwrap().method_ptr, ) };
        get_instance(method.unwrap())
    }
    pub fn is_exist_instance() -> bool { unsafe { hub_is_exist_instance(None) }}
    pub fn get_mini_map(&self) -> &HubMiniMap { unsafe { hub_get_mini_map(self, None)}}
    
    pub fn get_current_access_data(&self) -> &mut HubAccessManager { unsafe { hub_get_access_data(self, None)}}
    pub fn get_player_controller() -> Option<&'static HubPlayerController> { unsafe { hub_sequence_get_player_controller(None)}}
    pub fn get_locator_group(&self) -> &'static mut HubLocatorGroup { unsafe { hub_get_locator_group(self, None)}}
}



#[unity::from_offset("App", "HubSequence", "get_MiniMap")]
fn hub_get_mini_map(this: &HubSequence, method_info: OptionalMethod) -> &HubMiniMap;

#[unity::from_offset("App", "HubSequence", "get_CurrentAccessData")]
fn hub_get_access_data(this: &HubSequence, method_info: OptionalMethod) -> &'static mut HubAccessManager;

#[unity::from_offset("App", "HubSequence", "GetPlayerController")]
fn hub_sequence_get_player_controller(method_info: OptionalMethod) -> Option<&'static HubPlayerController>;

#[unity::from_offset("App", "HubSequence", "IsExistInstance")]
fn hub_is_exist_instance(method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubSequence", "get_LocatorGroup")]
fn hub_get_locator_group(this: &HubSequence, method_info: OptionalMethod) -> &'static mut HubLocatorGroup;

