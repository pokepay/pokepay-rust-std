//! pokepay's std lib

#![no_std]
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

#[cfg(feature = "std")]
extern crate std;

pub mod fmt;
pub mod time;
pub mod any;

#[macro_export]
///Extended `env!` macro that allows specifying default value
///
///## Mandatory env variable
///
///```compile_fail
///const VAR: &'static str = pokepay_std::env!("_UNKNOWN_ENV_VAR_"); //this fails
///
///```
///## Optional env variable (requires default value)
///
///```
///const VAR: &'static str = pokepay_std::env!("_UNKNOWN_ENV_VAR_"; "DEFAULT VAL");
///
///assert_eq!(VAR, "DEFAULT VAL");
///```
macro_rules! env {
    ($key:expr$(,)?) => {
        env!($key)
    };
    ($key:expr; $default:expr$(,)?) => {
        match option_env!($key) {
            Some(v) => v,
            None => $default,
        }
    }
}
