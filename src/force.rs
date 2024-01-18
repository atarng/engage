use unity::prelude::*;

use crate::gamedata::unit::Unit;

#[repr(C)]
pub enum ForceType {
    Player = 0,
    Enemy = 1,
    Ally = 2,
    Absent = 3,
    Dead = 4,
    Lost = 5,
    Temporary = 6,
    Empty = 7,
}

#[unity::class("App", "Force")]
pub struct Force {
    pub head: Option<&'static Unit>,
    pub tail: Option<&'static Unit>,
    // ...
}

impl Force {
    pub fn iter(&self) -> ForceIterator {
        ForceIterator(self.head)
    }

    pub fn get(ty: ForceType) -> Option<&'static mut Force> {
	unsafe { force_gettype(ty, None) }
    }
}

#[skyline::from_offset(0x2616200)]
fn force_gettype(ty: ForceType, _method_info: OptionalMethod) -> Option<&'static mut Force>;

pub struct ForceIterator(Option<&'static Unit>);

impl Iterator for ForceIterator {
    type Item = &'static Unit;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Some(unit) => {
                let res = Some(unit);
                self.0 = unit.next;
                res
            },
            None => None,
        }
    }
}
