use crate::layout::{dump_layout_cmd, DumpLayoutVerbosity};
use insta::assert_snapshot;

#[test]
fn block_layout_subtree_simple() {
    let mut dump_layout_cmd = dump_layout_cmd(DumpLayoutVerbosity::NonVerbose);
    dump_layout_cmd
        .arg("tests/websrc/block/block-layout-subtree-simple.html")
        .succeeds();
    assert_snapshot!(dump_layout_cmd.stdout());
}
#[test]
fn block_layout_subtrees() {
    let mut dump_layout_cmd = dump_layout_cmd(DumpLayoutVerbosity::NonVerbose);
    dump_layout_cmd
        .arg("tests/websrc/block/block-layout-subtrees.html")
        .succeeds();
    assert_snapshot!(dump_layout_cmd.stdout());
}
