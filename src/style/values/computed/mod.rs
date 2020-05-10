/// Some of this code was taken from Servo: https://github.com/servo/servo
/// Kosmonaut complies with Servo's license, the Mozilla Public License 2.0.
pub mod background;
pub mod border;
pub mod color;
pub mod display;
pub mod font;
pub mod height;
pub mod length;
pub mod margin;
pub mod padding;
pub mod percentage;
pub mod width;

use crate::style::values::computed::height::Height;
pub use crate::style::values::computed::margin::{
    MarginBottom, MarginLeft, MarginRight, MarginTop,
};
pub use crate::style::values::computed::padding::{
    PaddingBottom, PaddingLeft, PaddingRight, PaddingTop,
};
use crate::style::values::computed::width::Width;

use crate::dom::tree::NodeRef;
use crate::style::properties::id::LonghandId;
use crate::style::properties::PropertyDeclaration;
use crate::style::values::specified;

use crate::Side;
pub use background::BackgroundColor;
pub use border::LineStyle;
pub use border::{
    border_side_initial_style, BorderBottomColor, BorderBottomWidth, BorderLeftColor,
    BorderLeftWidth, BorderRightColor, BorderRightWidth, BorderTopColor, BorderTopWidth,
};
pub use color::Color;
use cssparser::RGBA;
pub use display::Display;
pub use font::FontSize;
pub use percentage::Percentage;
use strum::IntoEnumIterator;

/// A trait to represent the conversion between computed and specified values where a context is
/// required to properly compute the specified value.
pub trait ComputeValueWithContext {
    /// The computed value type we're going to be converted to.
    type ComputedValue;

    /// When starting from a specified value (e.g. when the cascade provides one), convert a
    /// specified value to a computed value, using itself and the data inside the `ComputeContext`.
    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue;
}

/// A trait to represent the conversion between computed and specified values.  This trait differs
/// from `ComputeValueWithContext` in that this trait is only implemented for types that can go from
/// specified value to computed value without the need for any `ComputeContext`, making this
/// trait more convenient to use.
pub trait ComputeValue {
    /// The computed value type we're going to be converted to.
    type ComputedValue;

    /// When starting from a specified value (e.g. when the cascade provides one), convert a
    /// specified value to a computed value.
    fn compute_value(&self) -> Self::ComputedValue;
}

/// Trait to represent the behavior of defaulting a property's value when the cascade doesn't
/// provide one.
///
/// https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#defaulting
pub trait ValueDefault {
    /// The computed value type resulting from default.
    type ComputedValue;

    /// Perform the value-default.
    fn value_default(context: &ComputeContext) -> Self::ComputedValue;
}

/// A finalized set of computed values.
///
/// https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#computed-value
#[derive(Debug, Clone, Builder)]
pub struct ComputedValues {
    pub background_color: BackgroundColor,
    pub border_bottom_color: BorderBottomColor,
    pub border_left_color: BorderLeftColor,
    pub border_right_color: BorderRightColor,
    pub border_top_color: BorderTopColor,
    pub border_bottom_style: LineStyle,
    pub border_left_style: LineStyle,
    pub border_right_style: LineStyle,
    pub border_top_style: LineStyle,
    pub border_bottom_width: BorderBottomWidth,
    pub border_left_width: BorderLeftWidth,
    pub border_right_width: BorderRightWidth,
    pub border_top_width: BorderTopWidth,
    pub color: Color,
    pub display: Display,
    pub font_size: FontSize,
    pub height: Height,
    pub margin_bottom: MarginBottom,
    pub margin_left: MarginLeft,
    pub margin_right: MarginRight,
    pub margin_top: MarginTop,
    pub padding_bottom: PaddingBottom,
    pub padding_left: PaddingLeft,
    pub padding_right: PaddingRight,
    pub padding_top: PaddingTop,
    pub width: Width,
}

impl ComputedValues {
    pub fn border_style(&self, side: Side) -> LineStyle {
        match side {
            Side::Bottom => self.border_bottom_style,
            Side::Left => self.border_left_style,
            Side::Right => self.border_right_style,
            Side::Top => self.border_top_style,
        }
    }

