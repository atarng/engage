use unity::prelude::*;

use crate::gamedata::unit::Unit;

#[unity::class("App", "Force")]
pub struct Force {
    pub head: &'static Unit,
	pub tail: &'static Unit,
    // ...
}

impl Force {
    pub fn iter(&self) -> ForceIterator {
        ForceIterator(Some(self.head))
    }
}

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