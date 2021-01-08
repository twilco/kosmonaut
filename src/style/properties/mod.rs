use std::cmp::Ordering;
use std::collections::HashSet;
use std::mem;

use cssparser::{
    parse_important, AtRuleParser, CowRcStr, DeclarationListParser, DeclarationParser, Delimiter,
    ParseError, Parser, SourceLocation,
};
use smallbitvec::SmallBitVec;

use crate::style::properties::id::{LonghandId, PropertyId, ShorthandId};
use crate::style::select::Specificity;
use crate::style::values::computed::direction::WritingMode;
use crate::style::values::computed::{Direction, Display, LineStyle};
use crate::style::values::specified::border::{
    BorderBottomColor, BorderLeftColor, BorderRightColor, BorderTopColor,
};
use crate::style::values::specified::margin::parse_margin_shorthand_into;
use crate::style::values::specified::{
    BackgroundColor, BorderBottomWidth, BorderLeftWidth, BorderRightWidth, BorderTopWidth, Color,
    FontSize, Height, MarginBottom, MarginLeft, MarginRight, MarginTop, PaddingBottom, PaddingLeft,
    PaddingRight, PaddingTop, Width,
};
use crate::style::CascadeOrigin;
use crate::style::{CssOrigin, StyleParseErrorKind};

pub mod id;

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
        self.declarations.remove(index);
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
            PropertyId::Longhand(longhand) => {
                PropertyDeclaration::parse_into_longhand(declarations, longhand, input)?
            }
            PropertyId::Shorthand(shorthand) => {
                PropertyDeclaration::parse_into_shorthand(declarations, shorthand, input)?
            }
        }
        Ok(())
    }

    #[allow(unreachable_patterns)]
    fn parse_into_longhand<'i, 't>(
        declarations: &mut Vec<PropertyDeclaration>,
        id: LonghandId,
        input: &mut Parser<'i, 't>,
    ) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
        match id {
            LonghandId::BackgroundColor => declarations.push(PropertyDeclaration::BackgroundColor(
                BackgroundColor::parse(input)?,
            )),
            LonghandId::BorderBottomColor => declarations.push(
                PropertyDeclaration::BorderBottomColor(BorderBottomColor::parse(input)?),
            ),
            LonghandId::BorderLeftColor => declarations.push(PropertyDeclaration::BorderLeftColor(
                BorderLeftColor::parse(input)?,
            )),
            LonghandId::BorderRightColor => declarations.push(
                PropertyDeclaration::BorderRightColor(BorderRightColor::parse(input)?),
            ),
            LonghandId::BorderTopColor => declarations.push(PropertyDeclaration::BorderTopColor(
                BorderTopColor::parse(input)?,
            )),
            LonghandId::BorderBottomStyle => declarations.push(
                PropertyDeclaration::BorderBottomStyle(LineStyle::parse(input)?),
            ),
            LonghandId::BorderLeftStyle => declarations.push(PropertyDeclaration::BorderLeftStyle(
                LineStyle::parse(input)?,
            )),
            LonghandId::BorderRightStyle => declarations.push(
                PropertyDeclaration::BorderRightStyle(LineStyle::parse(input)?),
            ),
            LonghandId::BorderTopStyle => declarations.push(PropertyDeclaration::BorderTopStyle(
                LineStyle::parse(input)?,
            )),
            LonghandId::BorderBottomWidth => declarations.push(
                PropertyDeclaration::BorderBottomWidth(BorderBottomWidth::parse(input)?),
            ),
            LonghandId::BorderLeftWidth => declarations.push(PropertyDeclaration::BorderLeftWidth(
                BorderLeftWidth::parse(input)?,
            )),
            LonghandId::BorderRightWidth => declarations.push(
                PropertyDeclaration::BorderRightWidth(BorderRightWidth::parse(input)?),
            ),
            LonghandId::BorderTopWidth => declarations.push(PropertyDeclaration::BorderTopWidth(
                BorderTopWidth::parse(input)?,
            )),
            LonghandId::Color => {
                declarations.push(PropertyDeclaration::Color(Color::parse(input)?))
            }
            LonghandId::Direction => {
                declarations.push(PropertyDeclaration::Direction(Direction::parse(input)?))
            }
            LonghandId::Display => {
                declarations.push(PropertyDeclaration::Display(Display::parse(input)?))
            }
            LonghandId::FontSize => {
                declarations.push(PropertyDeclaration::FontSize(FontSize::parse(input)?));
            }
            LonghandId::Height => {
                declarations.push(PropertyDeclaration::Height(Height::parse(input)?));
            }
            LonghandId::MarginBottom => {
                declarations.push(PropertyDeclaration::MarginBottom(MarginBottom::parse(
                    input,
                )?));
            }
            LonghandId::MarginLeft => {
                declarations.push(PropertyDeclaration::MarginLeft(MarginLeft::parse(input)?));
            }
            LonghandId::MarginRight => {
                declarations.push(PropertyDeclaration::MarginRight(MarginRight::parse(input)?));
            }
            LonghandId::MarginTop => {
                declarations.push(PropertyDeclaration::MarginTop(MarginTop::parse(input)?));
            }
            LonghandId::PaddingBottom => {
                declarations.push(PropertyDeclaration::PaddingBottom(PaddingBottom::parse(
                    input,
                )?));
            }
            LonghandId::PaddingLeft => {
                declarations.push(PropertyDeclaration::PaddingLeft(PaddingLeft::parse(input)?));
            }
            LonghandId::PaddingRight => {
                declarations.push(PropertyDeclaration::PaddingRight(PaddingRight::parse(
                    input,
                )?));
            }
            LonghandId::PaddingTop => {
                declarations.push(PropertyDeclaration::PaddingTop(PaddingTop::parse(input)?));
            }
            LonghandId::Width => {
                declarations.push(PropertyDeclaration::Width(Width::parse(input)?));
            }
            LonghandId::WritingMode => {
                declarations.push(PropertyDeclaration::WritingMode(WritingMode::parse(input)?));
            }
            _ => unimplemented!("{}", format!("value default by longhand for id: {:?}", id)),
        };
        Ok(())
    }

    #[allow(unreachable_patterns)]
    fn parse_into_shorthand<'i, 't>(
        declarations: &mut Vec<PropertyDeclaration>,
        id: ShorthandId,
        input: &mut Parser<'i, 't>,
    ) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
        match id {
            // ShorthandId::BorderWidth => {}
            // ShorthandId::BorderTop => {}
            // ShorthandId::BorderRight => {}
            // ShorthandId::BorderBottom => {}
            // ShorthandId::BorderLeft => {}
            // ShorthandId::Border => {}
            ShorthandId::Margin => parse_margin_shorthand_into(declarations, input)?,
            // ShorthandId::Padding => {}
            _ => unimplemented!("{}", format!("parse shorthand with id: {:?}", id)),
        };
        Ok(())
    }
}

