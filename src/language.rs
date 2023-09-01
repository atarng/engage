use unity::prelude::*;

#[unity::from_offset("App", "Language", "ReflectSetting")]
pub fn language_reflectchange(method_info: OptionalMethod);