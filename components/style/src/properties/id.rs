use crate::properties::{ContextualPropertyDeclaration, PropertyDeclaration};
use crate::values::computed::{ComputeContext, ComputedValuesBuilder, LineStyle, ValueDefault};
use crate::values::{computed, specified};

/// Representation of a CSS property, that is, either a longhand, a
/// shorthand, or a custom property.
///
/// This, and substructures within this enum, taken from [Servo](https://github.com/servo/servo).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum PropertyId {
    /// A longhand property.
    Longhand(LonghandId),
    /// A shorthand property.
    Shorthand(ShorthandId),
    // TODO: Support custom properties
    //    /// A custom property.
    //    Custom(crate::custom_properties::Name),
}

impl PropertyId {
    pub(super) fn parse(prop_name: &str) -> Option<PropertyId> {
        let id = match prop_name {
            // Longhands
            "background-color" => PropertyId::Longhand(LonghandId::BackgroundColor),
            "border-bottom-color" => PropertyId::Longhand(LonghandId::BorderBottomColor),
            "border-left-color" => PropertyId::Longhand(LonghandId::BorderLeftColor),
            "border-right-color" => PropertyId::Longhand(LonghandId::BorderRightColor),
            "border-top-color" => PropertyId::Longhand(LonghandId::BorderTopColor),
            "border-bottom-style" => PropertyId::Longhand(LonghandId::BorderBottomStyle),
            "border-left-style" => PropertyId::Longhand(LonghandId::BorderLeftStyle),
            "border-right-style" => PropertyId::Longhand(LonghandId::BorderRightStyle),
            "border-top-style" => PropertyId::Longhand(LonghandId::BorderTopStyle),
            "border-bottom-width" => PropertyId::Longhand(LonghandId::BorderBottomWidth),
            "border-left-width" => PropertyId::Longhand(LonghandId::BorderLeftWidth),
            "border-right-width" => PropertyId::Longhand(LonghandId::BorderRightWidth),
            "border-top-width" => PropertyId::Longhand(LonghandId::BorderTopWidth),
            "color" => PropertyId::Longhand(LonghandId::Color),
            "direction" => PropertyId::Longhand(LonghandId::Direction),
            "display" => PropertyId::Longhand(LonghandId::Display),
            //            "float" => PropertyId::Longhand(LonghandId::Float),
            //            "font-style" => PropertyId::Longhand(LonghandId::FontStyle),
            //            "font-weight" => PropertyId::Longhand(LonghandId::FontWeight),
            //            "visibility" => PropertyId::Longhand(LonghandId::Visibility),
            "font-size" => PropertyId::Longhand(LonghandId::FontSize),
            "height" => PropertyId::Longhand(LonghandId::Height),
            "margin-bottom" => PropertyId::Longhand(LonghandId::MarginBottom),
            "margin-left" => PropertyId::Longhand(LonghandId::MarginLeft),
            "margin-right" => PropertyId::Longhand(LonghandId::MarginRight),
            "margin-top" => PropertyId::Longhand(LonghandId::MarginTop),
            "padding-bottom" => PropertyId::Longhand(LonghandId::PaddingBottom),
            "padding-left" => PropertyId::Longhand(LonghandId::PaddingLeft),
            "padding-right" => PropertyId::Longhand(LonghandId::PaddingRight),
            "padding-top" => PropertyId::Longhand(LonghandId::PaddingTop),
            "width" => PropertyId::Longhand(LonghandId::Width),
            "writing-mode" => PropertyId::Longhand(LonghandId::WritingMode),
            // Shorthands
            "background" => PropertyId::Shorthand(ShorthandId::Background),
            "border-color" => PropertyId::Shorthand(ShorthandId::BorderColor),
            "border-style" => PropertyId::Shorthand(ShorthandId::BorderStyle),
            "border-width" => PropertyId::Shorthand(ShorthandId::BorderWidth),
            "border-top" => PropertyId::Shorthand(ShorthandId::BorderTop),
            "border-right" => PropertyId::Shorthand(ShorthandId::BorderRight),
            "border-bottom" => PropertyId::Shorthand(ShorthandId::BorderBottom),
            "border-left" => PropertyId::Shorthand(ShorthandId::BorderLeft),
            "border" => PropertyId::Shorthand(ShorthandId::Border),
            "margin" => PropertyId::Shorthand(ShorthandId::Margin),
            "padding" => PropertyId::Shorthand(ShorthandId::Padding),
            _ => return None,
        };
        Some(id)
    }
}

