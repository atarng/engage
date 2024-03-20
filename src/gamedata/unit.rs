use unity::prelude::*;
use unity::il2cpp::object::*;
use crate::force::Force;
use crate::random::*;
use super::{JobData, WeaponMask, PersonData, 
    item::{UnitItemList, ItemData}, 
    person::{CapabilitySbyte, Capability}, 
    skill::{SkillData, SkillArray},
    GodData,
};

#[unity::class("App", "GodUnit")]
pub struct GodUnit {
    parent: [u8; 0x10],
    pub m_Data: &'static GodData,
}

#[unity::class("App", "UnitRing")]
pub struct UnitRing { }

#[unity::class("App","UnitEdit")]
pub struct UnitEdit {
    pub name : Option<&'static Il2CppString>,
    pub morphname: Option<&'static Il2CppString>,
    pub gender: i32,
    pub langague: i32,
    pub BirthMonth: u8,
    pub BirthDay: u8,
}

#[unity::class("App", "Unit_Status")]
pub struct UnitStatus {
    pub value : u64,
}

#[unity::class("App", "UnitBaseCapability")]
pub struct UnitBaseCapability {
    pub capability: &'static mut Array<i8>,
}

#[unity::class("App", "Unit")]
pub struct Unit {
    pub status: &'static UnitStatus,
    pub prev: Option<&'static Unit>,
    pub next: Option<&'static Unit>,
    ai: &'static (),
    pub edit: &'static UnitEdit,
    pub ident: i32,
    pub person: &'static mut PersonData,
    pub m_Job : &'static mut JobData,
    pub m_Force : Option<&'static Force>,
    pub m_BaseCapability : &'static mut UnitBaseCapability,
    pub m_GrowCapability: &'static mut Capability,
    pub m_LevelCapability: &'static mut UnitBaseCapability,
    pub m_GrowSeed :i32,
    m_DropSeed :i32,
    m_Actor :u64,
    m_Info :u64,
    m_Index :u8,
    pub m_Level :u8,
    pub m_Exp :u8,
    pub hp_value: u8,
    pub hp_display: u8,
    pub m_HpStockCount :u8,
    pub m_HpStockCountMax :u8,
    m_ExtraHpStockCount :u8,
    m_ExtraHpStockCountMax :u8,
    m_EngageCount :u8,
    m_EngageTurn :u8,
    m_EngageCountView :u8,
    m_GodStates :u64,
    m_X :u8,
    m_Z :u8,
    m_DisposX :u8,
    m_DisposZ :u8,
    m_Angle :f32,
    m_DontAttackPerson :u64,
    m_DontAttackForceMask :i32,
    pub m_ItemList : &'static UnitItemList,
    m_ItemSelected :u64,
    m_AccessoryList :u64,
    pub m_GodUnit :Option<&'static GodUnit>,
    pub m_GodLink :Option<&'static GodUnit>,
    pub m_Ring :Option<&'static UnitRing>,
    m_ExtraSight :i32,
    pub m_MoveDistance :i32,
    pub m_MaskSkill : Option<&'static SkillArray>,
    pub m_EquipSkill :&'static SkillArray,
    pub m_PrivateSkill :&'static SkillArray,
    pub m_ReceiveSkill :&'static SkillArray,
    pub m_SupportedSkill :&'static SkillArray,
    pub m_EquipSkillPool :&'static SkillArray,
    pub m_LearnedJobSkill :&'static SkillArray,
    pub m_OriginalAptitude : &'static mut WeaponMask,
    pub m_Aptitude : &'static mut WeaponMask,
    pub m_WeaponMask :&'static mut WeaponMask,
    pub m_SelectedWeaponMask :&'static mut WeaponMask,
    m_EnhanceFactors :u64,
    m_EnhanceCalculator :u64,
    pub m_InternalLevel :i8,
    m_LastPickVoice :u8,
    m_AttackImage :u64,
    m_RodImage :u64,
    m_HealImage :u64,
    m_SupportImage :u64,
    m_InterferenceImage :u64,
    m_EngageImage :u64,
    m_MoveImage :u64,
    m_Record :u64,
    m_MapHistoryIndex :u8,
    m_MaskSkillLock :u64,
    m_FortuneTarget :u64,
    m_FortuneSeed :i32,
    m_RelayPlayerIndex :u8,
    m_SkillPoint :u8,
    m_OwnerUnit :u16,
    m_LockTargetX :u8,
    m_LockTargetZ :u8,
    _TotalOrder_k__BackingField :i32,
    _TotalAction_k__BackingField :i32,
    _TotalAttack_k__BackingField :i32,
    _TotalDamage_k__BackingField :i32,
    _TotalResult_k__BackingField :i32,
    _SideType_k__BackingField :i32,
    _BattleTemporary_k__BackingField :i32,
    m_CalcInfo :i32
}

