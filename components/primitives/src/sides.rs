#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PhysicalSide {
    Bottom,
    Left,
    Right,
    Top,
}

/// Sides relative to the flow of the page, rather than physical sides (e.g. left, top, ...).
///
/// https://drafts.csswg.org/css-writing-modes-4/#logical-directions
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlowSide {
    BlockStart,
    BlockEnd,
    InlineStart,
    InlineEnd,
}
