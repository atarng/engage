//! Structures and wrappers to display various Dialogs to the player. Tied to the [`ProcInst`](crate::proc::ProcInst) system.

use unity::prelude::*;

use crate::menu::{BasicMenu, BasicMenuItem};

pub mod yesno;

#[repr(C)]
pub struct BasicDialog<T: 'static> {
    parent: BasicMenu<T>,
    dialog_content: *const u8,
    bind_bg: bool,
    not_bind_bg: bool,
}

#[repr(C)]
#[unity::class("App", "BasicDialogItem")]
pub struct BasicDialogItem {
    pub parent: BasicMenuItem,
    text: &'static Il2CppString,
}

impl BasicDialogItem {
    pub fn new(text: impl AsRef<str>) -> &'static mut Il2CppObject<BasicDialogItem> {
        let item = BasicDialogItem::instantiate().unwrap();
        unsafe {
            dialog_item_ctor(item, text.into(), None);
        }
        item
    }
}

#[skyline::from_offset(0x2455f30)]
fn dialog_item_ctor(proc: &Il2CppObject<BasicDialogItem>, text: &Il2CppString, method_info: OptionalMethod);
