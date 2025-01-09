//! Structures representing a singular entry from the gamedata files in memory.

use std::ops::Deref;
use job::JobDataFlag;
use unity::{prelude::*, system::{List, ListFields}};
use person::CapabilitySbyte;
use unity::il2cpp::object::Array;

pub mod accessory;
pub mod person;
pub mod skill;
pub mod job;
pub mod dispos;
pub mod unit;
pub mod item;
pub mod cook;
pub mod animal;
pub mod god;
pub mod ring;
pub mod assettable;
pub mod terrain;
pub mod shop;
pub mod ai;

#[unity::class("App", "HubFacilityData")]
pub struct HubFacilityData {
    pub parent: StructBaseFields,
    pub aid: &'static Il2CppString,
    pub mid: &'static Il2CppString,
    pub condition_cid: &'static Il2CppString,
    pub icon_name: &'static Il2CppString,
}
impl Gamedata for HubFacilityData { }

impl HubFacilityData {
    pub fn is_complete(&self) -> bool { unsafe { hubdatafacility_iscomplete(self, None) } }

    pub fn set_first_access_flag(&self) { unsafe { hubdatafacility_set_first_access_flag(self, None); } }
}

#[skyline::from_offset(0x28a80d0)]
fn hubdatafacility_iscomplete(this: &HubFacilityData, _method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x28a7b30)]
fn hubdatafacility_set_first_access_flag(this: &HubFacilityData, _method_info: OptionalMethod);

#[unity::class("App", "JobData")]
pub struct JobData {
    pub parent: StructBaseFields, //0x0
    pub jid: &'static Il2CppString, //0x10
    pub name: &'static Il2CppString, //0x18
    pub aid: &'static Il2CppString, //0x20
    pub help: &'static Il2CppString, //0x28
    pub unit_icon_id_m : Option<&'static Il2CppString>, //0x30
    pub unit_icon_id_f : Option<&'static Il2CppString>, //0x38
    pub unit_icon_weapon_id: &'static Il2CppString, //0x40
    pub rank: i32,  //0x48
    __ : i32,   ///0x4c
    pub style_name: &'static Il2CppString, //0x50
    pub move_type: i32, //0x58
    pub step_frame: i32,
    pub max_level: u8,
    pub internal_level: i8,
    pub sort: u16,
    pub flag: &'static JobDataFlag,
    cc_item: &'static Array<&'static Il2CppString>,
    unique_item: &'static Array<&'static Il2CppString>,
    style: i32,
    pub weapons: &'static mut Array<i8>,
    pub max_weapon_level: &'static mut Array<&'static Il2CppString>,
    pub weapon_levels: &'static mut Array<i32>,
    junk: [u8; 0x60],
    pub learn_skill: Option<&'static Il2CppString>, // 0x100
    pub lunatic_skill: Option<&'static Il2CppString>, //0x108
}
impl Gamedata for JobData { }

#[unity::class("App", "PersonData")]
pub struct PersonData {
    pub parent: StructBaseFields,
    pub pid: &'static Il2CppString,
    pub name: Option<&'static Il2CppString>,
    pub jid: Option<&'static Il2CppString>,
    pub fid: Option<&'static Il2CppString>,
    pub aid: Option<&'static Il2CppString>,
    pub help: Option<&'static Il2CppString>,
    pub die: Option<&'static Il2CppString>,
    pub belong: Option<&'static Il2CppString>,
    pub unit_icon_id: Option<&'static Il2CppString>,
    pub age: i16,
    pub birth_month: u8,
    pub birth_day: u8,
    pub gender: i32,
    // ...
}
impl Gamedata for PersonData { }