#[derive(Clone, Debug)]
#[repr(u16)]
pub enum PropertyDeclaration {
    BackgroundColor(crate::style::values::specified::BackgroundColor),
    BorderBottomColor(crate::style::values::specified::BorderBottomColor),
    BorderLeftColor(crate::style::values::specified::BorderLeftColor),
    BorderRightColor(crate::style::values::specified::BorderRightColor),
    BorderTopColor(crate::style::values::specified::BorderTopColor),
    BorderBottomStyle(crate::style::values::computed::LineStyle),
    BorderLeftStyle(crate::style::values::computed::LineStyle),
    BorderRightStyle(crate::style::values::computed::LineStyle),
    BorderTopStyle(crate::style::values::computed::LineStyle),
    BorderBottomWidth(crate::style::values::specified::BorderBottomWidth),
    BorderLeftWidth(crate::style::values::specified::BorderLeftWidth),
    BorderRightWidth(crate::style::values::specified::BorderRightWidth),
    BorderTopWidth(crate::style::values::specified::BorderTopWidth),
    Color(crate::style::values::specified::Color),
    Direction(crate::style::values::computed::Direction),
    Display(crate::style::values::computed::Display),
    FontSize(crate::style::values::specified::FontSize),
    Height(crate::style::values::specified::Height),
    MarginBottom(crate::style::values::specified::MarginBottom),
    MarginLeft(crate::style::values::specified::MarginLeft),
    MarginRight(crate::style::values::specified::MarginRight),
    MarginTop(crate::style::values::specified::MarginTop),
    PaddingBottom(crate::style::values::specified::PaddingBottom),
    PaddingLeft(crate::style::values::specified::PaddingLeft),
    PaddingRight(crate::style::values::specified::PaddingRight),
    PaddingTop(crate::style::values::specified::PaddingTop),
    Width(crate::style::values::specified::Width),
    WritingMode(crate::style::values::computed::WritingMode),
}

