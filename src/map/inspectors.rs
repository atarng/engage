use std::iter::Map;

use unity::prelude::*;
use unity::system::List;
use crate::script::DynValue;
use crate::util::get_instance;
use unity::il2cpp::object::Array;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapInspectorKind {
	None = 0,
	Turn = 1,   //TurnCommon
	TurnAfter = 2,  //TurnCommon
	TurnEnd = 3,    //TurnCommon
	Area = 4,   //Area
	Tbox = 5,   //Poke
	Door = 6,   //Poke
	Torch = 7,  //Poke
	Visit = 8,  //Poke
	Escape = 9, //Poke
	Destroy = 10,   //Poke
	Breakdown = 11, //Poke
	BreakdownEnemy = 12,    //Poke
	Waypoint = 13,  //Poke
	Command = 14,   //Poke
	Die = 15,   //Unit
	ReviveBefore = 16,  //Unit
	ReviveAfter = 17,   //Unit
	Fixed = 18, //Unit
	Talk = 19,  //Each
	BattleBefore = 20,  //Each
	BattleTalk = 21,    //Each
	BattleAfter = 22,   //Each
	Pickup = 23,    //Person
	TargetSelect = 24,  //Person
	UnitCommandPrepare = 25,    //Person
	UnitCommandInterrupt = 26,  //Interrupt
	EngageBefore = 27,  //Person
	EngageAfter = 28,   //Person
	Cannon = 29,    //Cannon
	HelpSpot = 30,  //Poke
}


/// Singleton Class that contains all MapInspectors
#[unity::class("App", "MapInspectors")]
pub struct MapInspectors {
    parent: [u8; 0x10],
    pub inspectors: &'static mut List<MapInspector>,
    pub kind_inspectors: &'static mut Array<&'static mut List<MapInspector>>,
}

impl MapInspectors {
    pub fn get_instance() -> &'static mut MapInspectors { get_instance::<Self>() }
    pub fn add<T: Inspector>(inspector: &T) { unsafe { mapinspectors_add(inspector, None); } }
    pub fn get_kind_inspectors<T: Inspector>(kind: MapInspectorKind) -> Option<&'static mut List<T>> { 
        unsafe { std::mem::transmute( mapinspectors_get_kind(kind, None) ) }
    }
}

/// Base MapInspector Class
#[unity::class("App", "MapInspector")]
pub struct MapInspector {
    pub kind: MapInspectorKind,
    pub condition: Option<&'static DynValue>,
    pub function: Option<&'static DynValue>,
    pub args: Option<&'static Array<&'static DynValue>>,
}

impl MapInspector {
    pub fn cast<T: Inspector>(&self) -> &T {
        unsafe { std::mem::transmute::<&MapInspector, &T>(self) }
    }

    pub fn cast_mut<T: Inspector>(&mut self) -> &mut T {
        unsafe { std::mem::transmute::<&mut MapInspector, &mut T>(self) }
    }
}
pub trait Inspector: Il2CppClassData + Sized {}

/// Area (4)
#[unity::class("App", "AreaInspector")]
pub struct AreaInspector {
    pub parent: MapInspectorFields,
    pub x1: i32,
    pub z1: i32,
    pub x2: i32,
    pub z2: i32,
    pub force: i32,
}
impl Inspector for AreaInspector {}

/// Tbox (5), Door (6), Torch (7), Vist (8), Escape (9) Destory (10), Breakdown (11), BreakdownEnemy (12) WayPoint (13), Command (14), HelpSpot (30)
#[unity::class("App", "PokeInspector")]
pub struct PokeInspector {
    pub parent: MapInspectorFields,
    pub x: i32,
    pub z: i32,
    pub w: i32,
    pub h: i32,
    pub max_hp: i32,
    pub person: i32,
    pub hp_label: Option<&'static Il2CppString>,
}
impl Inspector for PokeInspector {}

/// Die (15), ReviveBefore (16), Revive (17), Fixed (18)
#[unity::class("App", "UnitInspector")]
pub struct UnitInspector {
    pub parent: MapInspectorFields,
    pub person: i32,
    pub force: i32,
}
impl Inspector for UnitInspector {}
// Talk (19), BattleBefore (20), BattleTalk (21), BattleAfter (22)
#[unity::class("App", "EachInspector")]
pub struct EachInspector {
    pub parent: MapInspectorFields,
    pub from_person: i32,
    pub from_force: i32,
    pub to_person: i32,
    pub to_force: i32,
    pub both: bool,
}
impl Inspector for EachInspector {}

/// Turn (1) TurnAfter (2), TurnEnd (3), 
#[unity::class("App", "TurnCommonInspector")]
pub struct TurnCommonInspector {
    pub parent: MapInspectorFields,
    pub min: i32,
    pub max: i32,
    pub force: i32,
}
impl Inspector for TurnCommonInspector {}

/// Pickup (23), TargetSelect (24), UnitCommand (25), EngageBefore (27), EngageAfter (28),
#[unity::class("App", "PersonInspector")]
pub struct PersonInspector {
    pub parent: MapInspectorFields,
    pub person: i32,
}
impl Inspector for PersonInspector {}

/// Type 29
#[unity::class("App", "CannonInspector")]
pub struct CannonInspector {
    pub parent: MapInspectorFields,
    pub x: i32,
    pub z: i32,
    pub max_shells: i32,
    pub key_shells: &'static Il2CppString,
}
impl Inspector for CannonInspector {}

#[unity::class("App", "InterruptInspector")]
pub struct InterruptInspector {
    pub parent: MapInspectorFields,
    pub person: i32,
    pub command: i32,
}
impl Inspector for InterruptInspector {}

#[unity::from_offset("App", "MapInspectors", "Add")]
fn mapinspectors_add<T: Inspector>(inspector: &T, method_info: OptionalMethod);

#[unity::from_offset("App", "MapInspectors", "GetKindInspectors")]
fn mapinspectors_get_kind(kind: MapInspectorKind, method_info: OptionalMethod) -> Option<&'static mut List<MapInspector>>;

