pub use unity::{prelude::*, system::{List, Dictionary}, il2cpp::object::Array};
use crate::gamedata::{*, item::ItemData, skill::SkillArray, WeaponMask};
use super::GodData;

impl GodData {
    pub fn get_gid(&self) -> Option<&'static Il2CppString> { unsafe { god_data_get_gid(self, None) }}
    pub fn get_engage_attack(&self) -> &'static Il2CppString { unsafe { goddata_get_engage_attack(self, None) }}
    pub fn get_engrave_avoid(&self) -> i8 { unsafe{ goddata_get_engrave_avoid(self, None) }}
    pub fn get_engage_attack_link(&self) -> Option<&'static Il2CppString> { unsafe { god_data_get_engage_link(self, None)}}
    pub fn get_engrave_critical(&self) -> i8 { unsafe{ goddata_get_engrave_critical(self, None) }}
    pub fn get_engrave_hit(&self) -> i8 { unsafe{ goddata_get_engrave_hit(self, None) }}
    pub fn get_engrave_power(&self) -> i8 { unsafe{ goddata_get_engrave_power(self, None) }}
    pub fn get_engrave_secure(&self) -> i8 { unsafe{ goddata_get_engrave_secure(self, None) }}
    pub fn get_engrave_weight(&self) -> i8 { unsafe{ goddata_get_engrave_weight(self, None) }}
    pub fn get_force_type(&self) -> i32 { unsafe {  god_data_force_type(self, None)}}
    pub fn get_link_gid(&self) -> Option<&'static Il2CppString> { unsafe { god_data_get_link_gid(self, None)}}
    pub fn get_link(&self) -> Option<&'static Il2CppString> { unsafe { god_data_get_link(self, None) }}
    pub fn get_flag(&self) -> &'static mut WeaponMask { unsafe { god_data_get_flag(self, None)}}
    pub fn get_grow_table(&self) -> Option<&'static Il2CppString> { unsafe { god_data_get_grow_table(self, None)}}
    pub fn load() { unsafe { goddata_load(None); }}
    pub fn on_complete(&self) { unsafe{ god_data_on_complete(self, None); }}

    pub fn set_engage_attack(&self, value: &Il2CppString) { unsafe { goddata_set_engage_attack(self, value, None); }}
    pub fn set_engage_attack_link(&self, value: &Il2CppString) { unsafe { god_data_set_engage_link(self, value, None); } }
    pub fn set_engrave_avoid(&self, value: i8) { unsafe{ goddata_set_engrave_avoid(self, value, None); }}
    pub fn set_engrave_critical(&self, value: i8) { unsafe{ goddata_set_engrave_critical(self, value, None); }}
    pub fn set_engrave_hit(&self, value: i8) { unsafe{ goddata_set_engrave_hit(self, value, None); }}
    pub fn set_engrave_power(&self, value: i8)  { unsafe{ goddata_set_engrave_power(self, value, None); }}
    pub fn set_engrave_secure(&self, value: i8)  { unsafe{ goddata_set_engrave_secure(self, value, None); }}
    pub fn set_engrave_weight(&self, value: i8) { unsafe{ goddata_set_engrave_weight(self, value, None); }}
    pub fn set_link_gid(&self, value: &Il2CppString) { unsafe { god_data_set_link_gid(self, value, None); }}
    pub fn set_link(&self, value: &Il2CppString) { unsafe { god_data_set_link(self, value, None); }}
    pub fn set_ascii_name(&self, value: &Il2CppString) { unsafe { god_data_set_ascii(self, value, None); }}
    pub fn set_engrave(&self, index: i32, value: i8){
        match index {
            0 => self.set_engrave_avoid(value),
            1 => self.set_engrave_critical(value),
            2 => self.set_engrave_hit(value),
            3 => self.set_engrave_power(value),
            4 => self.set_engrave_secure(value),
            5 => self.set_engrave_weight(value),
            _ => {},
        }
    }
    pub fn get_link_dictionary() -> &'static Dictionary<&'static Il2CppString, &'static GodData> {
        return Self::class().get_static_fields::<GodDataStaticFields>().link_dics;
    }
    pub fn try_get_link(person: &PersonData) -> Option<&'static GodData> {
        unsafe { goddata_try_get_link(person, None)}
    }

    pub fn get_level_data(&self) -> Option<&'static mut List<GodGrowthDataLevelData>> {
        if self.get_grow_table().is_none() { return None; }
        GodGrowthData::get_level_data(&self.get_grow_table().unwrap().get_string().unwrap())
    }
}

