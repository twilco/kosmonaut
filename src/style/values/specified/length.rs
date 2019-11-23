use crate::style::values::{computed, CSSFloat};
use cssparser::{Token, Parser, ParseError};
use crate::style::StyleParseErrorKind;

/// Number of app units per pixel
pub const AU_PER_PX: CSSFloat = 60.;
/// Number of app units per inch
pub const AU_PER_IN: CSSFloat = AU_PER_PX * 96.;
/// Number of app units per centimeter
pub const AU_PER_CM: CSSFloat = AU_PER_IN / 2.54;
/// Number of app units per millimeter
pub const AU_PER_MM: CSSFloat = AU_PER_IN / 25.4;
/// Number of app units per quarter
pub const AU_PER_Q: CSSFloat = AU_PER_MM / 4.;
/// Number of app units per point
pub const AU_PER_PT: CSSFloat = AU_PER_IN / 72.;
/// Number of app units per pica
pub const AU_PER_PC: CSSFloat = AU_PER_PT * 12.;

/// A `<length>` without taking `calc` expressions into account
///
/// <https://drafts.csswg.org/css-values/#lengths>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoCalcLength {
    /// An absolute length
    ///
    /// <https://drafts.csswg.org/css-values/#absolute-length>
    Absolute(AbsoluteLength),
}

/// Represents an absolute length with its unit
/// <https://drafts.csswg.org/css-values/#absolute-length>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbsoluteLength {
    /// An absolute length in pixels (px)
    Px(CSSFloat),
    /// An absolute length in inches (in)
    In(CSSFloat),
    /// An absolute length in centimeters (cm)
    Cm(CSSFloat),
    /// An absolute length in millimeters (mm)
    Mm(CSSFloat),
    /// An absolute length in quarter-millimeters (q)
    Q(CSSFloat),
    /// An absolute length in points (pt)
    Pt(CSSFloat),
    /// An absolute length in pica (pc)
    Pc(CSSFloat),
}

impl AbsoluteLength {
    fn is_zero(&self) -> bool {
        match *self {
            AbsoluteLength::Px(v) |
            AbsoluteLength::In(v) |
            AbsoluteLength::Cm(v) |
            AbsoluteLength::Mm(v) |
            AbsoluteLength::Q(v) |
            AbsoluteLength::Pt(v) |
            AbsoluteLength::Pc(v) => v == 0.,
        }
    }

    /// Convert this into a pixel value.
    #[inline]
    pub fn to_px(&self) -> CSSFloat {
        use std::f32;

        let pixel = match *self {
            AbsoluteLength::Px(value) => value,
            AbsoluteLength::In(value) => value * (AU_PER_IN / AU_PER_PX),
            AbsoluteLength::Cm(value) => value * (AU_PER_CM / AU_PER_PX),
            AbsoluteLength::Mm(value) => value * (AU_PER_MM / AU_PER_PX),
            AbsoluteLength::Q(value) => value * (AU_PER_Q / AU_PER_PX),
            AbsoluteLength::Pt(value) => value * (AU_PER_PT / AU_PER_PX),
            AbsoluteLength::Pc(value) => value * (AU_PER_PC / AU_PER_PX),
        };
        pixel.min(f32::MAX).max(f32::MIN)
    }
}

impl NoCalcLength {
    /// Parse a given absolute or relative dimension.
    pub fn parse_dimension(value: CSSFloat, unit: &str) -> Result<Self, ()> {
        Ok(match_ignore_ascii_case! { unit,
            "px" => NoCalcLength::Absolute(AbsoluteLength::Px(value)),
            "in" => NoCalcLength::Absolute(AbsoluteLength::In(value)),
            "cm" => NoCalcLength::Absolute(AbsoluteLength::Cm(value)),
            "mm" => NoCalcLength::Absolute(AbsoluteLength::Mm(value)),
            "q" => NoCalcLength::Absolute(AbsoluteLength::Q(value)),
            "pt" => NoCalcLength::Absolute(AbsoluteLength::Pt(value)),
            "pc" => NoCalcLength::Absolute(AbsoluteLength::Pc(value)),
            _ => return Err(())
        })
    }
}

/// A `<length-percentage>` value. This can be either a `<length>`, a
/// `<percentage>`, or a combination of both via `calc()`.
///
/// TODO: We don't yet support calc expressions.  If we did, we would need a specified::LengthPercentage
/// that had a Calc(Box<CalcLengthPercentage>) variant.
///
/// https://drafts.csswg.org/css-values-4/#typedef-length-percentage
#[derive(Clone, Debug, PartialEq)]
pub enum LengthPercentage {
    Length(NoCalcLength),
    Percentage(computed::Percentage),
    //    Calc(Box<CalcLengthPercentage>),
}

impl LengthPercentage {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        let location = input.current_source_location();
        let token = input.next()?;
        match *token {
            Token::Dimension {
                value, ref unit, ..
            } => {
                return NoCalcLength::parse_dimension(value, unit)
                    .map(LengthPercentage::Length)
                    .map_err(|()| location.new_unexpected_token_error(token.clone()));
            }
            Token::Percentage { unit_value, .. } => {
                return Ok(LengthPercentage::Percentage(computed::Percentage(unit_value)));
            }
            _ => return Err(location.new_unexpected_token_error(token.clone())),
        }
    }
}

