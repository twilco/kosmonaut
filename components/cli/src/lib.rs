use crate::matches::try_get_bool;
use clap::{App, Arg, ArgMatches, SubCommand};

const DUMP_LAYOUT_CMD_NAME: &str = "dump-layout";
const SIMILARITY_CMD_NAME: &str = "similarity";
const RENDER_INPUT_ARG_NAME: &str = "FILES OR URLS";
const DUMP_LAYOUT_INPUT_ARG_NAME: &str = "FILES";
const SIMILARITY_INPUT_ARG_NAME: &str = "FILES";

pub mod commands;
pub mod matches;

pub fn setup_and_get_cli_args<'a>() -> ArgMatches<'a> {
    let headed_or_headless_applicable =
        "Applicable in both headed and headless (e.g. the dump-layout and similarity commands) contexts.";
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
        .arg(width_arg.clone())
        .arg(height_arg.clone())
        .arg(scale_factor_arg.clone())
        .arg(
            Arg::with_name(RENDER_INPUT_ARG_NAME)
                .help("File(s) or URL(s) for Kosmonaut to render.")
                .index(1)
                .multiple(true)
        )
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
                .arg(
                    Arg::with_name(DUMP_LAYOUT_INPUT_ARG_NAME)
                        .help("File(s) for Kosmonaut dump the layout of.  Note only the first HTML file found is rendered, and beyond that only CSS files will be used.")
                        .index(1)
                        .required(true)
                        .min_values(1)
                )
                .arg(scale_factor_arg.clone().required(true))
                .arg(width_arg.clone().required(true))
                .arg(height_arg.clone().required(true))
        )
        .subcommand(
            SubCommand::with_name(SIMILARITY_CMD_NAME)
                .long_about("
Performs a pixel-by-pixel comparison of the renderings of two input HTML files, returning their \
similarity as a percentage.  If you pass any more or any less than two HTML files, this command will \
error.
                ".trim())
                .arg(
                    Arg::with_name("similarity-percent-only")
                        .long("similarity-percent-only")
                        .help("Set to true to make the command only output the similarity percent between the two renderings (e.g. \"99.32%\".")
                )
                .arg(
                    Arg::with_name(DUMP_LAYOUT_INPUT_ARG_NAME)
                        .help("Two HTML files for Kosmonaut to render and compare.")
                        .index(1)
                        .required(true)
                        .min_values(2)
                        .max_values(2)
                )
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
