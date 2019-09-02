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
    Parse(ParseError<'i, StyleParseErrorKind<'i>>)
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
    rules: Vec<CssRule>,
}

impl Stylesheet {
    pub fn new() -> Self {
        Stylesheet::default()
    }

    /// Adds a new rule to the stylesheet, de-depulicating rules with the same selectors and
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
        //                                        if new_prop
        //                                    }
        //                                }
        //                            }
        //                        },
        //                        CssRule::None => {}
        //                    }
        //                }
        //            },
        //            CssRule::None => {}
        //        }
        //    }
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