/// An identifier for a given longhand property.
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq)]
#[repr(u16)]
pub(crate) enum LonghandId {
    //    /// align-content
    //    AlignContent = 0,
    //    /// align-items
    //    AlignItems = 1,
    //    /// align-self
    //    AlignSelf = 2,
    //    /// aspect-ratio
    //    AspectRatio = 3,
    //    /// backface-visibility
    //    BackfaceVisibility = 4,
    //    /// border-collapse
    //    BorderCollapse = 5,
    //    /// border-image-repeat
    //    BorderImageRepeat = 6,
    //    /// box-sizing
    //    BoxSizing = 7,
    //    /// caption-side
    //    CaptionSide = 8,
    //    /// clear
    //    Clear = 9,
    //    /// column-count
    //    ColumnCount = 10,
    /// direction
    Direction = 11,
    /// display
    Display = 12,
    //    /// empty-cells
    //    EmptyCells = 13,
    //    /// flex-direction
    //    FlexDirection = 14,
    //    /// flex-wrap
    //    FlexWrap = 15,
    //    /// float
    //    Float = 16,
    //    /// font-stretch
    //    FontStretch = 17,
    //    /// font-style
    //    FontStyle = 18,
    //    /// font-variant-caps
    //    FontVariantCaps = 19,
    //    /// font-weight
    //    FontWeight = 20,
    //    /// image-rendering
    //    ImageRendering = 21,
    //    /// justify-content
    //    JustifyContent = 22,
    //    /// list-style-position
    //    ListStylePosition = 23,
    //    /// list-style-type
    //    ListStyleType = 24,
    //    /// mix-blend-mode
    //    MixBlendMode = 25,
    //    /// opacity
    //    Opacity = 26,
    //    /// order
    //    Order = 27,
    //    /// outline-style
    //    OutlineStyle = 28,
    //    /// overflow-wrap
    //    OverflowWrap = 29,
    //    /// pointer-events
    //    PointerEvents = 30,
    //    /// position
    //    Position = 31,
    //    /// table-layout
    //    TableLayout = 32,
    //    /// text-align
    //    TextAlign = 33,
    //    /// text-decoration-line
    //    TextDecorationLine = 34,
    //    /// text-justify
    //    TextJustify = 35,
    //    /// text-rendering
    //    TextRendering = 36,
    //    /// text-transform
    //    TextTransform = 37,
    //    /// transform-style
    //    TransformStyle = 38,
    //    /// unicode-bidi
    //    UnicodeBidi = 39,
    //    /// visibility
    //    Visibility = 40,
    //    /// white-space
    //    WhiteSpace = 41,
    //    /// word-break
    //    WordBreak = 42,
    /// writing-mode
    WritingMode = 43,
    //    /// z-index
    //    ZIndex = 44,
    //    /// flex-grow
    //    FlexGrow = 45,
    //    /// flex-shrink
    //    FlexShrink = 46,
    //    /// overflow-block
    //    OverflowBlock = 47,
    //    /// overflow-inline
    //    OverflowInline = 48,
    //    /// overflow-x
    //    OverflowX = 49,
    //    /// overflow-y
    //    OverflowY = 50,
    //    /// border-block-end-style
    //    BorderBlockEndStyle = 51,
    //    /// border-block-start-style
    //    BorderBlockStartStyle = 52,
    /// border-bottom-style
    BorderBottomStyle = 53,
    //    /// border-inline-end-style
    //    BorderInlineEndStyle = 54,
    //    /// border-inline-start-style
    //    BorderInlineStartStyle = 55,
    /// border-left-style
    BorderLeftStyle = 56,
    /// border-right-style
    BorderRightStyle = 57,
    /// border-top-style
    BorderTopStyle = 58,
    //    /// animation-delay
    //    AnimationDelay = 59,
    //    /// animation-direction
    //    AnimationDirection = 60,
    //    /// animation-duration
    //    AnimationDuration = 61,
    //    /// animation-fill-mode
    //    AnimationFillMode = 62,
    //    /// animation-iteration-count
    //    AnimationIterationCount = 63,
    //    /// animation-name
    //    AnimationName = 64,
    //    /// animation-play-state
    //    AnimationPlayState = 65,
    //    /// animation-timing-function
    //    AnimationTimingFunction = 66,
    //    /// background-attachment
    //    BackgroundAttachment = 67,
    //    /// background-clip
    //    BackgroundClip = 68,
    //    /// background-image
    //    BackgroundImage = 69,
    //    /// background-origin
    //    BackgroundOrigin = 70,
    //    /// background-position-x
    //    BackgroundPositionX = 71,
    //    /// background-position-y
    //    BackgroundPositionY = 72,
    //    /// background-repeat
    //    BackgroundRepeat = 73,
    //    /// background-size
    //    BackgroundSize = 74,
    //    /// border-image-outset
    //    BorderImageOutset = 75,
    //    /// border-image-slice
    //    BorderImageSlice = 76,
    //    /// border-image-source
    //    BorderImageSource = 77,
    //    /// border-image-width
    //    BorderImageWidth = 78,
    //    /// border-spacing
    //    BorderSpacing = 79,
    //    /// box-shadow
    //    BoxShadow = 80,
    //    /// clip
    //    Clip = 81,
    /// color
    Color = 82,
    //    /// column-gap
    //    ColumnGap = 83,
    //    /// column-width
    //    ColumnWidth = 84,
    //    /// content
    //    Content = 85,
    //    /// counter-increment
    //    CounterIncrement = 86,
    //    /// counter-reset
    //    CounterReset = 87,
    //    /// cursor
    //    Cursor = 88,
    //    /// filter
    //    Filter = 89,
    //    /// flex-basis
    //    FlexBasis = 90,
    //    /// font-family
    //    FontFamily = 91,
    /// font-size
    FontSize = 92,
    //    /// letter-spacing
    //    LetterSpacing = 93,
    //    /// line-height
    //    LineHeight = 94,
    //    /// list-style-image
    //    ListStyleImage = 95,
    //    /// outline-offset
    //    OutlineOffset = 96,
    //    /// perspective
    //    Perspective = 97,
    //    /// perspective-origin
    //    PerspectiveOrigin = 98,
    //    /// quotes
    //    Quotes = 99,
    //    /// rotate
    //    Rotate = 100,
    //    /// scale
    //    Scale = 101,
    //    /// text-indent
    //    TextIndent = 102,
    //    /// text-overflow
    //    TextOverflow = 103,
    //    /// text-shadow
    //    TextShadow = 104,
    //    /// transform
    //    Transform = 105,
    //    /// transform-origin
    //    TransformOrigin = 106,
    //    /// transition-delay
    //    TransitionDelay = 107,
    //    /// transition-duration
    //    TransitionDuration = 108,
    //    /// transition-property
    //    TransitionProperty = 109,
    //    /// transition-timing-function
    //    TransitionTimingFunction = 110,
    //    /// translate
    //    Translate = 111,
    //    /// vertical-align
    //    VerticalAlign = 112,
    //    /// word-spacing
    //    WordSpacing = 113,
    //    /// max-block-size
    //    MaxBlockSize = 114,
    //    /// max-height
    //    MaxHeight = 115,
    //    /// max-inline-size
    //    MaxInlineSize = 116,
    //    /// max-width
    //    MaxWidth = 117,
    //    /// border-bottom-left-radius
    //    BorderBottomLeftRadius = 118,
    //    /// border-bottom-right-radius
    //    BorderBottomRightRadius = 119,
    //    /// border-end-end-radius
    //    BorderEndEndRadius = 120,
    //    /// border-end-start-radius
    //    BorderEndStartRadius = 121,
    //    /// border-start-end-radius
    //    BorderStartEndRadius = 122,
    //    /// border-start-start-radius
    //    BorderStartStartRadius = 123,
    //    /// border-top-left-radius
    //    BorderTopLeftRadius = 124,
    //    /// border-top-right-radius
    //    BorderTopRightRadius = 125,
    //    /// padding-block-end
    //    PaddingBlockEnd = 126,
    //    /// padding-block-start
    //    PaddingBlockStart = 127,
    /// padding-bottom
    PaddingBottom = 128,
    /// padding-inline-end
    //    PaddingInlineEnd = 129,
    //    /// padding-inline-start
    //    PaddingInlineStart = 130,
    //    /// padding-left
    PaddingLeft = 131,
    /// padding-right
    PaddingRight = 132,
    /// padding-top
    PaddingTop = 133,
    //    /// block-size
    //    BlockSize = 134,
    /// height
    Height = 135,
    //    /// inline-size
    //    InlineSize = 136,
    //    /// min-block-size
    //    MinBlockSize = 137,
    //    /// min-height
    //    MinHeight = 138,
    //    /// min-inline-size
    //    MinInlineSize = 139,
    //    /// min-width
    //    MinWidth = 140,
    /// width
    Width = 141,
    //    /// border-block-end-width
    //    BorderBlockEndWidth = 142,
    //    /// border-block-start-width
    //    BorderBlockStartWidth = 143,
    /// border-bottom-width
    BorderBottomWidth = 144,
    //    /// border-inline-end-width
    //    BorderInlineEndWidth = 145,
    //    /// border-inline-start-width
    //    BorderInlineStartWidth = 146,
    /// border-left-width
    BorderLeftWidth = 147,
    /// border-right-width
    BorderRightWidth = 148,
    /// border-top-width
    BorderTopWidth = 149,
    //    /// outline-width
    //    OutlineWidth = 150,
    /// background-color
    BackgroundColor = 151,
    //    /// border-block-end-color
    //    BorderBlockEndColor = 152,
    //    /// border-block-start-color
    //    BorderBlockStartColor = 153,
    /// border-bottom-color
    BorderBottomColor = 154,
    //    /// border-inline-end-color
    //    BorderInlineEndColor = 155,
    //    /// border-inline-start-color
    //    BorderInlineStartColor = 156,
    /// border-left-color
    BorderLeftColor = 157,
    /// border-right-color
    BorderRightColor = 158,
    /// border-top-color
    BorderTopColor = 159,
    //    /// outline-color
    //    OutlineColor = 160,
    //    /// bottom
    //    Bottom = 161,
    //    /// inset-block-end
    //    InsetBlockEnd = 162,
    //    /// inset-block-start
    //    InsetBlockStart = 163,
    //    /// inset-inline-end
    //    InsetInlineEnd = 164,
    //    /// inset-inline-start
    //    InsetInlineStart = 165,
    //    /// left
    //    Left = 166,
    //    /// margin-block-end
    //    MarginBlockEnd = 167,
    //    /// margin-block-start
    //    MarginBlockStart = 168,
    /// margin-bottom
    MarginBottom = 169,
    //    /// margin-inline-end
    //    MarginInlineEnd = 170,
    //    /// margin-inline-start
    //    MarginInlineStart = 171,
    /// margin-left
    MarginLeft = 172,
    /// margin-right
    MarginRight = 173,
    /// margin-top
    MarginTop = 174,
    //    /// right
    //    Right = 175,
    //    /// top
    //    Top = 176,
}

