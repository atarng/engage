//! Methods and traits related to the [`ProcInst`] system.

use unity::prelude::*;
use std::cell::UnsafeCell;

pub mod desc;
use desc::*;

#[repr(C)]
pub struct Proc;

impl Proc {
    pub fn get_root_def() -> &'static mut ProcInst {
        unsafe { proc_getrootdef(None) }
    }

    pub fn vsync(vsync_mode: i32) -> &'static mut ProcDesc {
        unsafe { proc_vsync(vsync_mode, None) }
    }

    pub fn wait_is_loading() -> &'static mut ProcDesc {
        unsafe { proc_waitisloading(None) }
    }
}

/// Represents a Instruction unit for a [`Proc`].
///
/// The ProcInst is chained to other ProcInsts to be executed in order by one of the three Proc instances of the game.
///
/// They act as a high-level virtual machine to execute [instructions](crate::proc::ProcDesc) in sequence. Said instructions are queued in a ProcDesc array.
/// 
/// A lot of classes inherit from it, so implement [`IsProcInst`] on them to represent this relation.
///
/// If it is not possible for some reason, use the [cast](ProcInst::cast) methods to represent the chains as a ProcInst derivate of your choice.
///
/// Example:
///
/// ```
/// pub fn createmenubind_hook(proc: &mut Il2CppObject<ProcInst>) {
///     let casted_child = proc.child.cast_mut::<BasicMenu>();
/// }
/// ```
/// 
/// Keep in mind that [`IsProcInst`] is much more preferable, and [cast](ProcInst::cast) should only be used as a last resort.
#[repr(C)]
#[unity::class("App", "ProcInst")]
pub struct ProcInst {
    descs: &'static mut UnsafeCell<Il2CppArray<&'static mut ProcDesc>>,
    pub desc_index: i32,
    pub name: Option<&'static Il2CppString>,
    pub hashcode: i32,
    /// The ProcInst this instance is attached to
    pub parent: &'static mut ProcInst,
    pub child: Option<&'static mut ProcInst>,
    pub prev: &'static ProcInst,
    pub next: &'static ProcInst,
    /// Note:  Observed a ProcVoidMethod being set here
    persistent: *const u8,
    /// Note: Actually a bitfield to mark ProcInsts for death (to be destroyed)
    pub state: i32,
    pub suspend: i32,
    wait_time: f32,
    tick_time: f32,
    // RawValueStack
    stack: &'static Il2CppObject<RawValueStack>,
}

impl Drop for ProcInst {
    fn drop(&mut self) {
        panic!("ProcInst dropped");
    }
}

impl ProcInst {
    pub fn get_descs(&self) -> &Il2CppArray<&'static mut ProcDesc> {
        unsafe { &*self.descs.get() }
    }

    pub fn get_descs_mut(&self) -> &mut Il2CppArray<&'static mut ProcDesc> {
        unsafe {&mut *self.descs.get() }
    }

    pub fn cast<T: AsRef<ProcInstFields>>(&self) -> &T {
        unsafe { std::mem::transmute::<&ProcInst, &T>(self) }
    }

    pub fn cast_mut<T: AsMut<ProcInstFields>>(&mut self) -> &mut T {
        unsafe { std::mem::transmute::<&mut ProcInst, &mut T>(self) }
    }
}

/// Trait to simulate inheritance for [`ProcInst`].
/// 
/// If the trait is in scope, it is automatically implemented for objects that implement `AsMut<ProcInst>`.
/// 
/// A method expecting a `&impl Bindable` or `<P: Bindable>(parent: &P, ...)` will accept any type that inherits from [`ProcInst`].
pub trait Bindable {
    fn create_bind(&self, parent: &impl Bindable, descs: &'static mut Il2CppArray<&'static mut ProcDesc>, name: impl AsRef<str>) {
        unsafe { procinst_createbind(self, parent, descs, name.as_ref().into(), None) }
    }
}

impl Bindable for ProcInst { }


#[unity::from_offset("App", "Proc", "WaitIsLoading")]
fn proc_waitisloading(
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "Vsync")]
fn proc_vsync(
    vsync_mode: i32,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "GetRootDef")]
fn proc_getrootdef(
    method_info: OptionalMethod,
) -> &'static mut ProcInst;

#[unity::from_offset("App", "Proc", "Label")]
fn proc_label(
    label: i32,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "WaitTime")]
fn proc_wait_time(
    seconds: f32,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "Call")]
