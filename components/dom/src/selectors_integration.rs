use crate::attributes::ExpandedName;
use crate::iter::{NodeIterator, Select};
use crate::node_data_ref::NodeDataRef;
use crate::tree::{ElementData, Node, NodeData, NodeRef};
use html5ever::LocalName;
use html5ever::Namespace;
use kosmonaut_selectors::{
    KosmonautSelectors, PseudoClass, PseudoElement, Selector, Selectors, SELECTOR_WHITESPACE,
};
use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::context::QuirksMode;
use selectors::{matching, OpaqueElement};

impl selectors::Element for NodeDataRef<ElementData> {
    type Impl = KosmonautSelectors;

    #[inline]
    fn opaque(&self) -> OpaqueElement {
        let node: &Node = self.as_node();
        OpaqueElement::new(node)
    }

    #[inline]
    fn parent_element(&self) -> Option<Self> {
        self.as_node().parent().and_then(NodeRef::into_element_ref)
    }
    #[inline]
    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }
    #[inline]
    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    #[inline]
    fn prev_sibling_element(&self) -> Option<Self> {
        self.as_node().preceding_siblings().elements().next()
    }
    #[inline]
    fn next_sibling_element(&self) -> Option<Self> {
        self.as_node().following_siblings().elements().next()
    }
    #[inline]
    fn is_html_element_in_html_document(&self) -> bool {
        // TODO: Have a notion of HTML document vs. XML document?
        self.name.ns == ns!(html)
    }

    fn has_local_name(&self, local_name: &LocalName) -> bool {
        self.name.local == *local_name
    }

    fn has_namespace(&self, namespace: &Namespace) -> bool {
        self.name.ns == *namespace
    }

    fn is_same_type(&self, other: &Self) -> bool {
        self.name == other.name
    }

    #[inline]
    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&Namespace>,
        local_name: &LocalName,
        operation: &AttrSelectorOperation<&String>,
    ) -> bool {
        let attrs = self.attributes.borrow();
        match *ns {
            NamespaceConstraint::Any => attrs
                .map
                .iter()
                .any(|(name, attr)| name.local == *local_name && operation.eval_str(&attr.value)),
            NamespaceConstraint::Specific(ref ns_url) => attrs
                .map
                .get(&ExpandedName::new(&(*ns_url).clone(), local_name.clone()))
                .map_or(false, |attr| operation.eval_str(&attr.value)),
        }
    }

    fn match_non_ts_pseudo_class<F>(
        &self,
        pseudo: &PseudoClass,
        _context: &mut matching::MatchingContext<KosmonautSelectors>,
        _flags_setter: &mut F,
    ) -> bool
    where
        F: FnMut(&Self, matching::ElementSelectorFlags),
    {
        use self::PseudoClass::*;
        match *pseudo {
            Active | Focus | Hover | Enabled | Disabled | Checked | Indeterminate | Visited => {
                false
            }
            AnyLink | Link => {
                self.name.ns == ns!(html)
                    && matches!(
                        self.name.local,
                        local_name!("a") | local_name!("area") | local_name!("link")
                    )
                    && self.attributes.borrow().contains(local_name!("href"))
            }
        }
    }
    fn match_pseudo_element(
        &self,
        pseudo: &PseudoElement,
        _context: &mut matching::MatchingContext<KosmonautSelectors>,
    ) -> bool {
        match *pseudo {}
    }

    #[inline]
    fn is_link(&self) -> bool {
        self.name.ns == ns!(html)
            && matches!(
                self.name.local,
                local_name!("a") | local_name!("area") | local_name!("link")
            )
            && self
                .attributes
                .borrow()
                .map
                .contains_key(&ExpandedName::new(ns!(), local_name!("href")))
    }

    #[inline]
    fn is_html_slot_element(&self) -> bool {
        false
    }

    #[inline]
    fn has_id(&self, id: &LocalName, case_sensitivity: CaseSensitivity) -> bool {
        self.attributes
            .borrow()
            .get(local_name!("id"))
            .map_or(false, |id_attr| {
                case_sensitivity.eq(id.as_bytes(), id_attr.as_bytes())
            })
    }

    #[inline]
    fn has_class(&self, name: &LocalName, case_sensitivity: CaseSensitivity) -> bool {
        let name = name.as_bytes();
        !name.is_empty()
            && if let Some(class_attr) = self.attributes.borrow().get(local_name!("class")) {
                class_attr
                    .split(SELECTOR_WHITESPACE)
                    .any(|class| case_sensitivity.eq(class.as_bytes(), name))
            } else {
                false
            }
    }

    fn exported_part(&self, _name: &LocalName) -> Option<LocalName> {
        None
    }

    fn imported_part(&self, _name: &LocalName) -> Option<LocalName> {
        None
    }

    fn is_part(&self, _name: &LocalName) -> bool {
        false
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.as_node().children().all(|child| match *child.data() {
            NodeData::Element(_) => false,
            NodeData::Text(ref text) => text.borrow().is_empty(),
            _ => true,
        })
    }

    #[inline]
    fn is_root(&self) -> bool {
        match self.as_node().parent() {
            None => false,
            Some(parent) => matches!(*parent.data(), NodeData::Document(_)),
        }
    }
}

impl NodeDataRef<ElementData> {
    /// Returns whether the given selector matches this element.
    #[inline]
    pub fn matches(&self, selector: &Selector) -> bool {
        let mut context = matching::MatchingContext::new(
            matching::MatchingMode::Normal,
            None,
            None,
            QuirksMode::NoQuirks,
        );
        matching::matches_selector(
            selector.inner(),
            0,
            None,
            self,
            &mut context,
            &mut |_, _| {},
        )
    }

    /// Returns whether any of the given list of selectors matches this element.
    #[inline]
    pub fn matches_any(&self, selectors: &Selectors) -> bool {
        selectors.inner().iter().any(|s| self.matches(s))
    }

    /// Returns a reference to the most specific matching selector for this element.
    /// https://www.w3.org/TR/selectors/#specificity-rules
    ///   > If the selector is a selector list, this number is calculated for each selector in
    ///   > the list. For a given matching process against the list, the specificity in effect
    ///   > is that of the most specific selector in the list that matches.
    #[inline]
    pub fn most_specific_match<'a>(&self, selectors: &'a Selectors) -> Option<&'a Selector> {
        let mut highest_matching_spec_opt: Option<&Selector> = None;
        selectors.inner().iter().for_each(|selector| {
            if self.matches(selector) {
                match highest_matching_spec_opt {
                    Some(cur_highest) => {
                        if selector.specificity() > cur_highest.specificity() {
                            highest_matching_spec_opt = Some(selector)
                        }
                    }
                    None => highest_matching_spec_opt = Some(selector),
                }
            }
        });
        highest_matching_spec_opt
    }
}

/// Filter an element iterator, yielding those matching the given list of selectors.
#[inline]
pub(super) fn filter_selectors<I>(iter: I, selectors: &Selectors) -> Select<I, &Selectors>
where
    I: Iterator<Item = NodeDataRef<ElementData>>,
{
    Select { iter, selectors }
}