#[unity::class("App", "GodGrowthData")]
pub struct GodGrowthData {
    pub parent: StructDataArrayFields,
    pub ggid: &'static Il2CppString,
    pub level: i32, 
    __: i32,
    pub inheritance_skills: Option<&'static mut Array<&'static Il2CppString>>,	
    pub synchro_skills: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_skills: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_items: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_cooperations: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_horses: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_coverts: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_heavys: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_flys: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_magics: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_pranas: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_dragons: Option<&'static mut Array<&'static Il2CppString>>,
    pub aptitude: &'static mut WeaponMask,
}
impl GamedataArray for GodGrowthData {}

impl GodGrowthData {
    pub fn get_engage_skills(&self) -> &mut Array<&Il2CppString> { unsafe { ggd_get_engage_skills(self, None)}}
    pub fn get_inheritance_skills(&self) ->  Option<&mut Array<&Il2CppString>> { unsafe { ggd_set_inherit_skills(self, None)}}
    pub fn on_complete(&self) { unsafe { ggd_on_complete(self, None); }}
    pub fn on_complete_end() { unsafe { god_growth_on_completed_end(None, None); }}
    pub fn try_get_from_god_data(this: &GodData) -> Option<&'static mut List<GodGrowthData>> { unsafe { ggd_try_get_from_god(this, None)}}
    pub fn get_level_data(key: &str) -> Option<&'static mut List<GodGrowthDataLevelData>> {
        let level_list = Self::class().get_static_fields::<GodGrowthDataStaticFields>().level_list;
        let method = level_list.get_class().get_methods().iter()
        .find(|method| method.get_name() == Some(String::from("get_Item")))
        .unwrap();
        let get_keys = unsafe {
            std::mem::transmute::<_,
            extern "C" fn(&Dictionary<&'static Il2CppString, &'static mut List<GodGrowthDataLevelData>>, &Il2CppString, &MethodInfo)
             -> Option<&'static mut List<GodGrowthDataLevelData>>>(
                method.method_ptr,
            )
        };
        get_keys(level_list, key.into(), method)
    }
}

#[unity::class("App", "GodGrowthData.LevelData")]
pub struct GodGrowthDataLevelData {
    pub synchro_skills: &'static SkillArray,
    pub engaged_skills: &'static SkillArray,
    pub engage_skills: &'static SkillArray,
    pub style_items: &'static GodGrowthDataStyleItems,
    pub aptitude: &'static mut WeaponMask,
    pub flags: &'static WeaponMask,
}

pub struct GodGrowthDataStaticFields {
    pub level_list: &'static Dictionary<&'static Il2CppString, &'static mut List<GodGrowthDataLevelData>>,
}
pub struct GodDataStaticFields{
    pub link_dics: &'static Dictionary<&'static Il2CppString, &'static GodData>,
}

#[unity::class("App", "GodGrowthData.StyleItems")]
pub struct GodGrowthDataStyleItems {
    pub items: &'static Array<&'static List<ItemData>>,
    pub count: i32,
}

impl GodGrowthDataStyleItems {
    pub fn clear(&self){ unsafe { ggd_style_clear(self, None); }}
    pub fn add_item(&self, style: i32, item: &ItemData) { unsafe { ggd_style_add_item(self, style, item, None); }}
    pub fn get_items(&self, style: i32) -> &'static List<ItemData> { unsafe { ggd_get_style_items(self, style, None)}}
}

impl Deref for GodGrowthDataStyleItemsFields {
    type Target = Array<&'static List<ItemData>>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

#[unity::class("App", "GodBond")]
pub struct GodBond {
    pub god: &'static GodData,
    reliance_s: u64,
    pub pid: &'static Il2CppString,
    pub level: u8,
    __: u8,
    pub exp: u16,
}
impl GodBond {
    pub fn level_up(&self) { unsafe { level_up_bond(self, None);} }
}

#[skyline::from_offset(0x02b4dff0)]
fn level_up_bond(this: &GodBond, method_info: OptionalMethod);
// GodData
#[unity::from_offset("App", "GodData", "get_Gid")]
fn god_data_get_gid(this: &GodData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "GodData", "get_GrowTable")]
fn god_data_get_grow_table(this: &GodData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;
#[unity::from_offset("App", "GodData", "set_AsciiName")]
fn god_data_set_ascii(this: &GodData, value: &Il2CppString, method_info: OptionalMethod);
#[unity::from_offset("App", "GodData", "get_Flag")]
fn god_data_get_flag(this: &GodData, method_info: OptionalMethod) -> &'static mut WeaponMask;

#[unity::from_offset("App", "GodData", "get_Link")]
fn god_data_get_link(this: &GodData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "GodData", "get_EngageAttackLink")]
fn god_data_get_engage_link(this: &GodData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "GodData", "set_EngageAttackLink")]
fn god_data_set_engage_link(this: &GodData, value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "GodData", "set_LinkGid")]
fn god_data_set_link_gid(this: &GodData, value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "GodData", "get_LinkGid")]
fn god_data_get_link_gid(this: &GodData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "GodData", "get_ForceType")]
fn god_data_force_type(this: &GodData, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x0232e0e0)]
fn goddata_try_get_link(this: &PersonData, method_info: OptionalMethod) -> Option<&'static GodData>;