    pub fn border_color_rgba(&self, side: Side) -> RGBA {
        match side {
            Side::Bottom => self.border_bottom_color.rgba,
            Side::Left => self.border_left_color.rgba,
            Side::Right => self.border_right_color.rgba,
            Side::Top => self.border_top_color.rgba,
        }
    }
}

/// Create a default set of computed values.  Likely most useful for the case in which we're working
/// with the root node of a DOM, which has no parent to inherit from.
impl Default for ComputedValues {
    // TODO: We might eventually need to not use the `Default` trait here in case we need `ComputeContext`
    // to calculate the default computed values.
    fn default() -> Self {
        let initial_color_prop = Color::initial_value();
        let initial_border_style = border_side_initial_style();
        ComputedValues {
            background_color: BackgroundColor::initial_value(initial_color_prop.rgba()),
            border_bottom_color: BorderBottomColor::initial_value(initial_color_prop.rgba()),
            border_left_color: BorderLeftColor::initial_value(initial_color_prop.rgba()),
            border_right_color: BorderRightColor::initial_value(initial_color_prop.rgba()),
            border_top_color: BorderTopColor::initial_value(initial_color_prop.rgba()),
            border_bottom_style: initial_border_style,
            border_left_style: initial_border_style,
            border_right_style: initial_border_style,
            border_top_style: initial_border_style,
            border_bottom_width: BorderBottomWidth::initial_value(initial_border_style),
            border_left_width: BorderLeftWidth::initial_value(initial_border_style),
            border_right_width: BorderRightWidth::initial_value(initial_border_style),
            border_top_width: BorderTopWidth::initial_value(initial_border_style),
            color: initial_color_prop,
            display: Display::initial_value(),
            font_size: FontSize::initial_value(),
            height: Height::initial_value(),
            margin_bottom: MarginBottom::initial_value(),
            margin_left: MarginLeft::initial_value(),
            margin_right: MarginRight::initial_value(),
            margin_top: MarginTop::initial_value(),
            padding_bottom: PaddingBottom::initial_value(),
            padding_left: PaddingLeft::initial_value(),
            padding_right: PaddingRight::initial_value(),
            padding_top: PaddingTop::initial_value(),
            width: Width::initial_value(),
        }
    }
}

/// A `ComputeContext` is all the data a specified value could ever need to compute
/// itself and be transformed to a computed value.
pub struct ComputeContext<'a> {
    // TODO: Viewport dimensions will be needed
    /// The computed values of the parent for cases where inheritance is necessary.  If the current
    /// node has no parent (it is the root node), this is `ComputedValues::default()`.
    pub parent_computed_values: &'a ComputedValues,

    /// The computed value of the `color` property for the node being computed.  Some properties,
    /// such as `border-<side>-color` use the `currentColor` keyword, which refers to this value.
    ///
    /// `None` if `color` has not been computed yet.
    pub computed_color: Option<Color>,

    /// The computed value of the `border-<side>-style` properties for the node being computed.
    /// The computed values of `border-<side>-width` properties depend on the associated border
    /// style — namely, if the computed style is "none" or "hidden", then the border width is zero.
    ///
    /// `None` if these values haven't been computed yet.
    pub computed_border_styles: Option<BorderSideStyleContext>,
}

impl ComputeContext<'_> {
    pub fn color(&self) -> Color {
        self.computed_color
            .expect("color property not yet computed and applied to compute context")
    }

    pub fn border_bottom_style(&self) -> LineStyle {
        self.border_styles().bottom
    }
    pub fn border_left_style(&self) -> LineStyle {
        self.border_styles().left
    }
    pub fn border_right_style(&self) -> LineStyle {
        self.border_styles().right
    }
    pub fn border_top_style(&self) -> LineStyle {
        self.border_styles().top
    }

    fn border_styles(&self) -> BorderSideStyleContext {
        self.computed_border_styles
            .expect("border styles properties not yet computed and applied to compute context")
    }
}

