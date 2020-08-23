use crate::layout::formatting_context::FormattingContextContent;

pub struct BlockFormattingContext {
    /// The contents of this context -- likely a combination of other nested formatting contexts
    /// and boxes.
    contents: Vec<FormattingContextContent>
}

/// A block container either contains only inline-level boxes participating in an inline formatting
/// context, or contains only block-level boxes participating in a block formatting context.
///
/// ### Formatting context establishment
///
/// A block container that contains only inline-level content establishes a new inline formatting
/// context. The element then also generates a root inline box which wraps all of its inline
/// content.
///
/// A block container establishes a new block formatting context if its parent formatting context is
/// not a block formatting context; otherwise, when participating in a block formatting context
/// itself, it either establishes a new block formatting context for its contents or continues the
/// one in which it participates, as determined by the constraints of other properties (such as
/// overflow or align-content).
///
/// https://www.w3.org/TR/css-display/#block-container
pub struct BlockContainer {

}

/// Types of block-level boxes (boxes that participate in block layout).  Note that "participating
/// in block layout" is NOT the same as participating in a block formatting context.  Block-level
/// boxes could be participating in a flex formatting context, grid formatting context, inline
/// formatting context, etc.  In this context, "block layout" refers to the direction in which
/// layout progresses -- in the block direction (y-axis / logical height-wise in the common case).
///
/// https://drafts.csswg.org/css-display/#block-level
pub enum BlockBoxType {
    /// https://drafts.csswg.org/css-display/#block-container
    BlockContainer,
    /// A block-level box that is also a block container (not all block containers are block-level).
    /// https://drafts.csswg.org/css-display/#block-box
    BlockBox
}
