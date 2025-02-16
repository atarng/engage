pub use unity::{il2cpp::object::Array, prelude::*};
use super::{*, person::CapabilitySbyte};
// Structs, methods required for SkillArray and SkillData
pub struct SkillArrayEntity {
    pub value : u32,
}

#[unity::class("App", "SkillArrayList")]
pub struct SkillArrayEntityList {
    pub item: &'static mut Array<SkillArrayEntity>,
    pub size: i32,
    pub version: i32,
    sync_root: *const u8,
}

impl Deref for SkillArrayEntityListFields {
    type Target = [SkillArrayEntity];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.item.m_items.as_ptr(), self.size as usize) }
    }
}

impl DerefMut for SkillArrayEntityListFields  {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.item.m_items.as_mut_ptr(), self.size as usize) }
    }
}


#[unity::class("App", "SkillArray")]
pub struct SkillArray {
    mask: &'static Array<u8>,
    pub list: &'static mut SkillArrayEntityList,
    pub flags: i64,
    pub cycles: i32,
    pub timing: i32,
    pub efficacys: i32,
    pub efficacy_ignore: i32,
    pub bad_states: i32,
    pub bad_ignore: i32,
}
impl Deref for SkillArrayFields {
    type Target = [SkillArrayEntity];
    fn deref(&self) -> &Self::Target {
        &self.list.item
    }
}

#[unity::class("App", "SkillData")]
pub struct SkillData {
    pub parent: StructBaseFields,
    pub commands: *const u8,  // from the calculator part
    pub sid: &'static Il2CppString,
    pub name: Option<&'static Il2CppString>,
    pub help: Option<&'static Il2CppString>,
    pub command_name: Option<&'static Il2CppString>,
    pub command_help: Option<&'static Il2CppString>,
    pub command_warning: Option<&'static Il2CppString>,
    pub root_command_sid: Option<&'static Il2CppString>,
    pub icon_type: i64,
    pub icon_label: Option<&'static Il2CppString>,
    pub icon_bmap: Option<&'static Il2CppString>,
    pub stand: i32,
    pub action: i32,
    pub timing: i32,
    pub target: i32,
    pub equip_iids: Option<&'static Array<&'static Il2CppString>>,
}
impl Gamedata for SkillData{}

impl SkillArray {
    pub fn clear(&self){ unsafe { skillarray_clear(self, None); } }
    pub fn copy(&self, src: &SkillArray) { unsafe { skillarray_copy(self, src, None); }}
    pub fn ctor(&self, src: &SkillArray) { unsafe { skillarray_ctor(self, src, None); }}
    pub fn add_skill(&self, skill: &SkillData, category: i32, age: i32) -> bool { unsafe { skill_array_add_skill(self, skill, category, age, None) } }
    pub fn add_sid(&self, sid: &str, category: i32, age: i32) -> bool {
        if let Some(skill) = SkillData::get(sid) { self.add_skill(skill, category, age) }
        else {false }
    }
    pub fn get_category(&self, index: i32) -> i32 { unsafe { skill_array_get_category(self, index, None) }}
    pub fn find_sid(&self, sid: &Il2CppString) -> Option<&'static SkillData> { unsafe { skillarray_find(self, sid, None)}}
    pub fn remove_skill(&self, skill: &SkillData) -> bool { unsafe { skill_array_remove_skill(self, skill, None)}}
    pub fn remove_sid(&self, sid: &Il2CppString) -> bool { unsafe { skill_array_remove(self, sid, None)}}
    pub fn replace(&self, index: i32, skill: &SkillData, category: i32 ) { unsafe { skill_array_replace(self, index, skill, category, None); }}
    pub fn skill_array_add(&self, add: &SkillArray) -> bool { unsafe { add_skill_array(self, add, None)}}
    pub fn index_of(&self, sid: &Il2CppString) -> i32 { unsafe { sid_index_of(self, sid, None)}}

    pub fn replace_sid(&self, sid: &Il2CppString, skill: &SkillData) {
        let index = self.index_of(sid);
        if index == -1 { return; }
        let category = self.get_category(index);
        self.replace(index, skill, category);
    }
}