/// Container for computation context information about the `border-<side>-style` properties.
#[derive(Clone, Copy, Debug)]
pub struct BorderSideStyleContext {
    pub bottom: LineStyle,
    pub left: LineStyle,
    pub right: LineStyle,
    pub top: LineStyle,
}

pub fn compute_values(node: NodeRef) {
    let mut cv_builder = ComputedValuesBuilder::default();
    let parent = node.parent();
    // If this is the root node (aka there is no parent to inherit properties from), just default all properties to
    // their initial values.
    let parent_computed_values = parent.map_or(ComputedValues::default(), |p| {
        // TODO: This _could_ be an expensive clone when we actually support all CSS properties.
        p.computed_values().clone()
    });
    let mut context = ComputeContext {
        parent_computed_values: &parent_computed_values,
        computed_color: None,
        computed_border_styles: None,
    };
    compute_early_properties(node.clone(), &mut context);

    LonghandId::iter().for_each(|longhand: LonghandId| {
        match node.contextual_decls().get_by_longhand(longhand) {
            Some(contextual_decl) => {
                match &contextual_decl.inner_decl {
                    PropertyDeclaration::BackgroundColor(background_color) => {
                        cv_builder.background_color(
                            background_color.compute_value_with_context(&context),
                        );
                    }
                    PropertyDeclaration::BorderBottomColor(border_bottom_color) => {
                        cv_builder.border_bottom_color(
                            border_bottom_color.compute_value_with_context(&context),
                        );
                    }
                    PropertyDeclaration::BorderLeftColor(border_left_color) => {
                        cv_builder.border_left_color(
                            border_left_color.compute_value_with_context(&context),
                        );
                    }
                    PropertyDeclaration::BorderRightColor(border_right_color) => {
                        cv_builder.border_right_color(
                            border_right_color.compute_value_with_context(&context),
                        );
                    }
                    PropertyDeclaration::BorderTopColor(border_top_color) => {
                        cv_builder.border_top_color(
                            border_top_color.compute_value_with_context(&context),
                        );
                    }
                    PropertyDeclaration::BorderBottomStyle(_) => {
                        cv_builder.border_bottom_style(context.border_bottom_style());
                    }
                    PropertyDeclaration::BorderLeftStyle(_) => {
                        cv_builder.border_left_style(context.border_left_style());
                    }
                    PropertyDeclaration::BorderRightStyle(_) => {
                        cv_builder.border_right_style(context.border_right_style());
                    }
                    PropertyDeclaration::BorderTopStyle(_) => {
                        cv_builder.border_top_style(context.border_top_style());
                    }
                    PropertyDeclaration::BorderBottomWidth(border_bottom_width) => {
                        cv_builder.border_bottom_width(
                            border_bottom_width.compute_value_with_context(&context),
                        );
                    }
                    PropertyDeclaration::BorderLeftWidth(border_left_width) => {
                        cv_builder.border_left_width(
                            border_left_width.compute_value_with_context(&context),
                        );
                    }
                    PropertyDeclaration::BorderRightWidth(border_right_width) => {
                        cv_builder.border_right_width(
                            border_right_width.compute_value_with_context(&context),
                        );
                    }
                    PropertyDeclaration::BorderTopWidth(border_top_width) => {
                        cv_builder.border_top_width(
                            border_top_width.compute_value_with_context(&context),
                        );
                    }
                    PropertyDeclaration::Color(_) => {
                        cv_builder.color(context.color());
                    }
                    PropertyDeclaration::Display(display) => {
                        // TODO: Should we copying `display` here (taking the specified value), rather than computing the value?
                        // There is currently no `specified/display.rs`, so that would need to be remedied.
                        // Computing display might not be straightforward — see: https://github.com/w3c/csswg-drafts/issues/1716
                        cv_builder.display(*display);
                    }
                    PropertyDeclaration::Height(height) => {
                        cv_builder.height(height.compute_value_with_context(&context));
                    }
                    PropertyDeclaration::FontSize(font_size) => {
                        cv_builder.font_size(font_size.compute_value_with_context(&context));
                    }
                    PropertyDeclaration::MarginBottom(margin_bottom) => {
                        cv_builder
                            .margin_bottom(margin_bottom.compute_value_with_context(&context));
                    }
                    PropertyDeclaration::MarginLeft(margin_left) => {
                        cv_builder.margin_left(margin_left.compute_value_with_context(&context));
                    }
                    PropertyDeclaration::MarginRight(margin_right) => {
                        cv_builder.margin_right(margin_right.compute_value_with_context(&context));
                    }
                    PropertyDeclaration::MarginTop(margin_top) => {
                        cv_builder.margin_top(margin_top.compute_value_with_context(&context));
                    }
                    PropertyDeclaration::PaddingBottom(padding_bottom) => {
                        cv_builder
                            .padding_bottom(padding_bottom.compute_value_with_context(&context));
                    }
                    PropertyDeclaration::PaddingLeft(padding_left) => {
                        cv_builder.padding_left(padding_left.compute_value_with_context(&context));
                    }
                    PropertyDeclaration::PaddingRight(padding_right) => {
                        cv_builder
                            .padding_right(padding_right.compute_value_with_context(&context));
                    }
                    PropertyDeclaration::PaddingTop(padding_top) => {
                        cv_builder.padding_top(padding_top.compute_value_with_context(&context));
                    }
                    PropertyDeclaration::Width(width) => {
                        cv_builder.width(width.compute_value_with_context(&context));
                    }
                }
            }
            None => {
                longhand.value_default(&mut cv_builder, &context);
            }
        };
    });
    *node.computed_values_mut() = cv_builder
        .build()
        .expect("couldn't build computed values - maybe a field wasn't given to the builder?");
}

