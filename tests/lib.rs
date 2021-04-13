#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use crate::util::CommandUnderTest;
use cli::DumpLayoutVerbosity;
use insta::assert_snapshot;
use std::ffi::OsStr;
use std::path::Path;

mod util;

mod layout;
mod style;

pub(crate) const LAYOUT_DUMP_INNER_WINDOW_WIDTH_PX: f32 = 1920.;
pub(crate) const LAYOUT_DUMP_INNER_WINDOW_HEIGHT_PX: f32 = 1080.;
/// Unless otherwise specified, run layout tests with a scale factor of 1.0.
pub(crate) const LAYOUT_DUMP_DEFAULT_SCALE_FACTOR: f32 = 1.0;
pub(crate) const LAYOUT_DUMP_DEFAULT_VERBOSITY: DumpLayoutVerbosity =
    DumpLayoutVerbosity::NonVerbose;

/// This automatically finds and runs any <filename>.reftest.html and <filename>.expected.html as
/// a reftest.  A reftest renders the given documents headlessly and performs a pixel-by-pixel
/// comparison of the results.
#[datatest::files("tests/websrc", {
reftest_html_file in r"^(.*).reftest.html",
expected_html_file = r"${1}.expected.html",
})]
fn auto_reftests(reftest_html_file: &Path, expected_html_file: &Path) {
    reftest_expect_similar(
        reftest_html_file.to_str().unwrap(),
        expected_html_file.to_str().unwrap(),
    )
}

/// Automatically discovers and runs dump-layout tests with the proper filename.
///
/// This will only ever run dump-layout with a set of default settings.  If your test requires any
/// sort of customization (e.g. changing scale factor, screen width / height, etc), you will need
/// to manually create a test to consume `dump_layout_cmd`.
#[datatest::files("tests/websrc", {
dump_layout_html_file in r"^(.*).dumplayout.html",
})]
fn auto_dump_layout_tests(dump_layout_html_file: &Path) {
    let mut dump_layout_cmd = dump_layout_cmd(DumpLayoutVerbosity::NonVerbose);
    dump_layout_cmd
        .arg(dump_layout_html_file.to_str().unwrap())
        .succeeds();
    snapshot_dump_layout_cmd(dump_layout_cmd, dump_layout_html_file);
}

pub fn snapshot_dump_layout_cmd(dump_layout_cmd: CommandUnderTest, corpus_html_file: &Path) {
    snapshot_dump_layout_cmd_verbose_scaled(
        dump_layout_cmd,
        corpus_html_file,
        LAYOUT_DUMP_DEFAULT_VERBOSITY,
        LAYOUT_DUMP_DEFAULT_SCALE_FACTOR,
    )
}

pub fn snapshot_dump_layout_cmd_verbose(
    dump_layout_cmd: CommandUnderTest,
    corpus_html_file: &Path,
    verbosity: DumpLayoutVerbosity,
) {
    snapshot_dump_layout_cmd_verbose_scaled(
        dump_layout_cmd,
        corpus_html_file,
        verbosity,
        LAYOUT_DUMP_DEFAULT_SCALE_FACTOR,
    )
}

pub fn snapshot_dump_layout_cmd_scaled(
    dump_layout_cmd: CommandUnderTest,
    corpus_html_file: &Path,
    scale_factor: f32,
) {
    snapshot_dump_layout_cmd_verbose_scaled(
        dump_layout_cmd,
        corpus_html_file,
        LAYOUT_DUMP_DEFAULT_VERBOSITY,
        scale_factor,
    )
}

pub fn snapshot_dump_layout_cmd_verbose_scaled(
    dump_layout_cmd: CommandUnderTest,
    corpus_html_file: &Path,
    verbosity: DumpLayoutVerbosity,
    scale_factor: f32,
) {
    let settings = dump_layout_insta_settings(corpus_html_file, verbosity, scale_factor);
    let (insta_settings, snapshot_filename) = (settings.insta_settings, settings.snapshot_filename);
    insta_settings.bind(|| {
        assert_snapshot!(snapshot_filename, dump_layout_cmd.stdout());
    });
}

