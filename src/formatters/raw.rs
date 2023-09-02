use anyhow::Result;
use super::Formatter;
use std::io::Write;

pub struct RawFormater;

impl Formatter for RawFormater {
    fn push(&mut self, text: &str) -> Result<()> {
        todo!()
    }
}