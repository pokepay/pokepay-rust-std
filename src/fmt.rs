//!fmt module

pub use core::fmt::*;
use core::time;

///Pretty printer for `core::time::Duration`
pub struct Duration(pub time::Duration);

impl Display for Duration {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        if self.0.as_secs() > 0 {
            fmt.write_fmt(format_args!("{}.{}s", self.0.as_secs(), self.0.subsec_millis()))
        } else {
            fmt.write_fmt(format_args!("{}.{}ms", self.0.as_millis(), self.0.subsec_micros()))
        }
    }
}