impl LonghandId {
    #[allow(unreachable_patterns)]
    pub(crate) fn value_default(
        self,
        cv_builder: &mut ComputedValuesBuilder,
        ctx: &ComputeContext,
    ) {
        match self {
            LonghandId::BackgroundColor => {
                cv_builder.background_color(specified::BackgroundColor::value_default(ctx));
            }
            LonghandId::BorderBottomColor => {
                cv_builder.border_bottom_color(specified::BorderColor::value_default(ctx));
            }
            LonghandId::BorderLeftColor => {
                cv_builder.border_left_color(specified::BorderColor::value_default(ctx));
            }
            LonghandId::BorderRightColor => {
                cv_builder.border_right_color(specified::BorderColor::value_default(ctx));
            }
            LonghandId::BorderTopColor => {
                cv_builder.border_top_color(specified::BorderColor::value_default(ctx));
            }
            LonghandId::BorderBottomStyle => {
                cv_builder.border_bottom_style(LineStyle::None);
            }
            LonghandId::BorderLeftStyle => {
                cv_builder.border_left_style(LineStyle::None);
            }
            LonghandId::BorderRightStyle => {
                cv_builder.border_right_style(LineStyle::None);
            }
            LonghandId::BorderTopStyle => {
                cv_builder.border_top_style(LineStyle::None);
            }
            LonghandId::BorderBottomWidth => {
                cv_builder.border_bottom_width(specified::BorderBottomWidth::value_default(ctx));
            }
            LonghandId::BorderLeftWidth => {
                cv_builder.border_left_width(specified::BorderLeftWidth::value_default(ctx));
            }
            LonghandId::BorderRightWidth => {
                cv_builder.border_right_width(specified::BorderRightWidth::value_default(ctx));
            }
            LonghandId::BorderTopWidth => {
                cv_builder.border_top_width(specified::BorderTopWidth::value_default(ctx));
            }
            LonghandId::Color => {
                cv_builder.color(specified::Color::value_default(ctx));
            }
            LonghandId::Direction => {
                cv_builder.direction(computed::Direction::value_default(ctx));
            }
            LonghandId::Display => {
                cv_builder.display(computed::Display::value_default(ctx));
            }
            LonghandId::FontSize => {
                cv_builder.font_size(specified::FontSize::value_default(ctx));
            }
            LonghandId::Height => {
                cv_builder.height(specified::Height::value_default(ctx));
            }
            LonghandId::MarginBottom => {
                cv_builder.margin_bottom(specified::Margin::value_default(ctx));
            }
            LonghandId::MarginLeft => {
                cv_builder.margin_left(specified::Margin::value_default(ctx));
            }
            LonghandId::MarginRight => {
                cv_builder.margin_right(specified::Margin::value_default(ctx));
            }
            LonghandId::MarginTop => {
                cv_builder.margin_top(specified::Margin::value_default(ctx));
            }
            LonghandId::PaddingBottom => {
                cv_builder.padding_bottom(specified::Padding::value_default(ctx));
            }
            LonghandId::PaddingLeft => {
                cv_builder.padding_left(specified::Padding::value_default(ctx));
            }
            LonghandId::PaddingRight => {
                cv_builder.padding_right(specified::Padding::value_default(ctx));
            }
            LonghandId::PaddingTop => {
                cv_builder.padding_top(specified::Padding::value_default(ctx));
            }
            LonghandId::Width => {
                cv_builder.width(specified::Width::value_default(ctx));
            }
            LonghandId::WritingMode => {
                cv_builder.writing_mode(computed::WritingMode::value_default(ctx));
            }
            _ => unimplemented!(
                "{}",
                format!("value default by longhand for id: {:?}", self)
            ),
        }
    }
}

