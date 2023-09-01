use std::ops::{Deref, DerefMut};

use modular_bitfield::{bitfield, specifiers::B2};
use unity::{prelude::*, system::List};

use crate::proc::{desc::ProcDesc, IsProcInst, ProcInst};

pub mod config;
pub mod content;

/// Represents the base Menu from which every other inherits.
///
/// A Menu is a [ProcInst](crate::proc::ProcInst) with fields to keep every element together
///
/// You can usually use this instead of a class inheriting from it at the risk of missing fields and methods.
#[repr(C)]
#[unity::class("App", "BasicMenu")]
pub struct BasicMenu<T: 'static> {
    pub proc: ProcInst,
    pub menu_content: *const u8,
    pub menu_item_list: &'static mut Il2CppObject<List<T>>,
    pub full_menu_item_list: &'static mut Il2CppObject<List<T>>,
    pad: [u8; 0x30],
    pub reserved_show_row_num: i32,
    pub memory_display_index: i32,
    pub suspend: i32,
}

pub trait BasicMenuMethods {
    fn create_default_desc(&self) -> &'static mut Il2CppArray<&'static mut Il2CppObject<ProcDesc>> {
        unsafe { basicmenu_createdefaultdesc(self, None) }
    }

    fn set_show_row_num(&self, max_row: i32) {
        unsafe {
            basicmenu_setshowrownum(self, max_row, None);
        }
    }

    fn bind_parent_menu(&self) {
        unsafe {
            basicmenu_bindparentmenu(self, None);
        }
    }
}

impl<T> BasicMenuMethods for Il2CppObject<BasicMenu<T>> {}

impl<T> BasicMenu<T> {
    pub fn new(menu_item_list: &Il2CppObject<List<T>>, menu_content: &Il2CppObject<BasicMenuContent>) -> &'static mut Il2CppObject<BasicMenu<T>> {
        let instance: &'static mut Il2CppObject<BasicMenu<T>> = BasicMenu::<T>::instantiate().unwrap();
        unsafe { basicmenu_ctor(instance, menu_item_list, menu_content, None) };
        instance
    }
}

#[unity::from_offset("App", "BasicMenu", ".ctor")]
fn basicmenu_ctor<P: BasicMenuMethods + ?Sized, T>(
    this: &P,
    menu_item_list: &Il2CppObject<List<T>>,
    menu_content: &Il2CppObject<BasicMenuContent>,
    method_info: OptionalMethod,
);

#[unity::from_offset("App", "BasicMenu", "BindParentMenu")]
fn basicmenu_bindparentmenu<P: BasicMenuMethods + ?Sized>(this: &P, method_info: OptionalMethod);

#[unity::from_offset("App", "BasicMenu", "SetShowRowNum")]
fn basicmenu_setshowrownum<P: BasicMenuMethods + ?Sized>(this: &P, max_row: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "BasicMenu", "CreateDefaultDesc")]
fn basicmenu_createdefaultdesc<P: BasicMenuMethods + ?Sized>(
    this: &P,
    method_info: OptionalMethod,
) -> &'static mut Il2CppArray<&'static mut Il2CppObject<ProcDesc>>;

impl<T> AsRef<ProcInst> for BasicMenu<T> {
    fn as_ref(&self) -> &ProcInst {
        &self.proc
    }
}

impl<T> AsMut<ProcInst> for BasicMenu<T> {
    fn as_mut(&mut self) -> &mut ProcInst {
        &mut self.proc
    }
}

impl<T> Deref for BasicMenu<T> {
    type Target = ProcInst;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for BasicMenu<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.proc
    }
}

pub trait MenuSequence {
    fn bind(parent: &impl IsProcInst) {
        let proc = ProcInst::instantiate().unwrap();
        let descs = Il2CppArray::new_from(ProcDesc::get_class(), Self::get_proc_desc(proc)).unwrap();

        proc.create_bind(parent, descs, Self::proc_name());
    }

    fn get_proc_desc(_this: &'static Il2CppObject<ProcInst>) -> Vec<&'static mut Il2CppObject<ProcDesc>> {
        vec![ProcDesc::end().unwrap()]
    }

    fn proc_name() -> &'static str {
        "MenuSequence"
    }
}

