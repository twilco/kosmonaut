// Useful links:
//  * https://www.w3.org/TR/css-display-3/#css-box
//  * https://www.w3.org/TR/2018/WD-css-box-3-20181218/#intro

use crate::dom::tree::NodeRef;
use crate::layout::BoxType::Anonymous;
use crate::style::values::computed::Display;
use std::mem::discriminant;

/// Takes a DOM node and builds the corresponding layout tree of it and its children.
pub fn build_layout_tree(node: NodeRef) -> Option<LayoutBox> {
    let computed_values = &*node.computed_values();
    let mut layout_box = match computed_values.display {
        Display::Block => LayoutBox::new(BoxType::Block(node.clone())),
        Display::Inline => LayoutBox::new(BoxType::Inline(node.clone())),
        Display::None => {
            return None;
        }
    };

    for child in node.children() {
        let child_computed_values = &*child.computed_values();
        match child_computed_values.display {
            Display::Block => match build_layout_tree(child.clone()) {
                // TODO: We don't handle the case where a block-flow child box is added to an inline
                // box.  This current behavior is wrong.  To fix, see: https://www.w3.org/TR/CSS2/visuren.html#box-gen
                // Namely, the paragraph that begins with "When an inline box contains an in-flow block-level box"
                // This concept _might_ be called "fragmenting".
                Some(child_box) => layout_box.children.push(child_box),
                None => {}
            },
            Display::Inline => match build_layout_tree(child.clone()) {
                Some(child_box) => layout_box.get_inline_container().children.push(child_box),
                None => {}
            },
            Display::None => {}
        }
    }
    return Some(layout_box);
}

/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#box-model
#[derive(Clone, Copy, Debug, Default)]
struct Dimensions {
    // Position of the content area relative to the document origin:
    content: Rect,

    // Surrounding edges:
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

#[derive(Clone, Debug)]
pub struct LayoutBox {
    dimensions: Dimensions,
    box_type: BoxType,
    children: Vec<LayoutBox>,
}

impl LayoutBox {
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(), // initially set all fields to 0.0
            children: Vec::new(),
        }
    }

    /// Gets the proper `LayoutBox` container to insert inline-children in to.
    ///
    /// If a block box contains inline-children, an anonymous box must be used to contain them.
    ///
    /// If this box is already an inline or anonymous box, we can use ourself to contain the inline
    /// children.  Otherwise, find or create an anonymous box.
    fn get_inline_container(&mut self) -> &mut LayoutBox {
        match self.box_type {
            BoxType::Inline(_) | BoxType::Anonymous => self,
            BoxType::Block(_) => {
                match self.children.last() {
                    Some(last_child)
                        if discriminant(&last_child.box_type)
                            == discriminant(&BoxType::Anonymous) => {}
                    _ => self.children.push(LayoutBox::new(BoxType::Anonymous)),
                }
                self.children
                    .last_mut()
                    .expect("there should've been at least one child")
            }
        }
    }

    /// Calculates the dimensions of this box, and any child boxes.
    ///
    /// A block's width depends on that of its parent (called "containing block" in the spec), while
    /// a block's height depends on that of its children.  This is important to know in layout.
    ///
    /// In this step, we will be taking computed values and calculating actual, used values
    /// based on the constraint of our environment.
    fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BoxType::Block(_) => self.layout_block(containing_block),
            BoxType::Inline(_) => unimplemented!("layout inline box types"),
            BoxType::Anonymous => unimplemented!("layout anonymous box types"),
        }
    }

    /// Assuming `self` is a block-box, calculate the dimensions of this box and any children.
    fn layout_block(&mut self, containing_block: Dimensions) {
        // Child width can depend on parent width, so we need to calculate this box's width before
        // laying out its children.
        self.calculate_block_width(containing_block);

        // Determine where the box is located within its containing block.
        self.calculate_block_position(containing_block);

        // Recursively layout the children of this box.
        self.layout_block_children();

        // Parent height can depend on child height, so let's `calculate_height` now that we've
        // laid out our children.
        self.calculate_block_height();
    }

    fn calculate_block_width(&mut self, containing_block: Dimensions) {}

    fn calculate_block_height(&mut self) {}

    fn calculate_block_position(&mut self, containing_block: Dimensions) {}

    fn layout_block_children(&mut self) {}
}

#[derive(Clone, Debug)]
pub enum BoxType {
    Block(NodeRef),
    Inline(NodeRef),
    Anonymous,
}
