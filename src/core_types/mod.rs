#![allow(clippy::undocumented_unsafe_blocks)]

mod int;
mod pty_str;

use crate::prelude::*;
pub use int::Int;
pub use pty_str::PtyStr;

#[derive(Clone, Copy)]
pub struct Null;
impl CanObj for Null {}
unsafe impl ValueObj for Null {}
unsafe impl ZeroObj for Null {}

impl fmt::Debug for Null {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Null").finish()
    }
}
