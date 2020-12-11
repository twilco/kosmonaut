use crate::base_box_passthrough_impls;
use crate::dom::tree::NodeRef;
use crate::layout::dimensions::Dimensions;
use crate::layout::formatting_context::FormattingContextRef;
use crate::layout::layout_box::{BaseBox, LayoutBox};
use crate::layout::{DumpLayoutFormat, Layout, LayoutContext};
use crate::style::values::computed::ComputedValues;
use accountable_refcell::Ref;

/// Content that participates in inline layout. Specifically, inline-level boxes and text runs.
///
/// https://drafts.csswg.org/css-display/#inline-level
#[derive(Clone, Debug, IntoStaticStr)]
pub enum InlineLevelContent {
    InlineLevelBox(InlineLevelBox),
    /// A representation of the contents of a text DOM node.
    ///
    /// https://drafts.csswg.org/css-display-3/#text-run
    TextRun(TextRun),
}

impl InlineLevelContent {
    pub fn computed_values(&self) -> Ref<ComputedValues> {
        match self {
            InlineLevelContent::InlineLevelBox(ilb) => ilb.computed_values(),
            InlineLevelContent::TextRun(tr) => tr.computed_values(),
        }
    }

    pub fn dimensions(&self) -> Dimensions {
        match self {
            InlineLevelContent::InlineLevelBox(ilb) => ilb.dimensions(),
            InlineLevelContent::TextRun(tr) => tr.dimensions(),
        }
    }

    pub fn dimensions_mut(&mut self) -> &mut Dimensions {
        match self {
            InlineLevelContent::InlineLevelBox(ilb) => ilb.dimensions_mut(),
            InlineLevelContent::TextRun(tr) => tr.dimensions_mut(),
        }
    }

    pub fn formatting_context(&self) -> FormattingContextRef {
        match self {
            InlineLevelContent::InlineLevelBox(ilb) => ilb.formatting_context(),
            InlineLevelContent::TextRun(tr) => tr.formatting_context(),
        }
    }

    pub fn is_anonymous_inline(&self) -> bool {
        match self {
            InlineLevelContent::InlineLevelBox(ilb) => ilb.is_anonymous_inline(),
            InlineLevelContent::TextRun(_) => false,
        }
    }
}

impl DumpLayoutFormat for InlineLevelContent {
    fn dump_layout_format(&self) -> String {
        match self {
            InlineLevelContent::InlineLevelBox(ilb) => ilb.dump_layout_format(),
            InlineLevelContent::TextRun(tr) => tr.dump_layout_format(),
        }
    }
}

impl Layout for InlineLevelContent {
    fn layout(&mut self, context: LayoutContext) {
        match self {
            InlineLevelContent::InlineLevelBox(ilb) => ilb.layout(context),
            InlineLevelContent::TextRun(tr) => unimplemented!(
                "layout called on text run with contents",
                tr.contents.clone()
            ),
        }
    }
}

#[derive(Clone, Debug, IntoStaticStr)]
pub enum InlineLevelBox {
    /// An inline-level box not associated with any element.
    ///
    /// An example of an anonymous inline box is the root inline box generated by block containers
    /// who have inline content that needs a place to go.
    ///
    /// For more information about this box type, see: https://drafts.csswg.org/css-inline-3/#model
    ///
    /// An aside, quoting https://drafts.csswg.org/css-display/#block-container:
    ///   > Note, this root inline box concept effectively replaces the "anonymous inline element"
    ///     concept introduced in CSS2§9.2.2.1.
    AnonymousInline(AnonymousInlineBox),
    /// A non-replaced inline-level box whose inner display type is flow. The contents of an inline
    /// box participate in the same inline formatting context as the inline box itself.
    ///
    /// This is also known as an "inline-block".
    ///
    /// https://drafts.csswg.org/css-display/#inline-box
    InlineBox(InlineBox),
}

impl InlineLevelBox {
    pub fn add_child(&mut self, new_child: LayoutBox) {
        match self {
            InlineLevelBox::AnonymousInline(aib) => aib.children.push(new_child),
            InlineLevelBox::InlineBox(ib) => ib.children.push(new_child),
        }
    }

