use unity::{prelude::*, system::List};

use crate::{pad::Pad, proc::ProcInst, titlebar::TitleBar, util::get_instance};

use super::{configmenu_createbind, BasicMenu, BasicMenuResult};

#[repr(C)]
#[unity::class("", "ConfigBasicMenuItem")]
pub struct ConfigBasicMenuItem {
    // Inlined BasicMenuItem here because C ABI dumb
    pub menu: &'static mut BasicMenu<ConfigBasicMenuItem>,
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
    fn new() -> &'static mut ConfigBasicMenuItem {
        let item = il2cpp::instantiate_class(ConfigBasicMenuItem::class().clone()).unwrap();
        unsafe { configbasicmenuitem_ctor(item, None) };
        item
    }

    pub fn new_switch<Methods: ConfigBasicMenuItemSwitchMethods>(title: impl AsRef<str>) -> &'static mut ConfigBasicMenuItem {
        let mut item = Self::new();

        Methods::init_content(item);

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

    pub fn new_gauge<Methods: ConfigBasicMenuItemGaugeMethods>(title: impl AsRef<str>) -> &'static mut ConfigBasicMenuItem {
        let mut item = Self::new();

        Methods::init_content(item);

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

    pub fn new_command<Methods: ConfigBasicMenuItemCommandMethods>(title: impl AsRef<str>) -> &'static mut ConfigBasicMenuItem {
        let mut item = Self::new();

        Methods::init_content(item);

        item.config_method = 0;
        item.is_arrow = false;
        item.is_command_icon = true;

        item.get_class_mut()
            .get_virtual_method_mut("CustomCall")
            .map(|method| method.method_ptr = Methods::custom_call as _)
            .unwrap();

        item.get_class_mut()
            .get_virtual_method_mut("SetCommandText")
            .map(|method| method.method_ptr = Methods::set_command_text as _);

        item.get_class_mut()
            .get_virtual_method_mut("OnSelect")
            .map(|method| method.method_ptr = Self::on_select as _)
            .unwrap();

        item.get_class_mut()
            .get_virtual_method_mut("OnDeselect")
            .map(|method| method.method_ptr = Self::on_deselect as _)
            .unwrap();

        item.title_text = title.into();

        Methods::set_command_text(item, None);
        Methods::set_help_text(item, None);

        item
    }

    extern "C" fn on_select(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod) {
        unsafe { configbasicmenuitem_on_select(this, method_info) }
        this.is_arrow = false;
        unsafe { configbasicmenuitem_update_text(this, method_info) }
    }

    extern "C" fn on_deselect(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod) {
        unsafe { configbasicmenuitem_on_deselect(this, method_info) }
        this.is_arrow = false;
        unsafe { configbasicmenuitem_update_text(this, method_info) }
    }
    
    pub fn update_text(&self) {
        unsafe {
            configbasicmenuitem_update_text(self, None);
        }
    }

    pub fn change_key_value_b(value: bool) -> bool {
        if unsafe { configbasicmenuitem_change_key_value_int(value as i32, 0, 1, 1, None) } == 1 {
            true
        } else {
            false
        }
    }

    pub fn change_key_value_i(value: i32, min: i32, max: i32, step: i32) -> i32 {
        unsafe { configbasicmenuitem_change_key_value_int(value, min, max, step, None) }
    }

    pub fn change_key_value_f(value: f32, min: f32, max: f32, step: f32) -> f32 {
        unsafe { configbasicmenuitem_change_key_value_float(value, min, max, step, None) }
    }
}

pub trait ConfigBasicMenuItemSwitchMethods {
    fn init_content(this: &mut ConfigBasicMenuItem) {
        
    }
    
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod) -> BasicMenuResult;
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod);
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod);
}

pub trait ConfigBasicMenuItemCommandMethods {
    fn init_content(this: &mut ConfigBasicMenuItem) {
        
    }

    fn menu_entries() -> Vec<&'static mut ConfigBasicMenuItem>;
    
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod) -> BasicMenuResult {
        let pad_instance = get_instance::<Pad>();

        // Check if A is pressed before executing any of this
        if pad_instance.npad_state.buttons.a() {
            // Close the original Settings menu temporarily so it doesn't get drawn in the background
            this.menu.get_class().get_virtual_method("CloseAnimeAll").map(|method| {
                let close_anime_all =
                    unsafe { std::mem::transmute::<_, extern "C" fn(&BasicMenu<ConfigBasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                close_anime_all(this.menu, method.method_info);
            });

            // Initialize the menu
            unsafe { configmenu_createbind(this.menu, None) }
            
            let config_menu = this.menu.proc.child.cast_mut::<BasicMenu<ConfigBasicMenuItem>>();

            // Register a OnDispose callback to restore the previous menu
            config_menu
                .get_class_mut()
                .get_virtual_method_mut("OnDispose")
                .map(|method| method.method_ptr = open_anime_all_ondispose as _)
                .unwrap();

            // Clear the buttons in the List so we can add our own
            config_menu.full_menu_item_list.get_class().get_virtual_method("Clear").map(|method| {
                let clear = unsafe { std::mem::transmute::<_, extern "C" fn(&List<ConfigBasicMenuItem>, &MethodInfo)>(method.method_info.method_ptr) };
                clear(&config_menu.full_menu_item_list, method.method_info);
            }).unwrap();
            
            let mut entries = Self::menu_entries();
            
            entries.into_iter().for_each(|cb| {
                config_menu.add_item(cb);
            });
            
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod);
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod);
}

extern "C" fn open_anime_all_ondispose(this: &mut ProcInst, _method_info: OptionalMethod) {
    this.parent.get_class().get_virtual_method("OpenAnimeAll").map(|method| {
        let open_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&ProcInst, &MethodInfo)>(method.method_info.method_ptr) };
        open_anime_all(this.parent, method.method_info);
    });
}

pub trait ConfigBasicMenuItemGaugeMethods {
    fn init_content(this: &mut ConfigBasicMenuItem) { }
    
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod) -> BasicMenuResult;
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod);
}

#[skyline::from_offset(0x25379a0)]
pub fn configbasicmenuitem_ctor(this: &ConfigBasicMenuItem, method_info: OptionalMethod);

#[skyline::from_offset(0x2537920)]
fn configbasicmenuitem_change_key_value_int(value: i32, min: i32, max: i32, step: i32, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x2537970)]
fn configbasicmenuitem_change_key_value_float(value: f32, min: f32, max: f32, step: f32, method_info: OptionalMethod) -> f32;

#[unity::from_offset("", "ConfigBasicMenuItem", "UpdateText")]
fn configbasicmenuitem_update_text(this: &ConfigBasicMenuItem, method_info: OptionalMethod);

#[unity::from_offset("", "ConfigBasicMenuItem", "OnSelect")]
fn configbasicmenuitem_on_select(this: &ConfigBasicMenuItem, method_info: OptionalMethod);

#[unity::from_offset("", "ConfigBasicMenuItem", "OnDeselect")]
fn configbasicmenuitem_on_deselect(this: &ConfigBasicMenuItem, method_info: OptionalMethod);