pub struct ComputedPropertyDeclarations {}

/// A property declaration with contextual information, such as its importance, specificity,
/// origin, and source location, all of which likely deriving from its parent style rule.
#[derive(Clone, Debug)]
pub struct ContextualPropertyDeclaration {
    pub inner_decl: PropertyDeclaration,
    pub important: bool,
    pub origin: CssOrigin,
    pub source_location: Option<SourceLocation>,
    pub specificity: Specificity,
}

/// Wrapper over a Vec<PropertyDeclaration> to provide efficient helpers over common operations
/// such as determining the existence of a type of property declaration.
#[derive(Clone, Debug)]
pub struct ContextualPropertyDeclarations {
    /// The actual context property declarations.
    decls: Vec<ContextualPropertyDeclaration>,
    /// The LonghandIds present in this container.
    longhands: HashSet<LonghandId>,
    /// Whether or not the property declarations are entirely sorted according to the cascade
    /// algorithm.  Properties that lose the cascade will still be present in `decls`, but
    /// will be found later in the `Vec`.
    /// https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#cascading
    is_sorted: bool,
}

impl ContextualPropertyDeclarations {
    #[inline]
    pub fn new() -> Self {
        ContextualPropertyDeclarations::default()
    }

    /// Sort according to the cascade algorithm.  Declarations are sorted from least applicable to
    /// most applicable, meaning the last instance of a given property is the winner of the cascade.
    ///
    /// https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#cascading
    #[inline]
    pub fn cascade_sort(&mut self) {
        // ContextualPropertyDeclarations override `Ord`, so this will sort by origin, importance,
        // specificity.
        self.decls.as_mut_slice().sort();
        // After sorting by origin, importance, and specificity, rules must also be sorted by order
        // of appearance.  However, we don't need to do anything to uphold that variant, since later
        // rules are naturally pushed to the end of the Vec.
        self.is_sorted = true;
    }

    #[inline]
    pub fn contains(&self, longhand: LonghandId) -> bool {
        self.longhands.contains(&longhand)
    }

    /// Finds the first matching `ContextualPropertyDeclaration` by `LonghandId`.
    #[inline]
    pub fn get_by_longhand(&self, longhand: LonghandId) -> Option<&ContextualPropertyDeclaration> {
        if !self.contains(longhand) {
            None
        } else {
            self.decls
                .iter()
                .rev()
                .find(|decl| LonghandId::from(*decl).eq(&longhand))
        }
    }

    #[inline]
    pub fn add(&mut self, new_decl: ContextualPropertyDeclaration) {
        self.longhands
            .insert(LonghandId::from(&new_decl.inner_decl));
        self.decls.push(new_decl);
        self.is_sorted = false;
    }
}

impl Default for ContextualPropertyDeclarations {
    fn default() -> Self {
        ContextualPropertyDeclarations {
            decls: Vec::default(),
            longhands: HashSet::default(),
            is_sorted: true,
        }
    }
}

