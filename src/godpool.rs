use unity::prelude::*;
use unity::system::List;
use crate::gamedata::{Gamedata, GodData, unit::GodUnit};

// App.GodPool$$HasArmlet	71023359c0	bool App.GodPool$$HasArmlet(MethodInfo * method)	464
#[unity::class("App", "GodPool")]
pub struct GodPool {}

impl GodPool {
    pub fn create(god: &GodData) -> Option<&'static GodUnit> { unsafe { god_pool_create(god, None) } }
    pub fn create_by_gid<'a>(gid: impl Into<&'a Il2CppString>) -> Option<&'static GodUnit> {
        if let Some(god) = GodData::get(gid.into()) { Self::create(god)  }
        else { None }
    }
    pub fn try_get_gid(gid: &str, include_reserved: bool) -> Option<&GodUnit> { unsafe { god_pool_try_get_gid(gid.into(), include_reserved, None) }  }
    pub fn try_get(god: &GodData, include_reserved: bool) -> Option<&GodUnit> { unsafe { god_pool_try_get(god, include_reserved, None)} }
    pub fn delete(god: &GodUnit) { unsafe { god_pool_delete(god, None); }  }
}

#[unity::from_offset("App", "GodPool", "HasArmlet")]
pub fn has_armlet(method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02334570)]
pub fn god_pool_try_get_gid(gid: &Il2CppString, include_reserved: bool, method_info: OptionalMethod) -> Option<&GodUnit>;

#[skyline::from_offset(0x02334600)]
pub fn god_pool_try_get(god: &GodData, include_reserved: bool, method_info: OptionalMethod) -> Option<&GodUnit>;

#[skyline::from_offset(0x023349c0)]
pub fn god_pool_create(god: &GodData, method_info: OptionalMethod) -> Option<&'static GodUnit>;

#[skyline::from_offset(0x02334c30)]
fn god_pool_delete(god: &GodUnit, method_info: OptionalMethod);