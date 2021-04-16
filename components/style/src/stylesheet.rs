use std::mem::discriminant;

use cssparser::ParseError;

use crate::{
    parse_css_to_rules, CascadeOrigin, CssOrigin, CssRule, StyleParseErrorKind, StylesheetOrigin,
};

/// Parses string containing CSS into StyleRules.
pub fn parse_css_to_stylesheet(
    sheet_name: Option<String>,
    css_str: &mut str,
) -> Result<Stylesheet, (ParseError<StyleParseErrorKind>, &str)> {
    let mut sheet = if let Some(name) = sheet_name {
        Stylesheet::new_with_name(name)
    } else {
        Stylesheet::new()
    };
    for rule in parse_css_to_rules(css_str)? {
        sheet.add_rule(rule);
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

impl<'i> From<ParseError<'i, StyleParseErrorKind<'i>>> for StylesheetParseErr<'i> {
    fn from(e: ParseError<'i, StyleParseErrorKind<'i>>) -> Self {
        StylesheetParseErr::Parse(e)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Stylesheet {
    /// Name of the stylesheet
    name: String,
    /// These rules should be de-duplicated before being accepted into the Vec.
    rules: Vec<CssRule>,
}

impl Stylesheet {
    pub fn new() -> Self {
        Stylesheet::default()
    }

    pub fn new_with_name(name: String) -> Self {
        Stylesheet {
            name,
            ..Default::default()
        }
    }

    pub fn css_origin(&self, cascade_origin: CascadeOrigin) -> CssOrigin {
        CssOrigin::Sheet(StylesheetOrigin {
            sheet_name: self.name.clone(),
            cascade_origin,
        })
    }

    pub fn rules(&self) -> &Vec<CssRule> {
        &self.rules
    }

    /// Adds a new rule to the stylesheet, de-duplicating rules with the same selectors and
    /// conflicting `property: value`s.
    pub fn add_rule(&mut self, new_rule: CssRule) {
        let mut obsolete_rule_indices = Vec::new();
        match &new_rule {
            CssRule::Style(new_style) => {
                for (rule_index, existing_rule) in self.rules.iter_mut().enumerate() {
                    match existing_rule {
                        CssRule::Style(existing_style) => {
                            if existing_style.selectors.eq(&new_style.selectors) {
                                let mut obsolete_prop_indices = Vec::new();
                                for (prop_index, existing_prop) in
                                    existing_style.block.declarations().iter().enumerate()
                                {
                                    for new_prop in new_style.block.declarations() {
                                        if discriminant(new_prop) == discriminant(existing_prop) {
                                            // the props are the same "type", e.g. both `font-size, both `display`, etc
                                            // take the `new_prop`, since the latest/newest prop should always be taken
                                            obsolete_prop_indices.push(prop_index);
                                        }
                                    }
                                }

                                for index in obsolete_prop_indices {
                                    existing_style.block.remove_decl(index);
                                }
                                if existing_style.block.declarations().is_empty() {
                                    // we deleted all the declarations in this block, so it is no longer needed
                                    obsolete_rule_indices.push(rule_index)
                                }
                            }
                        }
                        CssRule::None => {}
                    }
                }

                for index in obsolete_rule_indices {
                    self.rules.remove(index);
                }
            }
            CssRule::None => {}
        }
        self.rules.push(new_rule);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::font_size_px_or_panic;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    // TODO: Create integration test that exercises this as well
    fn selects_last_rules_prop_in_dupes_across_rules() {
        let mut sheet_a = parse_css_to_stylesheet(None, &mut ".a { font-size: 12px; }".to_owned())
            .expect("failed getting sheet_a for cross-block deduping test");
        // We won't actually use this sheet — just extract the `font-size` rule from it
        let mut sheet_b = parse_css_to_stylesheet(None, &mut ".a { font-size: 16px; }".to_owned())
            .expect("failed getting sheet_b for cross-block deduping test");
        sheet_a.add_rule(sheet_b.rules.remove(0));

        // The only PropertyDeclaration in the first rule, `font-size: 12px`, is obsoleted by the
        // addition of the new rule containing the `font-size: 16px` PropertyDeclaration.  Therefore
        // it should be deleted entirely, since it has no more valid PropertyDeclarations.
        assert_eq!(sheet_a.rules.len(), 1);
        match sheet_a.rules.remove(0) {
            CssRule::Style(style_rule) => assert_eq!(
                &16.0,
                font_size_px_or_panic(&style_rule.block.declarations()[0])
            ),
            _ => panic!("should always be a `StyleRule` CssRule"),
        }
    }
}
