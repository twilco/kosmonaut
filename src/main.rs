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
use crate::layout::{global_layout, DumpLayout, LayoutViewportDimensions};

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
use crate::gfx::display::{build_display_list, DisplayCommand, DisplayList};
use crate::gfx::headed::init_window_and_gl;
use crate::gfx::headless::init_framebuffer_and_gl;
use crate::gfx::paint::MasterPainter;
use crate::gfx::{
    resize_window, LogGlInfo, DEFAULT_LAYOUT_VIEWPORT_HEIGHT_PX, DEFAULT_LAYOUT_VIEWPORT_WIDTH_PX,
};
use crate::layout::box_tree::build_box_tree;
use crate::layout::layout_box::LayoutBox;
use crate::style::dom_integration::{apply_styles, extract_embedded_styles};
use crate::style::parse_css_to_rules;
use crate::style::stylesheet::Stylesheet;
use clap::Values;
pub use common::Side;
use cssparser::RGBA;
use gl::pixels::RgbaPixel;
use gl::Gl;
use glutin::event_loop::ControlFlow;
use glutin::{PossiblyCurrent, WindowedContext};
use std::cmp::{max, min};
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

        let viewport = LayoutViewportDimensions::new_px(self.window_width, self.window_height);
        let write_to = &mut std::io::stdout();
        match build_box_tree(styled_dom, None) {
            Some(mut box_tree) => {
                global_layout(&mut box_tree, viewport, self.scale_factor);
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
        fn paint_and_get_pixels(
            box_tree: Option<LayoutBox>,
            viewport: LayoutViewportDimensions,
            scale_factor: f32,
        ) -> Result<Vec<RgbaPixel>, String> {
            let (viewport_width, viewport_height) = viewport.width_height_px();
            let headless_gfx_context =
                init_framebuffer_and_gl(viewport_width, viewport_height, LogGlInfo::No);
            // Bind our framebuffer in preparation for painting.
            headless_gfx_context.bind_framebuffer();
            let mut painter = MasterPainter::new(headless_gfx_context.gl(), scale_factor)?;
            let char_handle = CharHandle::new(headless_gfx_context.gl());
            layout_and_paint_headless(box_tree, viewport, &char_handle, &mut painter, scale_factor);
            Ok(headless_gfx_context.read_pixels(viewport_width, viewport_height))
        }

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

        let (html_file_one, html_file_two) = (
            html_file_paths.get(0).unwrap(),
            html_file_paths.get(1).unwrap(),
        );
        let (dom_one, dom_two) = (
            load_and_style_dom(html_file_one, vec![]),
            load_and_style_dom(html_file_two, vec![]),
        );
        let (box_tree_one, box_tree_two) =
            (build_box_tree(dom_one, None), build_box_tree(dom_two, None));
        let viewport = LayoutViewportDimensions::new_px(
            self.window_width
                .unwrap_or(DEFAULT_LAYOUT_VIEWPORT_WIDTH_PX),
            self.window_height
                .unwrap_or(DEFAULT_LAYOUT_VIEWPORT_HEIGHT_PX),
        );
        let scale_factor = self.scale_factor.unwrap_or(1.0);
        let pixels_one = paint_and_get_pixels(box_tree_one, viewport, scale_factor)?;
        let pixels_two = paint_and_get_pixels(box_tree_two, viewport, scale_factor)?;
        let longest_len = max(pixels_one.len(), pixels_two.len());
        let shortest_len = min(pixels_one.len(), pixels_two.len());
        let mut num_differing_pixels: u64 = 0;
        for i in 0..shortest_len {
            if pixels_one.get(i).unwrap() != pixels_two.get(i).unwrap() {
                num_differing_pixels += 1;
            }
        }
        num_differing_pixels += (pixels_one.len() as i64 - pixels_two.len() as i64).abs() as u64;
        let percent_similar_str = if num_differing_pixels == 0 {
            "100".to_owned()
        } else {
            let percent = 100.0 - (num_differing_pixels as f64 / longest_len as f64) * 100.0;
            let str = format!("{:.6}", percent);
            let trimmed_percent = str.trim_end_matches('0').trim_end_matches('.');
            trimmed_percent.to_owned()
        };

        if self.percent_only {
            println!("{}", percent_similar_str);
        } else {
            // TODO: Make this look prettier (colors, bolding)
            println!(
                "'{}' and '{}' are {}% similar in a pixel-by-pixel comparison ({} / {} pixels different)",
                html_file_one,
                html_file_two,
                percent_similar_str,
                num_differing_pixels,
                longest_len
            );
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
            init_window_and_gl(self.window_width, self.window_height, LogGlInfo::Yes);
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
    layout_and_paint_headed(
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
                    layout_and_paint_headed(
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
                    layout_and_paint_headed(
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
}

fn layout_and_paint_headed(
    box_tree_opt: Option<LayoutBox>,
    windowed_context: &WindowedContext<PossiblyCurrent>,
    char_handle: &CharHandle,
    painter: &mut MasterPainter,
    scale_factor: f32,
) {
    let display_list = display_list_from_box_tree(
        box_tree_opt,
        windowed_context.window().inner_size().into(),
        char_handle,
        scale_factor,
    );
    painter.paint_headed(&windowed_context, &display_list);
}

fn layout_and_paint_headless(
    box_tree_opt: Option<LayoutBox>,
    viewport: LayoutViewportDimensions,
    char_handle: &CharHandle,
    painter: &mut MasterPainter,
    scale_factor: f32,
) {
    let display_list =
        display_list_from_box_tree(box_tree_opt, viewport, char_handle, scale_factor);
    painter.paint_headless(viewport, &display_list);
}

fn display_list_from_box_tree(
    box_tree_opt: Option<LayoutBox>,
    viewport: LayoutViewportDimensions,
    char_handle: &CharHandle,
    scale_factor: f32,
) -> DisplayList {
    if let Some(mut box_tree) = box_tree_opt {
        global_layout(&mut box_tree, viewport, scale_factor);
        build_display_list(&box_tree, &char_handle, scale_factor)
    } else {
        // There is no box tree to paint (e.g. in the case of `html { display: none }`, so paint
        // only the viewport background.
        // TODO: The viewport background color should come from system colors, not be hardcoded
        // to white.
        vec![DisplayCommand::ViewportBackground(RGBA::new(
            255, 255, 255, 0,
        ))]
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
