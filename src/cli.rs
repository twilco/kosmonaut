use crate::cli::Command::Similarity;
use clap::{App, Arg, ArgMatches, SubCommand, Values};
use std::str::FromStr;

const DUMP_LAYOUT_CMD_NAME: &str = "dump-layout";
const SIMILARITY_CMD_NAME: &str = "similarity";

pub fn setup_and_get_cli_args<'a>() -> ArgMatches<'a> {
    let headed_or_headless_applicable =
        "Applicable in both headed and headless (e.g. the dump-layout and similarity commands) contexts.";
    let files_arg = Arg::with_name("files")
            .short("f")
            .long("files")
            .value_name("SPACE SEPARATED FILE PATHS")
            .help("Pass files for Kosmonaut to render.  This is also the flag that should be used to pass files to any sub-command (e.g. `dump-layout`, `similarity`).")
            .multiple(true)
            .takes_value(true);
    let files_arg_required = files_arg.clone().required(true);
    let scale_factor_help = format!(
        "Device/window scale factor.  {}",
        headed_or_headless_applicable
    );
    let scale_factor_arg = Arg::with_name("scale-factor")
        .short("s")
        .long("scale-factor")
        .value_name("NUMBER")
        .help(&scale_factor_help)
        .takes_value(true)
        .validator(is_num_validator);
    let scale_factor_arg_required = scale_factor_arg.clone().required(true);
    let width_help = format!("Window width.  {}", headed_or_headless_applicable);
    let width_arg = Arg::with_name("width")
        .short("w")
        .long("width")
        .value_name("NUMBER")
        .help(&width_help)
        .takes_value(true)
        .validator(is_num_validator);
    let width_arg_required = width_arg.clone().required(true);
    let height_help = format!("Window height.  {}", headed_or_headless_applicable);
    let height_arg = Arg::with_name("height")
        .short("h")
        .long("height")
        .value_name("NUMBER")
        .help(&height_help)
        .takes_value(true)
        .validator(is_num_validator);
    let height_arg_required = height_arg.clone().required(true);

    App::new("Kosmonaut")
        .version("0.1")
        .author("Tyler Wilcock (twilco)")
        .about("A web browser for the space age 🚀")
        .arg(files_arg.clone())
        .arg(width_arg.clone())
        .arg(height_arg.clone())
        .arg(scale_factor_arg.clone())
        .subcommand(
            SubCommand::with_name(DUMP_LAYOUT_CMD_NAME)
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
                .arg(files_arg_required.clone())
                .arg(scale_factor_arg_required)
                .arg(width_arg_required)
                .arg(height_arg_required)
        )
        .subcommand(
            SubCommand::with_name(SIMILARITY_CMD_NAME)
                .long_about("
Performs a pixel-by-pixel comparison of the renderings of two input HTML files, returning their \
similarity as a percentage.  Use the --files flag to pass input HTML files.  If you any more or \
any less than two files, this command will error.  If these two files are not HTML, this command \
will error.
                ".trim())
                .arg(
                    Arg::with_name("similarity-percent-only")
                        .long("similarity-percent-only")
                        .value_name("BOOLEAN")
                        .help("Set to true to make the command only output the similarity percent between the two renderings (e.g. \"99.32%\".")
                        .takes_value(true)
                        .validator(is_bool_validator)
                )
                .arg(files_arg_required.clone())
                .arg(scale_factor_arg)
                .arg(width_arg)
                .arg(height_arg)
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

pub fn html_file_path_from_files_opt(files_opt: Option<Values>) -> Option<&str> {
    files_opt.map(html_file_path_from_files).flatten()
}

pub fn html_file_path_from_files(mut files: Values) -> Option<&str> {
    files.find(|file| {
        let parts = file.split('.');
        if let Some(last_part) = parts.last() {
            return last_part == "html";
        }
        false
    })
}

pub fn css_file_paths_from_files_opt(files_opt: Option<Values>) -> Vec<&str> {
    files_opt.map(css_file_paths_from_files).unwrap_or_default()
}

pub fn css_file_paths_from_files(files: Values) -> Vec<&str> {
    files
        .filter(|file| {
            let parts = file.split('.');
            if let Some(last_part) = parts.last() {
                return last_part == "css";
            }
            false
        })
        .collect::<Vec<_>>()
}

pub fn has_dump_layout_tree_subcommand(arg_matches: &ArgMatches) -> bool {
    arg_matches.subcommand_matches("dump-layout").is_some()
}

pub fn has_similarity_subcommand(arg_matches: &ArgMatches) -> bool {
    arg_matches.subcommand_matches("similarity").is_some()
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

pub fn dump_layout_tree_verbose(
    dump_layout_arg_matches: &ArgMatches,
) -> Option<DumpLayoutVerbosity> {
    try_get_bool(dump_layout_arg_matches, "verbose").map(|bool_verbose| match bool_verbose {
        true => DumpLayoutVerbosity::Verbose,
        false => DumpLayoutVerbosity::NonVerbose,
    })
}

#[derive(Clone, Debug)]
pub struct RenderCmd<'a> {
    pub files: Option<Values<'a>>,
    pub window_width: Option<f32>,
    pub window_height: Option<f32>,
    pub scale_factor: Option<f32>,
}

#[derive(Clone, Debug)]
pub struct DumpLayoutCmd<'a> {
    pub files: Values<'a>,
    pub window_width: f32,
    pub window_height: f32,
    pub scale_factor: f32,
    pub verbosity: DumpLayoutVerbosity,
}

#[derive(Clone, Debug)]
pub struct SimilarityCmd<'a> {
    pub files: Values<'a>,
    pub window_width: Option<f32>,
    pub window_height: Option<f32>,
    pub scale_factor: Option<f32>,
}

pub enum Command<'a> {
    Render(RenderCmd<'a>),
    DumpLayout(DumpLayoutCmd<'a>),
    Similarity(SimilarityCmd<'a>),
}

impl<'a> From<DumpLayoutCmd<'a>> for Command<'a> {
    fn from(cmd: DumpLayoutCmd<'a>) -> Self {
        Command::DumpLayout(cmd)
    }
}

impl<'a> From<RenderCmd<'a>> for Command<'a> {
    fn from(cmd: RenderCmd<'a>) -> Self {
        Command::Render(cmd)
    }
}

impl<'a> From<SimilarityCmd<'a>> for Command<'a> {
    fn from(cmd: SimilarityCmd<'a>) -> Self {
        Command::Similarity(cmd)
    }
}