/// Computes "early phase" properties and adds them to the compute context.  Early phase properties
/// are those that are depended upon by other properties to compute properly (hence their addition
/// to the compute context).
fn compute_early_properties(node: NodeRef, context: &mut ComputeContext) {
    if let Some(contextual_decl) = node.contextual_decls().get_by_longhand(LonghandId::Color) {
        context.computed_color = match &contextual_decl.inner_decl {
            PropertyDeclaration::Color(color) => Some(color.compute_value_with_context(&context)),
            _ => panic!("needed color property declaration"),
        }
    } else {
        context.computed_color = Some(specified::Color::value_default(&context));
    }

    context.computed_border_styles = Some(compute_border_styles_early(node));
}

fn compute_border_styles_early(node: NodeRef) -> BorderSideStyleContext {
    let bottom = if let Some(contextual_decl) = node
        .contextual_decls()
        .get_by_longhand(LonghandId::BorderBottomStyle)
    {
        match &contextual_decl.inner_decl {
            PropertyDeclaration::BorderBottomStyle(line_style) => *line_style,
            _ => panic!("needed border-bottom-style property declaration"),
        }
    } else {
        border_side_initial_style()
    };

    let left = if let Some(contextual_decl) = node
        .contextual_decls()
        .get_by_longhand(LonghandId::BorderLeftStyle)
    {
        match &contextual_decl.inner_decl {
            PropertyDeclaration::BorderLeftStyle(line_style) => *line_style,
            _ => panic!("needed border-left-style property declaration"),
        }
    } else {
        border_side_initial_style()
    };

    let right = if let Some(contextual_decl) = node
        .contextual_decls()
        .get_by_longhand(LonghandId::BorderRightStyle)
    {
        match &contextual_decl.inner_decl {
            PropertyDeclaration::BorderRightStyle(line_style) => *line_style,
            _ => panic!("needed border-right-style property declaration"),
        }
    } else {
        border_side_initial_style()
    };

    let top = if let Some(contextual_decl) = node
        .contextual_decls()
        .get_by_longhand(LonghandId::BorderTopStyle)
    {
        match &contextual_decl.inner_decl {
            PropertyDeclaration::BorderTopStyle(line_style) => *line_style,
            _ => panic!("needed border-top-style property declaration"),
        }
    } else {
        border_side_initial_style()
    };

    BorderSideStyleContext {
        bottom,
        left,
        right,
        top,
    }
}