#[unity::class("App", "GodData")]
pub struct GodData {
    pub parent: StructBaseFields,
    pub gid: &'static Il2CppString,
    pub mid: &'static Il2CppString,
    pub nickname: &'static Il2CppString,
    pub help: &'static Il2CppString,
    pub sound_id: &'static Il2CppString,
    pub asset_id: &'static Il2CppString,
    pub face_icon_name: &'static Il2CppString,
    pub face_icon_name_darkness: &'static Il2CppString,
    pub ring_name: Option<&'static Il2CppString>,
    pub ring_help: Option<&'static Il2CppString>,
    pub unit_icon_id: Option<&'static Il2CppString>,
    pub change: Option<&'static Array<&'static Il2CppString>>,
    pub link: Option<&'static Il2CppString>,
    pub haunt: Option<&'static Il2CppString>,
    pub level: i32,
    pub force_type: i32,
    pub female: i32,
    pub good_weapon: i32,
    pub sort: i16,
    pub engage_count: i8,
    pub engage_attack: Option<&'static Il2CppString>,
    pub engage_attack_rampage: Option<&'static Il2CppString>,
    pub engage_attack_link: Option<&'static Il2CppString>,
    pub link_gid: Option<&'static Il2CppString>,
    pub gbid: Option<&'static Il2CppString>,
    pub grow_table: Option<&'static Il2CppString>,
    pub level_cap: u8,
    pub unlock_level_cap_flag: Option<&'static Il2CppString>,
    pub engrave_word: Option<&'static Il2CppString>,
    pub engrave_power: i8,
    pub engrave_weight: i8,
    pub engrave_hit: i8,
    pub engrave_critical: i8,
    pub engrave_avoid: i8,
    pub engrave_secure: i8,
    pub syncho_enhance: &'static CapabilitySbyte,
    main_data: &'static GodData,
    change_data: Option<&'static Array<&'static GodData>>,
    change_index: i32,
    pub ascii_name: Option<&'static Il2CppString>,
    pub flag: &'static WeaponMask,
}
impl Gamedata for GodData {}


#[unity::class("App", "StructData`1")]
pub struct StructDataGeneric { }

#[unity::class("App", "StructData`1")]
pub struct StructData { }

// pub static_fields: &'static StructDataStaticFields<T>,

#[derive(Clone, Copy)]
pub struct StructDataStaticFields<T: 'static> {
    pub s_list: &'static StructList<T>,
    pub loaded: bool,
}

#[unity::class("App", "StructList<`1>")]
pub struct StructList<T: 'static> {
    pub list: ListFields<T>,
}

impl<T> Deref for StructListFields<T> {
    type Target = ListFields<T>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}
use std::ops::DerefMut;
impl<T> DerefMut for StructListFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}

impl<T> StructList<T> {
    pub fn add(&mut self, element: &'static mut T) {
        let method = self.get_class().get_virtual_method("Add").unwrap();
        let add = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, &'static mut T, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };

        add(self, element, method.method_info);
    }
    pub fn insert(&mut self, index: i32, element: &'static mut T) {
        let method = self.get_class().get_virtual_method("Insert").unwrap();
        let insert = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, i32, &'static mut T, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };
        insert(self, index, element, method.method_info);
    }
}

impl<T> StructListFields<T> {
    pub fn len(&self) -> usize {
        self.size as _
    }

    pub fn capacity(&self) -> usize {
        self.items.len() as _
    }
}


#[unity::class("App", "StructBase")]
pub struct StructBase {
    pub index: i32,
    pub hash: i32,
    pub key: &'static Il2CppString,
}

#[unity::class("App", "WeaponMask")]
pub struct WeaponMask {
    pub value: i32,
}

