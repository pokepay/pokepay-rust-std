//!Any module

pub use core::any::*;

#[cfg(feature = "std")]
use std::borrow::Cow;

///Extension for `Any`
pub trait AnyExt {
    ///Executes function, if `self` can be downcasted to closure's argument.
    ///
    ///```
    ///use pokepay_std::any::AnyExt;
    ///
    ///match std::panic::catch_unwind(|| 1 ) {
    ///    Ok(res) => println!("Result {}", res),
    ///    Err(error) => {
    ///        error.visit_as(|msg: &String| {
    ///            eprintln!("Error: {}", msg);
    ///        });
    ///    }
    ///}
    ///```
    fn visit_as<T: Any, R, F: FnOnce(&T) -> R>(&self, cb: F) -> Option<R>;

    #[cfg(feature = "std")]
    ///Downcasts `self` into string
    fn as_str(&self) -> Option<Cow<'static, str>>;
}

macro_rules! impl_any_ext {
    ($($typ:ty),+) => {
        $(
            impl AnyExt for $typ {
                #[inline(always)]
                fn visit_as<T: Any, R, F: FnOnce(&T) -> R>(&self, cb: F) -> Option<R> {
                    self.downcast_ref().map(cb)
                }

                #[cfg(feature = "std")]
                #[inline(always)]
                fn as_str(&self) -> Option<Cow<'static, str>> {
                    if let Some(text) = self.downcast_ref::<&'static str>() {
                        Some((*text).into())
                    } else if let Some(text) = self.downcast_ref::<std::string::String>() {
                        Some(text.clone().into())
                    } else {
                        None
                    }
                }
            }
        )+
    }
}

impl_any_ext!(dyn Any, dyn Any + Send, dyn Any + Send + Sync);
#[cfg(feature = "std")]
use std::boxed::Box;
#[cfg(feature = "std")]
impl_any_ext!(Box<dyn Any>, Box<dyn Any + Send>, Box<dyn Any + Send + Sync>);