#[unity::from_offset("App","GodData","get_EngageAttack")]
fn goddata_get_engage_attack(this: &GodData, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::from_offset("App","GodData","get_EngraveAvoid")]
fn goddata_get_engrave_avoid(this: &GodData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App","GodData","get_EngraveCritical")]
fn goddata_get_engrave_critical(this: &GodData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App","GodData","get_EngraveHit")]
fn goddata_get_engrave_hit(this: &GodData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App","GodData","get_EngravePower")]
fn goddata_get_engrave_power(this: &GodData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App","GodData","get_EngraveSecure")]
fn goddata_get_engrave_secure(this: &GodData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App","GodData","get_EngraveWeight")]
fn goddata_get_engrave_weight(this: &GodData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App","GodData","set_EngageAttack")]
fn goddata_set_engage_attack(this: &GodData, value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","set_EngraveAvoid")]
fn goddata_set_engrave_avoid(this: &GodData, value: i8, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","set_EngraveCritical")]
fn goddata_set_engrave_critical(this: &GodData, value: i8, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","set_EngraveHit")]
fn goddata_set_engrave_hit(this: &GodData, value: i8, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","set_EngravePower")]
fn goddata_set_engrave_power(this: &GodData, value: i8, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","set_EngraveSecure")]
fn goddata_set_engrave_secure(this: &GodData, value: i8, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","set_EngraveWeight")]
fn goddata_set_engrave_weight(this: &GodData, value: i8, method_info: OptionalMethod);


#[unity::from_offset("App","GodData","Load")]
fn goddata_load(method_info: OptionalMethod);

#[unity::from_offset("App", "GodData", "OnCompleted")]
fn god_data_on_complete(this: &GodData, method_info: OptionalMethod);

// God Growth Data
#[unity::from_offset("App", "GodGrowthData", "get_EngageSkills")]
fn ggd_get_engage_skills(this: &GodGrowthData, method_info: OptionalMethod) -> &mut Array<&Il2CppString>;

#[unity::from_offset("App", "GodGrowthData", "get_InheritanceSkills")]
fn ggd_set_inherit_skills(this: &GodGrowthData, method_info: OptionalMethod) -> Option<&mut Array<&Il2CppString>>;

#[unity::from_offset("App", "GodGrowthData", "Load")]
fn ggd_load(method_info: OptionalMethod);

#[unity::from_offset("App", "GodGrowthData", "OnCompleted")]
fn ggd_on_complete(this: &GodGrowthData, method_info: OptionalMethod);

#[unity::from_offset("App", "GodGrowthData", "TryGetFromGodData")]
fn ggd_try_get_from_god(god: &GodData, method_info: OptionalMethod) -> Option<&'static mut List<GodGrowthData>>;

#[skyline::from_offset(0x02332320)]
fn god_growth_on_completed_end(this: Option<&GodGrowthData>, method_info: OptionalMethod);

// God Growth Data Style Item
#[skyline::from_offset(0x01cd8cd0)]
fn ggd_style_clear(this: &GodGrowthDataStyleItems, method_info: OptionalMethod);

#[skyline::from_offset(0x01cd8aa0)]
fn ggd_style_add_item(this: &GodGrowthDataStyleItems, style: i32, item: &ItemData, method_info: OptionalMethod);

#[skyline::from_offset(0x01cd8db0)]
fn ggd_get_style_items(this: &GodGrowthDataStyleItems, style: i32, method_info: OptionalMethod) -> &'static List<ItemData>;
#[unity::from_offset("App", "GodData", "set_Link")]
fn god_data_set_link(this: &GodData, value: &Il2CppString, method_info: OptionalMethod);
