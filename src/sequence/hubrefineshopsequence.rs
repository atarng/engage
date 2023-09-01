use unity::prelude::*;

use crate::proc::{desc::ProcDesc, ProcInst};

#[repr(C)]
#[unity::class("App", "HubRefineShopSequence")]
pub struct HubRefineShopSequence {}

#[repr(C)]
#[unity::class("App", "SortieSequenceItemShop")]
pub struct SortieSequenceItemShop {}

#[unity::from_offset("App", "SortieSequenceItemShop", "CreateBind")]
pub fn sortiesequenceitemshop_createbind(proc: &Il2CppObject<ProcInst>, method_info: OptionalMethod);

#[unity::from_offset("App", "HubAccessoryRoom", "CreateBind")]
pub fn hubaccessoryshopsequence_createbind(proc: &Il2CppObject<ProcInst>, shop: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x23dccb0)]
pub fn hubrefineshopsequence_createbind(proc: &Il2CppObject<ProcInst>, method_info: OptionalMethod);

#[unity::from_offset("App", "HubRefineShopSequence", ".ctor")]
pub fn hubrefineshopsequence_ctor(this: &Il2CppObject<ProcInst>, method_info: OptionalMethod);

#[unity::from_offset("App", "HubRefineShopSequence", "CreateDesc")]
pub fn hubrefineshopsequence_createdesc(
    this: &Il2CppObject<ProcInst>,
    method_info: OptionalMethod,
) -> &'static mut Il2CppArray<&'static mut Il2CppObject<ProcDesc>>;
