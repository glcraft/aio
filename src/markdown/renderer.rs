use crossterm::{style::*, queue};
use std::io::{stdout, Write};
use super::StyleKind;

#[derive(Debug)]
pub enum Error<BackendErrorType> 
where
    BackendErrorType: std::fmt::Debug,
{
    BackendError(BackendErrorType),
    NotSupported,
}
pub type Result<T, E> = std::result::Result<T, Error<E>>;
pub trait Renderer {
    type BackendErrorType: std::fmt::Debug;
    fn push_style(&mut self, style: StyleKind) -> Result<(), Self::BackendErrorType>;
    fn pop_style(&mut self) -> Result<(), Self::BackendErrorType>;
    fn print_text(&mut self, text: &str) -> Result<(), Self::BackendErrorType>;
    fn newline(&mut self) -> Result<(), Self::BackendErrorType>;
    fn flush(&mut self) -> Result<(), Self::BackendErrorType>;
}

pub struct TerminalRenderer;

impl TerminalRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for TerminalRenderer {
    type BackendErrorType = crossterm::ErrorKind;
    fn push_style(&mut self, style: StyleKind) -> Result<(), Self::BackendErrorType> {
        match style {
            StyleKind::Bold => queue!(stdout(), SetAttribute(Attribute::Bold)).map_err(Error::BackendError),
            StyleKind::Italic => queue!(stdout(), SetAttribute(Attribute::Italic)).map_err(Error::BackendError),
            StyleKind::BoldItalic => queue!(stdout(), SetAttribute(Attribute::Bold), SetAttribute(Attribute::Italic)).map_err(Error::BackendError),
            StyleKind::InlineCode => queue!(stdout(), SetForegroundColor(Color::Yellow)).map_err(Error::BackendError),
            _ => Ok(())
        }
    }
    fn pop_style(&mut self) -> Result<(), Self::BackendErrorType> {
        queue!(stdout(), ResetColor, SetAttribute(Attribute::Reset)).map_err(Error::BackendError)
    }
    fn print_text(&mut self, text: &str) -> Result<(), Self::BackendErrorType> {
        queue!(stdout(), Print(text)).map_err(Error::BackendError)
    }
    fn newline(&mut self) -> Result<(), Self::BackendErrorType> {
        queue!(stdout(), Print('\n')).map_err(Error::BackendError)
    }
    fn flush(&mut self) -> Result<(), Self::BackendErrorType> {
        stdout().flush().map_err(Error::BackendError)
    }
}