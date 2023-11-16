pub use unity::prelude::*;

use super::{JobData, WeaponMask, PersonData, item::ItemData};

#[unity::class("App", "GodUnit")]
pub struct GodUnit { }

#[unity::class("App", "UnitRing")]
pub struct UnitRing { }

#[unity::class("App", "Unit")]
pub struct Unit {
    status: &'static (),
    pub prev: Option<&'static Unit>,
    pub next: Option<&'static Unit>,
    ai: &'static (),
    edit: &'static (),
    pub ident: i32,
    pub person: &'static mut PersonData,
    m_Job :u64,
    m_Force :u64,
    m_BaseCapability :u64,
    m_GrowCapability :u64,
    m_LevelCapability :u64,
    m_GrowSeed :i32,
    m_DropSeed :i32,
    m_Actor :u64,
    m_Info :u64,
    m_Index :u8,
    m_Level :u8,
    m_Exp :u8,
    m_Hp :u16,
    m_HpStockCount :u8,
    m_HpStockCountMax :u8,
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
    m_ItemList :u64,
    m_ItemSelected :u64,
    m_AccessoryList :u64,
    m_GodUnit :Option<&'static GodUnit>,
    m_GodLink :Option<&'static GodUnit>,
    m_Ring :Option<&'static UnitRing>,
    m_ExtraSight :i32,
    m_MoveDistance :i32,
    m_MaskSkill :u64,
    m_EquipSkill :u64,
    m_PrivateSkill :u64,
    m_ReceiveSkill :u64,
    m_SupportedSkill :u64,
    m_EquipSkillPool :u64,
    m_LearnedJobSkill :u64,
    m_OriginalAptitude :u64,
    m_Aptitude :u64,
    m_WeaponMask :u64,
    m_SelectedWeaponMask :u64,
    m_EnhanceFactors :u64,
    m_EnhanceCalculator :u64,
    m_InternalLevel :u8,
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
        unsafe {
            unit_learnjobskill(self, job, None);
        }
    }

    /// Performs a class change on the unit without playing the sequence
    pub fn class_change(&self, job: &JobData) {
        unsafe {
            unit_classchange(self, job, 0 as _, None);
        }
    }

    /// Add item to the unit's inventory without a notification
    pub fn add_item(&self, item: &ItemData) {
        unsafe {
            unit_itemadd(self, item, None);
        }
    }

    pub fn set_level(&self, level: i32) {
        unsafe {
            unit_set_level(self, level, None);
        }
    }

    pub fn set_selected_weapon(&self, weapon_mask: &WeaponMask) {
        unsafe {
            unit_setselectedweapon(self, weapon_mask, None);
        }
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
}

#[skyline::from_offset(0x1a3f400)]
extern "C" fn unit_itemadd(this: &Unit, item: &ItemData, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "ClassChange")]
extern "C" fn unit_classchange(this: &Unit, job: &JobData, item: *const u8, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "LearnJobSkill")]
extern "C" fn unit_learnjobskill(this: &Unit, job: &JobData, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "set_Level")]
extern "C" fn unit_set_level(this: &Unit, level: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "get_Job")]
extern "C" fn unit_get_job(this: &Unit, method_info: OptionalMethod) -> &'static JobData;

#[unity::from_offset("App", "Unit", "SetSelectedWeapon")]
extern "C" fn unit_setselectedweapon(this: &Unit, weapon_mask: &WeaponMask, method_info: OptionalMethod);

// App.Unit$$IsEngaging	7101a265e0	bool App.Unit$$IsEngaging(App_Unit_o * __this, MethodInfo * method)	96
#[skyline::from_offset(0x1a265e0)]
extern "C" fn unit_is_engaging(this: &Unit, method_info: OptionalMethod) -> bool;

// App.Unit$$IsEngageOwner	7101a197a0	bool App.Unit$$IsEngageOwner(App_Unit_o * __this, MethodInfo * method)	112
#[skyline::from_offset(0x1a197a0)]
extern "C" fn unit_is_engage_owner(this: &Unit, method_info: OptionalMethod) -> bool;