impl SkillData {
    pub fn ctor(&self) { unsafe { skilldata_ctor(self, None); }}
    pub fn can_override_skill(&self) -> bool { unsafe {skilldata_can_override_skill(self, None)}}
    pub fn get_efficacy(&self) -> i32 { unsafe { skilldata_get_efficacy(self, None)} }
    pub fn get_efficacy_value(&self) -> i32 { unsafe { skilldata_get_efficacy_value(self, None)}}
    pub fn set_efficacy_value(&self, value: i32) { unsafe { skilldata_set_efficacy_value(self, value, None); }}
    pub fn get_enchance_value(&self) -> &'static CapabilitySbyte { unsafe { skilldata_get_enhance_value(self, None)}}
    pub fn get_flag(&self) -> i64{ unsafe { skilldata_get_flag(self, None)}}
    pub fn set_flag(&self, value: i64) { unsafe { skilldata_set_flag(self, value, None); }}
    pub fn get_inheritance_cost(&self) -> u16 { unsafe { skilldata_get_inherit_cost(self, None)}}
    pub fn get_inheritance_sort(&self) -> u16 { unsafe { skilldata_get_inherit_sort(self, None)}}
    pub fn get_range_target(&self) -> i32 { unsafe { skilldata_get_range_target(self, None)}}
    pub fn get_range_o(&self) -> i32 { unsafe { skilldata_getrange_o(self, None)}}
    pub fn get_weapon_prohibit(&mut self) -> &'static mut WeaponMask { unsafe { skilldata_weapon_prohibit(self, None)} }
    pub fn set_inherit_cost(&self, value: u16) { unsafe { skilldata_set_inherit_cost(self, value, None); }}
    pub fn set_range_target(&self, value: i32) { unsafe {skilldata_set_range_target(self, value, None); }}
    pub fn set_range_add(&self, value: i32) { unsafe {skilldata_set_range_add(self, value, None); }}
    pub fn get_style_skill(&mut self, style: i32) -> Option<&'static mut SkillData> { unsafe { skilldata_get_style_skill(self, style, None)}}
    pub fn get_priority(&self) -> i32 { unsafe { skilldata_priority(self, None) }}
    pub fn set_priority(&self, value: i32)  { unsafe { skilldata_set_priority(self, value, None); }}
    pub fn has_effect(&self) -> bool { unsafe { skilldata_has_effect(self, None)}}
    pub fn is_style_skill(&self) -> bool { unsafe { skilldata_is_style(self, None)}}
    pub fn load() { unsafe { skilldata_load(None); }}
    pub fn on_complete(&self) { unsafe { skilldata_oncomplete(self, None);} }
    pub fn on_build(&self) { unsafe{ skilldata_on_build(self, None); }}
}

impl SkillArrayEntity {
    pub fn get_skill(&self) -> Option<&'static SkillData> { unsafe { skill_array_entity_get_skill(self, None)}}
    pub fn get_age(&self) -> i32 { unsafe { skill_array_entity_get_age(self, None) }}
    pub fn get_category(&self) -> i32 { unsafe { skill_array_entity_get_category(self, None)}}
    pub fn set_index(&self, value: i32) { unsafe { skill_array_entity_set_index(self, value, None); } }
}
// Skill Array Entity Methods ()
#[skyline::from_offset(0x01d6e120)]
fn skill_array_entity_get_skill(this: &SkillArrayEntity, method_info: OptionalMethod) -> Option<&'static SkillData>;

#[skyline::from_offset(0x01d6e0e0)]
fn skill_array_entity_get_age(this: &SkillArrayEntity, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01d6e100)]
fn skill_array_entity_get_category(this: &SkillArrayEntity, method_info: OptionalMethod) -> i32;
#[skyline::from_offset(0x01d6e0b0)]
fn skill_array_entity_set_index(this: &SkillArrayEntity, index: i32, method_info: OptionalMethod);

//Skill Array
#[unity::from_offset("App","SkillArray", "Clear")]
fn skillarray_clear(this: &SkillArray, method_info: OptionalMethod);

#[skyline::from_offset(0x0247dda0)]
fn skillarray_ctor(this: &SkillArray, src: &SkillArray, method_info: OptionalMethod);

#[unity::from_offset("App","SkillArray", "Copy")]
fn skillarray_copy(this: &SkillArray, src: &SkillArray, method_info: OptionalMethod);

