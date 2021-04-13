#![feature(destructuring_assignment)]
#![feature(type_name_of_val)]

use std::fs::File;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::EventLoop;

use cli::commands::{get_command, Command, DumpLayoutCmd, RenderCmd, SimilarityCmd};
use cli::setup_and_get_cli_args;
use cssparser::RGBA;
use display_list::{build_display_list, DisplayCommand, DisplayList};
use dom::parser::parse_html;
use dom::styling::{apply_styles, extract_embedded_styles};
use dom::tree::NodeRef;
use gfx::char::CharHandle;
use gfx::headed::init_window_and_gl;
use gfx::headless::init_framebuffer_and_gl;
use gfx::paint::MasterPainter;
use gfx::{
    resize_window, LogGlInfo, DEFAULT_LAYOUT_VIEWPORT_HEIGHT_PX, DEFAULT_LAYOUT_VIEWPORT_WIDTH_PX,
};
use gl::pixels::RgbaPixel;
use gl::Gl;
use glutin::event_loop::ControlFlow;
use glutin::{PossiblyCurrent, WindowedContext};
use html5ever::tendril::TendrilSink;
use layout::box_tree::build_box_tree;
use layout::layout_box::LayoutBox;
use layout::{global_layout, DumpLayout, LayoutViewportDimensions};
use std::cmp::{max, min};
use std::error::Error;
use std::io::Write;
use std::path::Path;
use style::parse_css_to_rules;
use style::stylesheet::Stylesheet;
use url::Url;

const UA_STYLESHEET_STR: &str = include_str!("../web/useragent.css");

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

pub enum CommandReturn {
    DumpLayout(<DumpLayoutCmd as CliCommand>::RunReturn),
    Render(<RenderCmd as CliCommand>::RunReturn),
    Similarity(<SimilarityCmd as CliCommand>::RunReturn),
}

pub trait CliCommand {
    type RunReturn;
    fn run(&self) -> Result<Self::RunReturn, String>;
}

impl CliCommand for Command {
    type RunReturn = CommandReturn;

    fn run(&self) -> Result<Self::RunReturn, String> {
        match self {
            Command::Render(cmd) => cmd.run().map(|_| CommandReturn::Render(())),
            Command::DumpLayout(cmd) => cmd.run().map(|_| CommandReturn::DumpLayout(())),
            Command::Similarity(cmd) => cmd.run().map(CommandReturn::Similarity),
        }
    }
}

impl CliCommand for DumpLayoutCmd {
    type RunReturn = ();

    fn run(&self) -> Result<Self::RunReturn, String> {
        let html_file_path = html_file_path_from_files(self.file_paths.clone()).unwrap();
        let styled_dom = load_and_style_dom_from_file(
            html_file_path,
            get_author_sheets(self.file_paths.clone()),
        );

        let viewport = LayoutViewportDimensions::from_px(self.window_width, self.window_height);
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

impl CliCommand for SimilarityCmd {
    type RunReturn = f64;

    fn run(&self) -> Result<Self::RunReturn, String> {
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
        for file in self.file_paths.clone() {
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
            load_and_style_dom_from_file(html_file_one, vec![]),
            load_and_style_dom_from_file(html_file_two, vec![]),
        );
        let (box_tree_one, box_tree_two) =
            (build_box_tree(dom_one, None), build_box_tree(dom_two, None));
        let viewport = LayoutViewportDimensions::from_px(
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
            println!("{}", percent_similar_str)
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
        Ok(percent_similar_str.parse().expect(&*format!(
            "error running 'similarity' -- couldn't parse {} as float",
            percent_similar_str
        )))
    }
}

impl CliCommand for RenderCmd {
    type RunReturn = ();

    fn run(&self) -> Result<Self::RunReturn, String> {
        let fallback_local_html = "tests/websrc/rainbow-divs.html".to_owned();
        let author_sheets = self
            .files_or_urls
            .clone()
            .map(get_author_sheets)
            .unwrap_or_default();
        let styled_dom = if let Some(files_or_urls) = self.files_or_urls.clone() {
            style_dom(
                dom_from_file_or_url(files_or_urls.get(0).unwrap())?,
                author_sheets,
            )
        } else {
            load_and_style_dom_from_file(fallback_local_html, author_sheets)
        };
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

fn dom_from_file_or_url<S: AsRef<str>>(file_or_url: S) -> Result<NodeRef, String> {
    fn dom_from_readable<R: std::io::Read>(mut readable: R) -> Result<NodeRef, Box<dyn Error>> {
        parse_html()
            .from_utf8()
            .read_from(&mut readable)
            .map_err(|err| err.into())
    }
    let url_error = match Url::parse(file_or_url.as_ref()) {
        Ok(url) => match isahc::get(url.as_str()) {
            Ok(mut response) => {
                return dom_from_readable(response.body_mut()).map_err(|err| {
                    format!(
                        "failed to parse html with contents from url '{}': {}",
                        url.as_str(),
                        err
                    )
                });
            }
            Err(err) => format!("{}", err),
        },
        Err(err) => format!("{}", err),
    };
    let file_error = match File::open(file_or_url.as_ref()) {
        Ok(mut file) => {
            return dom_from_readable(&mut file).map_err(|err| {
                format!(
                    "failed to parse html with contents from file '{}': {}",
                    file_or_url.as_ref(),
                    err
                )
            })
        }
        Err(err) => err,
    };
    Err(format!(
        "input '{}' was neither a valid file path nor valid url\n{}\n{}",
        file_or_url.as_ref(),
        url_error,
        file_error.to_string()
    ))
}

fn html_file_path_from_files<S: AsRef<str>>(files: Vec<S>) -> Option<String> {
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

fn css_file_paths_from_files<S: AsRef<str>>(files: Vec<S>) -> Vec<String> {
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

fn style_dom(dom: NodeRef, author_sheets: Vec<Stylesheet>) -> NodeRef {
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
        &mut UA_STYLESHEET_STR.to_owned(),
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

fn load_and_style_dom_from_file<P: AsRef<Path>>(
    html_file_path: P,
    author_sheets: Vec<Stylesheet>,
) -> NodeRef {
    let dom = parse_html()
        .from_utf8()
        .read_from(&mut File::open(html_file_path).unwrap())
        .unwrap();
    style_dom(dom, author_sheets)
}

fn get_author_sheets<S: AsRef<str>>(file_paths: Vec<S>) -> Vec<Stylesheet> {
    css_file_paths_from_files(file_paths)
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
    _char_handle: &CharHandle,
    scale_factor: f32,
) -> DisplayList {
    if let Some(mut box_tree) = box_tree_opt {
        global_layout(&mut box_tree, viewport, scale_factor);
        build_display_list(&box_tree, scale_factor)
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
