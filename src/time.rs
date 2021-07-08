//!time module

#[cfg(feature = "std")]
pub use std::time::*;
#[cfg(not(feature = "std"))]
pub use core::time::*;

#[cfg(feature = "std")]
#[inline]
///Gets `duration` since unix epoch.
///
///## Panics
///
///- If current clock is earlier than Unix Epoch.
pub fn get_unix_timestamp() -> core::time::Duration {
    match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(result) => result,
        Err(_) => panic!("SystemTime is before UNIX_EPOCH. MAJI DE YABAI."),
    }
}

#[cfg(feature = "std")]
#[inline(always)]
///Gets unix timestamp, adding provided `duration` to it
///
///## Panics
///
///- If current clock is earlier than Unix Epoch.
pub fn get_unix_timestamp_relatieve(duration: Duration) -> core::time::Duration {
    get_unix_timestamp().saturating_add(duration)
}