    pub fn children(&self) -> &Vec<LayoutBox> {
        match self {
            InlineLevelBox::AnonymousInline(aib) => aib.children(),
            InlineLevelBox::InlineBox(ib) => ib.children(),
        }
    }

    pub fn computed_values(&self) -> Ref<ComputedValues> {
        match self {
            InlineLevelBox::AnonymousInline(aib) => aib.computed_values(),
            InlineLevelBox::InlineBox(ib) => ib.computed_values(),
        }
    }

    pub fn dimensions(&self) -> Dimensions {
        match self {
            InlineLevelBox::AnonymousInline(aib) => aib.dimensions(),
            InlineLevelBox::InlineBox(ib) => ib.dimensions(),
        }
    }

    pub fn dimensions_mut(&mut self) -> &mut Dimensions {
        match self {
            InlineLevelBox::AnonymousInline(aib) => aib.dimensions_mut(),
            InlineLevelBox::InlineBox(ib) => ib.dimensions_mut(),
        }
    }

    pub fn formatting_context(&self) -> FormattingContextRef {
        match self {
            InlineLevelBox::AnonymousInline(aib) => aib.formatting_context(),
            InlineLevelBox::InlineBox(ib) => ib.formatting_context(),
        }
    }

    pub fn is_anonymous_inline(&self) -> bool {
        match self {
            InlineLevelBox::AnonymousInline(_) => true,
            InlineLevelBox::InlineBox(_) => false,
        }
    }
}

impl DumpLayoutFormat for InlineLevelBox {
    fn dump_layout_format(&self) -> String {
        match self {
            InlineLevelBox::AnonymousInline(aib) => aib.dump_layout_format(),
            InlineLevelBox::InlineBox(ib) => ib.dump_layout_format(),
        }
    }
}

impl Layout for InlineLevelBox {
    fn layout(&mut self, _context: LayoutContext) {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct AnonymousInlineBox {
    base: BaseBox,
    children: Vec<LayoutBox>,
}

impl AnonymousInlineBox {
    base_box_passthrough_impls!();

    pub fn new(node: NodeRef, formatting_context: FormattingContextRef) -> Self {
        Self {
            base: BaseBox::new(node, formatting_context),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: LayoutBox) {
        self.children.push(child)
    }

    pub fn children(&self) -> &Vec<LayoutBox> {
        &self.children
    }
}

impl DumpLayoutFormat for AnonymousInlineBox {
    fn dump_layout_format(&self) -> String {
        // Anonymous boxes are not generaed from an element of the DOM, so return empty string.
        // `impl DumpLayout for LayoutBox` _should_ log this boxes type (AnonymousInlineBox)
        // TODO: ...it would probably be better just to log DOM node data _and_ box type here and
        // in other box DumpLayoutFormat impls
        "".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct InlineBox {
    base: BaseBox,
    children: Vec<LayoutBox>,
}

impl InlineBox {
    base_box_passthrough_impls!();

    pub fn new(node: NodeRef, formatting_context: FormattingContextRef) -> Self {
        Self {
            base: BaseBox::new(node, formatting_context),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: LayoutBox) {
        self.children.push(child)
    }

    pub fn children(&self) -> &Vec<LayoutBox> {
        &self.children
    }
}

impl DumpLayoutFormat for InlineBox {
    fn dump_layout_format(&self) -> String {
        self.node().data().dump_layout_format()
    }
}

/// A representation of the contents of a text DOM node.
///
/// https://drafts.csswg.org/css-display-3/#text-run
#[derive(Clone, Debug)]
pub struct TextRun {
    base: BaseBox,
    /// The text contents of the node.
    ///
    /// TODO: This can be an owned String for now for simplicity's sake, but it would be probably
    /// be more efficient if this were a `&'DOM_LIFETIME str`.
    contents: String,
}

impl TextRun {
    base_box_passthrough_impls!();

    pub fn new(node: NodeRef, formatting_context: FormattingContextRef, contents: String) -> Self {
        Self {
            base: BaseBox::new(node, formatting_context),
            contents,
        }
    }
}

impl DumpLayoutFormat for TextRun {
    fn dump_layout_format(&self) -> String {
        self.node().data().dump_layout_format()
    }
}
