use std::ops::{Deref, DerefMut};

use unity::prelude::*;

use super::{ProcBoolMethod, ProcVoidFunction, ProcVoidMethod};

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProcDescType {
    End = 0,
    Halt = 1,
    Jump = 2,
    Label = 3,
    Push = 4,
    Pop = 5,
    Persistent = 6,
    WaitTime = 7,
    WaitFrame = 8,
    Yield = 9,
    Call = 10,
    Tick = 11,
    Args = 12,
    WaitFunc = 13,
    JumpFunc = 14,
    User = 15,
    Max = 16,
}

/// A Descriptor to be executed as part of a [`ProcInst`]
///
/// Acts as a very high level assembly instruction that is executed sequentially.
#[repr(C)]
#[unity::class("App", "ProcDesc")]
pub struct ProcDesc {
    pub ty: ProcDescType,
}

impl ProcDesc {
    pub fn label(label: i32) -> Il2CppResult<&'static mut Il2CppObject<ProcDesc>> {
        ProcDescLabel::instantiate_as::<ProcDesc>().map(|desc| {
            let cast = desc.cast_mut::<ProcDescLabel>();
            cast.ty = ProcDescType::Label;
            cast.label = label;
            desc
        })
    }

    pub fn method_call<T>(method: &'static mut Il2CppObject<ProcVoidMethod<T>>) -> Il2CppResult<&'static mut Il2CppObject<ProcDesc>> {
        ProcDescMCall::<T>::instantiate_as::<ProcDesc>().map(|desc| {
            let cast = desc.cast_mut::<ProcDescMCall<T>>();
            cast.ty = ProcDescType::Call;
            cast.method = method;
            desc
        })
    }

    pub fn wait_while_true<T>(method: &'static mut Il2CppObject<ProcBoolMethod<T>>) -> Il2CppResult<&'static mut Il2CppObject<ProcDesc>> {
        ProcDescMWaitTrue::<T>::instantiate_as::<ProcDesc>().map(|desc| {
            let cast = desc.cast_mut::<ProcDescMWaitTrue<T>>();
            cast.ty = ProcDescType::WaitFunc;
            cast.method = method;
            desc
        })
    }

    pub fn function_call<T>(function: &'static mut Il2CppObject<ProcVoidFunction<T>>) -> Il2CppResult<&'static mut Il2CppObject<ProcDesc>> {
        ProcDescCall::<T>::instantiate_as::<ProcDesc>().map(|desc| {
            let cast = desc.cast_mut::<ProcDescCall<T>>();
            cast.ty = ProcDescType::Call;
            cast.function = function;
            desc
        })
    }

    pub fn jump<T>(to_label: T) -> Il2CppResult<&'static mut Il2CppObject<ProcDesc>> {
        ProcDescJump::<T>::instantiate_as::<ProcDesc>().map(|desc| {
            let cast = desc.cast_mut::<ProcDescJump<T>>();
            cast.ty = ProcDescType::Jump;
            cast.label = to_label;
            desc
        })
    }

    pub fn yiel() -> Il2CppResult<&'static mut Il2CppObject<ProcDesc>> {
        ProcDescYield::instantiate_as::<ProcDesc>().map(|desc| {
            desc.ty = ProcDescType::Yield;
            desc
        })
    }

    pub fn wait_time(time: f32) -> Il2CppResult<&'static mut Il2CppObject<ProcDesc>> {
        ProcDescWaitTime::instantiate_as::<ProcDesc>().map(|desc| {
            let cast = desc.cast_mut::<ProcDescWaitTime>();
            cast.ty = ProcDescType::WaitTime;
            cast.time = time;
            desc
        })
    }

    pub fn end() -> Il2CppResult<&'static mut Il2CppObject<ProcDesc>> {
        ProcDescEnd::instantiate_as::<ProcDesc>().map(|desc| {
            desc.ty = ProcDescType::End;
            desc
        })
    }

    pub fn cast<T: AsRef<ProcDesc>>(&self) -> &T {
        unsafe { std::mem::transmute::<&ProcDesc, &T>(&self) }
    }

    pub fn cast_mut<T: AsMut<ProcDesc>>(&mut self) -> &mut T {
        unsafe { std::mem::transmute::<&mut ProcDesc, &mut T>(self) }
    }

    pub fn get_type(&self) -> ProcDescType {
        self.ty
    }
}