/// Represents the base MenuItem from which every other inherits.
///
/// A MenuItem is the class representing things such as an entry in a menu's list.
///
/// You can usually use this instead of a class inheriting from it at the risk of not having default implementations for functions.
#[repr(C)]
#[unity::class("App", "BasicMenuItem")]
pub struct BasicMenuItem {
    pub menu: &'static mut Il2CppObject<BasicMenu<BasicMenuItem>>,
    menu_item_content: *const u8,
    name: &'static Il2CppString,
    index: i32,
    full_index: i32,
    attribute: i32,
    cursor_color: unity::engine::Color,
    active_text_color: unity::engine::Color,
    inactive_text_color: unity::engine::Color,
}

impl BasicMenuItem {
    pub fn new() -> &'static mut Il2CppObject<BasicMenuItem> {
        let item = BasicMenuItem::instantiate().unwrap();
        unsafe {
            basicmenuitem_ctor(item, None);
        }

        item
    }

    pub fn new_impl<Methods: BasicMenuItemMethods>() -> &'static mut Il2CppObject<BasicMenuItem> {
        let custom_class = BasicMenuItem::get_class().clone();
        let item = il2cpp::instantiate_class(&custom_class).unwrap();

        unsafe {
            basicmenuitem_ctor(item, None);
        }

        item.get_class_mut()
            .get_virtual_method_mut("GetName")
            .map(|method| method.method_ptr = Methods::get_name as _)
            .unwrap();

        // item
        //     .get_class_mut()
        //     .get_virtual_method_mut("GetHelpText")
        //     .map(|method| method.method_ptr = Methods::get_name as _);

        item.get_class_mut()
            .get_virtual_method_mut("ACall")
            .map(|method| method.method_ptr = Methods::a_call as _)
            .unwrap();

        item.get_class_mut()
            .get_virtual_method_mut("BuildAttribute")
            .map(|method| method.method_ptr = Methods::build_attributes as _)
            .unwrap();

        item
    }
}

pub trait BasicMenuItemMethods {
    extern "C" fn get_name(_this: &mut Il2CppObject<BasicMenuItem>, _method_info: OptionalMethod) -> &'static Il2CppString;

    extern "C" fn get_help_text(_this: &mut Il2CppObject<BasicMenuItem>, _method_info: OptionalMethod) -> &'static Il2CppString {
        "".into()
    }

    extern "C" fn a_call(_this: &'static mut Il2CppObject<BasicMenuItem>, _method_info: OptionalMethod) -> BasicMenuResult {
        BasicMenuResult::se_decide()
    }

    extern "C" fn build_attributes(_this: &mut Il2CppObject<BasicMenuItem>, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        BasicMenuItemAttribute::Enable
    }
}

#[skyline::from_offset(0x2455fc0)]
fn basicmenuitem_ctor(this: &Il2CppObject<BasicMenuItem>, method_info: OptionalMethod);

#[repr(C)]
#[unity::class("App", "BasicMenuContent")]
pub struct BasicMenuContent {
    pub base: u64,
    // ...
}

/// The return type for Call methods on classes inheriting from BasicMenuItem.
///
/// Used to play a sound related to the action performed.
///
/// A ``new()`` method is available as a Builder pattern for situations where the result you desire does not have a method.
#[repr(C)]
#[bitfield]
pub struct BasicMenuResult {
    pub close_this: bool,
    pub close_parent: bool,
    pub close_all: bool,
    pub delete_this: bool,

    pub delete_parent: bool,
    pub delete_all: bool,
    #[skip]
    __: bool,
    #[skip(getters)]
    pub se_decide: bool,

    #[skip(getters)]
    pub se_decide2: bool,
    #[skip(getters)]
    pub se_cancel: bool,
    #[skip]
    __: bool,

    #[skip(getters)]
    pub se_miss: bool,
    #[skip(getters)]
    pub se_cursor: bool,
    pub do_nothing: bool,
    #[skip]
    padding: B2,
}

impl BasicMenuResult {
    pub fn se_cursor() -> Self {
        Self::new().with_se_cursor(true)
    }

    pub fn se_decide() -> Self {
        Self::new().with_se_decide(true)
    }

    pub fn close_decide() -> Self {
        Self::new().with_close_this(true).with_se_decide(true)
    }

    pub fn close_parent_decide() -> Self {
        Self::new().with_close_parent(true).with_se_decide(true)
    }

    pub fn delete_decide() -> Self {
        Self::new().with_delete_this(true).with_se_decide(true)
    }
}

#[repr(C)]
pub enum BasicMenuItemAttribute {
    Enable = 1,
    Disable = 2,
    Hide = 4,
    Blank = 8,
    Select = 16,
}
