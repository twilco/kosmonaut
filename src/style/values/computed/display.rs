use crate::style::values::computed::{ComputeContext, ValueDefault};
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

/// The display property determines how elements generate boxes.
///
/// https://drafts.csswg.org/css-display/#the-display-properties
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Display {
    Full(FullDisplay),
    Box(DisplayBox),
}

impl Display {
    /// Create a new `Display` equivalent to what one would get specifying `display: block` in a
    /// stylesheet.
    pub fn new_block() -> Display {
        Display::new_full_display(OuterDisplay::Block, InnerDisplay::Flow)
    }

    /// Create a new `Display` equivalent to what one would get specifying `display: inline` in a
    /// stylesheet.
    pub fn new_inline() -> Display {
        Display::new_full_display(OuterDisplay::Inline, InnerDisplay::Flow)
    }

    pub fn new_none() -> Display {
        Display::Box(DisplayBox::None)
    }

    pub fn new_full_display(outer: OuterDisplay, inner: InnerDisplay) -> Display {
        Display::Full(FullDisplay::new(outer, inner))
    }

    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        // FIXME: Parse multi-ident 'full display' values (e.g. "block flow", "block flow-root", ...) here.
        try_match_ident_ignore_ascii_case! { input,
            "none" => Ok(Display::new_none()),
            "block" => Ok(Display::new_full_display(OuterDisplay::Block, InnerDisplay::Flow)),
            "flow-root" => Ok(Display::new_full_display(OuterDisplay::Block, InnerDisplay::FlowRoot)),
            "inline" => Ok(Display::new_full_display(OuterDisplay::Inline, InnerDisplay::Flow)),
            "inline-block" => Ok(Display::new_full_display(OuterDisplay::Inline, InnerDisplay::FlowRoot)),
        }
    }

    pub fn initial_value() -> Self {
        Display::new_full_display(OuterDisplay::Inline, InnerDisplay::Flow)
    }
}

impl ValueDefault for Display {
    type ComputedValue = Display;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        Display::initial_value()
    }
}

/// An inner and outer display, which gives us all the information we need to determine what type
/// of box should be generated in layout.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FullDisplay {
    outer: OuterDisplay,
    inner: InnerDisplay,
}

impl FullDisplay {
    pub fn new(outer: OuterDisplay, inner: InnerDisplay) -> FullDisplay {
        FullDisplay { outer, inner }
    }

    pub fn outer(&self) -> OuterDisplay {
        self.outer
    }

    pub fn inner(&self) -> InnerDisplay {
        self.inner
    }
}

/// These values determine how principal boxes participate in flow layout.
///
/// https://drafts.csswg.org/css-display/#outer-display-type
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OuterDisplay {
    Block,
    Inline,
    // TODO(run-in): Will Kosmonaut support run-in boxes?
    // RunIn
}

/// Inner display types define the type of formatting context used to lay out a box's contents.
///
/// https://drafts.csswg.org/css-display/#inner-model
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InnerDisplay {
    Flow,
    FlowRoot,
}

/// https://drafts.csswg.org/css-display/#typedef-display-box
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DisplayBox {
    None,
}
