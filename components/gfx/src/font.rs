use accountable_refcell::{Ref, RefCell};
use font_kit::error::{FontLoadingError, SelectionError};
use font_kit::loaders::default::Font;
use font_kit::source::SystemSource;
use std::collections::HashMap;

/// Provides a handle for loading and caching fonts that abstracts over all different font loaders
/// and sources.
#[derive(Default)]
struct FontHandle {
    cached_fonts: RefCell<HashMap<String, Font>>,
}

pub(super) type PostscriptName = String;

impl FontHandle {
    fn new() -> FontHandle {
        FontHandle {
            cached_fonts: RefCell::new(HashMap::new()),
        }
    }

    fn get_font(&self, postscript_name: &str) -> Result<Ref<Font>, FontError> {
        let key = postscript_name.to_owned();
        {
            let mut cached_fonts = self.cached_fonts.borrow_mut();
            if !cached_fonts.contains_key(&key) {
                let font = load_font(postscript_name)?;
                cached_fonts.insert(key.clone(), font);
            }
        }
        let cached_fonts = self.cached_fonts.borrow();
        let font = Ref::map(cached_fonts, |fonts| fonts.get(&key).unwrap());
        Ok(font)
    }
}

fn load_font(postscript_name: &str) -> Result<Font, FontError> {
    Ok(SystemSource::new()
        .select_by_postscript_name(postscript_name)?
        .load()?)
}

// TODO: This error type seems a bit too general.  May want to refactor as this module evolves.
#[derive(Debug)]
pub(super) enum FontError {
    Loading(FontLoadingError),
    Selection(SelectionError),
}

impl From<FontLoadingError> for FontError {
    fn from(err: FontLoadingError) -> Self {
        FontError::Loading(err)
    }
}

impl From<SelectionError> for FontError {
    fn from(err: SelectionError) -> Self {
        FontError::Selection(err)
    }
}
