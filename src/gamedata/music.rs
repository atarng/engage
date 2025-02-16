use unity::prelude::*;
use super::{Gamedata, StructBaseFields};

#[unity::class("App", "JukeboxData")]
pub struct JukeBoxData {
    pub parent: StructBaseFields,
    pub event_name: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub condition: &'static Il2CppString,
}
impl Gamedata for JukeBoxData {}

#[unity::class("App", "MusicData")]
pub struct MusicData {
    pub parent: StructBaseFields,
    pub event_name: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub help: &'static Il2CppString,
    pub condition: &'static Il2CppString,
    pub amiibo: &'static Il2CppString,
    pub change_event_name: Option<&'static Il2CppString>,
    pub is_change: bool,
}
impl Gamedata for MusicData {}
