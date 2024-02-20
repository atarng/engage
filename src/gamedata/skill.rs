pub use unity::prelude::*;
use unity::il2cpp::object::Array;
use unity::system::List;
use super::*;
use std::ops::Deref;
// Structs, methods required for SkillArray and SkillData

#[unity::class("App", "SkillArray")]
pub struct SkillArray {}

#[unity::class("App", "SkillData")]
pub struct SkillData {
    pub parent: StructBaseFields,
    pub commands: *const u8,  // from the calculator part
    pub sid: &'static Il2CppString,
    pub name: Option<&'static Il2CppString>,
}
impl Gamedata for SkillData{}

impl SkillArray {
    pub fn clear(&self){ unsafe { skillarray_clear(self, None); } }
    pub fn copy(&self, src: &SkillArray) { unsafe { skillarray_copy(self, src, None); }}
    pub fn ctor(&self, src: &SkillArray) { unsafe { skillarray_ctor(self, src, None); }}
    pub fn find_sid(&self, sid: &Il2CppString) -> Option<&'static SkillData> { unsafe { skillarray_find(self, sid, None)}}
    pub fn remove_sid(&self, sid: &Il2CppString) -> bool { unsafe { skill_array_remove(self, sid, None)}}
    pub fn skill_array_add(&self, add: &SkillArray) -> bool { unsafe { add_skill_array(self, add, None)}}
}

impl SkillData {
    pub fn get_efficacy(&self) -> i32 { unsafe { skilldata_get_efficacy(self, None)} }
    pub fn get_efficacy_value(&self) -> i32 { unsafe { skilldata_get_efficacy_value(self, None)}}
}

//Skill Array
#[unity::from_offset("App","SkillArray", "Clear")]
fn skillarray_clear(this: &SkillArray, method_info: OptionalMethod);

#[skyline::from_offset(0x0247dda0)]
fn skillarray_ctor(this: &SkillArray, src: &SkillArray, method_info: OptionalMethod);

#[unity::from_offset("App","SkillArray", "Copy")]
fn skillarray_copy(this: &SkillArray, src: &SkillArray, method_info: OptionalMethod);

#[skyline::from_offset(0x02482850)]
fn skill_array_remove(this: &SkillArray, sid: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x24820c0)]
fn add_skill_array(this: &SkillArray, src: &SkillArray, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02487990)]
fn skillarray_find(this: &SkillArray, sid: &Il2CppString, method_info: OptionalMethod) -> Option<&'static SkillData>;

//SkillData
#[unity::from_offset("App", "SkillData", "get_Efficacy")]
fn skilldata_get_efficacy(this: &SkillData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "SkillData", "get_EfficacyValue")]
fn skilldata_get_efficacy_value(this: &SkillData, method_info: OptionalMethod) -> i32;