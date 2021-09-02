//!Hints to compiler that affects how code should be emitted or optimized.
pub use core::hint::*;

#[cold]
#[inline(never)]
///Function hint to mark code block as unlikely to be executed affecting branch prediction.
pub fn unlikely<R, F: Fn() -> R>(cb: F) -> R {
    cb()
}