impl From<&PropertyDeclaration> for LonghandId {
    fn from(prop_decl: &PropertyDeclaration) -> Self {
        match prop_decl {
            PropertyDeclaration::BackgroundColor(_) => LonghandId::BackgroundColor,
            PropertyDeclaration::BorderBottomColor(_) => LonghandId::BorderBottomColor,
            PropertyDeclaration::BorderLeftColor(_) => LonghandId::BorderLeftColor,
            PropertyDeclaration::BorderRightColor(_) => LonghandId::BorderRightColor,
            PropertyDeclaration::BorderTopColor(_) => LonghandId::BorderTopColor,
            PropertyDeclaration::BorderBottomStyle(_) => LonghandId::BorderBottomStyle,
            PropertyDeclaration::BorderLeftStyle(_) => LonghandId::BorderLeftStyle,
            PropertyDeclaration::BorderRightStyle(_) => LonghandId::BorderRightStyle,
            PropertyDeclaration::BorderTopStyle(_) => LonghandId::BorderTopStyle,
            PropertyDeclaration::BorderBottomWidth(_) => LonghandId::BorderBottomWidth,
            PropertyDeclaration::BorderLeftWidth(_) => LonghandId::BorderLeftWidth,
            PropertyDeclaration::BorderRightWidth(_) => LonghandId::BorderRightWidth,
            PropertyDeclaration::BorderTopWidth(_) => LonghandId::BorderTopWidth,
            PropertyDeclaration::Color(_) => LonghandId::Color,
            PropertyDeclaration::Direction(_) => LonghandId::Direction,
            PropertyDeclaration::Display(_) => LonghandId::Display,
            PropertyDeclaration::FontSize(_) => LonghandId::FontSize,
            PropertyDeclaration::Height(_) => LonghandId::Height,
            PropertyDeclaration::MarginBottom(_) => LonghandId::MarginBottom,
            PropertyDeclaration::MarginLeft(_) => LonghandId::MarginLeft,
            PropertyDeclaration::MarginRight(_) => LonghandId::MarginRight,
            PropertyDeclaration::MarginTop(_) => LonghandId::MarginTop,
            PropertyDeclaration::PaddingBottom(_) => LonghandId::PaddingBottom,
            PropertyDeclaration::PaddingLeft(_) => LonghandId::PaddingLeft,
            PropertyDeclaration::PaddingRight(_) => LonghandId::PaddingRight,
            PropertyDeclaration::PaddingTop(_) => LonghandId::PaddingTop,
            PropertyDeclaration::Width(_) => LonghandId::Width,
            PropertyDeclaration::WritingMode(_) => LonghandId::WritingMode,
        }
    }
}

