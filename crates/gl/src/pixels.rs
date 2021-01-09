use crate::bindings::types::{GLint, GLsizei, GLvoid};
use crate::{Gl, RGBA, UNSIGNED_BYTE};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RgbaPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Reads all pixels rendered to this GL instance into the given buffer via `glReadPixels`.
///
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadPixels.xhtml
pub fn read_pixels(gl: &Gl, window_width: GLint, window_height: GLint) -> Vec<RgbaPixel> {
    let total_window_pixels = window_width * window_height;
    // The `4` in this multiplication is because the pixel format we request below is RGBA (one byte
    // for each value), so it takes 4 bytes to describe each window pixel.
    let len = (4 * total_window_pixels) as usize;
    let mut pixels: Vec<u8> = Vec::with_capacity(len);
    // Fill the buffer to force allocation (`Vec`s with length zero don't allocate, even after `with_capacity`).
    // Without this, `pixels` would report a length of zero, since OpenGL is writing into the internal
    // buffer but not updating the buffer length.
    pixels.extend([0u8].iter().cycle().take(len));
    unsafe {
        gl.ReadPixels(
            0,
            0,
            window_width as GLsizei,
            window_height as GLsizei,
            RGBA,
            UNSIGNED_BYTE,
            pixels.as_mut_ptr() as *mut GLvoid,
        )
    }
    let rgba_chunks = pixels.chunks_exact(4);
    let remainder_len = rgba_chunks.remainder().len();
    if remainder_len > 0 {
        panic!(
            "gl.ReadPixels resulted in an unexpected remainder of size {}",
            remainder_len
        );
    }
    rgba_chunks
        .map(|rgba| RgbaPixel {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
            a: rgba[3],
        })
        .collect::<Vec<_>>()
}
