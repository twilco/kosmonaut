use crate::util::CommandUnderTest;

pub use kosmonaut::cli::DumpLayoutVerbosity;

pub mod block;
pub mod directional;
pub mod display;

#[cfg(test)]
mod tests {
    use crate::layout::{dump_layout_cmd, dump_layout_cmd_verbose_scaled, DumpLayoutVerbosity};
    use insta::assert_snapshot;

    #[test]
    fn rainbow_divs_baseline() {
        let mut dump_layout_cmd = dump_layout_cmd(DumpLayoutVerbosity::NonVerbose);
        dump_layout_cmd
            .arg("--files")
            .arg("tests/websrc/rainbow-divs.html")
            .succeeds();
        assert_snapshot!(dump_layout_cmd.stdout());
    }

    #[test]
    fn rainbow_divs_baseline_two_scale_factor() {
        let mut dump_layout_cmd =
            dump_layout_cmd_verbose_scaled(DumpLayoutVerbosity::NonVerbose, 2.0);
        dump_layout_cmd
            .arg("--files")
            .arg("tests/websrc/rainbow-divs.html")
            .succeeds();
        assert_snapshot!(dump_layout_cmd.stdout());
    }
}

pub(crate) static LAYOUT_DUMP_INNER_WINDOW_WIDTH_PX: f32 = 1920.;
pub(crate) static LAYOUT_DUMP_INNER_WINDOW_HEIGHT_PX: f32 = 1080.;
/// Unless otherwise specified, run layout tests with a scale factor of 1.0.
pub(crate) static LAYOUT_DUMP_DEFAULT_SCALE_FACTOR: f32 = 1.0;

pub fn dump_layout_cmd_verbose_scaled(
    verbosity: DumpLayoutVerbosity,
    scale_factor: f32,
) -> CommandUnderTest {
    let mut cmd = CommandUnderTest::new();
    cmd.arg("dump-layout");
    cmd.arg("--width");
    cmd.arg(format!("{}", LAYOUT_DUMP_INNER_WINDOW_WIDTH_PX));
    cmd.arg("--height");
    cmd.arg(format!("{}", LAYOUT_DUMP_INNER_WINDOW_HEIGHT_PX));
    cmd.arg("--verbose");
    cmd.arg(format!("{}", verbosity.to_cli_string()));
    cmd.arg("--scale-factor");
    cmd.arg(format!("{}", scale_factor));
    cmd
}

pub fn dump_layout_cmd(verbosity: DumpLayoutVerbosity) -> CommandUnderTest {
    dump_layout_cmd_verbose_scaled(verbosity, LAYOUT_DUMP_DEFAULT_SCALE_FACTOR)
}
