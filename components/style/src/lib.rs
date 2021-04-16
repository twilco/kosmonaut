use std::convert::From;

use cssparser::{
    AtRuleParser, CowRcStr, ParseError, Parser, ParserInput, QualifiedRuleParser, RuleListParser,
    SourceLocation, Token,
};
use selectors::parser::SelectorParseErrorKind;

use crate::properties::{parse_property_declaration_list, PropertyDeclarationBlock};
use kosmonaut_selectors::Selectors;

#[macro_use]
extern crate cssparser;
#[macro_use]
extern crate derive_builder;
#[macro_use]
mod macros;
#[macro_use]
extern crate strum_macros;

pub mod properties;
pub mod stylesheet;
pub(crate) mod test_utils;
pub mod values;

// TODO: Servo supports many different types of rules, but we won't support those yet.  https://github.com/servo/servo/blob/d2856ce8aeca11e543bc4d9f869400d73451374e/components/style/stylesheets/mod.rs#L236
#[derive(Clone, Debug)]
pub enum CssRule {
    Style(StyleRule),
    None,
}

/// A style rule, with selectors and declarations.
#[derive(Clone, Debug)]
pub struct StyleRule {
    /// The list of selectors in this rule.
    pub selectors: Selectors,
    /// The declaration block with the properties it contains.
    pub block: PropertyDeclarationBlock,
    /// The location in the sheet where it was found.
    pub source_location: SourceLocation,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssOrigin {
    /// CSS found within `style` attribute on node
    /// In the cascade, inline styles are considered to have an author origin and a specificity
    /// higher than any other selector.
    /// https://www.w3.org/TR/css-style-attr/#interpret
    Inline,
    /// CSS found within <style></style> tags
    Embedded,
    /// CSS found within a stylesheet
    Sheet(StylesheetOrigin),
}

/// https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#cascading-origins
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CascadeOrigin {
    Author,
    User,
    UserAgent,
}

#[derive(Clone, Debug)]
pub struct StylesheetOrigin {
    pub(crate) sheet_name: String,
    pub(crate) cascade_origin: CascadeOrigin,
}

impl PartialEq for StylesheetOrigin {
    fn eq(&self, other: &Self) -> bool {
        self.cascade_origin == other.cascade_origin
    }
}

#[derive(Debug)]
pub(crate) struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub fn parse_css_to_rules(
    css_str: &mut str,
) -> Result<Vec<CssRule>, (ParseError<StyleParseErrorKind>, &str)> {
    let input = &mut ParserInput::new(css_str);
    let parser = &mut Parser::new(input);
    let rule_parser = RuleListParser::new_for_stylesheet(parser, TopLevelRuleParser {});
    let mut rules = Vec::new();
    for rule in rule_parser {
        rules.push(rule?)
    }
    Ok(rules)
}

/// Parser for top-level CSS rules.
pub(crate) struct TopLevelRuleParser {}

// TODO: Support @ rules
pub(crate) enum AtRuleNonBlockPrelude {}

pub(crate) enum AtRuleBlockPrelude {}

/// Kosmonaut currently does not support @rules, so fall back to the default @rule error impl.
impl<'i> AtRuleParser<'i> for TopLevelRuleParser {
    type PreludeNoBlock = AtRuleNonBlockPrelude;
    type PreludeBlock = AtRuleBlockPrelude;
    type AtRule = CssRule;
    type Error = StyleParseErrorKind<'i>;
}

impl<'i> QualifiedRuleParser<'i> for TopLevelRuleParser {
    type Prelude = Selectors;
    type QualifiedRule = CssRule;
    type Error = StyleParseErrorKind<'i>;

    #[inline]
    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        Selectors::compile(input).map_err(|err| err.into())
    }

    #[inline]
    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        source_location: SourceLocation,
        input: &mut Parser<'i, 't>,
    ) -> Result<CssRule, ParseError<'i, Self::Error>> {
        Ok(CssRule::Style(StyleRule {
            selectors,
            block: parse_property_declaration_list(input),
            source_location,
        }))
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
    /// An error was encountered while parsing a property value.
    ValueError(ValueParseErrorKind<'i>),
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

impl<'i> From<ValueParseErrorKind<'i>> for StyleParseErrorKind<'i> {
    fn from(this: ValueParseErrorKind<'i>) -> Self {
        StyleParseErrorKind::ValueError(this)
    }
}

/// Specific errors that can be encountered while parsing property values.
#[derive(Clone, Debug, PartialEq)]
pub enum ValueParseErrorKind<'i> {
    /// An invalid token was encountered while parsing a color value.
    InvalidColor(Token<'i>),
}

/// Value computations common to all CSS properties.
/// https://www.w3.org/TR/css3-values/#common-keywords
pub enum CssWideKeywords {
    /// Represents the value specified as the property’s initial value.
    Initial,
    /// Represents the computed value of the property on the element’s parent.
    Inherit,
    /// Acts as either inherit or initial, depending on whether the property is inherited or not.
    Unset,
}
