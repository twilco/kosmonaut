use crate::style::values::computed::{ComputeContext, ValueDefault};
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

/// The specified value of a CSS property is the value it receives from the document's style sheet.
/// The specified value for a given property is determined according to the following rules:
///
///   1. If the document's style sheet explicitly specifies a value for the property, the given
///      value will be used.
///   2. If the document's style sheet doesn't specify a value but it is an inherited property, the
///      value will be taken from the parent element.
///   3. If none of the above pertain, the element's initial value will be used.
///
/// https://developer.mozilla.org/en-US/docs/Web/CSS/specified_value
/// https://www.w3.org/TR/CSS22/cascade.html#specified-value

/// Defines an element’s display type, which consists of
/// the two basic qualities of how an element generates boxes
/// <https://drafts.csswg.org/css-display/#propdef-display>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Display {
    None,
    Block,
    //    FlowRoot,
    Inline,
    //    InlineBlock,
    //    ListItem,
    //    Table,
    //    InlineTable,
    //    TableRowGroup,
    //    TableColumn,
    //    TableColumnGroup,
    //    TableHeaderGroup,
    //    TableFooterGroup,
    //    TableRow,
    //    TableCell,
    //    TableCaption,
    //    Flex,
    //    InlineFlex,
    //    Grid,
    //    InlineGrid,
}

/// https://www.w3.org/TR/2019/CR-css-display-3-20190711/#property-index
impl Display {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        try_match_ident_ignore_ascii_case! { input,
            "none" => Ok(Display::None),
            "block" => Ok(Display::Block),
            "inline" => Ok(Display::Inline),
        }
    }

    pub fn initial_value() -> Self {
        Display::Inline
    }
}

impl ValueDefault for Display {
    type ComputedValue = Display;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        Display::initial_value()
    }
}
