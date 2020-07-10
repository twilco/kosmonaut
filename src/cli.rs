use crate::style;
use crate::style::stylesheet::Stylesheet;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::str::FromStr;

pub fn setup_and_get_cli_args<'a>() -> ArgMatches<'a> {
    let headed_or_headless_applicable =
        "Applicable in both headed and headless (e.g. dump-layout) contexts.";
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
                .takes_value(true)
                .global(true),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .help(&format!("Inner window width.  {}", headed_or_headless_applicable))
                .takes_value(true)
                .validator(is_num_validator)
                .global(true),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("HEIGHT")
                .help(&format!("Inner window height.  {}", headed_or_headless_applicable))
                .takes_value(true)
                .validator(is_num_validator)
                .global(true),
        )
        .arg(
            Arg::with_name("scale_factor")
                .short("s")
                .long("scale_factor")
                .value_name("SCALE_FACTOR")
                .help(&format!("Device/window scale factor.  {}", headed_or_headless_applicable))
                .takes_value(true)
                .validator(is_num_validator)
                .global(true),
        )
        .subcommand(SubCommand::with_name("dump-layout").about(
            "Dumps layout-tree as text to stdout after first global layout, exiting afterwards.  Scale factor must be sup",
        ))
        .get_matches()
}

fn is_num_validator(string: String) -> Result<(), String> {
    match string.parse::<f32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("given arg '{}' is not a number", string)),
    }
}

pub fn html_file_path_from_files<'a>(arg_matches: &'a ArgMatches<'a>) -> Option<&'a str> {
    let files_opt = arg_matches.values_of("files");
    files_opt
        .map(|mut files| {
            files.find(|file| {
                let parts = file.split('.');
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
        files
            .filter(|file| {
                let parts = file.split('.');
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
            .collect::<Vec<_>>()
    })
}

pub fn dump_layout_tree(arg_matches: &ArgMatches) -> bool {
    arg_matches.subcommand_matches("dump-layout").is_some()
}

pub fn inner_window_width(arg_matches: &ArgMatches) -> Option<f32> {
    try_get::<f32>(arg_matches, "width")
}

pub fn inner_window_height(arg_matches: &ArgMatches) -> Option<f32> {
    try_get::<f32>(arg_matches, "height")
}

pub fn scale_factor(arg_matches: &ArgMatches) -> Option<f32> {
    try_get::<f32>(arg_matches, "scale_factor")
}

fn try_get<'a, T: FromStr>(arg_matches: &ArgMatches, arg_name: &'a str) -> Option<T> {
    arg_matches.value_of(arg_name).map(|arg_str| {
        arg_str.parse::<T>().unwrap_or_else(|_| {
            panic!(format!(
                "couldn't parse '{}' arg as '{}'",
                arg_name,
                std::any::type_name::<T>()
            ))
        })
    })
}
