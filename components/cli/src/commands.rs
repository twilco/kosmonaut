use crate::matches::{
    files_or_urls, has_dump_layout_tree_subcommand, has_similarity_subcommand, scale_factor,
    similarity_percent_only, window_height, window_width,
};
use crate::{
    dump_layout_tree_verbose, DumpLayoutVerbosity, DUMP_LAYOUT_CMD_NAME,
    DUMP_LAYOUT_INPUT_ARG_NAME, RENDER_INPUT_ARG_NAME, SIMILARITY_CMD_NAME,
    SIMILARITY_INPUT_ARG_NAME,
};
use clap::ArgMatches;

#[derive(Clone, Debug)]
pub struct RenderCmd {
    pub files_or_urls: Option<Vec<String>>,
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
        let file_paths = files_or_urls(matches, DUMP_LAYOUT_INPUT_ARG_NAME).unwrap();
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
        let file_paths = files_or_urls(matches, SIMILARITY_INPUT_ARG_NAME).unwrap();
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
        // of the passed file / URL.
        let width = window_width(global_matches);
        let height = window_height(global_matches);
        let scale_factor = scale_factor(global_matches);
        RenderCmd {
            files_or_urls: files_or_urls(global_matches, RENDER_INPUT_ARG_NAME),
            window_width: width,
            window_height: height,
            scale_factor,
        }
        .into()
    }
}
