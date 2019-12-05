// Useful links:
//  * https://www.w3.org/TR/css-display-3/#css-box
//  * https://www.w3.org/TR/2018/WD-css-box-3-20181218/#intro

use crate::dom::tree::NodeRef;
use crate::style::values::computed::Display;

pub fn layout(node: &NodeRef) -> Option<LayoutBox> {
    let computed_opt= &*node.computed_values();
    let computed_values = computed_opt.as_ref().expect("layout called on a node that has not yet acquired computed values");
    let mut layout_box = match computed_values.display {
        Display::Block => {
            LayoutBox::new(BoxType::Block(node))
        }
        Display::Inline => {
            LayoutBox::new(BoxType::Inline(node))
        }
        Display::None => {
            return None;
        }
    };

    for child in node.children() {
        let child_computed_opt= &*child.computed_values();
        let child_computed_values = child_computed_opt.as_ref().expect("layout called on a node that has not yet acquired computed values");
        match child_computed_values.display {
            Display::Block => {
                layout_box.children.push(LayoutBox::new(BoxType::Block(&child)))
            }
            Display::Inline => {
                layout_box.children.push(LayoutBox::new(BoxType::Inline(&child)))
            }
            Display::None => {
                return None;
            }
        }
    }
    return Some(layout_box);
}

/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#box-model
#[derive(Clone, Debug, Default)]
struct Dimensions {
    // Position of the content area relative to the document origin:
    content: Rect,

    // Surrounding edges:
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

#[derive(Clone, Debug, Default)]
pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Clone, Debug, Default)]
pub struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

#[derive(Clone, Debug)]
pub struct LayoutBox<'a> {
    dimensions: Dimensions,
    box_type: BoxType<'a>,
    children: Vec<LayoutBox<'a>>,
}

impl LayoutBox<'_> {
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(), // initially set all fields to 0.0
            children: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum BoxType<'a> {
    // TODO: Do we need to take a reference of a `NodeRef`?  There is literally `Ref` in the name.  Try testing without the &.  Probably NodeRef#clone()?
    Block(&'a NodeRef),
    Inline(&'a NodeRef),
    Anonymous,
}