/// Much of Kosmonaut's cascade algorithm is in this implementation — namely, the first two top-level
/// bullet points.  The final deciding factor in the cascade, order of appearance, can't possibly
/// be exercised here.
///
/// https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#cascade-origin
/// The cascade sorts declarations according to the following criteria, in descending order of priority:
///
/// * Origin and Importance
///   The origin of a declaration is based on where it comes from and its importance is whether or
///   not it is declared !important (see below).  The precedence of the various origins is, in descending order:
///     1. Transition declarations [css-transitions-1]
///     2. Important user agent declarations
///     3. Important user declarations
///     4. Important author declarations
///     5. Animation declarations [css-animations-1]
///     6. Normal author declarations
///     7. Normal user declarations
///     8. Normal user agent declarations
///
///     Declarations from origins earlier in this list win over declarations from later origins.
/// * Specificity
///     The Selectors module [SELECT] describes how to compute the specificity of a selector. Each declaration
///     has the same specificity as the style rule it appears in. For the purpose of this step, declarations
///         * Declarations from style attributes are ordered according to the document order of the element the style attribute appears on, and are all placed after any style sheets.
impl Ord for ContextualPropertyDeclaration {
    fn cmp(&self, other: &Self) -> Ordering {
        if mem::discriminant(&self.inner_decl) == mem::discriminant(&other.inner_decl) {
            if self.important && !other.important {
                return Ordering::Greater;
            } else if !self.important && other.important {
                return Ordering::Less;
            } else if self.important && other.important {
                match cmp_important_origins(&self.origin, &other.origin) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => return self.specificity.cmp(&other.specificity),
                }
            } else if !self.important && !other.important {
                // When both declarations are not important, run the same routine as the one to
                // compare important declarations but flip the result.
                return match cmp_important_origins(&self.origin, &other.origin) {
                    Ordering::Less => Ordering::Greater,
                    Ordering::Greater => Ordering::Less,
                    Ordering::Equal => return self.specificity.cmp(&other.specificity),
                };
            }
        }
        return Ordering::Equal;

        fn cmp_important_origins(a: &CssOrigin, b: &CssOrigin) -> Ordering {
            match (a, b) {
                (CssOrigin::Inline, CssOrigin::Inline)
                | (CssOrigin::Inline, CssOrigin::Embedded)
                | (CssOrigin::Embedded, CssOrigin::Inline)
                | (CssOrigin::Embedded, CssOrigin::Embedded) => Ordering::Equal,
                (CssOrigin::Inline, CssOrigin::Sheet(other_sheet_origin))
                | (CssOrigin::Embedded, CssOrigin::Sheet(other_sheet_origin)) => {
                    match &other_sheet_origin.cascade_origin {
                        CascadeOrigin::UserAgent | CascadeOrigin::User => Ordering::Less,
                        CascadeOrigin::Author => Ordering::Equal,
                    }
                }
                (CssOrigin::Sheet(self_sheet_origin), CssOrigin::Inline)
                | (CssOrigin::Sheet(self_sheet_origin), CssOrigin::Embedded) => {
                    match &self_sheet_origin.cascade_origin {
                        CascadeOrigin::UserAgent | CascadeOrigin::User => Ordering::Greater,
                        CascadeOrigin::Author => Ordering::Equal,
                    }
                }
                (CssOrigin::Sheet(self_sheet_origin), CssOrigin::Sheet(other_sheet_origin)) => {
                    match (
                        &self_sheet_origin.cascade_origin,
                        &other_sheet_origin.cascade_origin,
                    ) {
                        (CascadeOrigin::UserAgent, CascadeOrigin::UserAgent) => Ordering::Equal,
                        (CascadeOrigin::UserAgent, CascadeOrigin::User)
                        | (CascadeOrigin::UserAgent, CascadeOrigin::Author) => Ordering::Greater,
                        (CascadeOrigin::User, CascadeOrigin::UserAgent) => Ordering::Less,
                        (CascadeOrigin::User, CascadeOrigin::User) => Ordering::Equal,
                        (CascadeOrigin::User, CascadeOrigin::Author) => Ordering::Greater,
                        (CascadeOrigin::Author, CascadeOrigin::UserAgent)
                        | (CascadeOrigin::Author, CascadeOrigin::User) => Ordering::Less,
                        (CascadeOrigin::Author, CascadeOrigin::Author) => Ordering::Equal,
                    }
                }
            }
        }
    }
}

impl PartialOrd for ContextualPropertyDeclaration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ContextualPropertyDeclaration {}