impl From<ProcDescLabel> for ProcDesc {
    fn from(value: ProcDescLabel) -> Self {
        value.desc
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescLabel")]
pub struct ProcDescLabel {
    pub desc: ProcDesc,
    pub label: i32,
}

impl AsRef<ProcDesc> for ProcDescLabel {
    fn as_ref(&self) -> &ProcDesc {
        &self.desc
    }
}

impl AsMut<ProcDesc> for ProcDescLabel {
    fn as_mut(&mut self) -> &mut ProcDesc {
        &mut self.desc
    }
}

impl Deref for ProcDescLabel {
    type Target = ProcDesc;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl DerefMut for ProcDescLabel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.desc
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescJump")]
pub struct ProcDescJump<T> {
    pub desc: ProcDesc,
    pub label: T,
}

impl<T> AsRef<ProcDesc> for ProcDescJump<T> {
    fn as_ref(&self) -> &ProcDesc {
        &self.desc
    }
}

impl<T> AsMut<ProcDesc> for ProcDescJump<T> {
    fn as_mut(&mut self) -> &mut ProcDesc {
        &mut self.desc
    }
}

impl<T> Deref for ProcDescJump<T> {
    type Target = ProcDesc;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for ProcDescJump<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescMCall")]
pub struct ProcDescMCall<T: 'static> {
    pub desc: ProcDesc,
    pub method: &'static mut Il2CppObject<ProcVoidMethod<T>>,
}

impl<T> AsRef<ProcDesc> for ProcDescMCall<T> {
    fn as_ref(&self) -> &ProcDesc {
        &self.desc
    }
}

impl<T> AsMut<ProcDesc> for ProcDescMCall<T> {
    fn as_mut(&mut self) -> &mut ProcDesc {
        &mut self.desc
    }
}

impl<T> Deref for ProcDescMCall<T> {
    type Target = ProcDesc;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for ProcDescMCall<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescMWaitTrue")]
pub struct ProcDescMWaitTrue<T: 'static> {
    pub desc: ProcDesc,
    pub method: &'static mut Il2CppObject<ProcBoolMethod<T>>,
}

impl<T> AsRef<ProcDesc> for ProcDescMWaitTrue<T> {
    fn as_ref(&self) -> &ProcDesc {
        &self.desc
    }
}

impl<T> AsMut<ProcDesc> for ProcDescMWaitTrue<T> {
    fn as_mut(&mut self) -> &mut ProcDesc {
        &mut self.desc
    }
}

impl<T> Deref for ProcDescMWaitTrue<T> {
    type Target = ProcDesc;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for ProcDescMWaitTrue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescCall")]
pub struct ProcDescCall<T: 'static> {
    pub desc: ProcDesc,
    pub function: &'static mut Il2CppObject<ProcVoidFunction<T>>,
}

impl<T> AsRef<ProcDesc> for ProcDescCall<T> {
    fn as_ref(&self) -> &ProcDesc {
        &self.desc
    }
}

impl<T> AsMut<ProcDesc> for ProcDescCall<T> {
    fn as_mut(&mut self) -> &mut ProcDesc {
        &mut self.desc
    }
}

impl<T> Deref for ProcDescCall<T> {
    type Target = ProcDesc;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for ProcDescCall<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescEnd")]
pub struct ProcDescEnd {
    pub desc: ProcDesc,
}

impl AsRef<ProcDesc> for ProcDescEnd {
    fn as_ref(&self) -> &ProcDesc {
        &self.desc
    }
}

impl AsMut<ProcDesc> for ProcDescEnd {
    fn as_mut(&mut self) -> &mut ProcDesc {
        &mut self.desc
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescYield")]
pub struct ProcDescYield {
    pub desc: ProcDesc,
}

impl AsRef<ProcDesc> for ProcDescYield {
    fn as_ref(&self) -> &ProcDesc {
        &self.desc
    }
}

impl AsMut<ProcDesc> for ProcDescYield {
    fn as_mut(&mut self) -> &mut ProcDesc {
        &mut self.desc
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescWaitTime")]
pub struct ProcDescWaitTime {
    pub desc: ProcDesc,
    pub time: f32,
}

impl AsRef<ProcDesc> for ProcDescWaitTime {
    fn as_ref(&self) -> &ProcDesc {
        &self.desc
    }
}

impl AsMut<ProcDesc> for ProcDescWaitTime {
    fn as_mut(&mut self) -> &mut ProcDesc {
        &mut self.desc
    }
}

impl Deref for ProcDescWaitTime {
    type Target = ProcDesc;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl DerefMut for ProcDescWaitTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}