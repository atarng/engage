use unity::prelude::*;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct MapRange {
  pub x: i32,
  pub z: i32,
  pub range: i32,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
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

impl Iterator for RangeEnumerator {
    // Maybe a (x, z) tuple with the current tile being checked?
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
      loop {
        if self.max_x == self.current.x {
          if self.min_z == self.current.z {
            // We went through all the possible positions
            return None;
          } else {
            // We haven't gone through every z position yet, restart the loop from the leftmost X position and check again
            self.current.z -= 1;
            self.current.x = self.min_x;
          }
        } else {
          // We haven't reached max_x yet, so continue
          self.current.x += 1;
        }

        let piv_x = (self.current.x - self.pivot_x).abs();
        let piv_z = (self.current.z - self.pivot_z).abs();

        let piv = piv_x + piv_z;

        if (piv >= self.near) && (piv <= self.far) {
          break;
        }
      }

      Some((self.current.x, self.current.z))
    }
}

#[skyline::from_offset(0x24c54d0)]
extern "C" fn rangeenumerator_getenumerator(this: &RangeEnumerator, method_info: OptionalMethod) -> RangeEnumerator;
