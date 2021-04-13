// TODO: I think OpenGL can do all of this NDC business for us.  The current implementation works, but I doubt it is idiomatic.
// I believe orthographic projection is what we need?
// https://learnopengl.com/Getting-started/Transformations
// https://learnopengl.com/Getting-started/Coordinate-Systems

use primitives::units::CSSFloat;

/// Converts `px_len` to an OpenGL normalized device coordinate (NDC), which is a float value
/// between -1 and 1 resulting from perspective division performed on clip coordinates.
///
/// An NDC value is what OpenGL uses when rendering vertices — if you give OpenGL a value
/// outside the -1 to 1 range, it will get clipped (made not visible on the screen).
///
/// `relative_to_px` is the length of the plane that `px_len` is relative to, which is required for
/// calculating the NDC.  For example, if `px_len` was an x-coord value, `relative_to` would be
/// the width of the layout viewport.
///
/// https://stackoverflow.com/a/52331832/2421349
/// https://www.khronos.org/opengl/wiki/Viewing_and_Transformations
#[inline(always)]
fn ndc(px_len: CSSFloat, relative_to_px: CSSFloat) -> f32 {
    (2.0 * (px_len + 0.5) / relative_to_px - 1.0) - distance_between_pixel_locations(relative_to_px)
}

/// Measures the distance between pixels in a direction, relative to the pixel length of the
/// dimension in question (width or height).
#[inline(always)]
fn distance_between_pixel_locations(relative_to_px: CSSFloat) -> f32 {
    1.0 / relative_to_px
}

/// Gets the NDC of an y-value, then flips the sign of the result to properly put content with a
/// lower pixel height towards the top of the screen rather than the bottom.  For example:
///
/// ```ignore
///               |||||> px height of the layout viewport
///      ndc(0.0, 800.0) === approx. -1.0, which is the bottom of the screen (wrong).
///          ^^^ px y-value of rectangle
///
///      ndc_y(0.0, 800.0) === approx. 1.0, which is the  of the screen (right).
/// ```
#[inline(always)]
pub fn ndc_y(px_len: CSSFloat, relative_to_height_px: CSSFloat) -> f32 {
    ndc(px_len, relative_to_height_px) * -1.0
}

/// Gets the NDC of an x-value.  The `ndc` function itself returns the correct value already from
/// the perspective of the layout viewport, so this function simply calls `ndc` as-is.
#[inline(always)]
pub fn ndc_x(px_len: CSSFloat, relative_to_width_px: CSSFloat) -> f32 {
    ndc(px_len, relative_to_width_px)
}
