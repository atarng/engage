pub use unity::prelude::*;

#[unity::class("App", "ItemData")]
pub struct ItemData {
	structbase: [u8; 0x10],
	pub iid: &'static Il2CppString,
	pub name: &'static Il2CppString,
	pub help: &'static Il2CppString,
	pub tutorial: &'static Il2CppString,
	pub aid: &'static Il2CppString,
	pub kind: u32,
	pub attr: u32,
	pub usetype: u32,
	pub weaponattr: u32,
	pub icon: &'static Il2CppString,
}