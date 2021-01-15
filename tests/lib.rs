use crate::util::CommandUnderTest;
use std::ffi::OsStr;

mod util;

mod layout;
mod style;

pub fn reftest_expect_similar<S: AsRef<str> + AsRef<OsStr>>(file_path_one: S, file_path_two: S) {
    let (file_path_one, file_path_two): (&str, &str) =
        (file_path_one.as_ref(), file_path_two.as_ref());
    match run_similarity_cmd(file_path_one, file_path_two) {
        Ok(percent_similar) => {
            if percent_similar != 100.0f64 {
                panic!(format!("reftest_expect_similar failure.  files '{}' and '{}' were only {}% similar (expected 100%)", file_path_one, file_path_two, percent_similar));
            }
        }
        Err(err) => panic!(format!(
            "reftest_expect_similar *internal* failure (this is bad) with files '{}' and '{}'.  error message was: {}",
            file_path_one,
            file_path_two,
            err
        )),
    }
}

fn run_similarity_cmd<S: AsRef<str> + AsRef<OsStr>>(
    file_path_one: S,
    file_path_two: S,
) -> Result<f64, String> {
    let mut cmd = CommandUnderTest::new();
    cmd.arg("similarity");
    cmd.arg("--files");
    cmd.arg(file_path_one);
    cmd.arg(file_path_two);
    cmd.arg("--similarity-percent-only");
    cmd.succeeds();
    let stdout = cmd.stdout().trim();
    stdout.parse::<f64>().map_err(|_| stdout.trim().to_owned())
}
