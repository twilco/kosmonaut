#[cfg(test)]
mod tests {
    use crate::layout::{dump_layout_cmd, DumpLayoutVerbosity};
    use insta::assert_snapshot;

    #[test]
    fn body_display_none_renders_no_children() {
        let mut dump_layout_cmd = dump_layout_cmd(DumpLayoutVerbosity::NonVerbose);
        dump_layout_cmd
            .arg("--files")
            .arg("tests/websrc/display/body-display-none.html")
            .arg("tests/websrc/display/body-display-none.css")
            .succeeds();
        assert_snapshot!(dump_layout_cmd.stdout());
    }

    #[test]
    fn subtree_display_none_renders_no_children() {
        let mut dump_layout_cmd = dump_layout_cmd(DumpLayoutVerbosity::NonVerbose);
        dump_layout_cmd
            .arg("--files")
            .arg("tests/websrc/display/subtree-display-none.html")
            .arg("tests/websrc/display/subtree-display-none.css")
            .succeeds();
        assert_snapshot!(dump_layout_cmd.stdout());
    }
}