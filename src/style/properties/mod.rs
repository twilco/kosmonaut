use std::collections::HashSet;
use std::mem;

use cssparser::{
    parse_important, AtRuleParser, CowRcStr, DeclarationListParser, DeclarationParser, Delimiter,
    ParseError, Parser, ToCss,
};
use smallbitvec::SmallBitVec;

use crate::dom::tree::NodeRef;
use crate::style::properties::id::{LonghandId, PropertyId};
use crate::style::stylesheet::{Stylesheet, StylesheetParseErr};
use crate::style::values::specified::FontSize;
use crate::style::CssRule;
use crate::style::{RuleOrigin, RuleWithOrigin, StyleParseErrorKind};

pub mod id;
pub mod longhands;

/// Parses raw parser input into a block of property declarations.
pub fn parse_property_declaration_list(input: &mut Parser) -> PropertyDeclarationBlock {
    let mut block = PropertyDeclarationBlock::new();
    let prop_parser = PropertyDeclarationParser {
        declarations: Vec::new(),
    };
    let mut decl_iter = DeclarationListParser::new(input, prop_parser);
    while let Some(declaration) = decl_iter.next() {
        match declaration {
            Ok(importance) => {
                let decls: Vec<PropertyDeclaration> =
                    decl_iter.parser.declarations.drain(..).collect();
                for decl in decls.iter() {
                    block.add_declaration(decl.clone(), importance);
                }
            }
            Err(parse_err) => {
                dbg!(parse_err);
            }
        }
    }
    block
}

/// A struct to parse property declarations.
pub struct PropertyDeclarationParser {
    declarations: Vec<PropertyDeclaration>,
    //    /// The last parsed property id (if any).
    //    last_parsed_property_id: Option<PropertyId>,
}

impl<'i> DeclarationParser<'i> for PropertyDeclarationParser {
    type Declaration = Importance;
    type Error = StyleParseErrorKind<'i>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Importance, ParseError<'i, Self::Error>> {
        // Try to match (parse) the specified declaration `name` into a known property ID.
        let id = match PropertyId::parse(&name) {
            Some(id) => id,
            None => {
                return Err(input.new_custom_error(StyleParseErrorKind::UnknownProperty(name)));
            }
        };
        input.parse_until_before(Delimiter::Bang, |input| {
            PropertyDeclaration::parse_into(&mut self.declarations, id, input)
        })?;
        let importance = match input.try_parse(parse_important) {
            Ok(()) => Importance::Important,
            Err(_) => Importance::Normal,
        };
        // In case there is still unparsed text in the declaration, we should roll back.
        input.expect_exhausted()?;
        Ok(importance)
    }
}

/// Kosmonaut currently doesn't support @rules.  Fallback to the default "error" implementation.
/// TODO: Support atrules
impl<'i> AtRuleParser<'i> for PropertyDeclarationParser {
    type PreludeNoBlock = ();
    type PreludeBlock = ();
    type AtRule = Importance;
    type Error = StyleParseErrorKind<'i>;
}

#[derive(Clone, Debug, Default)]
pub struct PropertyDeclarationBlock {
    /// The group of declarations, along with their importance.
    declarations: Vec<PropertyDeclaration>,

    /// The "important" flag for each declaration in `declarations`.
    declarations_importance: SmallBitVec,

    longhands: HashSet<LonghandId>,
}

impl PropertyDeclarationBlock {
    /// Adds a new declaration to the block, de-duping with any existing property declarations
    /// of the same type.
    pub fn add_declaration(
        &mut self,
        mut new_decl: PropertyDeclaration,
        new_importance: Importance,
    ) {
        let mut swap_index = None;
        for (i, existing_decl) in self.declarations.iter().enumerate() {
            if mem::discriminant(existing_decl) == mem::discriminant(&new_decl) {
                // the props are the same "type", e.g. both `font-size, both `display`, etc
                // take the `new_decl`, since the latest/newest prop should always be taken
                swap_index = Some(i);
            }
        }

        if let Some(idx) = swap_index {
            mem::swap(&mut self.declarations[idx], &mut new_decl);
            self.declarations_importance
                .set(idx, new_importance.important());
        } else {
            self.declarations.push(new_decl);
            self.declarations_importance
                .push(new_importance.important());
        }
    }
}

impl PropertyDeclarationBlock {
    pub fn new() -> PropertyDeclarationBlock {
        Self::default()
    }

    pub fn declarations(&self) -> &[PropertyDeclaration] {
        &self.declarations
    }

    pub fn remove_decl(&mut self, index: usize) {
        &self.declarations.remove(index);
    }

    pub fn declarations_importance(&self) -> &SmallBitVec {
        &self.declarations_importance
    }
}

impl PropertyDeclaration {
    pub fn parse_into<'i, 't>(
        declarations: &mut Vec<PropertyDeclaration>,
        id: PropertyId,
        input: &mut Parser<'i, 't>,
    ) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
        match id {
            PropertyId::Longhand(long_id) => match long_id {
                LonghandId::Display => {}
                LonghandId::FontSize => {
                    declarations.push(PropertyDeclaration::FontSize(FontSize::parse(input)?));
                }
                _ => {}
            },
            PropertyId::Shorthand(_short_id) => {}
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
#[repr(u16)]
pub enum PropertyDeclaration {
    // Property(value)
    Display(crate::style::values::specified::Display),
    FontSize(crate::style::values::specified::FontSize),
}

/// A declaration [importance][importance].
///
/// [importance]: https://drafts.csswg.org/css-cascade/#importance
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Importance {
    /// Indicates a declaration without `!important`.
    Normal,

    /// Indicates a declaration with `!important`.
    Important,
}

impl Importance {
    /// Return whether this is an important declaration.
    pub fn important(self) -> bool {
        match self {
            Importance::Normal => false,
            Importance::Important => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::style::properties::PropertyDeclaration;
    use crate::style::values::specified;
    use crate::style::values::specified::length::*;

    use super::*;
    use crate::style::test_utils::font_size_px_or_panic;

    #[test]
    fn dedupes_and_takes_newest_prop() {
        let mut decl_block = PropertyDeclarationBlock::new();
        decl_block.add_declaration(
            PropertyDeclaration::FontSize(FontSize::Length(LengthPercentage::Length(
                NoCalcLength::Absolute(AbsoluteLength::Px(12.0)),
            ))),
            Importance::Normal,
        );
        decl_block.add_declaration(
            PropertyDeclaration::FontSize(FontSize::Length(LengthPercentage::Length(
                NoCalcLength::Absolute(AbsoluteLength::Px(16.0)),
            ))),
            Importance::Normal,
        );
        decl_block.add_declaration(
            PropertyDeclaration::FontSize(FontSize::Length(LengthPercentage::Length(
                NoCalcLength::Absolute(AbsoluteLength::Px(24.0)),
            ))),
            Importance::Normal,
        );
        assert_eq!(decl_block.declarations.len(), 1);
        assert_eq!(&24.0, font_size_px_or_panic(&decl_block.declarations[0]));
    }
}