pub fn get_command<'a>(global_matches: &'a ArgMatches) -> Result<Command<'a>, String> {
    Ok(if has_dump_layout_tree_subcommand(global_matches) {
        let matches = global_matches
            .subcommand_matches(DUMP_LAYOUT_CMD_NAME)
            .unwrap();
        // unwraps safe here because these args are marked as required for `dump-layout`.
        DumpLayoutCmd {
            files: matches.values_of("files").unwrap(),
            window_width: window_width(matches).unwrap(),
            window_height: window_height(matches).unwrap(),
            scale_factor: scale_factor(matches).unwrap(),
            verbosity: dump_layout_tree_verbose(matches).unwrap_or(DumpLayoutVerbosity::NonVerbose),
        }
        .into()
    } else if has_similarity_subcommand(global_matches) {
        let matches = global_matches
            .subcommand_matches(SIMILARITY_CMD_NAME)
            .unwrap();
        let files = matches.values_of("files").unwrap();
        validate_similarity_subcommand(&files)?;
        SimilarityCmd {
            files,
            window_width: window_width(matches),
            window_height: window_height(matches),
            scale_factor: scale_factor(matches),
        }
        .into()
    } else {
        // If no sub-command was specified, assume the user wants to render the headed-representation
        // of the passed --files.
        let files = global_matches.values_of("files");
        let width = window_width(global_matches);
        let height = window_height(global_matches);
        let scale_factor = scale_factor(global_matches);
        RenderCmd {
            files,
            window_width: width,
            window_height: height,
            scale_factor,
        }
        .into()
    })
}

pub fn window_width(arg_matches: &ArgMatches) -> Option<f32> {
    try_get_arg::<f32>(arg_matches, "width")
}

pub fn window_height(arg_matches: &ArgMatches) -> Option<f32> {
    try_get_arg::<f32>(arg_matches, "height")
}

pub fn scale_factor(arg_matches: &ArgMatches) -> Option<f32> {
    try_get_arg::<f32>(arg_matches, "scale-factor")
}

fn validate_similarity_subcommand(files: &Values) -> Result<(), String> {
    let help_str = "Run --help to learn how to use `similarity`.";
    let err_message = match files.len() {
        len @ 0..=1 => {
            format!(
                "`similarity` sub-command was specified but only {} --files were passed.  {}",
                len, help_str
            )
        }
        2 => {
            let mut msg = "".to_owned();
            for file in files.clone() {
                let parts = file.split('.');
                let bad_extension_msg = &format!(
                    "The `similarity` command only accepts .html files (got {}).  {}",
                    file, help_str
                );
                match parts.last() {
                    Some(file_extension) => {
                        if file_extension != "html" {
                            msg += bad_extension_msg
                        }
                    }
                    None => msg += bad_extension_msg,
                }
            }
            msg
        }
        len => {
            format!(
                "`similarity` sub-command was specified but too many --files ({}) were passed.  {}",
                len, help_str
            )
        }
    };

    match err_message.trim().is_empty() {
        true => Ok(()),
        false => Err(err_message),
    }
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
