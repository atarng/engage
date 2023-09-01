use unity::{prelude::*, system::List};

use super::BasicMenuResult;
use crate::proc::ProcInst;

#[repr(C)]
#[unity::class("", "ConfigBasicMenuItem")]
pub struct ConfigBasicMenuItem {
    // Inlined BasicMenuItem here because C ABI dumb
    pub menu: &'static mut Il2CppObject<ProcInst>,
    menu_item_content: *const u8,
    name: &'static Il2CppString,
    index: i32,
    full_index: i32,
    attribute: i32,
    cursor_color: unity::engine::Color,
    active_text_color: unity::engine::Color,
    inactive_text_color: unity::engine::Color,
    config_method: i32,
    pub title_text: &'static Il2CppString,
    pub command_text: &'static Il2CppString,
    pub help_text: &'static Il2CppString,
    pub is_arrow: bool,
    pub is_command_icon: bool,
    pub gauge_ratio: f32,
}

impl ConfigBasicMenuItem {
    fn new() -> &'static mut Il2CppObject<ConfigBasicMenuItem> {
        let item = il2cpp::instantiate_class(ConfigBasicMenuItem::get_class().clone()).unwrap();
        unsafe { configbasicmenuitem_ctor(item, None) };
        item
    }

    pub fn new_switch<Methods: ConfigBasicMenuItemSwitchMethods>(title: impl AsRef<str>) -> &'static mut Il2CppObject<ConfigBasicMenuItem> {
        let item = Self::new();

        item.config_method = 0;

        item.get_class_mut()
            .get_virtual_method_mut("CustomCall")
            .map(|method| method.method_ptr = Methods::custom_call as _);

        item.get_class_mut()
            .get_virtual_method_mut("SetCommandText")
            .map(|method| method.method_ptr = Methods::set_command_text as _);

        item.get_class_mut()
            .get_virtual_method_mut("SetHelpText")
            .map(|method| method.method_ptr = Methods::set_help_text as _);

        item.title_text = title.into();

        Methods::set_command_text(item, None);
        Methods::set_help_text(item, None);

        item
    }

    pub fn new_gauge<Methods: ConfigBasicMenuItemGaugeMethods>(title: impl AsRef<str>) -> &'static mut Il2CppObject<ConfigBasicMenuItem> {
        let item = Self::new();

        item.config_method = 1;

        item.get_class_mut()
            .get_virtual_method_mut("CustomCall")
            .map(|method| method.method_ptr = Methods::custom_call as _);

        item.get_class_mut()
            .get_virtual_method_mut("SetHelpText")
            .map(|method| method.method_ptr = Methods::set_help_text as _);

        item.title_text = title.into();

        Methods::set_help_text(item, None);

        item
    }
}

pub trait ConfigBasicMenuItemSwitchMethods {
    extern "C" fn custom_call(this: &mut Il2CppObject<ConfigBasicMenuItem>, method_info: OptionalMethod) -> BasicMenuResult;
    extern "C" fn set_command_text(this: &mut Il2CppObject<ConfigBasicMenuItem>, method_info: OptionalMethod);
    extern "C" fn set_help_text(this: &mut Il2CppObject<ConfigBasicMenuItem>, method_info: OptionalMethod);
}

pub trait ConfigBasicMenuItemGaugeMethods {
    extern "C" fn custom_call(this: &mut Il2CppObject<ConfigBasicMenuItem>, method_info: OptionalMethod) -> BasicMenuResult;
    extern "C" fn set_help_text(this: &mut Il2CppObject<ConfigBasicMenuItem>, method_info: OptionalMethod);
}

#[skyline::from_offset(0x25379a0)]
pub fn configbasicmenuitem_ctor(this: &Il2CppObject<ConfigBasicMenuItem>, method_info: OptionalMethod);

#[unity::from_offset("", "ConfigBasicMenuItem", "ChangeKeyValue")]
pub fn configbasicmenuitem_change_key_value<T>(value: T, min: i32, max: i32, step: i32, method_info: OptionalMethod) -> T;

#[unity::from_offset("", "ConfigBasicMenuItem", "UpdateText")]
pub fn configbasicmenuitem_update_text(this: &mut Il2CppObject<ConfigBasicMenuItem>, method_info: OptionalMethod);

#[skyline::from_offset(0x3de9550)]
pub fn generic_list_add<T>(list: &mut Il2CppObject<List<T>>, instance: &T, method_id: *mut u64);