impl Unit {
    /// Learn the skill for a job regardless of the unit's level.
    pub fn learn_job_skill(&self, job: &JobData) {
        unsafe {  unit_learnjobskill(self, job, None); }
    }
    /// Performs a class change on the unit without playing the sequence
    pub fn class_change(&self, job: &JobData) {
        unsafe { unit_classchange(self, job, 0 as _, None);  }
    }

    /// Add item to the unit's inventory without a notification
    pub fn add_item(&self, item: &ItemData) {
        unsafe { unit_itemadd(self, item, None); }
    }

    pub fn set_selected_weapon(&self, weapon_mask: &WeaponMask) {
        unsafe { unit_setselectedweapon(self, weapon_mask, None); }
    }

    pub fn get_job(&self) -> &'static JobData {
        unsafe { unit_get_job(self, None) }
    }

    pub fn is_engaging(&self) -> bool {
        unsafe { unit_is_engaging(self, None) }
    }

    pub fn is_engage_owner(&self) -> bool {
        unsafe { unit_is_engage_owner(self, None) }
    }
    pub fn is_enchantment(&self) -> bool {
        unsafe { unit_is_enchantment(self, None) }
    }
    // Getters 
    pub fn get_aptitude(&self) -> &'static mut WeaponMask { unsafe { get_unit_aptitude(self, None) } }
    pub fn get_capability(&self, index: i32, enhance: bool) -> i32 { unsafe { unit_getcapability(self, index, enhance, None)} }
    pub fn get_capability_grow(&self, index: i32, auto_level: bool) -> i32 { unsafe { unit_get_capability_grow(self, index, auto_level, None)}}
    pub fn get_enchanced_level(&self) -> i32 { unsafe { unit_get_enhance_level(self, None)}}
    pub fn get_hp(&self) -> i32 { unsafe { unit_get_hp(self, None) } }
    pub fn get_learn_job_skill(&self) -> Option<&SkillData> { unsafe { learn_job_kill_unit(self, None)}}
    pub fn get_pid(&self) -> &'static Il2CppString {  unsafe { unit_get_pid(self, None)} }
    pub fn get_person(&self) -> &'static PersonData { unsafe { unit_get_person(self, None)}}
    // Setters
    pub fn set_base_capability(&self, index: i32, value: i32) { unsafe { unit_set_base_capability(self, index, value, None);}}
    pub fn set_exp(&self, exp: i32){  unsafe { unit_set_exp(self, exp, None); }  }
    pub fn set_hp(&self, HP: i32) {  unsafe { unit_set_Hp(self, HP, None); } }
    pub fn set_internal_level(&self, internal: i32) {  unsafe {unit_set_internal_level(self, internal, None); }  }
    pub fn set_job(&self, job: &JobData) { unsafe { unit_set_job(self, job, None); } }
    pub fn set_level(&self, level: i32){  unsafe {  unit_set_level(self, level, None);} }
    pub fn set_person(&self, person: &PersonData) { unsafe { unit_set_person(self, person, None); }}
    pub fn set_select_weapon_from_original_aptitude(&self, mask: &WeaponMask) { unsafe {  unit_set_select_weapon_from_original_aptitude(self, mask, None); } } 
    pub fn set_sp(&self, sp: i32) { unsafe { unit_set_sp(self, sp, None); } }
    pub fn set_god_unit(&self, god: &GodUnit) { unsafe { unit_set_god_unit(self, god, None); } }
    pub fn set_status(&self, status: i64) { unsafe { unit_set_status(self, status, None); }}

    // Others
    pub fn add_aptitude_from_weapon_mask(&self) { unsafe { unit_add_apt_from_weapon_mask(self, None); } } 
    pub fn add_sp(&self, added_sp: i32) {  unsafe { unit_add_sp(self, added_sp, None); }  }
    pub fn add_item_iid(&self, iid: &Il2CppString) -> i32 { unsafe { unit_add_item_iid(self, iid, None)}}
    pub fn add_item_iids(&self, iids: &Array<&Il2CppString>) -> bool { unsafe { unit_add_item_list(self, iids, None )}}

    //Creation of Unit
    pub fn create(&self, person: &PersonData, job: &JobData, level: i32, rng: &Random) { unsafe { unit_create(self, person, job, level, rng, None); }}
    pub fn create_impl1(&self, person: &PersonData, job: &JobData, level: i32, rng: &Random) { unsafe { unit_create_impl_1(self, person, job, level, rng, None); }}

    // skill check (checks mask skill)
    pub fn has_private_skill(&self, sid: &Il2CppString) -> bool { unsafe { unit_has_private_skill(self, sid, None) } }
    pub fn has_sid(&self, sid: &Il2CppString) -> bool { unsafe { unit_has_skill_sid(self, sid, None) }}
    pub fn has_skill(&self, skill: &SkillData) -> bool { unsafe { unit_has_skill(self, skill, None)}}

    pub fn auto_grow_capability(&self, level: i32, target_level: i32) { unsafe { unit_auto_grow_cap(self, level, target_level, None); }}
    pub fn level_up(&self, num_min_stats: i32) { unsafe { unit_level_up(self, num_min_stats, None); } }
    pub fn level_down(&self) { unsafe { unit_level_down(self, None); }}
    
    pub fn put_off_all_item(&self) { unsafe { unit_item_put_off_all(self, None); }}
    pub fn set_weapon_mask_from_person(&self) { unsafe { unit_set_weapon_mask_from_person(self, None); }}

    pub fn transfer(&self, force: i32, is_last: bool) { unsafe { unit_transfer(self, force, is_last, None); }}
    pub fn try_create_actor(&self) -> bool { unsafe { unit_try_create_actor(self, None) } }
    pub fn update_weapon_mask(&self) { unsafe { unit_update_weapon_mask(self, None); }}
}

