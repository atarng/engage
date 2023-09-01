use unity::prelude::*;

#[unity::from_offset("App", "GameUserGlobalData", "IsCompleted")]
pub fn is_completed(this: *const u8, cid: &Il2CppString, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "GameUserGlobalData", "SetCompleted")]
pub fn set_completed(this: *const u8, chapter_data: *const u8, method_info: OptionalMethod);

#[unity::from_offset("App", "GameUserGlobalData", "ClearCompleted")]
pub fn clear_completed(this: *const u8, chapter_data: *const u8, method_info: OptionalMethod);
