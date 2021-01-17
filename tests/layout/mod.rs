pub use kosmonaut::cli::DumpLayoutVerbosity;

pub mod directional;

#[cfg(test)]
mod tests {
    use crate::layout::DumpLayoutVerbosity;
    use crate::{
        dump_layout_cmd, dump_layout_cmd_verbose_scaled, snapshot_dump_layout_cmd,
        snapshot_dump_layout_cmd_scaled,
    };
    use std::path::Path;

    #[test]
    fn rainbow_divs_baseline() {
        let mut dump_layout_cmd = dump_layout_cmd(DumpLayoutVerbosity::NonVerbose);
        let path = Path::new("tests/websrc/rainbow-divs.html");
        dump_layout_cmd.arg(path.to_str().unwrap()).succeeds();
        snapshot_dump_layout_cmd(dump_layout_cmd, path);
    }

    #[test]
    fn rainbow_divs_baseline_two_scale_factor() {
        let scale_factor = 2.0;
        let mut dump_layout_cmd =
            dump_layout_cmd_verbose_scaled(DumpLayoutVerbosity::NonVerbose, scale_factor);
        let path = Path::new("tests/websrc/rainbow-divs.html");
        dump_layout_cmd.arg(path.to_str().unwrap()).succeeds();
        snapshot_dump_layout_cmd_scaled(dump_layout_cmd, path, scale_factor);
    }
}
