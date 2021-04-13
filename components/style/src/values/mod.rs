use crate::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

pub mod computed;
pub mod specified;
pub mod used;

/// A trait for entities that can be parsed into CSS values.
pub trait CssValueParse {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>>
    where
        Self: Sized;
}
