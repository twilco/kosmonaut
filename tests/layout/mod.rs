#[cfg(test)]
mod tests {
    use crate::util::new_dump_layout_cmd;
    use insta::assert_snapshot;

    #[test]
    fn rainbow_divs_baseline() {
        let mut dump_layout_cmd = new_dump_layout_cmd();
        dump_layout_cmd
            .arg("--files")
            .arg("tests/websrc/rainbow-divs.html")
            .arg("tests/websrc/rainbow-divs.css")
            .succeeds()
            .no_stderr();
        assert_snapshot!(dump_layout_cmd.stdout());
    }
}
