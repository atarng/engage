use unity::prelude::*;

#[repr(C)]
#[unity::class("App", "MsgFile")]
pub struct MsgFile {
    resource_file_ptr: *const u8,
    file_object_ptr: *const u8,
    pub reference_count: i32,
}

#[unity::from_offset("App", "Mess", "GetLanguageDirectoryName")]
pub fn get_language_directory_name(method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x25d54f0)]
pub fn mess_load_impl(filename: &Il2CppString, is_warning: bool, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x25d3e40)]
pub fn mess_load(filename: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2782e80)]
pub fn msgfile_ctor(this: &Il2CppObject<MsgFile>, method_info: OptionalMethod);

#[skyline::from_offset(0x1e96440)]
pub fn msbt_load(this: &mut Il2CppObject<MsgFile>, msbt: &Il2CppArray<u8>, method_info: OptionalMethod);

#[skyline::from_offset(0x1e97550)]
pub fn msbt_get_text_num(this: &Il2CppObject<MsgFile>, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x1e97470)]
pub fn msbt_get_label(this: &Il2CppObject<MsgFile>, index: usize, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x1e97770)]
pub fn msbt_get_text(this: &Il2CppObject<MsgFile>, index: usize, method_info: OptionalMethod) -> *const u8;

#[skyline::from_offset(0x3cc4c60)]
pub fn generic_dictionary_add(this: &Il2CppObject<()>, path: &Il2CppString, instance: *const u8, method_id: *mut u64);
