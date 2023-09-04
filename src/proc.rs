//! Methods and traits related to the [`ProcInst`] system.

use unity::prelude::*;

pub mod desc;
use desc::*;

#[repr(C)]
pub struct Proc;

/// Represents a Instruction unit for a [`Proc`].
///
/// The ProcInst is chained to other ProcInsts to be executed in order by one of the three Proc instances of the game.
///
/// They act as a high-level virtual machine to execute [instructions](crate::proc::ProcDesc) in sequence. Said instructions are queued in a ProcDesc array.
///
/// Use the [cast](ProcInst#cast) methods to represent the chains as a ProcInst derivate of your choice
///
/// Example:
///
/// ```
/// pub fn createmenubind_hook(proc: &mut Il2CppObject<ProcInst>) {
///     let casted_child = proc.child.cast_mut::<BasicMenu>();
/// }
/// ```
#[repr(C)]
#[unity::class("App", "ProcInst")]
pub struct ProcInst {
    pub descs: &'static Il2CppArray<&'static Il2CppObject<ProcDesc>>,
    desc_index: i32,
    // Rarely set
    pub name: Option<&'static Il2CppString>,
    hash: i32,
    /// The ProcInst this instance is attached to
    pub parent: &'static Il2CppObject<ProcInst>,
    pub child: &'static mut Il2CppObject<ProcInst>,
    pub prev: &'static Il2CppObject<ProcInst>,
    pub next: &'static Il2CppObject<ProcInst>,
    /// Note:  Observed a ProcVoidMethod being set here
    persistent: *const u8,
    /// Note: Actually a bitfield to mark ProcInsts for death (to be destroyed)
    state: i32,
    suspend: i32,
    wait_time: f32,
    tick_time: f32,
    // RawValueStack
    stack: &'static Il2CppObject<RawValueStack>,
}

impl ProcInst {
    pub fn get_child(&self) -> &Il2CppObject<ProcInst> {
        self.child
    }

    pub fn get_child_mut(&mut self) -> &mut Il2CppObject<ProcInst> {
        self.child
    }

    pub fn cast<T: AsRef<ProcInst>>(&self) -> &T {
        unsafe { std::mem::transmute::<&ProcInst, &T>(self) }
    }

    pub fn cast_mut<T: AsMut<ProcInst>>(&mut self) -> &mut T {
        unsafe { std::mem::transmute::<&mut ProcInst, &mut T>(self) }
    }
}

pub trait IsProcInst {
    fn create_bind(&self, parent: &impl IsProcInst, descs: &'static mut Il2CppArray<&'static mut Il2CppObject<ProcDesc>>, name: impl AsRef<str>) {
        unsafe { procinst_createbind(self, parent, descs, name.as_ref().into(), None) }
    }
}

impl IsProcInst for Il2CppObject<ProcInst> {}
impl<T: AsMut<ProcInst>> IsProcInst for Il2CppObject<T> {}

#[unity::from_offset("App", "Proc", "WaitIsLoading")]
pub fn proc_waitisloading(
    method_info: OptionalMethod,
) -> &'static mut Il2CppObject<ProcDesc>;

#[unity::from_offset("App", "Proc", "Vsync")]
pub fn proc_vsync(
    vsync_mode: i32,
    method_info: OptionalMethod,
) -> &'static mut Il2CppObject<ProcDesc>;

#[unity::from_offset("App", "Fade", "BlackOut")]
pub fn fade_blackout(
    duration: f32,
    layer: i32,
    method_info: OptionalMethod,
) -> &'static mut Il2CppObject<ProcDesc>;

#[unity::from_offset("App", "Fade", "BlackIn")]
pub fn fade_blackin(
    duration: f32,
    layer: i32,
    method_info: OptionalMethod,
) -> &'static mut Il2CppObject<ProcDesc>;

#[unity::from_offset("App", "Proc", "GetRootDef")]
pub fn proc_getrootdef(
    method_info: OptionalMethod,
) -> &'static mut Il2CppObject<ProcInst>;

