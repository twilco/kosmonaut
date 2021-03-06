use crate::reftest_expect_similar;

const BORDER_WEBSRC_DIR: &str = "tests/websrc/style/shorthands/border";
fn test_file_path<S: AsRef<str>>(filename: S) -> String {
    format!("{}/{}", BORDER_WEBSRC_DIR, filename.as_ref())
}

#[test]
fn border_shorthand() {
    reftest_expect_similar(
        test_file_path("border-shorthand.html"),
        test_file_path("all-border-longhands-specified.html"),
    )
}
#[test]
fn border_color_shorthand() {
    reftest_expect_similar(
        test_file_path("border-color-shorthand.html"),
        test_file_path("all-border-longhands-specified.html"),
    )
}
#[test]
fn border_style_shorthand() {
    reftest_expect_similar(
        test_file_path("border-style-shorthand.html"),
        test_file_path("all-border-longhands-specified.html"),
    )
}
#[test]
fn border_width_shorthand() {
    reftest_expect_similar(
        test_file_path("border-width-shorthand.html"),
        test_file_path("all-border-longhands-specified.html"),
    )
}
