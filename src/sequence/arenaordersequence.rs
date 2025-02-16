use unity::{
    prelude::*,
    system::List,
};
use crate::{
    battle::BattleCalculator,
    proc::{ProcInstFields, Bindable},
    gamedata::unit::{Unit, UnitRing, GodUnit},
};

#[unity::class("App", "ArenaOrderSequence")]
pub struct ArenaOrderSequence {
    pub proc: ProcInstFields,
    pub is_resume: bool,
    pub loaded: bool,
    pub unit_list: &'static List<Unit>,
    gods_info_list: *const u8,
    pub is_emblem_battle: bool,
    pub is_special_battle: bool, 
    pub training_type: i32,
    pub training_unit: &'static Unit,
    pub battle_unit: &'static Unit,
    pub battle_emblem: &'static GodUnit,
    pub emblem_type: i32,
    pub bond_exp: i32,
    pub calculator: &'static BattleCalculator,
    pub sim_calculator: &'static BattleCalculator,
    exp_unit_select_root: *const u8,
    bond_unit_select_root: *const u8,
    bond_emblem_select_root: *const u8,
    bond_level_select_root: *const u8,
    pub next_label: i32,
    pub is_back_bond_select_emblem: bool,
    arena_objects: *const u8,
    pub god_unit: Option<&'static GodUnit>,
    pub ring: Option<&'static UnitRing>,
}
impl Bindable for ArenaOrderSequence {}
impl AsMut<ProcInstFields> for ArenaOrderSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}

impl AsRef<ProcInstFields> for ArenaOrderSequence {
    fn as_ref(&self) -> &ProcInstFields {
        &self.proc
    }
}

#[unity::class("Combat", "ArenaCombatSequence")]
pub struct ArenaCombatSequence {
    pub proc: ProcInstFields,
    pub is_resume: bool,
    pub loaded: bool,
}


impl ArenaCombatSequence {
    pub fn ctor(&self, calc: &BattleCalculator, sim: &BattleCalculator, is_emblem: bool, is_special: bool, bond_exp: i32) {
        unsafe { combatarena_ctor(self, calc, sim, is_emblem, is_special, bond_exp, None);}
    }
}
#[unity::from_offset("Combat", "ArenaCombatSequence", ".ctor")]
fn combatarena_ctor(this: &ArenaCombatSequence, calc: &BattleCalculator, sim_calc: &BattleCalculator, is_emblem: bool, is_special: bool, bond_exp: i32, method_info: OptionalMethod);
