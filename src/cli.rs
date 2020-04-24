use crate::style;
use crate::style::stylesheet::Stylesheet;
use clap::{App, Arg, ArgMatches};

pub fn setup_and_get_cli_args<'a>() -> ArgMatches<'a> {
    App::new("Kosmonaut")
        .version("0.1")
        .author("Tyler Wilcock (twilco)")
        .about("A web browser for the space age🚀🚀")
        .arg(
            Arg::with_name("files")
                .short("f")
                .long("files")
                .value_name("FILES")
                .help("Pass files for Kosmonaut to render.")
                .multiple(true)
                .takes_value(true),
        )
        .get_matches()
}

pub fn html_file_path_from_files<'a>(arg_matches: &'a ArgMatches<'a>) -> Option<&'a str> {
    let files_opt = arg_matches.values_of("files");
    files_opt
        .map(|mut files| {
            files.find(|file| {
                let parts = file.split(".");
                if let Some(last_part) = parts.last() {
                    return last_part == "html";
                }
                false
            })
        })
        .flatten()
}

pub fn stylesheets_from_files<'a>(arg_matches: &'a ArgMatches<'a>) -> Option<Vec<Stylesheet>> {
    let files_opt = arg_matches.values_of("files");
    files_opt.map(|files| {
        return files
            .filter(|file| {
                let parts = file.split(".");
                if let Some(last_part) = parts.last() {
                    return last_part == "css";
                }
                false
            })
            .map(|stylesheet_path| {
                style::stylesheet::parse_css_to_stylesheet(
                    Some(stylesheet_path.to_owned()),
                    &mut std::fs::read_to_string(stylesheet_path).expect("file fail"),
                )
                .expect("error parsing stylesheet")
            })
            .collect::<Vec<_>>();
    })
}
