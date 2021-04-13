use crate::{dump_layout_cmd, snapshot_dump_layout_cmd_verbose};
use cli::DumpLayoutVerbosity;
use std::path::PathBuf;

const DIRECTIONAL_WEBSRC_DIR: &str = "tests/websrc/directional";
fn path(filename: &str) -> PathBuf {
    PathBuf::from(format!("{}/{}", DIRECTIONAL_WEBSRC_DIR, filename))
}

#[test]
fn ltr_vertical_lr_block_boxes_bottom_right_mbp_applied_physically() {
    let verbosity = DumpLayoutVerbosity::Verbose;
    let mut dump_layout_cmd = dump_layout_cmd(verbosity);
    let path = path("ltr-vertical-lr-block-boxes-bottom-right-mbp-applied-physically.html");
    dump_layout_cmd.arg(path.to_str().unwrap()).succeeds();
    snapshot_dump_layout_cmd_verbose(dump_layout_cmd, &path, verbosity);
}

#[test]
fn ltr_vertical_lr_block_boxes_top_left_right_mbp_applied_physically() {
    let verbosity = DumpLayoutVerbosity::Verbose;
    let mut dump_layout_cmd = dump_layout_cmd(verbosity);
    let path = path("ltr-vertical-lr-block-boxes-top-left-mbp-applied-physically.html");
    dump_layout_cmd.arg(path.to_str().unwrap()).succeeds();
    snapshot_dump_layout_cmd_verbose(dump_layout_cmd, &path, verbosity);
}
