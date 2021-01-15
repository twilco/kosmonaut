use clap::{App, Arg, ArgMatches, SubCommand};
use std::str::FromStr;

const DUMP_LAYOUT_CMD_NAME: &str = "dump-layout";
const SIMILARITY_CMD_NAME: &str = "similarity";

pub fn setup_and_get_cli_args<'a>() -> ArgMatches<'a> {
    let headed_or_headless_applicable =
        "Applicable in both headed and headless (e.g. the dump-layout and similarity commands) contexts.";
    let default_files_arg_value_name = "FILES";
    let files_arg = Arg::with_name("files")
            .short("f")
            .long("files")
            .help("Pass files for Kosmonaut to render.  This is also the flag that should be used to pass files to any sub-command (e.g. `dump-layout`, `similarity`).")
            .multiple(true)
            .takes_value(true);
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
    let width_help = format!("Window width.  {}", headed_or_headless_applicable);
    let width_arg = Arg::with_name("width")
        .short("w")
        .long("width")
        .value_name("NUMBER")
        .help(&width_help)
        .takes_value(true)
        .validator(is_num_validator);
    let height_help = format!("Window height.  {}", headed_or_headless_applicable);
    let height_arg = Arg::with_name("height")
        .short("h")
        .long("height")
        .value_name("NUMBER")
        .help(&height_help)
        .takes_value(true)
        .validator(is_num_validator);

    App::new("Kosmonaut")
        .version("0.1")
        .author("Tyler Wilcock (twilco)")
        .about("A web browser for the space age 🚀")
        .arg(files_arg.clone().value_name(default_files_arg_value_name))
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
                .arg(files_arg.clone().required(true).value_name(default_files_arg_value_name))
                .arg(scale_factor_arg.clone().required(true))
                .arg(width_arg.clone().required(true))
                .arg(height_arg.clone().required(true))
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
                        .help("Set to true to make the command only output the similarity percent between the two renderings (e.g. \"99.32%\".")
                )
                .arg(files_arg.required(true).min_values(2).max_values(2).value_name("EXACTLY TWO HTML FILES"))
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

pub fn html_file_path_from_files_opt<S: AsRef<str>>(files_opt: Option<Vec<S>>) -> Option<String> {
    files_opt.map(html_file_path_from_files).flatten()
}

pub fn html_file_path_from_files<S: AsRef<str>>(files: Vec<S>) -> Option<String> {
    files
        .iter()
        .find(|file| {
            let parts = file.as_ref().split('.');
            if let Some(last_part) = parts.last() {
                return last_part == "html";
            }
            false
        })
        .map(|file_path| file_path.as_ref().to_owned())
}

pub fn css_file_paths_from_files_opt<S: AsRef<str>>(files_opt: Option<Vec<S>>) -> Vec<String> {
    files_opt.map(css_file_paths_from_files).unwrap_or_default()
}

pub fn css_file_paths_from_files<S: AsRef<str>>(files: Vec<S>) -> Vec<String> {
    files
        .iter()
        .filter(|file| {
            let parts = file.as_ref().split('.');
            if let Some(last_part) = parts.last() {
                return last_part == "css";
            }
            false
        })
        .map(|file_path| file_path.as_ref().to_owned())
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

pub trait CliCommand {
    type RunReturn;

    fn run(&self) -> Result<Self::RunReturn, String>;
}

#[derive(Clone, Debug)]
pub struct RenderCmd {
    pub file_paths: Option<Vec<String>>,
    pub window_width: Option<f32>,
    pub window_height: Option<f32>,
    pub scale_factor: Option<f32>,
}

#[derive(Clone, Debug)]
pub struct DumpLayoutCmd {
    pub file_paths: Vec<String>,
    pub window_width: f32,
    pub window_height: f32,
    pub scale_factor: f32,
    pub verbosity: DumpLayoutVerbosity,
}

#[derive(Clone, Debug)]
pub struct SimilarityCmd {
    pub file_paths: Vec<String>,
    pub window_width: Option<f32>,
    pub window_height: Option<f32>,
    pub scale_factor: Option<f32>,
    pub percent_only: bool,
}

pub enum Command {
    Render(RenderCmd),
    DumpLayout(DumpLayoutCmd),
    Similarity(SimilarityCmd),
}

impl From<DumpLayoutCmd> for Command {
    fn from(cmd: DumpLayoutCmd) -> Self {
        Command::DumpLayout(cmd)
    }
}

impl From<RenderCmd> for Command {
    fn from(cmd: RenderCmd) -> Self {
        Command::Render(cmd)
    }
}

impl From<SimilarityCmd> for Command {
    fn from(cmd: SimilarityCmd) -> Self {
        Command::Similarity(cmd)
    }
}

pub fn get_command(global_matches: &ArgMatches) -> Command {
    if has_dump_layout_tree_subcommand(global_matches) {
        let matches = global_matches
            .subcommand_matches(DUMP_LAYOUT_CMD_NAME)
            .unwrap();
        // unwraps safe here because these args are marked as required for `dump-layout`.
        let file_paths = matches
            .values_of("files")
            .unwrap()
            .map(|value| value.to_owned())
            .collect::<Vec<_>>();
        DumpLayoutCmd {
            file_paths,
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
        let file_paths = matches
            .values_of("files")
            .unwrap()
            .map(|value| value.to_owned())
            .collect::<Vec<_>>();
        SimilarityCmd {
            file_paths,
            window_width: window_width(matches),
            window_height: window_height(matches),
            scale_factor: scale_factor(matches),
            percent_only: similarity_percent_only(matches),
        }
        .into()
    } else {
        // If no sub-command was specified, assume the user wants to render the headed-representation
        // of the passed --files.
        let file_paths = match global_matches.values_of("files") {
            Some(values) => Some(values.map(|value| value.to_owned()).collect::<Vec<_>>()),
            None => None,
        };
        let width = window_width(global_matches);
        let height = window_height(global_matches);
        let scale_factor = scale_factor(global_matches);
        RenderCmd {
            file_paths,
            window_width: width,
            window_height: height,
            scale_factor,
        }
        .into()
    }
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

pub fn similarity_percent_only(arg_matches: &ArgMatches) -> bool {
    arg_matches.is_present("similarity-percent-only")
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
