
pub struct RawFormater;
use super::Formatter;
use std::io::Write;

impl Formatter for RawFormater {
    type Error = std::io::Error;
    fn push(&mut self, text: &str) -> Result<(), Self::Error> {
        writeln!(std::io::stdout(), "{}", text)
    }
}