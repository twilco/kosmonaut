use crate::dom::tree::NodeRef;
use crate::layout::containing_block::ContainingBlock;
use crate::layout::dimensions::Dimensions;
use crate::layout::formatting_context::FormattingContextRef;
use crate::style::values::computed::ComputedValues;
use accountable_refcell::Ref;
use enum_dispatch::enum_dispatch;

/// This trait allows boxes to handle how they apply the box sizing properties to their dimensions.
///
/// https://www.w3.org/TR/css-sizing-3/#sizing-properties
#[enum_dispatch(LayoutBox, BlockLevelBox, InlineLevelBox)]
pub trait ApplyBoxSizingProperties {
    fn apply_box_sizing_properties(&mut self, containing_block: ContainingBlock);
}

#[macro_export]
macro_rules! apply_box_sizing_properties_base_box_passthrough_impls {
    () => {
        #[inline(always)]
        fn apply_box_sizing_properties(&mut self, containing_block: ContainingBlock) {
            self.base.apply_box_sizing_properties(containing_block);
        }
    };
}

/// Behaviors applicable to all members of the box-tree (including text runs).  Because this
/// encompasses a wide variation of box types, some basic behaviors, like anything to do with
/// children, are not present in this trait.  This is because some boxes may have children of a more
/// specific type than `LayoutBox` (e.g. only `BlockLevelBox`s or `InlineLevelBox`s), or have no
/// children at all (e.g. for text runs).
#[enum_dispatch(LayoutBox, BlockLevelBox, InlineLevelBox, InlineLevelContent)]
pub trait BaseLayoutBoxBehavior {
    fn computed_values(&self) -> Ref<ComputedValues>;
    fn dimensions(&self) -> Dimensions;
    fn dimensions_mut(&mut self) -> &mut Dimensions;
    fn formatting_context(&self) -> FormattingContextRef;
    fn is_root(&self) -> bool;
    fn node(&self) -> NodeRef;
}

#[macro_export]
macro_rules! layout_box_behavior_base_box_passthrough_impls {
    () => {
        #[inline(always)]
        fn computed_values(&self) -> Ref<ComputedValues> {
            self.base.computed_values()
        }

        #[inline(always)]
        fn dimensions(&self) -> Dimensions {
            self.base.dimensions()
        }

        #[inline(always)]
        fn dimensions_mut(&mut self) -> &mut Dimensions {
            self.base.dimensions_mut()
        }

        #[inline(always)]
        fn formatting_context(&self) -> FormattingContextRef {
            self.base.formatting_context()
        }

        #[inline(always)]
        fn is_root(&self) -> bool {
            self.base.is_root()
        }

        #[inline(always)]
        fn node(&self) -> NodeRef {
            self.base.node()
        }
    };
}
