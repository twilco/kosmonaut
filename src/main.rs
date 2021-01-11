#![feature(or_patterns)]
#![feature(type_name_of_val)]

#[macro_use]
extern crate cssparser;
#[macro_use]
extern crate html5ever;
#[macro_use]
extern crate matches;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate derive_builder;

use std::fs::File;

use crate::dom::parser::parse_html;
use crate::dom::traits::TendrilSink;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::EventLoop;

use crate::dom::tree::NodeRef;
use crate::layout::{global_layout, DumpLayout};

pub mod cli;
pub mod common;
pub mod dom;
pub mod gfx;
pub mod layout;
pub mod style;

use crate::cli::{
    css_file_paths_from_files, get_command, html_file_path_from_files,
    html_file_path_from_files_opt, setup_and_get_cli_args, CliCommand, Command, DumpLayoutCmd,
    RenderCmd, SimilarityCmd,
};
use crate::gfx::char::CharHandle;
use crate::gfx::display::{build_display_list, DisplayCommand};
use crate::gfx::headed::init_window_and_gl;
use crate::gfx::paint::MasterPainter;
use crate::gfx::resize_window;
use crate::layout::box_tree::build_box_tree;
use crate::layout::layout_box::LayoutBox;
use crate::style::dom_integration::{apply_styles, extract_embedded_styles};
use crate::style::parse_css_to_rules;
use crate::style::stylesheet::Stylesheet;
use clap::Values;
pub use common::Side;
use cssparser::RGBA;
use gl::Gl;
use glutin::event_loop::ControlFlow;
use glutin::{PossiblyCurrent, WindowedContext};
use std::io::Write;

/// Welcome to Kosmonaut.
///
/// > The path of a kosmonaut is not an easy, triumphant march to glory. You have to get to know the
/// > meaning not just of joy but also of grief before being allowed in the spacecraft cabin.
///     - Yuri Gagarin
#[allow(unused_variables)]
fn main() {
    let arg_matches = setup_and_get_cli_args();
    if let Err(err_msg) = get_command(&arg_matches).run() {
        eprintln!("{}", err_msg);
    };
}

impl<'a> CliCommand for Command<'a> {
    fn run(&self) -> Result<(), String> {
        match self {
            Command::Render(cmd) => cmd.run(),
            Command::DumpLayout(cmd) => cmd.run(),
            Command::Similarity(cmd) => cmd.run(),
        }
    }
}

impl CliCommand for DumpLayoutCmd<'_> {
    fn run(&self) -> Result<(), String> {
        let html_file_path = html_file_path_from_files(self.files.clone()).ok_or(
            "a .html file must be passed via --files when running the 'dump-layout' command",
        )?;
        let styled_dom = load_and_style_dom(html_file_path, get_author_sheets(self.files.clone()));

        let write_to = &mut std::io::stdout();
        match build_box_tree(styled_dom, None) {
            Some(mut box_tree) => {
                global_layout(
                    &mut box_tree,
                    self.window_width,
                    self.window_height,
                    self.scale_factor,
                );
                box_tree.dump_layout(write_to, 0, self.verbosity);
            }
            None => {
                write_to
                    .write_all("empty box tree".as_bytes())
                    .expect("could not write to stdout during layout dump");
            }
        };
        Ok(())
    }
}

impl CliCommand for SimilarityCmd<'_> {
    fn run(&self) -> Result<(), String> {
        let mut html_file_paths = Vec::new();
        for file in self.files.clone() {
            let parts = file.split('.');
            if let Some(file_extension) = parts.last() {
                if file_extension != "html" {
                    return Err(format!("The `similarity` command only accepts .html files (got {}).  Run --help to learn how to use `similarity`.", file));
                }
                html_file_paths.push(file);
            }
        }
        let num_paths = html_file_paths.len();
        if num_paths != 2 {
            return Err(format!("The `similarity` command requires that exactly 2 .html files are specified -- got {}", num_paths));
        }
        Ok(())
    }
}

impl CliCommand for RenderCmd<'_> {
    fn run(&self) -> Result<(), String> {
        let fallback_local_html = "tests/websrc/rainbow-divs.html";
        let html_file_path =
            html_file_path_from_files_opt(self.files.clone()).unwrap_or(fallback_local_html);
        let author_sheets = self
            .files
            .clone()
            .map(get_author_sheets)
            .unwrap_or_default();
        let styled_dom = load_and_style_dom(html_file_path, author_sheets);
        let (windowed_context, event_loop, gl) =
            init_window_and_gl(self.window_width, self.window_height);
        run_event_loop(
            event_loop,
            gl,
            styled_dom,
            windowed_context,
            self.scale_factor,
        );
        Ok(())
    }
}

