use clap::{App, Arg, ArgMatches, SubCommand};
use std::str::FromStr;

pub fn setup_and_get_cli_args<'a>() -> ArgMatches<'a> {
    let headed_or_headless_applicable =
        "Applicable in both headed and headless (e.g. dump-layout) contexts.";
    App::new("Kosmonaut")
        .version("0.1")
        .author("Tyler Wilcock (twilco)")
        .about("A web browser for the space age 🚀")
        .arg(
            Arg::with_name("files")
                .short("f")
                .long("files")
                .value_name("SPACE SEPARATED FILE PATHS")
                .help("Pass files for Kosmonaut to render.")
                .multiple(true)
                .takes_value(true)
                .global(true),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("NUMBER")
                .help(&format!("Inner window width.  {}", headed_or_headless_applicable))
                .takes_value(true)
                .validator(is_num_validator)
                .global(true),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("NUMBER")
                .help(&format!("Inner window height.  {}", headed_or_headless_applicable))
                .takes_value(true)
                .validator(is_num_validator)
                .global(true),
        )
        .arg(
            Arg::with_name("scale-factor")
                .short("s")
                .long("scale-factor")
                .value_name("NUMBER")
                .help(&format!("Device/window scale factor.  {}", headed_or_headless_applicable))
                .takes_value(true)
                .validator(is_num_validator)
                .global(true),
        )
        .subcommand(
            SubCommand::with_name("dump-layout")
                .about("Dumps layout-tree as text to stdout after first global layout, exiting afterwards.")
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .value_name("BOOLEAN")
                        .help("Set to true to make layout dumps more verbose (e.g. include margin, border, padding values).")
                        .takes_value(true)
                        .validator(is_bool_validator)
                )
        )
        .get_matches()
}

fn is_num_validator(string: String) -> Result<(), String> {
    match string.parse::<f32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("given arg '{}' is not a number", string)),
    }
}

fn is_bool_validator(string: String) -> Result<(), String> {
    match string.parse::<bool>() {
        Ok(_) => Ok(()),
        Err(_) => match &string[..] {
            "0" | "1" => Ok(()),
            _ => Err(format!("given arg '{}' is not a bool value", string)),
        },
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

pub fn css_file_paths_from_files<'a>(arg_matches: &'a ArgMatches<'a>) -> Option<Vec<&'a str>> {
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
            .collect::<Vec<_>>()
    })
}

pub fn dump_layout_tree(arg_matches: &ArgMatches) -> bool {
    arg_matches.subcommand_matches("dump-layout").is_some()
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DumpLayoutVerbosity {
    /// Includes more information in layout-dump, such as margin, border, and padding values for
    /// each box.
    Verbose,
    /// The most minimal layout-dump representation, including information such as box size, box
    /// type, xy position coordinates, and more.
    NonVerbose,
}

impl DumpLayoutVerbosity {
    pub fn to_cli_string(&self) -> String {
        // The CLI form of this flag is currently a boolean.
        match self {
            DumpLayoutVerbosity::Verbose => "1",
            DumpLayoutVerbosity::NonVerbose => "0",
        }
        .to_owned()
    }
}

pub fn dump_layout_tree_verbose(arg_matches: &ArgMatches) -> Option<DumpLayoutVerbosity> {
    arg_matches
        .subcommand_matches("dump-layout")
        .and_then(|dump_layout_arg_matches| try_get_bool(dump_layout_arg_matches, "verbose"))
        .map(|bool_verbose| match bool_verbose {
            true => DumpLayoutVerbosity::Verbose,
            false => DumpLayoutVerbosity::NonVerbose,
        })
}

pub fn inner_window_width(arg_matches: &ArgMatches) -> Option<f32> {
    try_get_arg::<f32>(arg_matches, "width")
}

pub fn inner_window_height(arg_matches: &ArgMatches) -> Option<f32> {
    try_get_arg::<f32>(arg_matches, "height")
}

pub fn scale_factor(arg_matches: &ArgMatches) -> Option<f32> {
    try_get_arg::<f32>(arg_matches, "scale-factor")
}

fn try_get_arg<'a, T: FromStr>(arg_matches: &ArgMatches, arg_name: &'a str) -> Option<T> {
    arg_matches
        .value_of(arg_name)
        .map(|arg_str| arg_str.parse::<T>().ok())
        .unwrap_or(None)
}

fn try_get_bool<'a>(arg_matches: &ArgMatches, arg_name: &'a str) -> Option<bool> {
    try_get_arg::<bool>(arg_matches, arg_name).or_else(|| {
        let arg_match = arg_matches.value_of(arg_name);
        arg_match.and_then(|val| match val {
            "0" => Some(false),
            "1" => Some(true),
            _ => None,
        })
    })
}
