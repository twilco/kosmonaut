use std::fs::File;
use std::io::Read;
use std::path::Path;

use cssparser::{ParseError, Parser, ParserInput, RuleListParser};

use crate::style::{CssRule, StyleParseErrorKind, StyleRule, TopLevelRuleParser};

/// Parses stylesheet file into StyleRules.
///   * path_and_name - Path to and filename of the stylesheet
pub fn parse_str_to_stylesheet(stylesheet_str: &mut str) -> Result<Stylesheet, (ParseError<StyleParseErrorKind>, &str)> {
    let input = &mut ParserInput::new(stylesheet_str);
    let parser = &mut Parser::new(input);
    let mut rule_parser = RuleListParser::new_for_stylesheet(parser, TopLevelRuleParser {});
    let mut sheet = Stylesheet::new();
    while let Some(rule) = rule_parser.next() {
        sheet.add_rule(rule?);
    }
    Ok(sheet)
}

#[derive(Debug)]
pub enum StylesheetParseErr<'i> {
    Io(std::io::Error),
    Parse(ParseError<'i, StyleParseErrorKind<'i>>),
}

impl<'i> From<std::io::Error> for StylesheetParseErr<'i> {
    fn from(e: std::io::Error) -> Self {
        StylesheetParseErr::Io(e)
    }
}

//impl<'i> From<(ParseError<'i, StyleParseErrorKind<'i>>, &str)> for StylesheetParseErr<'i> {
//    fn from(e: (ParseError<'i, StyleParseErrorKind<'i>>, &str)) -> Self {
//        StylesheetParseErr::Parse(e.0)
//    }
//}

impl<'i> From<ParseError<'i, StyleParseErrorKind<'i>>> for StylesheetParseErr<'i> {
    fn from(e: ParseError<'i, StyleParseErrorKind<'i>>) -> Self {
        StylesheetParseErr::Parse(e)
    }
}

#[derive(Debug, Default)]
pub struct Stylesheet {
    /// These rules should be de-duplicated before being accepted into the Vec.
    rules: Vec<CssRule>,
}

impl Stylesheet {
    pub fn new() -> Self {
        Stylesheet::default()
    }

    /// Adds a new rule to the stylesheet, de-duplicating rules with the same selectors and
    /// conflicting `property: value`s.
    pub fn add_rule(&mut self, new_rule: CssRule) {
//        match new_rule {
//            CssRule::Style(new_style) => {
//                for existing_rule in self.rules {
//                    match existing_rule {
//                        CssRule::Style(existing_style) => {
//                            if existing_style.selectors.eq(new_style.selectors) {
//                                for existing_prop in existing_style.block.declarations() {
//                                    for new_prop in new_style.block.declarations() {
//                                        if discriminant(new_prop) == discriminant(existing_prop) {
//                                            // the props are the same "type", e.g. both `font-size, both `display`, etc
//                                            // take the `new_prop`, since the latest/newest prop should always be taken
//                                        }
//                                    }
//                                }
//                            }
//                        }
//                        CssRule::None => {}
//                    }
//                }
//            }
//            CssRule::None => {}
//        }
        self.rules.push(new_rule);
    }
}

/// Stylesheets of the user agent (Kosmonaut's default stylesheets).
pub struct UserAgentStylesheets {
    sheets: Vec<Stylesheet>,
}

/// Stylesheets of the browser user.
pub struct UserStylesheets {
    sheets: Vec<Stylesheet>,
}

/// Stylesheets of the author of the webpage (the web developer).
pub struct AuthorStylesheets {
    sheets: Vec<Stylesheet>,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::style::properties::PropertyDeclaration;
    use crate::style::values::specified;
    use crate::style::values::specified::length::*;

    fn selects_last_blocks_prop_in_dupes_across_blocks() {
        assert!(true)
    }
}

// TODO: 1. Fix tests 2. Create test to dedupe props across blocks 3. Fix warnings 4. Cargo fmt
