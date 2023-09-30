use unity::prelude::*;

use crate::proc::{desc::ProcDesc, ProcInst, ProcInstFields, Bindable};

#[repr(C)]
#[unity::class("App", "HubRefineShopSequence")]
pub struct HubRefineShopSequence {
    proc: ProcInstFields,
    // ...
}

impl Bindable for HubRefineShopSequence { }


#[repr(C)]
#[unity::class("App", "SortieSequenceItemShop")]
pub struct SortieSequenceItemShop {
    base: HubRefineShopSequenceFields,
}

impl Bindable for SortieSequenceItemShop { }

impl SortieSequenceItemShop {
    pub fn new() -> &'static mut Self {
        let item = il2cpp::instantiate_class(Self::class_mut().clone()).unwrap();

        unsafe {
            hubrefineshopsequence_ctor(item, None);
        }

        item
    }

    pub fn create_desc(&self) -> &'static mut Il2CppArray<&'static mut ProcDesc> {
        unsafe { hubrefineshopsequence_createdesc(self, None) }
    }
}

#[unity::from_offset("App", "SortieSequenceItemShop", "CreateBind")]
pub fn sortiesequenceitemshop_createbind(proc: &ProcInst, method_info: OptionalMethod);

#[unity::from_offset("App", "HubAccessoryRoom", "CreateBind")]
pub fn hubaccessoryshopsequence_createbind(proc: &ProcInst, shop: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x23dccb0)]
pub fn hubrefineshopsequence_createbind(proc: &ProcInst, method_info: OptionalMethod);

#[unity::from_offset("App", "HubRefineShopSequence", ".ctor")]
pub fn hubrefineshopsequence_ctor<P: Bindable>(this: &P, method_info: OptionalMethod);

#[unity::from_offset("App", "HubRefineShopSequence", "CreateDesc")]
pub fn hubrefineshopsequence_createdesc<P: Bindable>(
    this: &P,
    method_info: OptionalMethod,
) -> &'static mut Il2CppArray<&'static mut ProcDesc>;
