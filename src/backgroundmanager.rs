use unity::prelude::*;

#[unity::class("App", "BackgroundManager")]
pub struct BackgroundManager {}

impl BackgroundManager {
    pub fn bind() { unsafe { background_manager_bind(None); }}
    pub fn unbind() { unsafe { background_manager_unbind(None); }}
    pub fn is_bind() -> bool { unsafe { background_manager_isbind(None) }}
}

#[skyline::from_offset(0x02122980)]
fn background_manager_bind(method_info: OptionalMethod);

#[skyline::from_offset(0x02122b00)]
fn background_manager_unbind(method_info: OptionalMethod);

#[unity::from_offset("App", "BackgroundManager", "IsBind")]
fn background_manager_isbind(method_info: OptionalMethod) -> bool;
