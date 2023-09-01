use unity::prelude::*;

#[repr(C)]
#[unity::class("Combat", "Character")]
pub struct Character;

// Combat.Character$$get_Side	7102afc5a0	int32_t Combat.Character$$get_Side(Combat_Character_o * __this, MethodInfo * method)	8
#[unity::from_offset("Combat", "Character", "get_Side")]
pub fn character_get_side(this: &Il2CppObject<Character>, method_info: OptionalMethod) -> i32;

#[repr(C)]
#[unity::class("Combat", "CharacterSound")]
pub struct CharacterSound;

#[repr(C)]
#[unity::class("Combat", "Phase")]
pub struct Phase {
    i_dont_care: [u8; 0x10],
    kind: i32,
    hit_type: i32,
    detail: i32,
    attack_side: i32,
    pub attack_hash: i32,
    pub damage_hash: i32,
}

#[repr(C)]
#[derive(Debug)]
pub enum DamageEffectLevel {
    Low,
    Medium,
    High,
}

#[repr(C)]
#[unity::class("Combat", "MagicSignalProcessor")]
pub struct MagicSignalProcessor {
    monobehaviour_fields: [u8; 0x8],
    pub character: &'static Il2CppObject<Character>,
}

#[repr(C)]
#[unity::class("UnityEngine", "AnimationEvent")]
pub struct AnimationEvent;

#[unity::from_offset("Combat", "Phase", "get_DamageEffectLevel")]
pub fn phase_get_damage_effect_level(this: &Il2CppObject<Phase>, method_info: OptionalMethod) -> DamageEffectLevel;

#[unity::from_offset("Combat", "RuntimeAnimUtil", "IsEvasion")]
pub fn runtimeanimutil_is_evasion(hash: i32, method_info: OptionalMethod) -> bool;

#[unity::from_offset("Combat", "RuntimeAnimUtil", "IsParry")]
pub fn runtimeanimutil_is_parry(hash: i32, method_info: OptionalMethod) -> bool;

#[unity::from_offset("Combat", "RuntimeAnimUtil", "IsGuard")]
pub fn runtimeanimutil_is_guard(hash: i32, method_info: OptionalMethod) -> bool;

#[unity::from_offset("Combat", "Phase", "get_IsCritical")]
pub fn phase_get_is_critical(this: &Il2CppObject<Phase>, method_info: OptionalMethod) -> bool;

// Combat.Character$$get_Phase	7102afcb70	Combat_Phase_o * Combat.Character$$get_Phase(Combat_Character_o * __this, MethodInfo * method)	336
#[unity::from_offset("Combat", "Character", "get_Phase")]
pub fn character_get_phase(this: &Il2CppObject<Character>, method_info: OptionalMethod) -> &Il2CppObject<Phase>;

// Combat.CharacterSound$$get_CP	71025efef0	Combat_Character_o * Combat.CharacterSound$$get_CP(Combat_CharacterSound_o * __this, MethodInfo * method)	180
#[unity::from_offset("Combat", "CharacterSound", "get_CP")]
pub fn charactersound_get_cp(this: &Il2CppObject<CharacterSound>, method_info: OptionalMethod) -> &Il2CppObject<Character>;

// Combat.Phase$$get_IsPlayerSideAttack	7101f2b2d0	bool Combat.Phase$$get_IsPlayerSideAttack(Combat_Phase_o * __this, MethodInfo * method)	12
#[unity::from_offset("Combat", "Phase", "get_IsPlayerSideAttack")]
pub fn phase_get_is_player_side_attack(this: &Il2CppObject<Phase>, method_info: OptionalMethod) -> bool;

// Combat.Side$$IsMaster	710247cad0	bool Combat.Side$$IsMaster(int32_t i, MethodInfo * method)	12
#[unity::from_offset("Combat", "Side", "IsMaster")]
pub fn side_is_master(i: i32, method_info: OptionalMethod) -> bool;

// Combat.Side$$IsChainAtk	710247cb00	bool Combat.Side$$IsChainAtk(int32_t i, MethodInfo * method)	16
#[unity::from_offset("Combat", "Side", "IsChainAtk")]
pub fn side_is_chain_atk(i: i32, method_info: OptionalMethod) -> bool;
