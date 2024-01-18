//! Structures representing a singular entry from the gamedata files in memory.

use std::ops::Deref;

use unity::{prelude::*, system::ListFields};

pub mod unit;
pub mod item;

#[unity::class("App", "JobData")]
pub struct JobData {
    pub parent: StructBaseFields,
    pub jid: &'static Il2CppString,
    // ...
}

impl Gamedata for JobData { }

#[unity::class("App", "PersonData")]
pub struct PersonData {
    pub parent: StructBaseFields,
    pub pid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    // ...
}

impl Gamedata for PersonData { }

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
    pub fn is_complete(&self) -> bool {
        unsafe { hubdatafacility_iscomplete(self, None) }
    }
}

#[skyline::from_offset(0x28a80d0)]
fn hubdatafacility_iscomplete(this: &HubFacilityData, _method_info: OptionalMethod) -> bool;

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
    fn get(name: &str) -> Option<&'static Self> {
        let method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get"))).unwrap();
    
        let get = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Il2CppString, &MethodInfo) -> Option<&'static Self>>(
                method.method_ptr,
            )
        };
    
        get(name.into(), method)
    }
    
    fn get_list() -> Option<&'static StructList<Self>> {
        let method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList"))).unwrap();
    
        let get_list = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> Option<&'static StructList<Self>>>(
                method.method_ptr,
            )
        };
    
        get_list(method)
    }
    
    fn get_list_mut() -> Option<&'static mut StructList<Self>> {
        let method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList"))).unwrap();
    
        let get_list = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> Option<&'static mut StructList<Self>>>(
                method.method_ptr,
            )
        };
    
        get_list(method)
    }
}
