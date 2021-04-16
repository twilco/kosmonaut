use clap::ArgMatches;
use std::str::FromStr;

pub(super) fn has_dump_layout_tree_subcommand(arg_matches: &ArgMatches) -> bool {
    arg_matches.subcommand_matches("dump-layout").is_some()
}

pub(super) fn has_similarity_subcommand(arg_matches: &ArgMatches) -> bool {
    arg_matches.subcommand_matches("similarity").is_some()
}

pub(super) fn files_or_urls(arg_matches: &ArgMatches, arg_name: &str) -> Option<Vec<String>> {
    arg_matches
        .values_of(arg_name)
        .map(|values| values.map(|value| value.to_owned()).collect::<Vec<_>>())
}

pub(super) fn window_width(arg_matches: &ArgMatches) -> Option<f32> {
    try_get_arg::<f32>(arg_matches, "width")
}

pub(super) fn window_height(arg_matches: &ArgMatches) -> Option<f32> {
    try_get_arg::<f32>(arg_matches, "height")
}

pub(super) fn scale_factor(arg_matches: &ArgMatches) -> Option<f32> {
    try_get_arg::<f32>(arg_matches, "scale-factor")
}

pub(super) fn similarity_percent_only(arg_matches: &ArgMatches) -> bool {
    arg_matches.is_present("similarity-percent-only")
}

fn try_get_arg<'a, T: FromStr>(arg_matches: &ArgMatches, arg_name: &'a str) -> Option<T> {
    arg_matches
        .value_of(arg_name)
        .map(|arg_str| arg_str.parse::<T>().ok())
        .unwrap_or(None)
}

pub(super) fn try_get_bool<'a>(arg_matches: &ArgMatches, arg_name: &'a str) -> Option<bool> {
    try_get_arg::<bool>(arg_matches, arg_name).or_else(|| {
        let arg_match = arg_matches.value_of(arg_name);
        arg_match.and_then(|val| match val {
            "0" => Some(false),
            "1" => Some(true),
            _ => None,
        })
    })
}