#[skyline::from_offset(0x02482850)]
fn skill_array_remove(this: &SkillArray, sid: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02483080)]
fn skill_array_remove_skill(this: &SkillArray, skill: &SkillData, method_info: OptionalMethod) -> bool;
#[skyline::from_offset(0x024877e0)]
fn skill_array_get_category(this: &SkillArray, index: i32, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x24820c0)]
fn add_skill_array(this: &SkillArray, src: &SkillArray, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02480750)]
fn skill_array_add_skill(this: &SkillArray, skill: &SkillData, category: i32, age: i32, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02487990)]
fn skillarray_find(this: &SkillArray, sid: &Il2CppString, method_info: OptionalMethod) -> Option<&'static SkillData>;

#[skyline::from_offset(0x02487020)]
fn sid_index_of(this: &SkillArray, sid: &Il2CppString, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x02483760)]
fn skill_array_replace(this: &SkillArray, index: i32, skill: &SkillData, category: i32, method_info: OptionalMethod);
//SkillData
#[unity::from_offset("App", "SkillData", "get_Efficacy")]
fn skilldata_get_efficacy(this: &SkillData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "SkillData", "get_EfficacyValue")]
fn skilldata_get_efficacy_value(this: &SkillData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "SkillData", "get_Flag")]
fn skilldata_get_flag(this: &SkillData, method_info: OptionalMethod) -> i64;

#[unity::from_offset("App", "SkillData", "set_Flag")]
fn skilldata_set_flag(this: &SkillData, value: i64, method_info: OptionalMethod);

#[unity::from_offset("App", "SkillData", "IsStyleSkill")]
fn skilldata_is_style(this: &SkillData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "SkillData", "CanOverrideSkill")]
fn skilldata_can_override_skill(this: &SkillData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "SkillData", "HasEffect")]
fn skilldata_has_effect(this: &SkillData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "SkillData", "get_Priority")]
fn skilldata_priority(this: &SkillData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "SkillData", "set_Priority")]
fn skilldata_set_priority(this: &SkillData, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "SkillData", "get_WeaponProhibit")]
fn skilldata_weapon_prohibit(this: &SkillData, method_info: OptionalMethod) -> &'static mut WeaponMask;

#[unity::from_offset("App", "SkillData", "get_InheritanceCost")]
fn skilldata_get_inherit_cost(this: &SkillData, method_info: OptionalMethod) -> u16;

#[unity::from_offset("App", "SkillData", "get_InheritanceSort")]
fn skilldata_get_inherit_sort(this: &SkillData, method_info: OptionalMethod) -> u16;

#[unity::from_offset("App", "SkillData", "set_InheritanceCost")]
fn skilldata_set_inherit_cost(this: &SkillData, value: u16, method_info: OptionalMethod);

#[unity::from_offset("App", "SkillData", "get_EnhanceValue")]
fn skilldata_get_enhance_value(this: &SkillData, method_info: OptionalMethod) -> &'static CapabilitySbyte;

#[unity::from_offset("App", "SkillData", "set_RangeTarget")]
fn skilldata_set_range_target(this: &SkillData, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "SkillData", "get_RangeTarget")]
fn skilldata_get_range_target(this: &SkillData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "SkillData", "get_RangeO")]
fn skilldata_getrange_o(this: &SkillData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "SkillData", "set_RangeAdd")]
fn skilldata_set_range_add(this: &SkillData, range: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "SkillData", "Load")]
fn skilldata_load(method_info: OptionalMethod);

#[unity::from_offset("App", "SkillData", ".ctor")]
fn skilldata_ctor(this: &SkillData, method_info: OptionalMethod);

#[unity::from_offset("App", "SkillData", "OnBuild")]
fn skilldata_on_build(this: &SkillData, method_info: OptionalMethod);

#[unity::from_offset("App", "SkillData", "OnCompleted")]
fn skilldata_oncomplete(this: &SkillData,method_info: OptionalMethod);

#[unity::from_offset("App", "SkillData", "set_EfficacyValue")]
fn skilldata_set_efficacy_value(this: &SkillData, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "SkillData", "GetStyleSkill")]
fn skilldata_get_style_skill(this: &SkillData, ty: i32, method_info: OptionalMethod) -> Option<&'static mut SkillData>;