impl PartialEq for ContextualPropertyDeclaration {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(&self.inner_decl) == mem::discriminant(&other.inner_decl)
            && self.origin == other.origin
    }
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
    use crate::style::test_utils::{display_by_type, font_size_px, font_size_px_or_panic};

    use super::*;
    use crate::style::values::computed::Display;
    use crate::style::values::specified::{AbsoluteLength, LengthPercentage, NoCalcLength};
    use crate::style::StylesheetOrigin;
    use std::clone::Clone;

    #[test]
    fn decl_cmp_specificity() {
        let zero_spec = ContextualPropertyDeclaration {
            inner_decl: PropertyDeclaration::FontSize(FontSize::Length(LengthPercentage::Length(
                NoCalcLength::Absolute(AbsoluteLength::Px(12.0)),
            ))),
            important: true,
            origin: CssOrigin::Inline,
            source_location: None,
            specificity: Specificity::new(0),
        };
        let mut one_thousand_spec = zero_spec.clone();
        one_thousand_spec.specificity = Specificity::new(1000);
        let mut two_thousand_spec = zero_spec.clone();
        two_thousand_spec.specificity = Specificity::new(2049);

        assert!(two_thousand_spec > one_thousand_spec);
        assert!(two_thousand_spec > zero_spec);
        assert!(one_thousand_spec > zero_spec);

        assert_eq!(
            two_thousand_spec.cmp(&two_thousand_spec.clone()),
            Ordering::Equal
        );
        assert_eq!(
            one_thousand_spec.cmp(&one_thousand_spec.clone()),
            Ordering::Equal
        );
        assert_eq!(zero_spec.cmp(&zero_spec.clone()), Ordering::Equal);
    }

    #[test]
    fn decl_cmp_importance_ordering() {
        let imp = ContextualPropertyDeclaration {
            inner_decl: PropertyDeclaration::FontSize(FontSize::Length(LengthPercentage::Length(
                NoCalcLength::Absolute(AbsoluteLength::Px(12.0)),
            ))),
            important: true,
            origin: CssOrigin::Inline,
            source_location: None,
            specificity: Specificity::new(0),
        };
        let mut not_imp = imp.clone();
        not_imp.important = false;

        assert!(imp > not_imp);
        assert!(not_imp < imp);
        assert_eq!(imp.cmp(&imp.clone()), Ordering::Equal);
        assert_eq!(not_imp.cmp(&not_imp.clone()), Ordering::Equal);
    }

    #[test]
    fn decl_cmp_both_important_sheet_origin() {
        let ua_decl = ContextualPropertyDeclaration {
            inner_decl: PropertyDeclaration::FontSize(FontSize::Length(LengthPercentage::Length(
                NoCalcLength::Absolute(AbsoluteLength::Px(12.0)),
            ))),
            important: true,
            origin: CssOrigin::Sheet(StylesheetOrigin {
                sheet_name: "file.css".to_owned(),
                cascade_origin: CascadeOrigin::UserAgent,
            }),
            source_location: None,
            specificity: Specificity::new(0),
        };
        let mut user_decl = ua_decl.clone();
        let mut author_decl = ua_decl.clone();
        user_decl.origin = CssOrigin::Sheet(StylesheetOrigin {
            sheet_name: "file.css".to_owned(),
            cascade_origin: CascadeOrigin::User,
        });
        author_decl.origin = CssOrigin::Sheet(StylesheetOrigin {
            sheet_name: "file.css".to_owned(),
            cascade_origin: CascadeOrigin::Author,
        });

        assert!(ua_decl > user_decl);
        assert!(ua_decl > author_decl);

        assert!(user_decl > author_decl);

        assert_eq!(ua_decl.cmp(&ua_decl.clone()), Ordering::Equal);
        assert_eq!(user_decl.cmp(&user_decl.clone()), Ordering::Equal);
        assert_eq!(author_decl.cmp(&author_decl.clone()), Ordering::Equal);
    }

    #[test]
    fn decl_cmp_both_unimportant_sheet_origin() {
        let ua_decl = ContextualPropertyDeclaration {
            inner_decl: PropertyDeclaration::FontSize(FontSize::Length(LengthPercentage::Length(
                NoCalcLength::Absolute(AbsoluteLength::Px(12.0)),
            ))),
            important: false,
            origin: CssOrigin::Sheet(StylesheetOrigin {
                sheet_name: "file.css".to_owned(),
                cascade_origin: CascadeOrigin::UserAgent,
            }),
            source_location: None,
            specificity: Specificity::new(0),
        };
        let mut user_decl = ua_decl.clone();
        let mut author_decl = ua_decl.clone();
        user_decl.origin = CssOrigin::Sheet(StylesheetOrigin {
            sheet_name: "file.css".to_owned(),
            cascade_origin: CascadeOrigin::User,
        });
        author_decl.origin = CssOrigin::Sheet(StylesheetOrigin {
            sheet_name: "file.css".to_owned(),
            cascade_origin: CascadeOrigin::Author,
        });

        assert!(author_decl > user_decl);
        assert!(author_decl > ua_decl);

        assert!(user_decl > ua_decl);

        assert_eq!(ua_decl.cmp(&ua_decl.clone()), Ordering::Equal);
        assert_eq!(user_decl.cmp(&user_decl.clone()), Ordering::Equal);
        assert_eq!(author_decl.cmp(&author_decl.clone()), Ordering::Equal);
    }

    #[test]
    fn author_sheet_rule_preferred_over_user_agent_sheet_rule_both_unimportant() {
        let ua_decl = ContextualPropertyDeclaration {
            inner_decl: PropertyDeclaration::Display(Display::new_block()),
            important: false,
            origin: CssOrigin::Sheet(StylesheetOrigin {
                sheet_name: "useragent.css".to_owned(),
                cascade_origin: CascadeOrigin::UserAgent,
            }),
            source_location: None,
            specificity: Specificity::new(0),
        };
        let mut author_decl = ua_decl.clone();
        author_decl.inner_decl = PropertyDeclaration::Display(Display::new_none());
        author_decl.origin = CssOrigin::Sheet(StylesheetOrigin {
            sheet_name: "author_sheet.css".to_owned(),
            cascade_origin: CascadeOrigin::Author,
        });
        let mut decls = ContextualPropertyDeclarations::new();
        decls.add(ua_decl);
        decls.add(author_decl);
        decls.cascade_sort();
        assert!(decls.is_sorted);
        let last_display = decls
            .get_by_longhand(LonghandId::Display)
            .expect("decl_sort_order_of_appearance should get display");
        match last_display.inner_decl {
            PropertyDeclaration::Display(display) => {
                assert_eq!(display, Display::new_none())
            }
            _ => panic!("this should've been a display property"),
        }
    }

    #[test]
    fn decl_sort_order_of_appearance() {
        let mut decls = ContextualPropertyDeclarations::new();
        decls.add(font_size_px(12.0));
        decls.add(display_by_type(Display::new_block()));
        decls.add(font_size_px(14.0));
        decls.add(font_size_px(16.0));
        decls.add(font_size_px(18.0));
        decls.add(font_size_px(20.0));
        decls.add(display_by_type(Display::new_inline()));

        decls.cascade_sort();
        assert!(decls.is_sorted);
        let last_font_size = decls
            .get_by_longhand(LonghandId::FontSize)
            .expect("decl_sort_order_of_appearance should get font_size");
        // This should've been sorted to the top, since most recent / latest added declarations that are otherwise equal
        // take precedence over later ones.
        assert_eq!(font_size_px_or_panic(&last_font_size.inner_decl), &20.0);
        let last_display = decls
            .get_by_longhand(LonghandId::Display)
            .expect("decl_sort_order_of_appearance should get display");
        match last_display.inner_decl {
            PropertyDeclaration::Display(display_type) => {
                assert_eq!(display_type, Display::new_inline());
            }
            _ => panic!("`last_display` should have property decl type of display"),
        }
    }

    #[test]
    fn decl_cmp_diff_prop_types_are_equal() {
        let font_size = ContextualPropertyDeclaration {
            inner_decl: PropertyDeclaration::FontSize(FontSize::Length(LengthPercentage::Length(
                NoCalcLength::Absolute(AbsoluteLength::Px(12.0)),
            ))),
            important: false,
            origin: CssOrigin::Inline,
            source_location: None,
            specificity: Specificity::new(0),
        };
        let display = ContextualPropertyDeclaration {
            inner_decl: PropertyDeclaration::Display(Display::new_block()),
            important: false,
            origin: CssOrigin::Inline,
            source_location: None,
            specificity: Specificity::new(0),
        };
        assert_eq!(font_size.cmp(&display), Ordering::Equal);
    }

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
