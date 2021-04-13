use crate::tree::{NodeData, NodeRef};
use style::properties::ContextualPropertyDeclaration;
use style::stylesheet::Stylesheet;
use style::values::computed::{compute_values, ComputedValues};
use style::{CascadeOrigin, CssOrigin, CssRule};

pub fn apply_styles(
    dom: NodeRef,
    embedded_styles: &[CssRule],
    ua_sheets: &[Stylesheet],
    user_sheets: &[Stylesheet],
    author_sheets: &[Stylesheet],
) {
    // https://www.w3.org/TR/css-cascade-3/#value-stages
    // The final value of a CSS property for a given element or box is the result of a multi-step calculation:

    // 1. First, all the declared values applied to an element are collected, for each property on each element. There may be zero or many declared values applied to the element.
    ua_sheets.iter().for_each(|stylesheet| {
        apply_css_rules_to_node(
            &dom,
            stylesheet.rules(),
            stylesheet.css_origin(CascadeOrigin::UserAgent),
        );
    });

    user_sheets.iter().for_each(|stylesheet| {
        apply_css_rules_to_node(
            &dom,
            stylesheet.rules(),
            stylesheet.css_origin(CascadeOrigin::User),
        );
    });

    author_sheets.iter().for_each(|stylesheet| {
        apply_css_rules_to_node(
            &dom,
            stylesheet.rules(),
            stylesheet.css_origin(CascadeOrigin::Author),
        );
    });
    apply_css_rules_to_node(&dom, embedded_styles, CssOrigin::Embedded);
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
        // Step 2
        node.contextual_decls_mut().cascade_sort();

        // Step 3 and 4
        // If this is the root node (aka there is no parent to inherit properties from), just
        // default all properties to their initial values.
        let parent_computed_values = node.parent().map_or(ComputedValues::default(), |p| {
            // TODO: This _could_ be an expensive clone when we actually support all CSS properties.
            p.computed_values().clone()
        });
        *node.computed_values_mut() =
            compute_values(&*node.contextual_decls(), &parent_computed_values);
    });
}

pub fn extract_embedded_styles(node: NodeRef) -> String {
    node.inclusive_descendants()
        .filter(|child| {
            if let NodeData::Element(element_data) = child.data() {
                local_name!("style") == element_data.name.local
            } else {
                false
            }
        })
        .fold(String::new(), |accumulator, style_node| {
            let styles_in_style_node =
                style_node
                    .children()
                    .fold(String::new(), |node_accumulator, child| {
                        let style_contents = if let NodeData::Text(contents) = child.data() {
                            contents.take()
                        } else {
                            "".to_owned()
                        };
                        node_accumulator + style_contents.trim()
                    });
            accumulator + &styles_in_style_node
        })
}

pub fn apply_css_rules_to_node(node: &NodeRef, rules: &[CssRule], origin: CssOrigin) {
    rules.iter().for_each(|rule| {
        if let CssRule::Style(style_rule) = rule {
            node.select(&style_rule.selectors)
                .for_each(|matching_node| {
                    style_rule
                        .block
                        .declarations()
                        .iter()
                        .enumerate()
                        .for_each(|(index, decl)| {
                            matching_node.as_node().add_decl(ContextualPropertyDeclaration {
                                inner_decl: decl.clone(),
                                important: style_rule
                                    .block
                                    .declarations_importance()
                                    .get(index)
                                    .expect("important bit not set for declaration"),
                                origin: origin.clone(),
                                source_location: Some(style_rule.source_location),
                                specificity: matching_node.most_specific_match(&style_rule.selectors).expect("there should be at least one matching selector at this point").specificity()
                            });
                        });
                });
        }
    });
}
