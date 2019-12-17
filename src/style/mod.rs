use std::convert::From;

use cssparser::{AtRuleParser, CowRcStr, ParseError, Parser, QualifiedRuleParser, SourceLocation};
use selectors::parser::SelectorParseErrorKind;

use crate::dom::tree::{debug_recursive, NodeData, NodeRef};
use crate::style::properties::id::LonghandId;
use crate::style::properties::{
    parse_property_declaration_list, PropertyDeclaration, PropertyDeclarationBlock,
};
use crate::style::select::Selectors;
use crate::style::stylesheet::{apply_stylesheet_to_node, Stylesheet};
use crate::style::values::computed::ComputedValuesBuilder;
use crate::style::values::computed::{ComputeContext, ComputedValues, ToComputedValue};
use strum::IntoEnumIterator;

#[macro_use]
mod macros;

pub mod properties;
pub mod select;
pub mod stylesheet;
pub mod test_utils;
pub mod values;

pub fn apply_styles(
    dom: NodeRef,
    ua_sheets: Vec<Stylesheet>,
    user_sheets: Vec<Stylesheet>,
    author_sheets: Vec<Stylesheet>,
) {
    // https://www.w3.org/TR/css-cascade-3/#value-stages
    // The final value of a CSS property for a given element or box is the result of a multi-step calculation:

    // 1. First, all the declared values applied to an element are collected, for each property on each element. There may be zero or many declared values applied to the element.
    // TODO: Need to collect embedded styles (<style></style>)
    ua_sheets.iter().for_each(|stylesheet| {
        apply_stylesheet_to_node(&dom, stylesheet, CascadeOrigin::UserAgent);
    });

    user_sheets.iter().for_each(|stylesheet| {
        apply_stylesheet_to_node(&dom, stylesheet, CascadeOrigin::User);
    });

    author_sheets.iter().for_each(|stylesheet| {
        apply_stylesheet_to_node(&dom, stylesheet, CascadeOrigin::Author);
    });

    // collect all inline styles
    dom.inclusive_descendants().for_each(|node| {
        if let NodeData::Element(element_data) = node.data() {
            match element_data.attributes.try_borrow() {
                Ok(attrs) => {
                    if let Some(style_str) = attrs.get("style") {
                        // TODO: Parse inline style and apply it to node.  Make sure these styles have a greater specificity than anything else
                        // cssparser::ParserInput::new(style_str)
                        dbg!("found inline style but did not collect it: {:?}", style_str);
                    }
                }
                Err(_e) => {
                    dbg!("couldn't borrow node attributes");
                }
            }
        }
    });

    cascade_and_compute(&dom);
    debug_recursive(&dom);

    // 5. Formatting the document yields the used value. An element only has a used value for a given property if that property applies to the element.
    // 6. Finally, the used value is transformed to the actual value based on constraints of the display environment. As with the used value, there may or may not be an actual value for a given property on an element.
}

/// Performs steps 2-4 of https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#value-stages.
///
/// Specifically, this is:
///
/// 2) Cascading — https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#cascade
/// 3) Defaulting to specified values — https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#specified-value
/// 4) Resolving specified values to computed values — https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#computed
pub fn cascade_and_compute(start_node: &NodeRef) {
    start_node.inclusive_descendants().for_each(|node| {
        node.contextual_decls_mut().cascade_sort();
        let mut cv_builder = ComputedValuesBuilder::default();
        let parent = node.parent();
        // If this is the root node (aka there is no parent to inherit properties from), just default all properties.
        let parent_computed_values_opt = parent.map_or(None, |p| {
            // TODO: This _could_ be an expensive clone when we actually support all CSS properties.
            p.computed_values().clone()
        });
        let parent_computed_values =
            parent_computed_values_opt.unwrap_or(ComputedValues::default());
        let context = ComputeContext {
            parent_computed_values: &parent_computed_values,
        };
        LonghandId::iter().for_each(|longhand: LonghandId| {
            match node.contextual_decls().get_by_longhand(longhand) {
                Some(contextual_decl) => {
                    match &contextual_decl.inner_decl {
                        PropertyDeclaration::Display(display) => {
                            // TODO: Should we cloning here (taking the specified value), rather than computing the value?
                            // There is currently no `specified/display.rs`, so that would need to be remedied.
                            // Computing display might not be straightforward — see: https://github.com/w3c/csswg-drafts/issues/1716
                            cv_builder.display(display.clone());
                        }
                        PropertyDeclaration::FontSize(font_size) => {
                            cv_builder.font_size(font_size.to_computed_value(&context));
                        }
                        PropertyDeclaration::MarginBottom(margin_bottom) => {
                            cv_builder.margin_bottom(margin_bottom.to_computed_value(&context));
                        }
                        PropertyDeclaration::MarginLeft(margin_left) => {
                            cv_builder.margin_left(margin_left.to_computed_value(&context));
                        }
                        PropertyDeclaration::MarginRight(margin_right) => {
                            cv_builder.margin_right(margin_right.to_computed_value(&context));
                        }
                        PropertyDeclaration::MarginTop(margin_top) => {
                            cv_builder.margin_top(margin_top.to_computed_value(&context));
                        }
                        PropertyDeclaration::PaddingBottom(padding_bottom) => {
                            cv_builder.padding_bottom(padding_bottom.to_computed_value(&context));
                        }
                        PropertyDeclaration::PaddingLeft(padding_left) => {
                            cv_builder.padding_left(padding_left.to_computed_value(&context));
                        }
                        PropertyDeclaration::PaddingRight(padding_right) => {
                            cv_builder.padding_right(padding_right.to_computed_value(&context));
                        }
                        PropertyDeclaration::PaddingTop(padding_top) => {
                            cv_builder.padding_top(padding_top.to_computed_value(&context));
                        }
                    }
                }
                None => {
                    longhand.value_default(&mut cv_builder, &context);
                }
            };
        });
        *node.computed_values_mut() =
            Some(dbg!(cv_builder.build()).expect(
                "couldn't build computed values - maybe a field wasn't given to the builder?",
            ));
    });
}

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
#[derive(Clone, Debug, PartialEq)]
pub enum CascadeOrigin {
    Author,
    User,
    UserAgent,
}

#[derive(Clone, Debug)]
pub struct StylesheetOrigin {
    pub sheet_name: String,
    pub cascade_origin: CascadeOrigin,
}

impl PartialEq for StylesheetOrigin {
    fn eq(&self, other: &Self) -> bool {
        return &self.cascade_origin == &other.cascade_origin;
    }
}

#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/// Parser for top-level CSS rules.
pub struct TopLevelRuleParser {}

// TODO: Support @ rules
pub enum AtRuleNonBlockPrelude {}

pub enum AtRuleBlockPrelude {}

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
        Selectors::compile(input)
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