fn proc_call<D: Delegate>(
    method: &'static mut D,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "WaitWhileTrue")]
fn proc_wait_while_true<T>(
    method: &'static mut ProcBoolMethod<T>,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "End")]
fn proc_end(
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "ProcInst", "CreateBind")]
pub fn procinst_createbind<T: Bindable + ?Sized, P: Bindable>(
    this: &T,
    parent: &P,
    descs: &'static Il2CppArray<&'static mut ProcDesc>,
    name: &'static Il2CppString,
    method_info: OptionalMethod,
);

#[unity::from_offset("App", "ProcInst", "OnDispose")]
pub fn procinst_ondispose(this: &ProcInst, method_info: OptionalMethod);

/// A structure representing a call to a method that returns nothing.
#[repr(C)]
#[unity::class("App", "ProcVoidMethod")]
pub struct ProcVoidMethod<T: 'static + Bindable> {
    method_ptr: *const u8,
    invoke_impl: *const u8,
    // Usually the ProcInst
    target: Option<&'static T>,
    // MethodInfo
    method: *const MethodInfo,
    __: [u8; 0x38],
    delegates: *const u8,
    // ...
}

pub trait Delegate { }

impl<T: Bindable> Delegate for ProcVoidFunction<T> { }
impl<T: Bindable> Delegate for ProcVoidMethod<T> { }

impl<T: Bindable> ProcVoidMethod<T> {
    /// Prepare a ProcVoidMethod using your target and method of choice.
    ///
    /// Do be aware that despite the target argument being immutable, the receiving method can, in fact, mutate the target.
    pub fn new(
        target: impl Into<Option<&'static T>>,
        method: extern "C" fn(&'static mut T, OptionalMethod),
    ) -> &'static mut ProcVoidMethod<T> {
        ProcVoidMethod::<T>::instantiate().map(|proc| {
            proc.method_ptr = method as _;
            proc.target = target.into();
            proc.method = Box::leak(Box::new(MethodInfo::new())) as *mut MethodInfo;
            proc
        }).unwrap()
    }
}

#[repr(C)]
#[unity::class("App", "ProcBoolMethod")]
pub struct ProcBoolMethod<T: 'static> {
    method_ptr: *const u8,
    invoke_impl: *const u8,
    // Usually the ProcInst
    target: &'static T,
    // MethodInfo
    method: *const MethodInfo,
    __: [u8; 0x38],
    delegates: *const u8,
    // ...
}

/// A structure representing a call to a method that returns a bool.
impl<T> ProcBoolMethod<T> {
    /// Prepare a ProcVoidMethod using your target and method of choice.
    ///
    /// Do be aware that despite the target argument being immutable, the receiving method can, in fact, mutate the target.
    pub fn new(
        target: &'static T,
        method: extern "C" fn(&'static mut T, OptionalMethod) -> bool,
    ) -> &'static mut ProcBoolMethod<T> {
        ProcBoolMethod::<T>::instantiate().map(|proc| {
            proc.method_ptr = method as _;
            proc.target = target;
            proc.method = Box::leak(Box::new(MethodInfo::new())) as *mut MethodInfo;
            proc
        }).unwrap()
    }
}

#[repr(C)]
#[unity::class("App", "ProcVoidFunction")]
pub struct ProcVoidFunction<T: 'static> {
    method_ptr: extern "C" fn(&'static mut T, OptionalMethod),
    invoke_impl: *const u8,
    // Usually the ProcInst
    target: Option<&'static T>,
    // MethodInfo
    method: *const MethodInfo,
    // ...
}

impl<T> ProcVoidFunction<T> {
    /// Prepare a ProcVoidMethod using your target and method of choice.
    ///
    /// Do be aware that despite the target argument being immutable, the receiving method can, in fact, mutate to target.
    pub fn new(
        target: impl Into<Option<&'static T>>,
        method: extern "C" fn(&'static mut T, OptionalMethod),
    ) -> &'static mut ProcVoidFunction<T> {
        ProcVoidFunction::<T>::instantiate().map(|proc| {
            proc.method_ptr = method;
            proc.target = target.into();
            proc.method = Box::leak(Box::new(MethodInfo::new())) as *mut MethodInfo;
            proc
        }).unwrap()
    }
}

#[repr(C)]
pub struct RawValueStack {
    count: i32,
    // ValueTypes array
    values: &'static Il2CppArray<&'static ValueType>,
}

#[repr(C)]
pub struct ValueType;
