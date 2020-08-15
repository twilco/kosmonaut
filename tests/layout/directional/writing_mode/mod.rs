use crate::util::dump_layout_cmd;
use insta::assert_snapshot;

#[test]
fn vertical_lr_block_box_only() {
    let mut dump_layout_cmd = dump_layout_cmd();
    dump_layout_cmd
        .arg("--files")
        .arg("tests/websrc/directional/writing_mode/vertical-lr-block-box-only.html")
        .arg("tests/websrc/directional/writing_mode/vertical-lr-block-box-only.css")
        .succeeds();
    assert_snapshot!(dump_layout_cmd.stdout());
}