pub trait Gamedata: Il2CppClassData + Sized {
    fn ctor(&self) {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from(".ctor")));
        if method.is_none() { return; }
        let ctor = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> ()>(
                method.unwrap().method_ptr,
            )
        };
    
        ctor(self, method.unwrap());
    }
    fn get<'a>(name: impl Into<&'a Il2CppString>) -> Option<&'static Self> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get")));
        }
        if method.is_none() {
            return None;
        }
        let get = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Il2CppString, &MethodInfo) -> Option<&'static Self>>(
                method.unwrap().method_ptr,
            )
        };
    
        get(name.into(), method.unwrap())
    }
    
    fn get_mut<'a>(name: impl Into<&'a Il2CppString>) -> Option<&'static mut Self> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get")));
        }
        if method.is_none() {
            return None;
        }
        let get = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Il2CppString, &MethodInfo) -> Option<&'static mut Self>>(
                method.unwrap().method_ptr,
            )
        };
        get(name.into(), method.unwrap())
    }

    fn get_index<'a>(name: impl Into<&'a Il2CppString>) ->  i32 {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetIndex")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetIndex")));
        }
        if method.is_none() {
            return -1;
        }
        let get = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Il2CppString, &MethodInfo) -> i32>(
                method.unwrap().method_ptr,
            )
        };
        get(name.into(), method.unwrap())
    }

    fn get_list() -> Option<&'static StructList<Self>> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        }
        if method.is_none() {
            return None;
        }
        let get_list = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> Option<&'static StructList<Self>>>(
                method.unwrap().method_ptr,
            )
        };
    
        get_list(method?)
    }
    
    fn get_list_mut() -> Option<&'static mut StructList<Self>> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        }
        if method.is_none() {
            return None;
        }
        let get_list = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> Option<&'static mut StructList<Self>>>(
                method.unwrap().method_ptr,
            )
        };
    
        get_list(method?)
    }

    fn get_count() -> i32 {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetCount")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetCount")));
        }
        if method.is_none() {
            return -1;
        }
        let get_count = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> i32>(
                method.unwrap().method_ptr,
            )
        };
        get_count(method.unwrap())
    }
    fn unload() {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Unload")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Unload")));
        }
        if method.is_none() {
            return;
        }
        let unload = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> ()> (
                method.unwrap().method_ptr,
            )
        };
        unload(method.unwrap());
    }
    fn on_build(&self) {
        let mut method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from("OnBuild")));
        if method.is_none() {
            method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("OnBuild")));
        }
        if method.is_none() { return; }
        let fxn = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> ()> (
                method.unwrap().method_ptr,
            )
        };
        fxn(self, method.unwrap());
    }
    fn on_release(&self) {
        let mut method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from("OnRelease")));
        if method.is_none() {
            method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("OnRelease")));
        }
        if method.is_none() { return; }
        let fxn = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> ()> (
                method.unwrap().method_ptr,
            )
        };
        fxn(self, method.unwrap());
    }
    fn on_completed(&self) {
        let mut method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from("OnCompleted")));
        if method.is_none() {
            method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("OnCompleted")));
        }
        if method.is_none() {
            return; 
        }
        let fxn = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> ()> (
                method.unwrap().method_ptr,
            )
        };
        fxn(self, method.unwrap());
    }
    fn on_completed_end(&self) {
        let mut method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from("OnCompletedEnd")));
        if method.is_none() {
            method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("OnCompletedEnd")));
        }
        if method.is_none() { return; }
        let fxn = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> ()> (
                method.unwrap().method_ptr,
            )
        };
        fxn(self, method.unwrap());
    }
    fn load_data() {
        //From the class itself instead of StructData since StructData load requires arguments to load the xml data
        let mut method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from("Load")));
        if method.is_none() { 
            return; 
        }
        let load = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> ()> (
                method.unwrap().method_ptr,
            )
        };
        load(method.unwrap());
    }
    fn try_index_get(index: i32) -> Option<&'static Self> {
        let mut method = 
        if Self::class()._1.parent.get_methods().len() < 10 { Self::class()._1.parent._1.parent.get_methods()[9] }
        else { Self::class()._1.parent.get_methods()[9] };
        let get = unsafe {
            std::mem::transmute::<_, extern "C" fn(i32, &MethodInfo) -> Option<&'static Self>>(
                method.method_ptr,
            )
        };
        get(index, method)
    }
    
    fn try_index_get_mut(index: i32) -> Option<&'static mut Self> {
        let mut method = if Self::class()._1.parent.get_methods().len() < 9 {   //SkillData's Get method is one level lower
            Self::class()._1.parent._1.parent.get_methods()[9]
        } else {
            Self::class()._1.parent.get_methods()[9]
        };
        let get = unsafe {
            std::mem::transmute::<_, extern "C" fn(i32, &MethodInfo) -> Option<&'static mut Self>>(
                method.method_ptr,
            )
        };
        get(index, method)
    }
    fn try_get_hash(hash: i32) -> Option<&'static Self> {
        let mut method = if Self::class()._1.parent.get_methods().len() < 11 {
            Self::class()._1.parent._1.parent.get_methods()[10]
        }
        else {
            Self::class()._1.parent.get_methods()[10]
        };
        let get = unsafe {
            std::mem::transmute::<_, extern "C" fn(i32, &MethodInfo) -> Option<&'static Self>>(
                method.method_ptr,
            )
        };
        get(hash, method)
    }
}

