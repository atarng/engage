use crate::singleton::{SingletonClass, SingletonMonoBehaviour, SingletonProcInst};
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};

/// Utility function to get the instance of a singleton class.
pub fn get_instance<T: unity::prelude::Il2CppClassData>() -> &'static mut T {
    let idk = get_generic_class!(SingletonClass<T>).unwrap();
    let pointer = unsafe {
        &*(idk.rgctx_data as *const Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 6])
    };
    let get_instance = unsafe {
        std::mem::transmute::<_, extern "C" fn(OptionalMethod) -> &'static mut T>(
            pointer[5].method_ptr,
        )
    };

    get_instance(Some(&pointer[5]))
}

pub fn get_instance_monobehaviour<T: unity::prelude::Il2CppClassData>() -> &'static mut T {
    let idk = get_generic_class!(SingletonMonoBehaviour<T>).unwrap();
    let pointer = unsafe {
        &*(idk.rgctx_data as *const Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 6])
    };
    let get_instance = unsafe {
        std::mem::transmute::<_, extern "C" fn(OptionalMethod) -> &'static mut T>(
            pointer[3].method_ptr,
        )
    };

    get_instance(Some(&pointer[5]))
}

pub fn get_singleton_proc_instance<T: unity::prelude::Il2CppClassData>() -> Option<&'static mut T> {
    let idk = get_generic_class!(SingletonProcInst<T>).unwrap();

    let pointer = unsafe {
        &*(idk.rgctx_data as *const il2cpp::class::Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 6])
    };

    pointer.get(4).map(|method| {
        let func = unsafe { std::mem::transmute::<_, extern "C" fn(OptionalMethod) -> Option<&'static mut T>>(
            method.method_ptr,
        ) };

        func(Some(method))
    }).unwrap()
}
