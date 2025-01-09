use super::*;

#[unity::class("App", "ItemShopData")]
pub struct ItemShopData {
    pub parent: StructDataArrayFields,
    pub iid: &'static Il2CppString,
    pub stock: i32, 
    pub attribute: i32,
}
impl GamedataArray for ItemShopData {}
impl ShopData for ItemShopData {}

#[unity::class("App", "WeaponShopData")]
pub struct WeaponShopData {
    pub parent: StructDataArrayFields,
    pub iid: &'static Il2CppString,
    pub stock: i32, 
    pub attribute: i32,
}
impl GamedataArray for WeaponShopData {}
impl ShopData for WeaponShopData {}

#[unity::class("App", "FleaMarketData")]
pub struct FleaMarketData {
    pub parent: StructDataArrayFields,
    pub iid: &'static Il2CppString,
    pub stock: i32, 
    pub attribute: i32,
}
#[unity::class("App", "AccessoryShopData")]
pub struct AccessoryShopData {
    pub parent: StructDataArrayFields,
    pub aid: &'static Il2CppString,
}
impl GamedataArray for AccessoryShopData {}
impl ShopData for AccessoryShopData {}
impl GamedataArray for FleaMarketData {}
impl ShopData for FleaMarketData {}

pub trait ShopData: Il2CppClassData + Sized {
    fn register() {
        let mut method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from("Regist")));
        if method.is_none() {
            method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Regist")));
        }
        let regist = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> () >(
                method.unwrap().method_ptr,
            )
        };
        regist(method.unwrap());
    }
    fn ctor(&self) {
        let method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from(".ctor")));
        if method.is_none() { return; }
        let ctor = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> () >(
                method.unwrap().method_ptr,
            )
        };
        ctor(self, method.unwrap());
    }
}

