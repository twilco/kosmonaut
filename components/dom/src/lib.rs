/// Much of this module is a direct copy-paste from [Kuchiki](https://github.com/kuchiki-rs/kuchiki).
/// Kuchiki's DOM implementation seems like it will work well for Kosmonaut, but unfortunately
/// Kuchiki doesn't expose some key structures, such as its implementation of `selectors::Parser`,
/// that we need in Kosmonaut.  The authors of Kosmonaut will upstream any improvements back to
/// Kuchiki where it makes sense.  Thanks to the authors of Kuchiki for their work.
pub mod attributes;
pub mod cell_extras;
pub mod iter;
pub mod node_data_ref;
pub mod parser;
pub mod selectors_integration;
pub mod serializer;
pub mod styling;
#[cfg(test)]
pub mod tests;
pub mod tree;

#[macro_use]
extern crate html5ever;
