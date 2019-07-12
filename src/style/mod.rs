pub mod select;

use crate::style::select::{KosmonautParser, KosmonautSelectors};

use std::convert::From;
use std::fs::File;
use std::path::Path;
use cssparser::{CowRcStr, ParseError, Parser, QualifiedRuleParser, SourceLocation};
use cssparser::DeclarationParser;
use selectors::parser::{
    SelectorList,
    SelectorParseError,
    SelectorParseErrorKind
};

/// CSS can come from: <style> tags (embedded), (inline) style on node itself, (external) stylesheets
pub struct NodeStyle {
    pub display: Display,
}

// https://www.w3schools.com/CSSref/pr_class_display.asp
pub enum Display {
    None,
    Inline,
    Block,
    InlineBlock,
    ListItem,
}

// TODO: Servo supports many different types of rules, but we won't support those yet.  https://github.com/servo/servo/blob/d2856ce8aeca11e543bc4d9f869400d73451374e/components/style/stylesheets/mod.rs#L236
pub enum CssRule {
    Style(StyleRule),
    None
}

/// A style rule, with selectors and declarations.
#[derive(Debug)]
pub struct StyleRule {
    /// The list of selectors in this rule.
//    pub selectors: Selectors,
    /// The declaration block with the properties it contains.
    pub block: Declaration,
    /// The location in the sheet where it was found.
    pub source_location: SourceLocation,
}

/// Example: margin: auto;
/// TODO: This should be statically typed, not stringly typed.
#[derive(Debug)]
pub struct Declaration {
    name: String,
    value: Value,
}

#[derive(Debug)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
    // insert more values here
}

#[derive(Debug)]
pub enum Unit {
    Px,
    // insert more units here
}

#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/// Parses stylesheet file into StyleRules.
///   * path_and_name - Path to and filename of the stylesheet
pub fn parse_stylesheet_to_rules<P: AsRef<Path>>(path_and_name: P) -> Result<Vec<StyleRule>, std::io::Error> {
    let stylesheet = File::open(path_and_name)?;

    Ok(Vec::new())
}

/// Parser for top-level CSS rules.
pub struct TopLevelRuleParser {}

impl<'i> QualifiedRuleParser<'i> for TopLevelRuleParser {
    type Prelude = SelectorList<KosmonautSelectors>;
    type QualifiedRule = CssRule;
    type Error = StyleParseErrorKind<'i>;

    #[inline]
    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        SelectorList::parse(&KosmonautParser, input)
    }

    #[inline]
    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        location: SourceLocation,
        input: &mut Parser<'i, 't>,
    ) -> Result<CssRule, ParseError<'i, Self::Error>> {
        Ok(CssRule::None)
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Errors that can be encountered while parsing CSS values.
/// This was taken from Servo: https://github.com/servo/servo/blob/30ca50ae985fca0c7f6a97bdad05921bb411cd3c/components/style_traits/lib.rs#L111
/// We must comply with the Mozilla Public License 2.0.
pub enum StyleParseErrorKind<'i> {
    /// A bad URL token in a DVB.
    BadUrlInDeclarationValueBlock(CowRcStr<'i>),
    /// A bad string token in a DVB.
    BadStringInDeclarationValueBlock(CowRcStr<'i>),
    /// Unexpected closing parenthesis in a DVB.
    UnbalancedCloseParenthesisInDeclarationValueBlock,
    /// Unexpected closing bracket in a DVB.
    UnbalancedCloseSquareBracketInDeclarationValueBlock,
    /// Unexpected closing curly bracket in a DVB.
    UnbalancedCloseCurlyBracketInDeclarationValueBlock,
    /// A property declaration value had input remaining after successfully parsing.
    PropertyDeclarationValueNotExhausted,
    /// An unexpected dimension token was encountered.
    UnexpectedDimension(CowRcStr<'i>),
    /// Missing or invalid media feature name.
    MediaQueryExpectedFeatureName(CowRcStr<'i>),
    /// Missing or invalid media feature value.
    MediaQueryExpectedFeatureValue,
    /// A media feature range operator was not expected.
    MediaQueryUnexpectedOperator,
    /// min- or max- properties must have a value.
    RangedExpressionWithNoValue,
    /// A function was encountered that was not expected.
    UnexpectedFunction(CowRcStr<'i>),
    /// @namespace must be before any rule but @charset and @import
    UnexpectedNamespaceRule,
    /// @import must be before any rule but @charset
    UnexpectedImportRule,
    /// Unexpected @charset rule encountered.
    UnexpectedCharsetRule,
    /// Unsupported @ rule
    UnsupportedAtRule(CowRcStr<'i>),
    /// A placeholder for many sources of errors that require more specific variants.
    UnspecifiedError,
//    /// An unexpected token was found within a namespace rule.
//    UnexpectedTokenWithinNamespace(Token<'i>),
//    /// An error was encountered while parsing a property value.
//    ValueError(ValueParseErrorKind<'i>),
    /// An error was encountered while parsing a selector
    SelectorError(SelectorParseErrorKind<'i>),

    /// The property declaration was for an unknown property.
    UnknownProperty(CowRcStr<'i>),
    /// The property declaration was for a disabled experimental property.
    ExperimentalProperty,
//    /// The property declaration contained an invalid color value.
//    InvalidColor(CowRcStr<'i>, Token<'i>),
//    /// The property declaration contained an invalid filter value.
//    InvalidFilter(CowRcStr<'i>, Token<'i>),
    /// The property declaration contained an invalid value.
    OtherInvalidValue(CowRcStr<'i>),
    /// The declaration contained an animation property, and we were parsing
    /// this as a keyframe block (so that property should be ignored).
    ///
    /// See: https://drafts.csswg.org/css-animations/#keyframes
    AnimationPropertyInKeyframeBlock,
    /// The property is not allowed within a page rule.
    NotAllowedInPageRule,
}

impl<'i> From<selectors::parser::SelectorParseErrorKind<'i>> for StyleParseErrorKind<'i> {
    fn from(sel_parse_err: SelectorParseErrorKind<'i>) -> Self {
        StyleParseErrorKind::SelectorError(sel_parse_err)
    }
}


