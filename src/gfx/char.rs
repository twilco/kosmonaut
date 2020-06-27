use crate::gfx::display::{CharCommand, DisplayCommand, DisplayList};
use crate::gfx::font::{FontError, PostscriptName};
use crate::style::values::CSSFloat;
use accountable_refcell::{Ref, RefCell};
use app_units::Au;
use cssparser::RGBA;
use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::error::GlyphLoadingError;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use gl::texture::{Texture, TextureKind};
use gl::{
    Gl, CLAMP_TO_EDGE, LINEAR, RED, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER,
    TEXTURE_WRAP_S, TEXTURE_WRAP_T, UNPACK_ALIGNMENT, UNSIGNED_BYTE,
};
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use std::collections::HashMap;
use std::os::raw::c_void;
use gl::types::GLint;

#[derive(Debug)]
pub struct OpenglChar {
    /// Offset to advance to next glyph.
    /// TODO: This is in 1/64th pixels...this fact still needs to be handled somewhere.
    advance: Vector2F,
    /// Offset from baseline to left/top of glyph.
    bearing: Vector2F,
    /// The size of the character in pixels.
    size_px: CSSFloat,
    /// The OpenGL texture associated with this character.
    texture: Texture,
    /// The actual character value.
    val: char,
}

impl OpenglChar {
    pub fn new(ch: char, size_px: i32, font: &Font, gl: &Gl) -> Result<OpenglChar, CharError> {
        let glyph_id = match font.glyph_for_char(ch) {
            Some(id) => id,
            None => return Err(CharError::NoIdForChar),
        };
        let mut canvas = Canvas::new(Vector2I::splat(size_px), Format::A8);
        font.rasterize_glyph(
            &mut canvas,
            glyph_id,
            size_px as f32,
            Transform2F::from_translation(Vector2F::new(0.0, 32.0)),
            HintingOptions::None,
            RasterizationOptions::GrayscaleAa,
        )?;

        // TODO: Are advance and bearing correct?
        Ok(OpenglChar {
            advance: font.advance(glyph_id)?,
            bearing: font.origin(glyph_id)?,
            size_px: size_px as f32,
            texture: OpenglChar::setup_texture(gl, &canvas),
            val: ch,
        })
    }

    pub fn advance(&self) -> Vector2F {
        self.advance
    }

    pub fn bearing(&self) -> Vector2F {
        self.bearing
    }

    pub fn size_px(&self) -> CSSFloat {
        self.size_px
    }

    fn setup_texture(gl: &Gl, rasterized_canvas: &Canvas) -> Texture {
        let texture = Texture::new(TextureKind::TwoDimensional, gl);
        unsafe {
            // Set alignment to a single byte since we only use one byte per pixel.
            gl.PixelStorei(UNPACK_ALIGNMENT, 1);
            gl.BindTexture(TEXTURE_2D, texture.id());
            gl.TexImage2D(
                TEXTURE_2D,
                0,
                RED as GLint,
                rasterized_canvas.size.x(),
                rasterized_canvas.size.y(),
                0,
                RED,
                UNSIGNED_BYTE,
                rasterized_canvas.pixels.as_ptr() as *const c_void,
            );
            gl.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as GLint);
            gl.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as GLint);
            gl.TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as GLint);
            gl.TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as GLint);
            // Reset byte-alignment to default value of 4.
            gl.PixelStorei(UNPACK_ALIGNMENT, 4);
        }
        texture
    }
}

#[derive(Debug)]
pub enum CharError {
    Font(FontError),
    Loading(GlyphLoadingError),
    NoIdForChar,
}

impl From<GlyphLoadingError> for CharError {
    fn from(err: GlyphLoadingError) -> Self {
        CharError::Loading(err)
    }
}

impl From<FontError> for CharError {
    fn from(err: FontError) -> Self {
        CharError::Font(err)
    }
}

pub struct CharHandle {
    cached_chars: RefCell<HashMap<(PostscriptName, Au), HashMap<char, OpenglChar>>>,
    gl: Gl,
}

impl CharHandle {
    pub fn new(gl: &Gl) -> Self {
        CharHandle {
            cached_chars: RefCell::new(HashMap::new()),
            gl: gl.clone(),
        }
    }

    pub fn get_char(
        &self,
        font: &Font,
        font_size: Au,
        ch: char,
    ) -> Result<Ref<OpenglChar>, CharError> {
        let postscript_name = font.postscript_name().unwrap_or_else(|| {
            panic!(
                "couldn't get font postscript name for font with `full_name`: {}",
                font.full_name()
            )
        });
        let font_and_size_key = (postscript_name, font_size);
        {
            let mut font_to_chars = self.cached_chars.borrow_mut();
            let chars = match font_to_chars.get_mut(&font_and_size_key) {
                Some(chars) => chars,
                None => {
                    font_to_chars.insert(font_and_size_key.clone(), HashMap::new());
                    font_to_chars.get_mut(&font_and_size_key).unwrap()
                }
            };
            if !chars.contains_key(&ch) {
                chars.insert(
                    ch,
                    OpenglChar::new(ch, font_size.to_px(), font, &self.gl)?
                );
            }
        }
        let cached_chars = self.cached_chars.borrow();
        let char_ref = Ref::map(cached_chars, |font_to_chars| {
            let chars = font_to_chars.get(&font_and_size_key).unwrap();
            chars.get(&ch).unwrap()
        });
        Ok(char_ref)
    }

    // TODO: Not sure these `prepare` functions belong here in CharHandle...
    pub fn prepare_char(
        &self,
        display_list: &mut DisplayList,
        ch: char,
        color: RGBA,
        font: &Font,
        size: Au,
    ) -> Result<(), CharError> {
        let opengl_char = self.get_char(font, size, ch)?;
        display_list.push(DisplayCommand::Char(CharCommand::new(
            opengl_char.advance(),
            opengl_char.bearing(),
            ch,
            color,
            Vector2F::new(size.to_f32_px(), size.to_f32_px()),
            // TODO: These starting x and y coordinates need to be determined by layout once inline
            // layout is implemented.  For now, all characters will be painted over the top of  each
            // other at 0,0.
            Vector2F::new(0., 0.),
            opengl_char.texture.id(),
        )));
        Ok(())
    }

    pub fn prepare_str(
        &self,
        display_list: &mut DisplayList,
        color: RGBA,
        font: &Font,
        str: &str,
        size: Au,
    ) -> Result<(), CharError> {
        for char in str.chars() {
            self.prepare_char(display_list, char, color, font, size)?;
        }
        Ok(())
    }
}