/// Computes the directory to save the snapshot to based on the corpus.  The root test websrc
/// directory (which contains our corpus test files) is stripped off the front of the given
/// path, and the filename of this specific testcase is stripped off the end.
///
/// "tests/websrc/directional/rtl-horizontal-tb-block-boxes.layoutsnap.html"
///   becomes
/// "snapshots/directional"
pub fn compute_snapshot_dir(dump_layout_html_file: &Path) -> String {
    let file_name = dump_layout_html_file.file_name().unwrap();
    let path = dump_layout_html_file.to_str().unwrap();
    let trimmed_path = path
        .trim_start_matches("tests/websrc/")
        .trim_end_matches(file_name.to_str().unwrap())
        .trim_end_matches("/");
    format!("{}/{}", "snapshots", trimmed_path)
}

pub struct DumpLayoutInstaSettings {
    pub insta_settings: insta::Settings,
    pub snapshot_filename: String,
}

fn dump_layout_insta_settings(
    corpus_html_file_path: &Path,
    verbosity: DumpLayoutVerbosity,
    scale_factor: f32,
) -> DumpLayoutInstaSettings {
    let mut settings = insta::Settings::clone_current();
    settings.set_input_file(corpus_html_file_path);
    settings.set_snapshot_path(compute_snapshot_dir(corpus_html_file_path));
    settings.set_prepend_module_to_snapshot(false);
    let snapshot_suffix = match verbosity {
        LAYOUT_DUMP_DEFAULT_VERBOSITY if scale_factor == LAYOUT_DUMP_DEFAULT_SCALE_FACTOR => {
            "".to_owned()
        }
        verbosity if scale_factor == LAYOUT_DUMP_DEFAULT_SCALE_FACTOR => match verbosity {
            DumpLayoutVerbosity::Verbose => "verbose",
            LAYOUT_DUMP_DEFAULT_VERBOSITY => "",
        }
        .to_owned(),
        LAYOUT_DUMP_DEFAULT_VERBOSITY if scale_factor != LAYOUT_DUMP_DEFAULT_SCALE_FACTOR => {
            format!("scaled-{}", scale_factor)
        }
        _ if scale_factor != LAYOUT_DUMP_DEFAULT_SCALE_FACTOR => {
            format!("verbose-scaled-{}", scale_factor)
        }
        _ => panic!(
            "failure to create snapshot suffix with verbosity '{:?}' and scale_factor '{}'",
            verbosity, scale_factor
        ),
    };
    settings.set_snapshot_suffix(snapshot_suffix);
    let file_name = corpus_html_file_path.file_name().unwrap();
    DumpLayoutInstaSettings {
        insta_settings: settings,
        snapshot_filename: file_name.to_str().unwrap().to_owned(),
    }
}

pub fn reftest_expect_similar<S: AsRef<str> + AsRef<OsStr>>(file_path_one: S, file_path_two: S) {
    let (file_path_one, file_path_two): (&str, &str) =
        (file_path_one.as_ref(), file_path_two.as_ref());
    match run_similarity_cmd(file_path_one, file_path_two) {
        Ok(percent_similar) => {
            if percent_similar != 100.0f64 {
                panic!("reftest_expect_similar failure.  files '{}' and '{}' were only {}% similar (expected 100%)", file_path_one, file_path_two, percent_similar);
            }
        }
        Err(err) => panic!(
            "reftest_expect_similar *internal* failure (this is bad) with files '{}' and '{}'.  error message was: {}",
            file_path_one,
            file_path_two,
            err
        ),
    }
}

fn run_similarity_cmd<S: AsRef<str> + AsRef<OsStr>>(
    file_path_one: S,
    file_path_two: S,
) -> Result<f64, String> {
    let mut cmd = CommandUnderTest::new();
    cmd.arg("similarity");
    cmd.arg(file_path_one);
    cmd.arg(file_path_two);
    cmd.arg("--similarity-percent-only");
    cmd.succeeds();
    let stdout = cmd.stdout().trim();
    stdout.parse::<f64>().map_err(|_| stdout.trim().to_owned())
}

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