fn load_and_style_dom(html_file_path: &str, author_sheets: Vec<Stylesheet>) -> NodeRef {
    let dom = parse_html()
        .from_utf8()
        .read_from(&mut File::open(html_file_path).unwrap())
        .unwrap();
    let mut embedded_styles_str = extract_embedded_styles(dom.clone());
    let embedded_styles = match parse_css_to_rules(&mut embedded_styles_str) {
        Ok(rules) => rules,
        Err(parse_error) => {
            println!("error parsing embedded styles: {:?}", parse_error);
            Vec::new()
        }
    };
    let ua_sheet = style::stylesheet::parse_css_to_stylesheet(
        Some("browser.css".to_owned()),
        &mut std::fs::read_to_string("web/browser.css").expect("file fail"),
    )
    .expect("parse stylesheet fail");
    apply_styles(
        dom.clone(),
        &embedded_styles,
        &[ua_sheet],
        &[],
        &author_sheets,
    );
    dom
}

fn get_author_sheets(files: Values) -> Vec<Stylesheet> {
    css_file_paths_from_files(files)
        .iter()
        .map(|css_file_path| {
            style::stylesheet::parse_css_to_stylesheet(
                Some((*css_file_path).to_owned()),
                &mut std::fs::read_to_string(css_file_path)
                    .expect("couldn't read css file to string"),
            )
            .expect("error parsing stylesheet")
        })
        .collect::<Vec<_>>()
}

pub fn run_event_loop(
    event_loop: EventLoop<()>,
    gl: Gl,
    styled_dom: NodeRef,
    windowed_context: WindowedContext<PossiblyCurrent>,
    cli_specified_scale_factor: Option<f32>,
) {
    // An un-laid-out tree of boxes, to be cloned from whenever a global layout is required.
    // This saves us from having to rebuild the entire box tree from the DOM when necessary,
    // instead only needing a clone.
    let clean_box_tree = build_box_tree(styled_dom, None);
    let char_handle = CharHandle::new(&gl);
    let mut scale = cli_specified_scale_factor.unwrap_or_else(|| {
        sanitize_windowed_context_scale_factor(windowed_context.window().scale_factor() as f32)
    });
    let mut master_painter = MasterPainter::new(&gl, scale).unwrap();
    paint(
        clean_box_tree.clone(),
        &windowed_context,
        &char_handle,
        &mut master_painter,
        scale,
    );
    event_loop.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    resize_window(&gl, &windowed_context, physical_size);
                    paint(
                        clean_box_tree.clone(),
                        &windowed_context,
                        &char_handle,
                        &mut master_painter,
                        scale,
                    )
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    new_inner_size,
                } => {
                    scale = *scale_factor as f32;
                    resize_window(&gl, &windowed_context, new_inner_size);
                    paint(
                        clean_box_tree.clone(),
                        &windowed_context,
                        &char_handle,
                        &mut master_painter,
                        scale,
                    )
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });

    fn paint(
        box_tree_opt: Option<LayoutBox>,
        windowed_context: &WindowedContext<PossiblyCurrent>,
        char_handle: &CharHandle,
        painter: &mut MasterPainter,
        scale_factor: f32,
    ) {
        let display_list = if let Some(mut box_tree) = box_tree_opt {
            let inner_window_size = windowed_context.window().inner_size();
            global_layout(
                &mut box_tree,
                inner_window_size.width as f32,
                inner_window_size.width as f32,
                scale_factor,
            );
            build_display_list(&box_tree, &char_handle, scale_factor)
        } else {
            // There is no box tree to paint (e.g. in the case of `html { display: none }`, so paint
            // only the viewport background.
            // TODO: The viewport background color should come from system colors, not be hardcoded
            // to white.
            vec![DisplayCommand::ViewportBackground(RGBA::new(
                255, 255, 255, 0,
            ))]
        };
        painter.paint(&windowed_context, &display_list);
    }
}

fn sanitize_windowed_context_scale_factor(scale_factor: f32) -> f32 {
    // Round the scale factor Glutin / Winit reports to the nearest integer.
    // This is a hack, and should go away eventually.  I've done it to make Kosmonaut match Firefox's
    // scale factor on X11, as before we were getting a scale factor of 1.16 while Firefox and others
    // use a scale factor of 1.  This behavior is definitely wrong, as sometimes fractional scaling
    // _is_ correct (e.g. Windows allows 1.25, 1.5, etc).  Read more here:
    // https://docs.rs/winit/0.24.0/winit/dpi/index.html#how-is-the-scale-factor-calculated
    scale_factor.round()
}
