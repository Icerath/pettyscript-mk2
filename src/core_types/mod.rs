#![allow(clippy::undocumented_unsafe_blocks)]

use crate::prelude::*;
impl CanObj for i64 {}
unsafe impl ValueObj for i64 {}

pub struct Null;

impl CanObj for Null {}

impl fmt::Debug for Null {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Null").finish()
    }
}
impl CanObj for Box<str> {}
