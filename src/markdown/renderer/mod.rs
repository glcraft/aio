mod terminal;

use crossterm::{style::*, queue};
use std::io::{stdout, Write};
use super::token;
pub use terminal::TerminalRenderer;

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
    fn push_token(&mut self, style: token::Token) -> Result<(), Self::BackendErrorType>;
}