impl From<PropertyDeclaration> for LonghandId {
    fn from(prop_decl: PropertyDeclaration) -> Self {
        LonghandId::from(&prop_decl)
    }
}

impl From<&ContextualPropertyDeclaration> for LonghandId {
    fn from(contextual_decl: &ContextualPropertyDeclaration) -> Self {
        LonghandId::from(&contextual_decl.inner_decl)
    }
}

impl From<ContextualPropertyDeclaration> for LonghandId {
    fn from(contextual_decl: ContextualPropertyDeclaration) -> Self {
        LonghandId::from(&contextual_decl)
    }
}

/// An identifier for a given shorthand property.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
pub(super) enum ShorthandId {
    /// background
    Background = 0,
    //    /// background-position
    //    BackgroundPosition = 1,
    /// border-color
    BorderColor = 2,
    /// border-style
    BorderStyle = 3,
    /// border-width
    BorderWidth = 4,
    /// border-top
    BorderTop = 5,
    /// border-right
    BorderRight = 6,
    /// border-bottom
    BorderBottom = 7,
    /// border-left
    BorderLeft = 8,
    //    /// border-block-start
    //    BorderBlockStart = 9,
    //    /// border-block-end
    //    BorderBlockEnd = 10,
    //    /// border-inline-start
    //    BorderInlineStart = 11,
    //    /// border-inline-end
    //    BorderInlineEnd = 12,
    /// border
    Border = 13,
    //    /// border-radius
    //    BorderRadius = 14,
    //    /// border-image
    //    BorderImage = 15,
    //    /// border-block-width
    //    BorderBlockWidth = 16,
    //    /// border-block-style
    //    BorderBlockStyle = 17,
    //    /// border-block-color
    //    BorderBlockColor = 18,
    //    /// border-inline-width
    //    BorderInlineWidth = 19,
    //    /// border-inline-style
    //    BorderInlineStyle = 20,
    //    /// border-inline-color
    //    BorderInlineColor = 21,
    //    /// border-block
    //    BorderBlock = 22,
    //    /// border-inline
    //    BorderInline = 23,
    //    /// overflow
    //    Overflow = 24,
    //    /// transition
    //    Transition = 25,
    //    /// animation
    //    Animation = 26,
    //    /// columns
    //    Columns = 27,
    //    /// font
    //    Font = 28,
    //    /// font-variant
    //    FontVariant = 29,
    //    /// list-style
    //    ListStyle = 30,
    /// margin
    Margin = 31,
    //    /// margin-block
    //    MarginBlock = 32,
    //    /// margin-inline
    //    MarginInline = 33,
    //    /// outline
    //    Outline = 34,
    /// padding
    Padding = 35,
    //    /// padding-block
    //    PaddingBlock = 36,
    //    /// padding-inline
    //    PaddingInline = 37,
    //    /// flex-flow
    //    FlexFlow = 38,
    //    /// flex
    //    Flex = 39,
    //    /// inset
    //    Inset = 40,
    //    /// inset-block
    //    InsetBlock = 41,
    //    /// inset-inline
    //    InsetInline = 42,
    //    /// text-decoration
    //    TextDecoration = 43,
    //    /// all
    //    All = 44,
}
