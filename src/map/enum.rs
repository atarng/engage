use unity::prelude::*;

#[repr(C)]
#[derive(Default, Debug)]
pub struct MapRange {
  pub x: i32,
  pub z: i32,
  pub range: i32,
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct RangeEnumerator {
  pub current: MapRange,
  pub pivot_x: i32,
  pub pivot_z: i32,
  pub min_x: i32,
  pub min_z: i32,
  pub max_x: i32,
  pub max_z: i32,
  pub near: i32,
  pub far: i32,
}

impl RangeEnumerator {
  pub fn get_enumerator(&self) -> RangeEnumerator {
    // The logic is that we basically initialize a RangeEnumerator to store the data into, have the function fill it, and then return it.
    // I think that's what happens on the C# side but abstractions make it look more convoluted than it is.
    // Truth is, this probably should just be reimplemented as a Rust iterator once the loop logic is understood.
    unsafe { rangeenumerator_getenumerator(self, None)}
  }
}


#[skyline::from_offset(0x24c54d0)]
extern "C" fn rangeenumerator_getenumerator(this: &RangeEnumerator, method_info: OptionalMethod) -> RangeEnumerator;