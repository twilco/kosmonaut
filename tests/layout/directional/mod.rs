use crate::layout::{dump_layout_cmd, DumpLayoutVerbosity};
use insta::assert_snapshot;

const DIRECTIONAL_WEBSRC_DIR: &str = "tests/websrc/directional";
fn path(filename: &str) -> String {
    format!("{}/{}", DIRECTIONAL_WEBSRC_DIR, filename)
}

#[test]
fn ltr_vertical_lr_block_boxes() {
    let mut dump_layout_cmd = dump_layout_cmd(DumpLayoutVerbosity::NonVerbose);
    dump_layout_cmd
        .arg(path("ltr-vertical-lr-block-boxes.html"))
        .succeeds();
    assert_snapshot!(dump_layout_cmd.stdout());
}

#[test]
fn ltr_vertical_lr_block_boxes_bottom_right_mbp_applied_physically() {
    let mut dump_layout_cmd = dump_layout_cmd(DumpLayoutVerbosity::Verbose);
    dump_layout_cmd
        .arg(path("ltr-vertical-lr-block-boxes-bottom-right-mbp-applied-physically.html"))
        .succeeds();
    assert_snapshot!(dump_layout_cmd.stdout());
}

#[test]
fn ltr_vertical_lr_block_boxes_top_left_right_mbp_applied_physically() {
    let mut dump_layout_cmd = dump_layout_cmd(DumpLayoutVerbosity::Verbose);
    dump_layout_cmd
        .arg(path("ltr-vertical-lr-block-boxes-top-left-mbp-applied-physically.html"))
        .succeeds();
    assert_snapshot!(dump_layout_cmd.stdout());
}

#[test]
fn rtl_horizontal_tb_block_boxes() {
    let mut dump_layout_cmd = dump_layout_cmd(DumpLayoutVerbosity::NonVerbose);
    dump_layout_cmd
        .arg(path("rtl-horizontal-tb-block-boxes.html"))
        .succeeds();
    assert_snapshot!(dump_layout_cmd.stdout());
}
