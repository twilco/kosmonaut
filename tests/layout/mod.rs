pub mod directional;

#[cfg(test)]
mod tests {
    use crate::util::{dump_layout_cmd, dump_layout_cmd_scaled};
    use insta::assert_snapshot;

    #[test]
    fn rainbow_divs_baseline() {
        let mut dump_layout_cmd = dump_layout_cmd();
        dump_layout_cmd
            .arg("--files")
            .arg("tests/websrc/rainbow-divs.html")
            .arg("tests/websrc/rainbow-divs.css")
            .succeeds()
            .no_stderr();
        assert_snapshot!(dump_layout_cmd.stdout());
    }

    #[test]
    fn rainbow_divs_baseline_two_scale_factor() {
        let mut dump_layout_cmd = dump_layout_cmd_scaled(2.0);
        dump_layout_cmd
            .arg("--files")
            .arg("tests/websrc/rainbow-divs.html")
            .arg("tests/websrc/rainbow-divs.css")
            .succeeds()
            .no_stderr();
        assert_snapshot!(dump_layout_cmd.stdout());
    }
}
