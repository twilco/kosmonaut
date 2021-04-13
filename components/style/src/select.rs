// /// This file is a direct copy-paste from [Kuchiki](https://github.com/kuchiki-rs/kuchiki/blob/master/src/select.rs).
// /// Thanks to the authors of Kuchiki for their work.
// use crate::dom::attributes::ExpandedName;
// use crate::dom::iter::{NodeIterator, Select};
// use crate::dom::node_data_ref::NodeDataRef;
// use crate::dom::tree::{ElementData, Node, NodeData, NodeRef};
//
// use html5ever::{LocalName, Namespace};
// use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
// use selectors::context::QuirksMode;
// use selectors::parser::{
//     NonTSPseudoClass, Parser, Selector as GenericSelector, SelectorImpl, SelectorList,
//     SelectorParseErrorKind,
// };
// use selectors::{self, matching, OpaqueElement};

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use crate::test_utils::get_div;
//
//     #[test]
//     fn match_most_specific_works() {
//         let selectors =
//             Selectors::compile_str("div, div.specific, div.specific.even-more-specific")
//                 .expect("should've been able to compile selectors in match_most_specific_works()");
//         let div = get_div("specific even-more-specific", "hello");
//         // specificity of most specific selector, `div.specific.even-more-specific`, is 2049
//         assert_eq!(
//             selectors
//                 .most_specific_match(
//                     &div.into_element_ref()
//                         .expect("should be able to get element ref for canned node")
//                 )
//                 .expect("should've found a most-specific match")
//                 .specificity(),
//             Specificity(2049)
//         )
//     }
// }