impl UnitEdit {
    pub fn set_gender(&self, gender: i32) { unsafe { unit_edit_set_gender(self, gender, None);}}
    pub fn set_name(&self, name: &Il2CppString) { unsafe { unit_edit_set_name(self, name, None); }}
}

impl GodUnit {
    pub fn get_escape(&self) -> bool { unsafe { god_unit_escaped(self, None)}}
}


#[skyline::from_offset(0x1a3f400)]
extern "C" fn unit_itemadd(this: &Unit, item: &ItemData, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "ClassChange")]
extern "C" fn unit_classchange(this: &Unit, job: &JobData, item: *const u8, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "LearnJobSkill")]
extern "C" fn unit_learnjobskill(this: &Unit, job: &JobData, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "set_Level")]
extern fn unit_set_level(this: &Unit, level: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "get_Job")]
extern "C" fn unit_get_job(this: &Unit, method_info: OptionalMethod) -> &'static JobData;

#[unity::from_offset("App", "Unit", "get_Pid")]
extern fn unit_get_pid(this: &Unit,  method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::from_offset("App", "Unit", "SetSelectedWeapon")]
extern "C" fn unit_setselectedweapon(this: &Unit, weapon_mask: &WeaponMask, method_info: OptionalMethod);

// App.Unit$$IsEngaging	7101a265e0	bool App.Unit$$IsEngaging(App_Unit_o * __this, MethodInfo * method)	96
#[skyline::from_offset(0x1a265e0)]
extern "C" fn unit_is_engaging(this: &Unit, method_info: OptionalMethod) -> bool;

// App.Unit$$IsEngageOwner	7101a197a0	bool App.Unit$$IsEngageOwner(App_Unit_o * __this, MethodInfo * method)	112
#[skyline::from_offset(0x1a197a0)]
extern "C" fn unit_is_engage_owner(this: &Unit, method_info: OptionalMethod) -> bool;

//triabolical added
#[unity::from_offset("App", "Unit", "AddSkillPoint")]
fn unit_add_sp(this: &Unit, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App","Unit", "set_Hp")]
fn unit_set_Hp(this: &Unit, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "set_Exp")]
fn unit_set_exp(this: &Unit, exp: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "GetEnhancedLevel")]
fn unit_get_enhanced_level(this: &Unit, method_info: OptionalMethod) ->i32;

