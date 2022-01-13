//! String extension module.

///Extension traits for `String`
pub trait StringExt {
    ///Trims whitespaces from the end of `String` in-place.
    fn trim_end(&mut self);
}

impl StringExt for std::string::String {
    fn trim_end(&mut self) {
        while let Some(ch) = self.pop() {
            if !ch.is_whitespace() {
                self.push(ch);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::string::StringExt;
    use std::borrow::ToOwned;

    #[test]
    fn should_trim_string_of_white_spaces_at_the_end() {
        const INIT: &str = "  test  1  ";
        let mut test = INIT.to_owned();
        test.trim_end();
        assert_eq!(test, INIT.trim_end());
    }
}