//StructDataArray for RewardData, DisposData, GodGrowthData, etc..
#[unity::class("App", "StructDataArray`1")]
pub struct StructDataArray {
    pub parent: StructBaseFields,
    pub array_name: &'static Il2CppString,
}
#[unity::class("App", "StructDataArrayList<`1>")]
pub struct StructDataArrayList<T: 'static>  {
    pub parent: StructListFields<T>,
    pub array_name: &'static Il2CppString,
    pub array_hash: i32, 
}

impl<T> Deref for StructDataArrayListFields<T> {
    type Target = ListFields<T>;
    fn deref(&self) -> &Self::Target {
        &self.parent.list
    }
}
impl<T> DerefMut for StructDataArrayListFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent.list
    }
}
impl<T> StructDataArrayList<T> {
    pub fn add(&mut self, element: &'static mut T) {
        let method = self.get_class().get_virtual_method("Add").unwrap();
        let add = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, &'static mut T, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };
        add(self, element, method.method_info);
    }
    pub fn insert(&mut self, index: i32, element: &'static mut T) {
        let method = self.get_class().get_virtual_method("Insert").unwrap();
        let insert = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, i32, &'static mut T, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };
        insert(self, index, element, method.method_info);
    }
}

//Making sure len() returns the size of the StructList instead of it's capacity 
impl<T> StructDataArrayListFields<T> {
    pub fn len(&self) -> usize { self.size as _ }
    pub fn capacity(&self) -> usize { self.items.len() as _ }
}

pub trait GamedataArray: Il2CppClassData + Sized {
    fn get_list_mut() -> Option<&'static mut List<StructDataArrayList<Self>>> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        }
        let get_list = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> Option<&'static mut List<StructDataArrayList<Self>>>>(
                method.unwrap().method_ptr,
            )
        };
        get_list(method.unwrap())
    }
    fn get_list() -> Option<&'static List<StructDataArrayList<Self>>> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        }
        let get_list = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> Option<&'static List<StructDataArrayList<Self>>>>(
                method.unwrap().method_ptr,
            )
        };
        get_list(method.unwrap())
    }
    fn try_get_mut<'a>(name: impl Into<&'a Il2CppString>) -> Option<&'static mut StructDataArrayList<Self>> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("TryGet")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("TryGet")));
        }
        if method.is_none() { return None; }
        let get = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Il2CppString, &MethodInfo) -> Option<&'static mut StructDataArrayList<Self>>> (
                method.unwrap().method_ptr,
            )
        };
        get(name.into(), method.unwrap())
    }
    fn unload() {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Unload")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Unload")));
        }
        if method.is_none() { return; }
        let unload = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> ()> ( method.unwrap().method_ptr, )
        };
        unload(method.unwrap());
    }
    fn load() {
        let method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from("Load")));
        if method.is_none() { return; }
        let load = unsafe { std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> ()> ( method.unwrap().method_ptr, )  };
        load(method.unwrap());
    }

    fn on_completed(&self) {
        let mut method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from("OnCompleted")));
        if method.is_none() {
            method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("OnCompleted")));
        }
        if method.is_none() { return; }
        let fxn = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> ()> (
                method.unwrap().method_ptr,
            )
        };
        fxn(self, method.unwrap());
    }
    fn on_completed_end(&self) {
        let mut method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from("OnCompletedEnd")));
        if method.is_none() {
            method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("OnCompletedEnd")));
        }
        if method.is_none() { return; }
        let fxn = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> ()> (
                method.unwrap().method_ptr,
            )
        };
        fxn(self, method.unwrap());
    }
}