#[unity::from_offset("App", "ProcInst", "CreateBind")]
fn procinst_createbind<T: IsProcInst + ?Sized, P: IsProcInst>(
    this: &T,
    parent: &P,
    descs: &'static mut Il2CppArray<&'static mut Il2CppObject<ProcDesc>>,
    name: &'static Il2CppString,
    method_info: OptionalMethod,
);

#[unity::from_offset("App", "ProcInst", "OnDispose")]
pub fn procinst_ondispose(this: &Il2CppObject<ProcInst>, method_info: OptionalMethod);

/// A structure representing a call to a method that returns nothing.
#[repr(C)]
#[unity::class("App", "ProcVoidMethod")]
pub struct ProcVoidMethod<T: 'static> {
    method_ptr: *const u8,
    invoke_impl: *const u8,
    // Usually the ProcInst
    target: Option<&'static Il2CppObject<T>>,
    // MethodInfo
    method: *const MethodInfo,
    __: [u8; 0x38],
    delegates: *const u8,
    // ...
}

impl<T> ProcVoidMethod<T> {
    /// Prepare a ProcVoidMethod using your target and method of choice.
    ///
    /// Do be aware that despite the target argument being immutable, the receiving method can, in fact, mutate the target.
    pub fn new(
        target: Option<&'static Il2CppObject<T>>,
        method: extern "C" fn(&'static mut Il2CppObject<T>, OptionalMethod),
    ) -> Il2CppResult<&'static mut Il2CppObject<ProcVoidMethod<T>>> {
        ProcVoidMethod::<T>::instantiate().map(|proc| {
            proc.method_ptr = method as _;
            proc.target = target;
            proc.method = Box::leak(Box::new(MethodInfo::new())) as *mut MethodInfo;
            proc
        })
    }
}

#[repr(C)]
#[unity::class("App", "ProcBoolMethod")]
pub struct ProcBoolMethod<T: 'static> {
    method_ptr: *const u8,
    invoke_impl: *const u8,
    // Usually the ProcInst
    target: &'static Il2CppObject<T>,
    // MethodInfo
    method: *const MethodInfo,
    __: [u8; 0x38],
    delegates: *const u8,
    // ...
}

impl<T> ProcBoolMethod<T> {
    /// Prepare a ProcVoidMethod using your target and method of choice.
    ///
    /// Do be aware that despite the target argument being immutable, the receiving method can, in fact, mutate the target.
    pub fn new(
        target: &'static Il2CppObject<T>,
        method: extern "C" fn(&'static mut Il2CppObject<T>, OptionalMethod) -> bool,
    ) -> Il2CppResult<&'static mut Il2CppObject<ProcBoolMethod<T>>> {
        ProcBoolMethod::<T>::instantiate().map(|proc| {
            proc.method_ptr = method as _;
            proc.target = target;
            proc.method = Box::leak(Box::new(MethodInfo::new())) as *mut MethodInfo;
            proc
        })
    }
}

#[repr(C)]
#[unity::class("App", "ProcVoidFunction")]
pub struct ProcVoidFunction<T: 'static> {
    method_ptr: extern "C" fn(&'static mut Il2CppObject<T>, OptionalMethod),
    invoke_impl: *const u8,
    // Usually the ProcInst
    target: &'static Il2CppObject<T>,
    // MethodInfo
    method: *const MethodInfo,
    // ...
}

impl<T> ProcVoidFunction<T> {
    /// Prepare a ProcVoidMethod using your target and method of choice.
    ///
    /// Do be aware that despite the target argument being immutable, the receiving method can, in fact, mutate to target.
    pub fn new(
        target: &'static Il2CppObject<T>,
        method: extern "C" fn(&'static mut Il2CppObject<T>, OptionalMethod),
    ) -> Il2CppResult<&'static mut Il2CppObject<ProcVoidFunction<T>>> {
        ProcVoidFunction::<T>::instantiate().map(|proc| {
            proc.method_ptr = method;
            proc.target = target;
            proc.method = Box::leak(Box::new(MethodInfo::new())) as *mut MethodInfo;
            proc
        })
    }
}

#[repr(C)]
pub struct RawValueStack {
    count: i32,
    // ValueTypes array
    values: &'static Il2CppArray<&'static Il2CppObject<ValueType>>,
}

#[repr(C)]
pub struct ValueType;