#[unity::from_offset("App", "Unit", "set_InternalLevel")]
fn unit_set_internal_level(this: &Unit, level: i32, method_info: OptionalMethod);

#[unity::from_offset("App","Unit", "get_Hp")]
fn unit_get_hp(this: &Unit, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01a5ba20)]
fn unit_getcapability(this: &Unit, type_: i32, calcEnhance: bool, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "set_SkillPoint")]
fn unit_set_sp(this: &Unit, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x01a378b0)]
fn unit_has_private_skill(this: &Unit, sid: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01a3a040)]
fn unit_level_up(this: &Unit, min_stats: i32, method_info: OptionalMethod);

#[unity::from_offset("App","Unit", "SetWeaponMaskFromParson")]
fn unit_set_weapon_mask_from_person(this: &Unit, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "get_Aptitude")]
fn get_unit_aptitude(this: &Unit, method_info: OptionalMethod) -> &'static mut WeaponMask;

#[skyline::from_offset(0x01a3aba0)]
fn unit_level_down(this: &Unit, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "CreateImpl1")]
fn unit_create_impl_1(this: &Unit, person: &PersonData, job: &JobData, level: i32, random: &Random, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "Create")]
fn unit_create(this: &Unit, person: &PersonData, job: &JobData, level: i32, random: &Random, method_info: OptionalMethod);

#[skyline::from_offset(0x01a3c290)]
fn learn_job_kill_unit(this: &Unit, method_info: OptionalMethod) -> Option<&SkillData>;

#[unity::from_offset("App", "Unit", "ItemPutOffAll")]
fn unit_item_put_off_all(this: &Unit, method_info: OptionalMethod);

#[skyline::from_offset(0x01a3f480)]
fn unit_add_item_list(this: &Unit, iid: &Array<&Il2CppString>, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01a0c990)]
fn unit_add_item_iid(this: &Unit, iid: &Il2CppString, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "set_Job")]
fn unit_set_job(this: &Unit, value: &JobData, method_info: OptionalMethod);

#[skyline::from_offset(0x01a35520)]
fn unit_has_skill(this: &Unit, skill: &SkillData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01a35540)]
fn unit_has_skill_sid(this: &Unit, sid: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01a23a40)]
fn unit_is_enchantment(this: &Unit, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "Unit", "SetBaseCapability")]
fn unit_set_base_capability(this: &Unit, index: i32, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x01a2ff20)]
fn unit_get_capability_grow(this: &Unit, index: i32, is_auto_grow: bool, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "GetEnhancedLevel")]
fn unit_get_enhance_level(this: &Unit, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "Transfer")]
fn unit_transfer(this: &Unit, force: i32, is_last: bool, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "TryCreateActor")]
fn unit_try_create_actor(this: &Unit, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "Unit", "SetGodUnit")]
fn unit_set_god_unit(this: &Unit, god: &GodUnit, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "SetStatus")]
fn unit_set_status(this: &Unit, status: i64, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "UpdateWeaponMask")]
fn unit_update_weapon_mask(this: &Unit, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "set_Person")]
fn unit_set_person(this: &Unit, person: &PersonData, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "SetSelectedWeaponFromOriginalAptitude")]
fn unit_set_select_weapon_from_original_aptitude(this: &Unit, mask: &WeaponMask, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "AddAptitudeFromWeaponMask")]
fn unit_add_apt_from_weapon_mask(this: &Unit, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "get_Person")]
fn unit_get_person(this: &Unit, method_info: OptionalMethod) -> &'static PersonData;

#[skyline::from_offset(0x01a0b1b0)]
fn unit_auto_grow_cap(this: &Unit, level: i32, target_level: i32, method_info: OptionalMethod);

// UnitEdit 
#[skyline::from_offset(0x01f73e50)]
fn unit_edit_set_gender(this: &UnitEdit, gender: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitEdit", "SetName")]
fn unit_edit_set_name(this: &UnitEdit, name: &Il2CppString, method_info: OptionalMethod);


// God Unit
#[skyline::from_offset(0x0233eae0)]
pub fn god_unit_escaped(this: &GodUnit, method_info: OptionalMethod) -> bool;

