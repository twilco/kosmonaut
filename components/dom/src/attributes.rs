/// Convenience methods arround XML attributes.
///
/// Most of this file is a direct copy-paste from [Kuchiki](https://github.com/kuchiki-rs/kuchiki/blob/master/src/attributes.rs).
/// Thanks to the authors of Kuchiki for their work.
use html5ever::{LocalName, Namespace, Prefix};
use std::collections::btree_map::{BTreeMap, Entry};

/// Convenience wrapper around a btreemap that adds method for attributes in the null namespace.
#[derive(Debug, PartialEq, Clone)]
pub struct Attributes {
    /// A map of attributes whose name can have namespaces.
    pub map: BTreeMap<ExpandedName, Attribute>,
}

/// <https://www.w3.org/TR/REC-xml-names/#dt-expname>
/// The combination of namespace name + localname creates an _ExpandedName_.  It is the combination
/// of URI namespaces with local names that prevent clashes in definitions of tags and attributes
/// in XML.
///
/// <x xmlns:edi='http://ecommerce.example.org/schema'>
///  <!-- the 'taxClass' attribute's namespace is http://ecommerce.example.org/schema -->
///  <edi:lineItem edi:taxClass="exempt">Baby food</edi:lineItem>
/// </x>
///
/// The above XML declares the _namespace_ for the `edi` prefix as http://ecommerce.example.org/schema.
/// In `<edi:lineItem ...>`, `lineItem` is the _local name_.
///
/// `ExpandedName` is different from a `QualifiedName`, in that a `QualifiedName` is a
/// `prefix:localName` OR simply just a `localName`.  Prefixes can be bound to namespaces.
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct ExpandedName {
    /// Namespace URI
    /// Namespace is a map from prefixes (e.g. `edi`) to namespace URIs.
    /// <https://en.m.wikipedia.org/wiki/XML_namespace#Namespace_names>
    pub ns: Namespace,
    /// "Local" part of the name
    /// e.g. `lineItem` in `edi:lineItem`
    pub local: LocalName,
}

impl ExpandedName {
    /// Trivial constructor
    pub fn new<N: Into<Namespace>, L: Into<LocalName>>(ns: N, local: L) -> Self {
        ExpandedName {
            ns: ns.into(),
            local: local.into(),
        }
    }
}

/// The non-identifying parts of an attribute.
/// Example: In `<Book metadata:pageCount="643">`, `metadata` is the _prefix_ and `643` is the _value_.
#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    /// The namespace prefix, if any
    pub prefix: Option<Prefix>,
    /// The attribute value
    pub value: String,
}

impl Attributes {
    /// Like BTreeMap::contains
    pub fn contains<A: Into<LocalName>>(&self, local_name: A) -> bool {
        self.map.contains_key(&ExpandedName::new(ns!(), local_name))
    }

    /// Like BTreeMap::get
    pub fn get<A: Into<LocalName>>(&self, local_name: A) -> Option<&str> {
        self.map
            .get(&ExpandedName::new(ns!(), local_name))
            .map(|attr| &*attr.value)
    }

    /// Like BTreeMap::get_mut
    pub fn get_mut<A: Into<LocalName>>(&mut self, local_name: A) -> Option<&mut String> {
        self.map
            .get_mut(&ExpandedName::new(ns!(), local_name))
            .map(|attr| &mut attr.value)
    }

    /// Like BTreeMap::entry
    pub fn entry<A: Into<LocalName>>(&mut self, local_name: A) -> Entry<ExpandedName, Attribute> {
        self.map.entry(ExpandedName::new(ns!(), local_name))
    }

    /// Like BTreeMap::insert
    pub fn insert<A: Into<LocalName>>(
        &mut self,
        local_name: A,
        value: String,
    ) -> Option<Attribute> {
        self.map.insert(
            ExpandedName::new(ns!(), local_name),
            Attribute {
                prefix: None,
                value,
            },
        )
    }

    /// Like BTreeMap::remove
    pub fn remove<A: Into<LocalName>>(&mut self, local_name: A) -> Option<Attribute> {
        self.map.remove(&ExpandedName::new(ns!(), local_name))
    }
}
