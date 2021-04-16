use crate::values::specified::ColorUnit;
use crate::values::CssValueParse;
use crate::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

/// Specified value for the `background-color` property.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#propdef-background-color
#[derive(Clone, Copy, Debug)]
pub struct BackgroundColor(ColorUnit);

impl BackgroundColor {
    pub fn initial_value() -> Self {
        BackgroundColor(ColorUnit::transparent())
    }

    pub(crate) fn unit(self) -> ColorUnit {
        self.0
    }
}

impl CssValueParse for BackgroundColor {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| ColorUnit::parse(i))
            .map(BackgroundColor)
    }
}
