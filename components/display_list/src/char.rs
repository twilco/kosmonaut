use cssparser::RGBA;
use gl::texture::TextureId;
use pathfinder_geometry::vector::Vector2F;

#[derive(Clone, Debug)]
pub struct CharCommand {
    /// The horizontal and vertical distance to the next glyph.
    advance: Vector2F,
    /// Offset from baseline to left/top of glyph.
    bearing: Vector2F,
    /// The char to be rendered.
    ch: char,
    /// The color to render the char as.
    color: RGBA,
    /// The size to render the char as.
    size: Vector2F,
    /// The x and y coordinates of where the glyph origin should be placed on the layout viewport.
    start_coords: Vector2F,
    // TODO: It would be better if CharCommand wasn't OpenGL-specific.
    // Refactor to CharCommand<T>, where T holds any data specific to a certain painting context.
    /// The OpenGL texture associated with this character.
    texture_id: TextureId,
}

impl CharCommand {
    pub fn new(
        advance: Vector2F,
        bearing: Vector2F,
        ch: char,
        color: RGBA,
        size: Vector2F,
        start_coords: Vector2F,
        texture_id: TextureId,
    ) -> Self {
        CharCommand {
            advance,
            bearing,
            ch,
            color,
            size,
            start_coords,
            texture_id,
        }
    }

    fn advance(&self) -> Vector2F {
        self.advance
    }

    pub fn bearing(&self) -> Vector2F {
        self.bearing
    }

    fn ch(&self) -> char {
        self.ch
    }

    pub fn color(&self) -> RGBA {
        self.color
    }

    pub fn size(&self) -> Vector2F {
        self.size
    }

    pub fn start_coords(&self) -> Vector2F {
        self.start_coords
    }

    pub fn texture_id(&self) -> TextureId {
        self.texture_id
    }
}
