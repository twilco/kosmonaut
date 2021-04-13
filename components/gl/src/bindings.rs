// Auto-generated code below:
mod __gl_imports {
    pub use std::marker::Send;
    pub use std::mem;
    pub use std::os::raw;
}

pub mod types {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        dead_code,
        missing_copy_implementations,
        clippy::upper_case_acronyms
    )]

    // Common types from OpenGL 1.1
    pub type GLenum = super::__gl_imports::raw::c_uint;
    pub type GLboolean = super::__gl_imports::raw::c_uchar;
    pub type GLbitfield = super::__gl_imports::raw::c_uint;
    pub type GLvoid = super::__gl_imports::raw::c_void;
    pub type GLbyte = super::__gl_imports::raw::c_char;
    pub type GLshort = super::__gl_imports::raw::c_short;
    pub type GLint = super::__gl_imports::raw::c_int;
    pub type GLclampx = super::__gl_imports::raw::c_int;
    pub type GLubyte = super::__gl_imports::raw::c_uchar;
    pub type GLushort = super::__gl_imports::raw::c_ushort;
    pub type GLuint = super::__gl_imports::raw::c_uint;
    pub type GLsizei = super::__gl_imports::raw::c_int;
    pub type GLfloat = super::__gl_imports::raw::c_float;
    pub type GLclampf = super::__gl_imports::raw::c_float;
    pub type GLdouble = super::__gl_imports::raw::c_double;
    pub type GLclampd = super::__gl_imports::raw::c_double;
    pub type GLeglImageOES = *const super::__gl_imports::raw::c_void;
    pub type GLchar = super::__gl_imports::raw::c_char;
    pub type GLcharARB = super::__gl_imports::raw::c_char;

    #[cfg(target_os = "macos")]
    pub type GLhandleARB = *const super::__gl_imports::raw::c_void;
    #[cfg(not(target_os = "macos"))]
    pub type GLhandleARB = super::__gl_imports::raw::c_uint;

    pub type GLhalfARB = super::__gl_imports::raw::c_ushort;
    pub type GLhalf = super::__gl_imports::raw::c_ushort;

    // Must be 32 bits
    pub type GLfixed = GLint;

    pub type GLintptr = isize;
    pub type GLsizeiptr = isize;
    pub type GLint64 = i64;
    pub type GLuint64 = u64;
    pub type GLintptrARB = isize;
    pub type GLsizeiptrARB = isize;
    pub type GLint64EXT = i64;
    pub type GLuint64EXT = u64;

    pub enum __GLsync {}
    pub type GLsync = *const __GLsync;

    // compatible with OpenCL cl_context
    pub enum _cl_context {}
    pub enum _cl_event {}

    pub type GLDEBUGPROC = Option<
        extern "system" fn(
            source: GLenum,
            gltype: GLenum,
            id: GLuint,
            severity: GLenum,
            length: GLsizei,
            message: *const GLchar,
            userParam: *mut super::__gl_imports::raw::c_void,
        ),
    >;
    pub type GLDEBUGPROCARB = Option<
        extern "system" fn(
            source: GLenum,
            gltype: GLenum,
            id: GLuint,
            severity: GLenum,
            length: GLsizei,
            message: *const GLchar,
            userParam: *mut super::__gl_imports::raw::c_void,
        ),
    >;
    pub type GLDEBUGPROCKHR = Option<
        extern "system" fn(
            source: GLenum,
            gltype: GLenum,
            id: GLuint,
            severity: GLenum,
            length: GLsizei,
            message: *const GLchar,
            userParam: *mut super::__gl_imports::raw::c_void,
        ),
    >;

    // GLES 1 types
    // "pub type GLclampx = i32;",

    // GLES 1/2 types (tagged for GLES 1)
    // "pub type GLbyte = i8;",
    // "pub type GLubyte = u8;",
    // "pub type GLfloat = GLfloat;",
    // "pub type GLclampf = GLfloat;",
    // "pub type GLfixed = i32;",
    // "pub type GLint64 = i64;",
    // "pub type GLuint64 = u64;",
    // "pub type GLintptr = intptr_t;",
    // "pub type GLsizeiptr = ssize_t;",

    // GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
    // "pub type GLbyte = i8;",
    // "pub type GLubyte = u8;",
    // "pub type GLfloat = GLfloat;",
    // "pub type GLclampf = GLfloat;",
    // "pub type GLfixed = i32;",
    // "pub type GLint64 = i64;",
    // "pub type GLuint64 = u64;",
    // "pub type GLint64EXT = i64;",
    // "pub type GLuint64EXT = u64;",
    // "pub type GLintptr = intptr_t;",
    // "pub type GLsizeiptr = ssize_t;",

    // GLES 2 types (none currently)

    // Vendor extension types
    pub type GLDEBUGPROCAMD = Option<
        extern "system" fn(
            id: GLuint,
            category: GLenum,
            severity: GLenum,
            length: GLsizei,
            message: *const GLchar,
            userParam: *mut super::__gl_imports::raw::c_void,
        ),
    >;
    pub type GLhalfNV = super::__gl_imports::raw::c_ushort;
    pub type GLvdpauSurfaceNV = GLintptr;
}
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_ATTRIBUTES: types::GLenum = 0x8B89;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: types::GLenum = 0x8B8A;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_TEXTURE: types::GLenum = 0x84E0;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_UNIFORMS: types::GLenum = 0x8B86;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_UNIFORM_BLOCKS: types::GLenum = 0x8A36;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: types::GLenum = 0x8A35;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_UNIFORM_MAX_LENGTH: types::GLenum = 0x8B87;
#[allow(dead_code, non_upper_case_globals)]
pub const ALIASED_LINE_WIDTH_RANGE: types::GLenum = 0x846E;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA: types::GLenum = 0x1906;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA_REF_COMMAND_NV: types::GLenum = 0x000F;
#[allow(dead_code, non_upper_case_globals)]
pub const ALREADY_SIGNALED: types::GLenum = 0x911A;
#[allow(dead_code, non_upper_case_globals)]
pub const ALWAYS: types::GLenum = 0x0207;
#[allow(dead_code, non_upper_case_globals)]
pub const AND: types::GLenum = 0x1501;
#[allow(dead_code, non_upper_case_globals)]
pub const AND_INVERTED: types::GLenum = 0x1504;
#[allow(dead_code, non_upper_case_globals)]
pub const AND_REVERSE: types::GLenum = 0x1502;
#[allow(dead_code, non_upper_case_globals)]
pub const ANY_SAMPLES_PASSED: types::GLenum = 0x8C2F;
#[allow(dead_code, non_upper_case_globals)]
pub const ARRAY_BUFFER: types::GLenum = 0x8892;
#[allow(dead_code, non_upper_case_globals)]
pub const ARRAY_BUFFER_BINDING: types::GLenum = 0x8894;
#[allow(dead_code, non_upper_case_globals)]
pub const ATTACHED_SHADERS: types::GLenum = 0x8B85;
#[allow(dead_code, non_upper_case_globals)]
pub const ATTRIBUTE_ADDRESS_COMMAND_NV: types::GLenum = 0x0009;
#[allow(dead_code, non_upper_case_globals)]
pub const BACK: types::GLenum = 0x0405;
#[allow(dead_code, non_upper_case_globals)]
pub const BACK_LEFT: types::GLenum = 0x0402;
#[allow(dead_code, non_upper_case_globals)]
pub const BACK_RIGHT: types::GLenum = 0x0403;
#[allow(dead_code, non_upper_case_globals)]
pub const BGR: types::GLenum = 0x80E0;
#[allow(dead_code, non_upper_case_globals)]
pub const BGRA: types::GLenum = 0x80E1;
#[allow(dead_code, non_upper_case_globals)]
pub const BGRA_INTEGER: types::GLenum = 0x8D9B;
#[allow(dead_code, non_upper_case_globals)]
pub const BGR_INTEGER: types::GLenum = 0x8D9A;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND: types::GLenum = 0x0BE2;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_COLOR: types::GLenum = 0x8005;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_COLOR_COMMAND_NV: types::GLenum = 0x000B;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_DST: types::GLenum = 0x0BE0;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_DST_ALPHA: types::GLenum = 0x80CA;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_DST_RGB: types::GLenum = 0x80C8;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_EQUATION: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_EQUATION_ALPHA: types::GLenum = 0x883D;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_EQUATION_RGB: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_SRC: types::GLenum = 0x0BE1;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_SRC_ALPHA: types::GLenum = 0x80CB;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_SRC_RGB: types::GLenum = 0x80C9;
#[allow(dead_code, non_upper_case_globals)]
pub const BLUE: types::GLenum = 0x1905;
#[allow(dead_code, non_upper_case_globals)]
pub const BLUE_INTEGER: types::GLenum = 0x8D96;
#[allow(dead_code, non_upper_case_globals)]
pub const BOOL: types::GLenum = 0x8B56;
#[allow(dead_code, non_upper_case_globals)]
pub const BOOL_VEC2: types::GLenum = 0x8B57;
#[allow(dead_code, non_upper_case_globals)]
pub const BOOL_VEC3: types::GLenum = 0x8B58;
#[allow(dead_code, non_upper_case_globals)]
pub const BOOL_VEC4: types::GLenum = 0x8B59;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_ACCESS: types::GLenum = 0x88BB;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_ACCESS_FLAGS: types::GLenum = 0x911F;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_MAPPED: types::GLenum = 0x88BC;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_MAP_LENGTH: types::GLenum = 0x9120;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_MAP_OFFSET: types::GLenum = 0x9121;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_MAP_POINTER: types::GLenum = 0x88BD;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_SIZE: types::GLenum = 0x8764;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_USAGE: types::GLenum = 0x8765;
#[allow(dead_code, non_upper_case_globals)]
pub const BYTE: types::GLenum = 0x1400;
#[allow(dead_code, non_upper_case_globals)]
pub const CCW: types::GLenum = 0x0901;
#[allow(dead_code, non_upper_case_globals)]
pub const CLAMP_READ_COLOR: types::GLenum = 0x891C;
#[allow(dead_code, non_upper_case_globals)]
pub const CLAMP_TO_BORDER: types::GLenum = 0x812D;
#[allow(dead_code, non_upper_case_globals)]
pub const CLAMP_TO_EDGE: types::GLenum = 0x812F;
#[allow(dead_code, non_upper_case_globals)]
pub const CLEAR: types::GLenum = 0x1500;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE0: types::GLenum = 0x3000;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE1: types::GLenum = 0x3001;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE2: types::GLenum = 0x3002;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE3: types::GLenum = 0x3003;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE4: types::GLenum = 0x3004;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE5: types::GLenum = 0x3005;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE6: types::GLenum = 0x3006;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE7: types::GLenum = 0x3007;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR: types::GLenum = 0x1800;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT0: types::GLenum = 0x8CE0;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT1: types::GLenum = 0x8CE1;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT10: types::GLenum = 0x8CEA;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT11: types::GLenum = 0x8CEB;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT12: types::GLenum = 0x8CEC;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT13: types::GLenum = 0x8CED;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT14: types::GLenum = 0x8CEE;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT15: types::GLenum = 0x8CEF;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT16: types::GLenum = 0x8CF0;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT17: types::GLenum = 0x8CF1;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT18: types::GLenum = 0x8CF2;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT19: types::GLenum = 0x8CF3;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT2: types::GLenum = 0x8CE2;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT20: types::GLenum = 0x8CF4;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT21: types::GLenum = 0x8CF5;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT22: types::GLenum = 0x8CF6;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT23: types::GLenum = 0x8CF7;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT24: types::GLenum = 0x8CF8;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT25: types::GLenum = 0x8CF9;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT26: types::GLenum = 0x8CFA;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT27: types::GLenum = 0x8CFB;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT28: types::GLenum = 0x8CFC;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT29: types::GLenum = 0x8CFD;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT3: types::GLenum = 0x8CE3;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT30: types::GLenum = 0x8CFE;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT31: types::GLenum = 0x8CFF;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT4: types::GLenum = 0x8CE4;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT5: types::GLenum = 0x8CE5;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT6: types::GLenum = 0x8CE6;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT7: types::GLenum = 0x8CE7;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT8: types::GLenum = 0x8CE8;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT9: types::GLenum = 0x8CE9;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_BUFFER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_CLEAR_VALUE: types::GLenum = 0x0C22;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_LOGIC_OP: types::GLenum = 0x0BF2;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_WRITEMASK: types::GLenum = 0x0C23;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPARE_REF_TO_TEXTURE: types::GLenum = 0x884E;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPILE_STATUS: types::GLenum = 0x8B81;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RED: types::GLenum = 0x8225;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RED_RGTC1: types::GLenum = 0x8DBB;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RG: types::GLenum = 0x8226;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGB: types::GLenum = 0x84ED;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGBA: types::GLenum = 0x84EE;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RG_RGTC2: types::GLenum = 0x8DBD;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SIGNED_RED_RGTC1: types::GLenum = 0x8DBC;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SIGNED_RG_RGTC2: types::GLenum = 0x8DBE;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB: types::GLenum = 0x8C48;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB_ALPHA: types::GLenum = 0x8C49;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A3;
#[allow(dead_code, non_upper_case_globals)]
pub const CONDITION_SATISFIED: types::GLenum = 0x911C;
#[allow(dead_code, non_upper_case_globals)]
pub const CONSTANT_ALPHA: types::GLenum = 0x8003;
#[allow(dead_code, non_upper_case_globals)]
pub const CONSTANT_COLOR: types::GLenum = 0x8001;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_CORE_PROFILE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_FLAGS: types::GLenum = 0x821E;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_PROFILE_MASK: types::GLenum = 0x9126;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY: types::GLenum = 0x1503;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_INVERTED: types::GLenum = 0x150C;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_READ_BUFFER: types::GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_WRITE_BUFFER: types::GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)]
pub const CULL_FACE: types::GLenum = 0x0B44;
#[allow(dead_code, non_upper_case_globals)]
pub const CULL_FACE_MODE: types::GLenum = 0x0B45;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_PROGRAM: types::GLenum = 0x8B8D;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_QUERY: types::GLenum = 0x8865;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_VERTEX_ATTRIB: types::GLenum = 0x8626;
#[allow(dead_code, non_upper_case_globals)]
pub const CW: types::GLenum = 0x0900;
#[allow(dead_code, non_upper_case_globals)]
pub const DECR: types::GLenum = 0x1E03;
#[allow(dead_code, non_upper_case_globals)]
pub const DECR_WRAP: types::GLenum = 0x8508;
#[allow(dead_code, non_upper_case_globals)]
pub const DELETE_STATUS: types::GLenum = 0x8B80;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH: types::GLenum = 0x1801;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH24_STENCIL8: types::GLenum = 0x88F0;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH32F_STENCIL8: types::GLenum = 0x8CAD;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_ATTACHMENT: types::GLenum = 0x8D00;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_BUFFER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_CLAMP: types::GLenum = 0x864F;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_CLEAR_VALUE: types::GLenum = 0x0B73;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT: types::GLenum = 0x1902;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT16: types::GLenum = 0x81A5;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT24: types::GLenum = 0x81A6;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT32: types::GLenum = 0x81A7;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT32F: types::GLenum = 0x8CAC;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_FUNC: types::GLenum = 0x0B74;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_RANGE: types::GLenum = 0x0B70;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_STENCIL: types::GLenum = 0x84F9;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_STENCIL_ATTACHMENT: types::GLenum = 0x821A;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_TEST: types::GLenum = 0x0B71;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_WRITEMASK: types::GLenum = 0x0B72;
#[allow(dead_code, non_upper_case_globals)]
pub const DITHER: types::GLenum = 0x0BD0;
#[allow(dead_code, non_upper_case_globals)]
pub const DONT_CARE: types::GLenum = 0x1100;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE: types::GLenum = 0x140A;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLEBUFFER: types::GLenum = 0x0C32;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_ARRAYS_COMMAND_NV: types::GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_ARRAYS_INSTANCED_COMMAND_NV: types::GLenum = 0x0007;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_ARRAYS_STRIP_COMMAND_NV: types::GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER: types::GLenum = 0x0C01;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER0: types::GLenum = 0x8825;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER1: types::GLenum = 0x8826;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER10: types::GLenum = 0x882F;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER11: types::GLenum = 0x8830;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER12: types::GLenum = 0x8831;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER13: types::GLenum = 0x8832;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER14: types::GLenum = 0x8833;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER15: types::GLenum = 0x8834;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER2: types::GLenum = 0x8827;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER3: types::GLenum = 0x8828;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER4: types::GLenum = 0x8829;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER5: types::GLenum = 0x882A;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER6: types::GLenum = 0x882B;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER7: types::GLenum = 0x882C;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER8: types::GLenum = 0x882D;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER9: types::GLenum = 0x882E;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_ELEMENTS_COMMAND_NV: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_ELEMENTS_INSTANCED_COMMAND_NV: types::GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_ELEMENTS_STRIP_COMMAND_NV: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_FRAMEBUFFER: types::GLenum = 0x8CA9;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)]
pub const DST_ALPHA: types::GLenum = 0x0304;
#[allow(dead_code, non_upper_case_globals)]
pub const DST_COLOR: types::GLenum = 0x0306;
#[allow(dead_code, non_upper_case_globals)]
pub const DYNAMIC_COPY: types::GLenum = 0x88EA;
#[allow(dead_code, non_upper_case_globals)]
pub const DYNAMIC_DRAW: types::GLenum = 0x88E8;
#[allow(dead_code, non_upper_case_globals)]
pub const DYNAMIC_READ: types::GLenum = 0x88E9;
#[allow(dead_code, non_upper_case_globals)]
pub const ELEMENT_ADDRESS_COMMAND_NV: types::GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)]
pub const ELEMENT_ARRAY_BUFFER: types::GLenum = 0x8893;
#[allow(dead_code, non_upper_case_globals)]
pub const ELEMENT_ARRAY_BUFFER_BINDING: types::GLenum = 0x8895;
#[allow(dead_code, non_upper_case_globals)]
pub const EQUAL: types::GLenum = 0x0202;
#[allow(dead_code, non_upper_case_globals)]
pub const EQUIV: types::GLenum = 0x1509;
#[allow(dead_code, non_upper_case_globals)]
pub const EXTENSIONS: types::GLenum = 0x1F03;
#[allow(dead_code, non_upper_case_globals)]
pub const FALSE: types::GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const FASTEST: types::GLenum = 0x1101;
#[allow(dead_code, non_upper_case_globals)]
pub const FILL: types::GLenum = 0x1B02;
#[allow(dead_code, non_upper_case_globals)]
pub const FIRST_VERTEX_CONVENTION: types::GLenum = 0x8E4D;
#[allow(dead_code, non_upper_case_globals)]
pub const FIXED_ONLY: types::GLenum = 0x891D;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT: types::GLenum = 0x1406;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_32_UNSIGNED_INT_24_8_REV: types::GLenum = 0x8DAD;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT2: types::GLenum = 0x8B5A;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT2x3: types::GLenum = 0x8B65;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT2x4: types::GLenum = 0x8B66;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT3: types::GLenum = 0x8B5B;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT3x2: types::GLenum = 0x8B67;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT3x4: types::GLenum = 0x8B68;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT4: types::GLenum = 0x8B5C;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT4x2: types::GLenum = 0x8B69;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT4x3: types::GLenum = 0x8B6A;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_VEC2: types::GLenum = 0x8B50;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_VEC3: types::GLenum = 0x8B51;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_VEC4: types::GLenum = 0x8B52;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_SHADER: types::GLenum = 0x8B30;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_SHADER_DERIVATIVE_HINT: types::GLenum = 0x8B8B;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER: types::GLenum = 0x8D40;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: types::GLenum = 0x8215;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: types::GLenum = 0x8214;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: types::GLenum = 0x8210;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: types::GLenum = 0x8211;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: types::GLenum = 0x8216;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: types::GLenum = 0x8213;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_LAYERED: types::GLenum = 0x8DA7;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: types::GLenum = 0x8CD1;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: types::GLenum = 0x8CD0;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: types::GLenum = 0x8212;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: types::GLenum = 0x8217;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: types::GLenum = 0x8CD3;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: types::GLenum = 0x8CD4;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: types::GLenum = 0x8CD2;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_COMPLETE: types::GLenum = 0x8CD5;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT: types::GLenum = 0x8218;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: types::GLenum = 0x8CD6;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: types::GLenum = 0x8CDB;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: types::GLenum = 0x8DA8;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: types::GLenum = 0x8CD7;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: types::GLenum = 0x8D56;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: types::GLenum = 0x8CDC;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_SRGB: types::GLenum = 0x8DB9;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_UNDEFINED: types::GLenum = 0x8219;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_UNSUPPORTED: types::GLenum = 0x8CDD;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT: types::GLenum = 0x0404;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_AND_BACK: types::GLenum = 0x0408;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_FACE: types::GLenum = 0x0B46;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_FACE_COMMAND_NV: types::GLenum = 0x0012;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_LEFT: types::GLenum = 0x0400;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_RIGHT: types::GLenum = 0x0401;
#[allow(dead_code, non_upper_case_globals)]
pub const FUNC_ADD: types::GLenum = 0x8006;
#[allow(dead_code, non_upper_case_globals)]
pub const FUNC_REVERSE_SUBTRACT: types::GLenum = 0x800B;
#[allow(dead_code, non_upper_case_globals)]
pub const FUNC_SUBTRACT: types::GLenum = 0x800A;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_INPUT_TYPE: types::GLenum = 0x8917;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_OUTPUT_TYPE: types::GLenum = 0x8918;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_SHADER: types::GLenum = 0x8DD9;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_VERTICES_OUT: types::GLenum = 0x8916;
#[allow(dead_code, non_upper_case_globals)]
pub const GEQUAL: types::GLenum = 0x0206;
#[allow(dead_code, non_upper_case_globals)]
pub const GREATER: types::GLenum = 0x0204;
#[allow(dead_code, non_upper_case_globals)]
pub const GREEN: types::GLenum = 0x1904;
#[allow(dead_code, non_upper_case_globals)]
pub const GREEN_INTEGER: types::GLenum = 0x8D95;
#[allow(dead_code, non_upper_case_globals)]
pub const HALF_FLOAT: types::GLenum = 0x140B;
#[allow(dead_code, non_upper_case_globals)]
pub const INCR: types::GLenum = 0x1E02;
#[allow(dead_code, non_upper_case_globals)]
pub const INCR_WRAP: types::GLenum = 0x8507;
#[allow(dead_code, non_upper_case_globals)]
pub const INFO_LOG_LENGTH: types::GLenum = 0x8B84;
#[allow(dead_code, non_upper_case_globals)]
pub const INT: types::GLenum = 0x1404;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERLEAVED_ATTRIBS: types::GLenum = 0x8C8C;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_2_10_10_10_REV: types::GLenum = 0x8D9F;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_1D: types::GLenum = 0x8DC9;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_1D_ARRAY: types::GLenum = 0x8DCE;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D: types::GLenum = 0x8DCA;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D_ARRAY: types::GLenum = 0x8DCF;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x9109;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910C;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D_RECT: types::GLenum = 0x8DCD;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_3D: types::GLenum = 0x8DCB;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_BUFFER: types::GLenum = 0x8DD0;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_CUBE: types::GLenum = 0x8DCC;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_VEC2: types::GLenum = 0x8B53;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_VEC3: types::GLenum = 0x8B54;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_VEC4: types::GLenum = 0x8B55;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_ENUM: types::GLenum = 0x0500;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_FRAMEBUFFER_OPERATION: types::GLenum = 0x0506;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_INDEX: types::GLuint = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_OPERATION: types::GLenum = 0x0502;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_VALUE: types::GLenum = 0x0501;
#[allow(dead_code, non_upper_case_globals)]
pub const INVERT: types::GLenum = 0x150A;
#[allow(dead_code, non_upper_case_globals)]
pub const KEEP: types::GLenum = 0x1E00;
#[allow(dead_code, non_upper_case_globals)]
pub const LAST_VERTEX_CONVENTION: types::GLenum = 0x8E4E;
#[allow(dead_code, non_upper_case_globals)]
pub const LEFT: types::GLenum = 0x0406;
#[allow(dead_code, non_upper_case_globals)]
pub const LEQUAL: types::GLenum = 0x0203;
#[allow(dead_code, non_upper_case_globals)]
pub const LESS: types::GLenum = 0x0201;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE: types::GLenum = 0x1B01;
#[allow(dead_code, non_upper_case_globals)]
pub const LINEAR: types::GLenum = 0x2601;
#[allow(dead_code, non_upper_case_globals)]
pub const LINEAR_MIPMAP_LINEAR: types::GLenum = 0x2703;
#[allow(dead_code, non_upper_case_globals)]
pub const LINEAR_MIPMAP_NEAREST: types::GLenum = 0x2701;
#[allow(dead_code, non_upper_case_globals)]
pub const LINES: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)]
pub const LINES_ADJACENCY: types::GLenum = 0x000A;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_LOOP: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_SMOOTH: types::GLenum = 0x0B20;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_SMOOTH_HINT: types::GLenum = 0x0C52;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_STRIP: types::GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_STRIP_ADJACENCY: types::GLenum = 0x000B;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_WIDTH: types::GLenum = 0x0B21;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_WIDTH_COMMAND_NV: types::GLenum = 0x000D;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)]
pub const LINK_STATUS: types::GLenum = 0x8B82;
#[allow(dead_code, non_upper_case_globals)]
pub const LOGIC_OP_MODE: types::GLenum = 0x0BF0;
#[allow(dead_code, non_upper_case_globals)]
pub const LOWER_LEFT: types::GLenum = 0x8CA1;
#[allow(dead_code, non_upper_case_globals)]
pub const MAJOR_VERSION: types::GLenum = 0x821B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_FLUSH_EXPLICIT_BIT: types::GLenum = 0x0010;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_INVALIDATE_BUFFER_BIT: types::GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_INVALIDATE_RANGE_BIT: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_READ_BIT: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_UNSYNCHRONIZED_BIT: types::GLenum = 0x0020;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_WRITE_BIT: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX: types::GLenum = 0x8008;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_3D_TEXTURE_SIZE: types::GLenum = 0x8073;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ARRAY_TEXTURE_LAYERS: types::GLenum = 0x88FF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_CLIP_DISTANCES: types::GLenum = 0x0D32;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COLOR_ATTACHMENTS: types::GLenum = 0x8CDF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COLOR_TEXTURE_SAMPLES: types::GLenum = 0x910E;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: types::GLenum = 0x8A33;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: types::GLenum = 0x8A32;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_UNIFORM_BLOCKS: types::GLenum = 0x8A2E;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: types::GLenum = 0x8A31;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_CUBE_MAP_TEXTURE_SIZE: types::GLenum = 0x851C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DEPTH_TEXTURE_SAMPLES: types::GLenum = 0x910F;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DRAW_BUFFERS: types::GLenum = 0x8824;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: types::GLenum = 0x88FC;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ELEMENTS_INDICES: types::GLenum = 0x80E9;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ELEMENTS_VERTICES: types::GLenum = 0x80E8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_INPUT_COMPONENTS: types::GLenum = 0x9125;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_UNIFORM_BLOCKS: types::GLenum = 0x8A2D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: types::GLenum = 0x8B49;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_INPUT_COMPONENTS: types::GLenum = 0x9123;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: types::GLenum = 0x9124;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_OUTPUT_VERTICES: types::GLenum = 0x8DE0;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8C29;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: types::GLenum = 0x8DE1;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_UNIFORM_BLOCKS: types::GLenum = 0x8A2C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: types::GLenum = 0x8DDF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_INTEGER_SAMPLES: types::GLenum = 0x9110;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_PROGRAM_TEXEL_OFFSET: types::GLenum = 0x8905;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_RECTANGLE_TEXTURE_SIZE: types::GLenum = 0x84F8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_RENDERBUFFER_SIZE: types::GLenum = 0x84E8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SAMPLES: types::GLenum = 0x8D57;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SAMPLE_MASK_WORDS: types::GLenum = 0x8E59;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SERVER_WAIT_TIMEOUT: types::GLenum = 0x9111;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_BUFFER_SIZE: types::GLenum = 0x8C2B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8872;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_LOD_BIAS: types::GLenum = 0x84FD;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_SIZE: types::GLenum = 0x0D33;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: types::GLenum = 0x8C8A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: types::GLenum = 0x8C8B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: types::GLenum = 0x8C80;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_UNIFORM_BLOCK_SIZE: types::GLenum = 0x8A30;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_UNIFORM_BUFFER_BINDINGS: types::GLenum = 0x8A2F;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VARYING_COMPONENTS: types::GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VARYING_FLOATS: types::GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIBS: types::GLenum = 0x8869;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_OUTPUT_COMPONENTS: types::GLenum = 0x9122;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_UNIFORM_BLOCKS: types::GLenum = 0x8A2B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_UNIFORM_COMPONENTS: types::GLenum = 0x8B4A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VIEWPORT_DIMS: types::GLenum = 0x0D3A;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN: types::GLenum = 0x8007;
#[allow(dead_code, non_upper_case_globals)]
pub const MINOR_VERSION: types::GLenum = 0x821C;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN_PROGRAM_TEXEL_OFFSET: types::GLenum = 0x8904;
#[allow(dead_code, non_upper_case_globals)]
pub const MIRRORED_REPEAT: types::GLenum = 0x8370;
#[allow(dead_code, non_upper_case_globals)]
pub const MULTISAMPLE: types::GLenum = 0x809D;
#[allow(dead_code, non_upper_case_globals)]
pub const NAND: types::GLenum = 0x150E;
#[allow(dead_code, non_upper_case_globals)]
pub const NEAREST: types::GLenum = 0x2600;
#[allow(dead_code, non_upper_case_globals)]
pub const NEAREST_MIPMAP_LINEAR: types::GLenum = 0x2702;
#[allow(dead_code, non_upper_case_globals)]
pub const NEAREST_MIPMAP_NEAREST: types::GLenum = 0x2700;
#[allow(dead_code, non_upper_case_globals)]
pub const NEVER: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)]
pub const NICEST: types::GLenum = 0x1102;
#[allow(dead_code, non_upper_case_globals)]
pub const NONE: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const NOOP: types::GLenum = 0x1505;
#[allow(dead_code, non_upper_case_globals)]
pub const NOP_COMMAND_NV: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)]
pub const NOR: types::GLenum = 0x1508;
#[allow(dead_code, non_upper_case_globals)]
pub const NOTEQUAL: types::GLenum = 0x0205;
#[allow(dead_code, non_upper_case_globals)]
pub const NO_ERROR: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A2;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_EXTENSIONS: types::GLenum = 0x821D;
#[allow(dead_code, non_upper_case_globals)]
pub const OBJECT_TYPE: types::GLenum = 0x9112;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_CONSTANT_ALPHA: types::GLenum = 0x8004;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_CONSTANT_COLOR: types::GLenum = 0x8002;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_DST_ALPHA: types::GLenum = 0x0305;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_DST_COLOR: types::GLenum = 0x0307;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_SRC1_ALPHA: types::GLenum = 0x88FB;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_SRC1_COLOR: types::GLenum = 0x88FA;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_SRC_ALPHA: types::GLenum = 0x0303;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_SRC_COLOR: types::GLenum = 0x0301;
#[allow(dead_code, non_upper_case_globals)]
pub const OR: types::GLenum = 0x1507;
#[allow(dead_code, non_upper_case_globals)]
pub const OR_INVERTED: types::GLenum = 0x150D;
#[allow(dead_code, non_upper_case_globals)]
pub const OR_REVERSE: types::GLenum = 0x150B;
#[allow(dead_code, non_upper_case_globals)]
pub const OUT_OF_MEMORY: types::GLenum = 0x0505;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_ALIGNMENT: types::GLenum = 0x0D05;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_IMAGE_HEIGHT: types::GLenum = 0x806C;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_LSB_FIRST: types::GLenum = 0x0D01;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_ROW_LENGTH: types::GLenum = 0x0D02;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_SKIP_IMAGES: types::GLenum = 0x806B;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_SKIP_PIXELS: types::GLenum = 0x0D04;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_SKIP_ROWS: types::GLenum = 0x0D03;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_SWAP_BYTES: types::GLenum = 0x0D00;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_PACK_BUFFER: types::GLenum = 0x88EB;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_PACK_BUFFER_BINDING: types::GLenum = 0x88ED;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_UNPACK_BUFFER: types::GLenum = 0x88EC;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_UNPACK_BUFFER_BINDING: types::GLenum = 0x88EF;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT: types::GLenum = 0x1B00;
#[allow(dead_code, non_upper_case_globals)]
pub const POINTS: types::GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_FADE_THRESHOLD_SIZE: types::GLenum = 0x8128;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SIZE: types::GLenum = 0x0B11;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SPRITE_COORD_ORIGIN: types::GLenum = 0x8CA0;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_MODE: types::GLenum = 0x0B40;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_COMMAND_NV: types::GLenum = 0x000E;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_FILL: types::GLenum = 0x8037;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_LINE: types::GLenum = 0x2A02;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_POINT: types::GLenum = 0x2A01;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_SMOOTH: types::GLenum = 0x0B41;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_SMOOTH_HINT: types::GLenum = 0x0C53;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVES_GENERATED: types::GLenum = 0x8C87;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVE_RESTART: types::GLenum = 0x8F9D;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVE_RESTART_INDEX: types::GLenum = 0x8F9E;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_POINT_SIZE: types::GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)]
pub const PROVOKING_VERTEX: types::GLenum = 0x8E4F;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_1D: types::GLenum = 0x8063;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_1D_ARRAY: types::GLenum = 0x8C19;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_2D: types::GLenum = 0x8064;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_2D_ARRAY: types::GLenum = 0x8C1B;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_2D_MULTISAMPLE: types::GLenum = 0x9101;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9103;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_3D: types::GLenum = 0x8070;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_CUBE_MAP: types::GLenum = 0x851B;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_RECTANGLE: types::GLenum = 0x84F7;
#[allow(dead_code, non_upper_case_globals)]
pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: types::GLenum = 0x8E4C;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BY_REGION_NO_WAIT: types::GLenum = 0x8E16;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BY_REGION_WAIT: types::GLenum = 0x8E15;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_COUNTER_BITS: types::GLenum = 0x8864;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_NO_WAIT: types::GLenum = 0x8E14;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_RESULT: types::GLenum = 0x8866;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_RESULT_AVAILABLE: types::GLenum = 0x8867;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_WAIT: types::GLenum = 0x8E13;
#[allow(dead_code, non_upper_case_globals)]
pub const R11F_G11F_B10F: types::GLenum = 0x8C3A;
#[allow(dead_code, non_upper_case_globals)]
pub const R16: types::GLenum = 0x822A;
#[allow(dead_code, non_upper_case_globals)]
pub const R16F: types::GLenum = 0x822D;
#[allow(dead_code, non_upper_case_globals)]
pub const R16I: types::GLenum = 0x8233;
#[allow(dead_code, non_upper_case_globals)]
pub const R16UI: types::GLenum = 0x8234;
#[allow(dead_code, non_upper_case_globals)]
pub const R16_SNORM: types::GLenum = 0x8F98;
#[allow(dead_code, non_upper_case_globals)]
pub const R32F: types::GLenum = 0x822E;
#[allow(dead_code, non_upper_case_globals)]
pub const R32I: types::GLenum = 0x8235;
#[allow(dead_code, non_upper_case_globals)]
pub const R32UI: types::GLenum = 0x8236;
#[allow(dead_code, non_upper_case_globals)]
pub const R3_G3_B2: types::GLenum = 0x2A10;
#[allow(dead_code, non_upper_case_globals)]
pub const R8: types::GLenum = 0x8229;
#[allow(dead_code, non_upper_case_globals)]
pub const R8I: types::GLenum = 0x8231;
#[allow(dead_code, non_upper_case_globals)]
pub const R8UI: types::GLenum = 0x8232;
#[allow(dead_code, non_upper_case_globals)]
pub const R8_SNORM: types::GLenum = 0x8F94;
#[allow(dead_code, non_upper_case_globals)]
pub const RASTERIZER_DISCARD: types::GLenum = 0x8C89;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_BUFFER: types::GLenum = 0x0C02;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_FRAMEBUFFER: types::GLenum = 0x8CA8;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_FRAMEBUFFER_BINDING: types::GLenum = 0x8CAA;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_ONLY: types::GLenum = 0x88B8;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_WRITE: types::GLenum = 0x88BA;
#[allow(dead_code, non_upper_case_globals)]
pub const RED: types::GLenum = 0x1903;
#[allow(dead_code, non_upper_case_globals)]
pub const RED_INTEGER: types::GLenum = 0x8D94;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER: types::GLenum = 0x8D41;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_ALPHA_SIZE: types::GLenum = 0x8D53;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_BINDING: types::GLenum = 0x8CA7;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_BLUE_SIZE: types::GLenum = 0x8D52;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_DEPTH_SIZE: types::GLenum = 0x8D54;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_GREEN_SIZE: types::GLenum = 0x8D51;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_HEIGHT: types::GLenum = 0x8D43;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_INTERNAL_FORMAT: types::GLenum = 0x8D44;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_RED_SIZE: types::GLenum = 0x8D50;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_SAMPLES: types::GLenum = 0x8CAB;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_STENCIL_SIZE: types::GLenum = 0x8D55;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_WIDTH: types::GLenum = 0x8D42;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERER: types::GLenum = 0x1F01;
#[allow(dead_code, non_upper_case_globals)]
pub const REPEAT: types::GLenum = 0x2901;
#[allow(dead_code, non_upper_case_globals)]
pub const REPLACE: types::GLenum = 0x1E01;
#[allow(dead_code, non_upper_case_globals)]
pub const RG: types::GLenum = 0x8227;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16: types::GLenum = 0x822C;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16F: types::GLenum = 0x822F;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16I: types::GLenum = 0x8239;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16UI: types::GLenum = 0x823A;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16_SNORM: types::GLenum = 0x8F99;
#[allow(dead_code, non_upper_case_globals)]
pub const RG32F: types::GLenum = 0x8230;
#[allow(dead_code, non_upper_case_globals)]
pub const RG32I: types::GLenum = 0x823B;
#[allow(dead_code, non_upper_case_globals)]
pub const RG32UI: types::GLenum = 0x823C;
#[allow(dead_code, non_upper_case_globals)]
pub const RG8: types::GLenum = 0x822B;
#[allow(dead_code, non_upper_case_globals)]
pub const RG8I: types::GLenum = 0x8237;
#[allow(dead_code, non_upper_case_globals)]
pub const RG8UI: types::GLenum = 0x8238;
#[allow(dead_code, non_upper_case_globals)]
pub const RG8_SNORM: types::GLenum = 0x8F95;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB: types::GLenum = 0x1907;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB10: types::GLenum = 0x8052;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB10_A2: types::GLenum = 0x8059;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB10_A2UI: types::GLenum = 0x906F;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB12: types::GLenum = 0x8053;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16: types::GLenum = 0x8054;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16F: types::GLenum = 0x881B;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16I: types::GLenum = 0x8D89;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16UI: types::GLenum = 0x8D77;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16_SNORM: types::GLenum = 0x8F9A;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB32F: types::GLenum = 0x8815;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB32I: types::GLenum = 0x8D83;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB32UI: types::GLenum = 0x8D71;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB4: types::GLenum = 0x804F;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB5: types::GLenum = 0x8050;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB5_A1: types::GLenum = 0x8057;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB8: types::GLenum = 0x8051;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB8I: types::GLenum = 0x8D8F;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB8UI: types::GLenum = 0x8D7D;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB8_SNORM: types::GLenum = 0x8F96;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB9_E5: types::GLenum = 0x8C3D;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA: types::GLenum = 0x1908;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA12: types::GLenum = 0x805A;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16: types::GLenum = 0x805B;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16F: types::GLenum = 0x881A;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16I: types::GLenum = 0x8D88;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16UI: types::GLenum = 0x8D76;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16_SNORM: types::GLenum = 0x8F9B;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA2: types::GLenum = 0x8055;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA32F: types::GLenum = 0x8814;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA32I: types::GLenum = 0x8D82;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA32UI: types::GLenum = 0x8D70;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA4: types::GLenum = 0x8056;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA8: types::GLenum = 0x8058;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA8I: types::GLenum = 0x8D8E;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA8UI: types::GLenum = 0x8D7C;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA8_SNORM: types::GLenum = 0x8F97;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA_INTEGER: types::GLenum = 0x8D99;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB_INTEGER: types::GLenum = 0x8D98;
#[allow(dead_code, non_upper_case_globals)]
pub const RG_INTEGER: types::GLenum = 0x8228;
#[allow(dead_code, non_upper_case_globals)]
pub const RIGHT: types::GLenum = 0x0407;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_1D: types::GLenum = 0x8B5D;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_1D_ARRAY: types::GLenum = 0x8DC0;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_1D_ARRAY_SHADOW: types::GLenum = 0x8DC3;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_1D_SHADOW: types::GLenum = 0x8B61;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D: types::GLenum = 0x8B5E;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_ARRAY: types::GLenum = 0x8DC1;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_ARRAY_SHADOW: types::GLenum = 0x8DC4;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x9108;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910B;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_RECT: types::GLenum = 0x8B63;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_RECT_SHADOW: types::GLenum = 0x8B64;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_SHADOW: types::GLenum = 0x8B62;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_3D: types::GLenum = 0x8B5F;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_BINDING: types::GLenum = 0x8919;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_BUFFER: types::GLenum = 0x8DC2;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_CUBE: types::GLenum = 0x8B60;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_CUBE_SHADOW: types::GLenum = 0x8DC5;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLES: types::GLenum = 0x80A9;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLES_PASSED: types::GLenum = 0x8914;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_ALPHA_TO_COVERAGE: types::GLenum = 0x809E;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_ALPHA_TO_ONE: types::GLenum = 0x809F;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_BUFFERS: types::GLenum = 0x80A8;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_COVERAGE: types::GLenum = 0x80A0;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_COVERAGE_INVERT: types::GLenum = 0x80AB;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_COVERAGE_VALUE: types::GLenum = 0x80AA;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_MASK: types::GLenum = 0x8E51;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_MASK_VALUE: types::GLenum = 0x8E52;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_POSITION: types::GLenum = 0x8E50;
#[allow(dead_code, non_upper_case_globals)]
pub const SCISSOR_BOX: types::GLenum = 0x0C10;
#[allow(dead_code, non_upper_case_globals)]
pub const SCISSOR_COMMAND_NV: types::GLenum = 0x0011;
#[allow(dead_code, non_upper_case_globals)]
pub const SCISSOR_TEST: types::GLenum = 0x0C11;
#[allow(dead_code, non_upper_case_globals)]
pub const SEPARATE_ATTRIBS: types::GLenum = 0x8C8D;
#[allow(dead_code, non_upper_case_globals)]
pub const SET: types::GLenum = 0x150F;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_SOURCE_LENGTH: types::GLenum = 0x8B88;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_TYPE: types::GLenum = 0x8B4F;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADING_LANGUAGE_VERSION: types::GLenum = 0x8B8C;
#[allow(dead_code, non_upper_case_globals)]
pub const SHORT: types::GLenum = 0x1402;
#[allow(dead_code, non_upper_case_globals)]
pub const SIGNALED: types::GLenum = 0x9119;
#[allow(dead_code, non_upper_case_globals)]
pub const SIGNED_NORMALIZED: types::GLenum = 0x8F9C;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH_LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH_LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH_POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH_POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC1_ALPHA: types::GLenum = 0x8589;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC1_COLOR: types::GLenum = 0x88F9;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC_ALPHA: types::GLenum = 0x0302;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC_ALPHA_SATURATE: types::GLenum = 0x0308;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC_COLOR: types::GLenum = 0x0300;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB: types::GLenum = 0x8C40;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB8: types::GLenum = 0x8C41;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB8_ALPHA8: types::GLenum = 0x8C43;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB_ALPHA: types::GLenum = 0x8C42;
#[allow(dead_code, non_upper_case_globals)]
pub const STATIC_COPY: types::GLenum = 0x88E6;
#[allow(dead_code, non_upper_case_globals)]
pub const STATIC_DRAW: types::GLenum = 0x88E4;
#[allow(dead_code, non_upper_case_globals)]
pub const STATIC_READ: types::GLenum = 0x88E5;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL: types::GLenum = 0x1802;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_ATTACHMENT: types::GLenum = 0x8D20;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_FAIL: types::GLenum = 0x8801;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_FUNC: types::GLenum = 0x8800;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_PASS_DEPTH_FAIL: types::GLenum = 0x8802;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_PASS_DEPTH_PASS: types::GLenum = 0x8803;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_REF: types::GLenum = 0x8CA3;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_VALUE_MASK: types::GLenum = 0x8CA4;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_WRITEMASK: types::GLenum = 0x8CA5;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BUFFER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_CLEAR_VALUE: types::GLenum = 0x0B91;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_FAIL: types::GLenum = 0x0B94;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_FUNC: types::GLenum = 0x0B92;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX: types::GLenum = 0x1901;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX1: types::GLenum = 0x8D46;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX16: types::GLenum = 0x8D49;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX4: types::GLenum = 0x8D47;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX8: types::GLenum = 0x8D48;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_PASS_DEPTH_FAIL: types::GLenum = 0x0B95;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_PASS_DEPTH_PASS: types::GLenum = 0x0B96;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_REF: types::GLenum = 0x0B97;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_REF_COMMAND_NV: types::GLenum = 0x000C;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_TEST: types::GLenum = 0x0B90;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_VALUE_MASK: types::GLenum = 0x0B93;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_WRITEMASK: types::GLenum = 0x0B98;
#[allow(dead_code, non_upper_case_globals)]
pub const STEREO: types::GLenum = 0x0C33;
#[allow(dead_code, non_upper_case_globals)]
pub const STREAM_COPY: types::GLenum = 0x88E2;
#[allow(dead_code, non_upper_case_globals)]
pub const STREAM_DRAW: types::GLenum = 0x88E0;
#[allow(dead_code, non_upper_case_globals)]
pub const STREAM_READ: types::GLenum = 0x88E1;
#[allow(dead_code, non_upper_case_globals)]
pub const SUBPIXEL_BITS: types::GLenum = 0x0D50;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_CONDITION: types::GLenum = 0x9113;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_FENCE: types::GLenum = 0x9116;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_FLAGS: types::GLenum = 0x9115;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_FLUSH_COMMANDS_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_GPU_COMMANDS_COMPLETE: types::GLenum = 0x9117;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_STATUS: types::GLenum = 0x9114;
#[allow(dead_code, non_upper_case_globals)]
pub const TERMINATE_SEQUENCE_COMMAND_NV: types::GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE: types::GLenum = 0x1702;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE0: types::GLenum = 0x84C0;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE1: types::GLenum = 0x84C1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE10: types::GLenum = 0x84CA;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE11: types::GLenum = 0x84CB;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE12: types::GLenum = 0x84CC;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE13: types::GLenum = 0x84CD;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE14: types::GLenum = 0x84CE;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE15: types::GLenum = 0x84CF;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE16: types::GLenum = 0x84D0;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE17: types::GLenum = 0x84D1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE18: types::GLenum = 0x84D2;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE19: types::GLenum = 0x84D3;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE2: types::GLenum = 0x84C2;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE20: types::GLenum = 0x84D4;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE21: types::GLenum = 0x84D5;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE22: types::GLenum = 0x84D6;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE23: types::GLenum = 0x84D7;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE24: types::GLenum = 0x84D8;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE25: types::GLenum = 0x84D9;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE26: types::GLenum = 0x84DA;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE27: types::GLenum = 0x84DB;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE28: types::GLenum = 0x84DC;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE29: types::GLenum = 0x84DD;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE3: types::GLenum = 0x84C3;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE30: types::GLenum = 0x84DE;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE31: types::GLenum = 0x84DF;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE4: types::GLenum = 0x84C4;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE5: types::GLenum = 0x84C5;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE6: types::GLenum = 0x84C6;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE7: types::GLenum = 0x84C7;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE8: types::GLenum = 0x84C8;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE9: types::GLenum = 0x84C9;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_1D: types::GLenum = 0x0DE0;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_1D_ARRAY: types::GLenum = 0x8C18;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_2D: types::GLenum = 0x0DE1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_2D_ARRAY: types::GLenum = 0x8C1A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_2D_MULTISAMPLE: types::GLenum = 0x9100;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9102;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_3D: types::GLenum = 0x806F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_ALPHA_SIZE: types::GLenum = 0x805F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_ALPHA_TYPE: types::GLenum = 0x8C13;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BASE_LEVEL: types::GLenum = 0x813C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_1D: types::GLenum = 0x8068;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_1D_ARRAY: types::GLenum = 0x8C1C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_2D: types::GLenum = 0x8069;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_2D_ARRAY: types::GLenum = 0x8C1D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_2D_MULTISAMPLE: types::GLenum = 0x9104;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9105;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_3D: types::GLenum = 0x806A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_BUFFER: types::GLenum = 0x8C2C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_CUBE_MAP: types::GLenum = 0x8514;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_RECTANGLE: types::GLenum = 0x84F6;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BLUE_SIZE: types::GLenum = 0x805E;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BLUE_TYPE: types::GLenum = 0x8C12;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BORDER_COLOR: types::GLenum = 0x1004;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER: types::GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER_DATA_STORE_BINDING: types::GLenum = 0x8C2D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPARE_FUNC: types::GLenum = 0x884D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPARE_MODE: types::GLenum = 0x884C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSED: types::GLenum = 0x86A1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_IMAGE_SIZE: types::GLenum = 0x86A0;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSION_HINT: types::GLenum = 0x84EF;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP: types::GLenum = 0x8513;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: types::GLenum = 0x8516;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: types::GLenum = 0x8518;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: types::GLenum = 0x851A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_POSITIVE_X: types::GLenum = 0x8515;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: types::GLenum = 0x8517;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: types::GLenum = 0x8519;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_SEAMLESS: types::GLenum = 0x884F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_DEPTH: types::GLenum = 0x8071;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_DEPTH_SIZE: types::GLenum = 0x884A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_DEPTH_TYPE: types::GLenum = 0x8C16;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: types::GLenum = 0x9107;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GREEN_SIZE: types::GLenum = 0x805D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GREEN_TYPE: types::GLenum = 0x8C11;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_HEIGHT: types::GLenum = 0x1001;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_INTERNAL_FORMAT: types::GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_LOD_BIAS: types::GLenum = 0x8501;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MAG_FILTER: types::GLenum = 0x2800;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MAX_LEVEL: types::GLenum = 0x813D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MAX_LOD: types::GLenum = 0x813B;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MIN_FILTER: types::GLenum = 0x2801;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MIN_LOD: types::GLenum = 0x813A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_RECTANGLE: types::GLenum = 0x84F5;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_RED_SIZE: types::GLenum = 0x805C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_RED_TYPE: types::GLenum = 0x8C10;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SAMPLES: types::GLenum = 0x9106;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SHARED_SIZE: types::GLenum = 0x8C3F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_STENCIL_SIZE: types::GLenum = 0x88F1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_A: types::GLenum = 0x8E45;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_B: types::GLenum = 0x8E44;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_G: types::GLenum = 0x8E43;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_R: types::GLenum = 0x8E42;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_RGBA: types::GLenum = 0x8E46;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_WIDTH: types::GLenum = 0x1000;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_WRAP_R: types::GLenum = 0x8072;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_WRAP_S: types::GLenum = 0x2802;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_WRAP_T: types::GLenum = 0x2803;
#[allow(dead_code, non_upper_case_globals)]
pub const TIMEOUT_EXPIRED: types::GLenum = 0x911B;
#[allow(dead_code, non_upper_case_globals)]
pub const TIMEOUT_IGNORED: types::GLuint64 = 0xFFFFFFFFFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)]
pub const TIMESTAMP: types::GLenum = 0x8E28;
#[allow(dead_code, non_upper_case_globals)]
pub const TIME_ELAPSED: types::GLenum = 0x88BF;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER: types::GLenum = 0x8C8E;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: types::GLenum = 0x8C8F;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_MODE: types::GLenum = 0x8C7F;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: types::GLenum = 0x8C85;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_START: types::GLenum = 0x8C84;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: types::GLenum = 0x8C88;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_VARYINGS: types::GLenum = 0x8C83;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: types::GLenum = 0x8C76;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLES: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLES_ADJACENCY: types::GLenum = 0x000C;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLE_FAN: types::GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLE_STRIP: types::GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLE_STRIP_ADJACENCY: types::GLenum = 0x000D;
#[allow(dead_code, non_upper_case_globals)]
pub const TRUE: types::GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_ADDRESS_COMMAND_NV: types::GLenum = 0x000A;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_ARRAY_STRIDE: types::GLenum = 0x8A3C;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: types::GLenum = 0x8A42;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: types::GLenum = 0x8A43;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_BINDING: types::GLenum = 0x8A3F;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_DATA_SIZE: types::GLenum = 0x8A40;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_INDEX: types::GLenum = 0x8A3A;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_NAME_LENGTH: types::GLenum = 0x8A41;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x8A46;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: types::GLenum = 0x8A45;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x8A44;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER: types::GLenum = 0x8A11;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER_BINDING: types::GLenum = 0x8A28;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x8A34;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER_SIZE: types::GLenum = 0x8A2A;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER_START: types::GLenum = 0x8A29;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_IS_ROW_MAJOR: types::GLenum = 0x8A3E;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_MATRIX_STRIDE: types::GLenum = 0x8A3D;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_NAME_LENGTH: types::GLenum = 0x8A39;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_OFFSET: types::GLenum = 0x8A3B;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_SIZE: types::GLenum = 0x8A38;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_TYPE: types::GLenum = 0x8A37;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_ALIGNMENT: types::GLenum = 0x0CF5;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_IMAGE_HEIGHT: types::GLenum = 0x806E;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_LSB_FIRST: types::GLenum = 0x0CF1;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_ROW_LENGTH: types::GLenum = 0x0CF2;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_SKIP_IMAGES: types::GLenum = 0x806D;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_SKIP_PIXELS: types::GLenum = 0x0CF4;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_SKIP_ROWS: types::GLenum = 0x0CF3;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_SWAP_BYTES: types::GLenum = 0x0CF0;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNALED: types::GLenum = 0x9118;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_BYTE: types::GLenum = 0x1401;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_BYTE_2_3_3_REV: types::GLenum = 0x8362;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_BYTE_3_3_2: types::GLenum = 0x8032;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT: types::GLenum = 0x1405;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_10F_11F_11F_REV: types::GLenum = 0x8C3B;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_10_10_10_2: types::GLenum = 0x8036;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_24_8: types::GLenum = 0x84FA;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_2_10_10_10_REV: types::GLenum = 0x8368;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_5_9_9_9_REV: types::GLenum = 0x8C3E;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_8_8_8_8: types::GLenum = 0x8035;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_8_8_8_8_REV: types::GLenum = 0x8367;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_1D: types::GLenum = 0x8DD1;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: types::GLenum = 0x8DD6;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D: types::GLenum = 0x8DD2;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: types::GLenum = 0x8DD7;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x910A;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910D;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_RECT: types::GLenum = 0x8DD5;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_3D: types::GLenum = 0x8DD3;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_BUFFER: types::GLenum = 0x8DD8;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_CUBE: types::GLenum = 0x8DD4;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_VEC2: types::GLenum = 0x8DC6;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_VEC3: types::GLenum = 0x8DC7;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_VEC4: types::GLenum = 0x8DC8;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_NORMALIZED: types::GLenum = 0x8C17;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT: types::GLenum = 0x1403;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_1_5_5_5_REV: types::GLenum = 0x8366;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_4_4_4_4: types::GLenum = 0x8033;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_4_4_4_4_REV: types::GLenum = 0x8365;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_5_5_5_1: types::GLenum = 0x8034;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_5_6_5: types::GLenum = 0x8363;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_5_6_5_REV: types::GLenum = 0x8364;
#[allow(dead_code, non_upper_case_globals)]
pub const UPPER_LEFT: types::GLenum = 0x8CA2;
#[allow(dead_code, non_upper_case_globals)]
pub const VALIDATE_STATUS: types::GLenum = 0x8B83;
#[allow(dead_code, non_upper_case_globals)]
pub const VENDOR: types::GLenum = 0x1F00;
#[allow(dead_code, non_upper_case_globals)]
pub const VERSION: types::GLenum = 0x1F02;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ARRAY_BINDING: types::GLenum = 0x85B5;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: types::GLenum = 0x889F;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_DIVISOR: types::GLenum = 0x88FE;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_ENABLED: types::GLenum = 0x8622;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_INTEGER: types::GLenum = 0x88FD;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: types::GLenum = 0x886A;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_POINTER: types::GLenum = 0x8645;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_SIZE: types::GLenum = 0x8623;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_STRIDE: types::GLenum = 0x8624;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_TYPE: types::GLenum = 0x8625;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_PROGRAM_POINT_SIZE: types::GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_SHADER: types::GLenum = 0x8B31;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEWPORT: types::GLenum = 0x0BA2;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEWPORT_COMMAND_NV: types::GLenum = 0x0010;
#[allow(dead_code, non_upper_case_globals)]
pub const WAIT_FAILED: types::GLenum = 0x911D;
#[allow(dead_code, non_upper_case_globals)]
pub const WRITE_ONLY: types::GLenum = 0x88B9;
#[allow(dead_code, non_upper_case_globals)]
pub const XOR: types::GLenum = 0x1506;
#[allow(dead_code, non_upper_case_globals)]
pub const ZERO: types::GLenum = 0;

#[allow(dead_code, missing_copy_implementations)]
#[derive(Clone)]
pub struct FnPtr {
    /// The function pointer that will be used when calling the function.
    f: *const __gl_imports::raw::c_void,
    /// True if the pointer points to a real function, false if points to a `panic!` fn.
    is_loaded: bool,
}

impl FnPtr {
    /// Creates a `FnPtr` from a load attempt.
    fn new(ptr: *const __gl_imports::raw::c_void) -> FnPtr {
        if ptr.is_null() {
            FnPtr {
                f: missing_fn_panic as *const __gl_imports::raw::c_void,
                is_loaded: false,
            }
        } else {
            FnPtr {
                f: ptr,
                is_loaded: true,
            }
        }
    }

    /// Returns `true` if the function has been successfully loaded.
    ///
    /// If it returns `false`, calling the corresponding function will fail.
    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }
}

#[inline(never)]
fn missing_fn_panic() -> ! {
    panic!("gl function was not loaded")
}

#[allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    clippy::manual_non_exhaustive
)]
#[derive(Clone)]
pub struct Gl {
    /// Fallbacks: ActiveTextureARB
    pub ActiveTexture: FnPtr,
    /// Fallbacks: AttachObjectARB
    pub AttachShader: FnPtr,
    /// Fallbacks: BeginConditionalRenderNV
    pub BeginConditionalRender: FnPtr,
    /// Fallbacks: BeginQueryARB
    pub BeginQuery: FnPtr,
    /// Fallbacks: BeginTransformFeedbackEXT, BeginTransformFeedbackNV
    pub BeginTransformFeedback: FnPtr,
    /// Fallbacks: BindAttribLocationARB
    pub BindAttribLocation: FnPtr,
    /// Fallbacks: BindBufferARB
    pub BindBuffer: FnPtr,
    /// Fallbacks: BindBufferBaseEXT, BindBufferBaseNV
    pub BindBufferBase: FnPtr,
    /// Fallbacks: BindBufferRangeEXT, BindBufferRangeNV
    pub BindBufferRange: FnPtr,
    /// Fallbacks: BindFragDataLocationEXT
    pub BindFragDataLocation: FnPtr,
    /// Fallbacks: BindFragDataLocationIndexedEXT
    pub BindFragDataLocationIndexed: FnPtr,
    pub BindFramebuffer: FnPtr,
    pub BindRenderbuffer: FnPtr,
    pub BindSampler: FnPtr,
    /// Fallbacks: BindTextureEXT
    pub BindTexture: FnPtr,
    /// Fallbacks: BindVertexArrayOES
    pub BindVertexArray: FnPtr,
    /// Fallbacks: BlendColorEXT
    pub BlendColor: FnPtr,
    /// Fallbacks: BlendEquationEXT
    pub BlendEquation: FnPtr,
    /// Fallbacks: BlendEquationSeparateEXT
    pub BlendEquationSeparate: FnPtr,
    pub BlendFunc: FnPtr,
    /// Fallbacks: BlendFuncSeparateEXT, BlendFuncSeparateINGR
    pub BlendFuncSeparate: FnPtr,
    /// Fallbacks: BlitFramebufferEXT, BlitFramebufferNV
    pub BlitFramebuffer: FnPtr,
    /// Fallbacks: BufferDataARB
    pub BufferData: FnPtr,
    /// Fallbacks: BufferSubDataARB
    pub BufferSubData: FnPtr,
    pub CallCommandListNV: FnPtr,
    /// Fallbacks: CheckFramebufferStatusEXT
    pub CheckFramebufferStatus: FnPtr,
    /// Fallbacks: ClampColorARB
    pub ClampColor: FnPtr,
    pub Clear: FnPtr,
    pub ClearBufferfi: FnPtr,
    pub ClearBufferfv: FnPtr,
    pub ClearBufferiv: FnPtr,
    pub ClearBufferuiv: FnPtr,
    pub ClearColor: FnPtr,
    pub ClearDepth: FnPtr,
    pub ClearStencil: FnPtr,
    /// Fallbacks: ClientWaitSyncAPPLE
    pub ClientWaitSync: FnPtr,
    pub ColorMask: FnPtr,
    /// Fallbacks: ColorMaskIndexedEXT, ColorMaskiEXT, ColorMaskiOES
    pub ColorMaski: FnPtr,
    pub ColorP3ui: FnPtr,
    pub ColorP3uiv: FnPtr,
    pub ColorP4ui: FnPtr,
    pub ColorP4uiv: FnPtr,
    pub CommandListSegmentsNV: FnPtr,
    pub CompileCommandListNV: FnPtr,
    /// Fallbacks: CompileShaderARB
    pub CompileShader: FnPtr,
    /// Fallbacks: CompressedTexImage1DARB
    pub CompressedTexImage1D: FnPtr,
    /// Fallbacks: CompressedTexImage2DARB
    pub CompressedTexImage2D: FnPtr,
    /// Fallbacks: CompressedTexImage3DARB
    pub CompressedTexImage3D: FnPtr,
    /// Fallbacks: CompressedTexSubImage1DARB
    pub CompressedTexSubImage1D: FnPtr,
    /// Fallbacks: CompressedTexSubImage2DARB
    pub CompressedTexSubImage2D: FnPtr,
    /// Fallbacks: CompressedTexSubImage3DARB
    pub CompressedTexSubImage3D: FnPtr,
    /// Fallbacks: CopyBufferSubDataNV
    pub CopyBufferSubData: FnPtr,
    /// Fallbacks: CopyTexImage1DEXT
    pub CopyTexImage1D: FnPtr,
    /// Fallbacks: CopyTexImage2DEXT
    pub CopyTexImage2D: FnPtr,
    /// Fallbacks: CopyTexSubImage1DEXT
    pub CopyTexSubImage1D: FnPtr,
    /// Fallbacks: CopyTexSubImage2DEXT
    pub CopyTexSubImage2D: FnPtr,
    /// Fallbacks: CopyTexSubImage3DEXT
    pub CopyTexSubImage3D: FnPtr,
    pub CreateCommandListsNV: FnPtr,
    /// Fallbacks: CreateProgramObjectARB
    pub CreateProgram: FnPtr,
    /// Fallbacks: CreateShaderObjectARB
    pub CreateShader: FnPtr,
    pub CreateStatesNV: FnPtr,
    pub CullFace: FnPtr,
    /// Fallbacks: DeleteBuffersARB
    pub DeleteBuffers: FnPtr,
    pub DeleteCommandListsNV: FnPtr,
    /// Fallbacks: DeleteFramebuffersEXT
    pub DeleteFramebuffers: FnPtr,
    pub DeleteProgram: FnPtr,
    /// Fallbacks: DeleteQueriesARB
    pub DeleteQueries: FnPtr,
    /// Fallbacks: DeleteRenderbuffersEXT
    pub DeleteRenderbuffers: FnPtr,
    pub DeleteSamplers: FnPtr,
    pub DeleteShader: FnPtr,
    pub DeleteStatesNV: FnPtr,
    /// Fallbacks: DeleteSyncAPPLE
    pub DeleteSync: FnPtr,
    pub DeleteTextures: FnPtr,
    /// Fallbacks: DeleteVertexArraysAPPLE, DeleteVertexArraysOES
    pub DeleteVertexArrays: FnPtr,
    pub DepthFunc: FnPtr,
    pub DepthMask: FnPtr,
    pub DepthRange: FnPtr,
    /// Fallbacks: DetachObjectARB
    pub DetachShader: FnPtr,
    pub Disable: FnPtr,
    /// Fallbacks: DisableVertexAttribArrayARB
    pub DisableVertexAttribArray: FnPtr,
    /// Fallbacks: DisableIndexedEXT, DisableiEXT, DisableiNV, DisableiOES
    pub Disablei: FnPtr,
    /// Fallbacks: DrawArraysEXT
    pub DrawArrays: FnPtr,
    /// Fallbacks: DrawArraysInstancedANGLE, DrawArraysInstancedARB, DrawArraysInstancedEXT, DrawArraysInstancedNV
    pub DrawArraysInstanced: FnPtr,
    pub DrawBuffer: FnPtr,
    /// Fallbacks: DrawBuffersARB, DrawBuffersATI, DrawBuffersEXT
    pub DrawBuffers: FnPtr,
    pub DrawCommandsAddressNV: FnPtr,
    pub DrawCommandsNV: FnPtr,
    pub DrawCommandsStatesAddressNV: FnPtr,
    pub DrawCommandsStatesNV: FnPtr,
    pub DrawElements: FnPtr,
    /// Fallbacks: DrawElementsBaseVertexEXT, DrawElementsBaseVertexOES
    pub DrawElementsBaseVertex: FnPtr,
    /// Fallbacks: DrawElementsInstancedANGLE, DrawElementsInstancedARB, DrawElementsInstancedEXT, DrawElementsInstancedNV
    pub DrawElementsInstanced: FnPtr,
    /// Fallbacks: DrawElementsInstancedBaseVertexEXT, DrawElementsInstancedBaseVertexOES
    pub DrawElementsInstancedBaseVertex: FnPtr,
    /// Fallbacks: DrawRangeElementsEXT
    pub DrawRangeElements: FnPtr,
    /// Fallbacks: DrawRangeElementsBaseVertexEXT, DrawRangeElementsBaseVertexOES
    pub DrawRangeElementsBaseVertex: FnPtr,
    pub Enable: FnPtr,
    /// Fallbacks: EnableVertexAttribArrayARB
    pub EnableVertexAttribArray: FnPtr,
    /// Fallbacks: EnableIndexedEXT, EnableiEXT, EnableiNV, EnableiOES
    pub Enablei: FnPtr,
    /// Fallbacks: EndConditionalRenderNV, EndConditionalRenderNVX
    pub EndConditionalRender: FnPtr,
    /// Fallbacks: EndQueryARB
    pub EndQuery: FnPtr,
    /// Fallbacks: EndTransformFeedbackEXT, EndTransformFeedbackNV
    pub EndTransformFeedback: FnPtr,
    /// Fallbacks: FenceSyncAPPLE
    pub FenceSync: FnPtr,
    pub Finish: FnPtr,
    pub Flush: FnPtr,
    /// Fallbacks: FlushMappedBufferRangeAPPLE, FlushMappedBufferRangeEXT
    pub FlushMappedBufferRange: FnPtr,
    /// Fallbacks: FramebufferRenderbufferEXT
    pub FramebufferRenderbuffer: FnPtr,
    /// Fallbacks: FramebufferTextureARB, FramebufferTextureEXT, FramebufferTextureOES
    pub FramebufferTexture: FnPtr,
    /// Fallbacks: FramebufferTexture1DEXT
    pub FramebufferTexture1D: FnPtr,
    /// Fallbacks: FramebufferTexture2DEXT
    pub FramebufferTexture2D: FnPtr,
    /// Fallbacks: FramebufferTexture3DEXT
    pub FramebufferTexture3D: FnPtr,
    /// Fallbacks: FramebufferTextureLayerARB, FramebufferTextureLayerEXT
    pub FramebufferTextureLayer: FnPtr,
    pub FrontFace: FnPtr,
    /// Fallbacks: GenBuffersARB
    pub GenBuffers: FnPtr,
    /// Fallbacks: GenFramebuffersEXT
    pub GenFramebuffers: FnPtr,
    /// Fallbacks: GenQueriesARB
    pub GenQueries: FnPtr,
    /// Fallbacks: GenRenderbuffersEXT
    pub GenRenderbuffers: FnPtr,
    pub GenSamplers: FnPtr,
    pub GenTextures: FnPtr,
    /// Fallbacks: GenVertexArraysAPPLE, GenVertexArraysOES
    pub GenVertexArrays: FnPtr,
    /// Fallbacks: GenerateMipmapEXT
    pub GenerateMipmap: FnPtr,
    /// Fallbacks: GetActiveAttribARB
    pub GetActiveAttrib: FnPtr,
    /// Fallbacks: GetActiveUniformARB
    pub GetActiveUniform: FnPtr,
    pub GetActiveUniformBlockName: FnPtr,
    pub GetActiveUniformBlockiv: FnPtr,
    pub GetActiveUniformName: FnPtr,
    pub GetActiveUniformsiv: FnPtr,
    pub GetAttachedShaders: FnPtr,
    /// Fallbacks: GetAttribLocationARB
    pub GetAttribLocation: FnPtr,
    /// Fallbacks: GetBooleanIndexedvEXT
    pub GetBooleani_v: FnPtr,
    pub GetBooleanv: FnPtr,
    pub GetBufferParameteri64v: FnPtr,
    /// Fallbacks: GetBufferParameterivARB
    pub GetBufferParameteriv: FnPtr,
    /// Fallbacks: GetBufferPointervARB, GetBufferPointervOES
    pub GetBufferPointerv: FnPtr,
    /// Fallbacks: GetBufferSubDataARB
    pub GetBufferSubData: FnPtr,
    pub GetCommandHeaderNV: FnPtr,
    /// Fallbacks: GetCompressedTexImageARB
    pub GetCompressedTexImage: FnPtr,
    pub GetDoublev: FnPtr,
    pub GetError: FnPtr,
    pub GetFloatv: FnPtr,
    /// Fallbacks: GetFragDataIndexEXT
    pub GetFragDataIndex: FnPtr,
    /// Fallbacks: GetFragDataLocationEXT
    pub GetFragDataLocation: FnPtr,
    /// Fallbacks: GetFramebufferAttachmentParameterivEXT
    pub GetFramebufferAttachmentParameteriv: FnPtr,
    pub GetInteger64i_v: FnPtr,
    /// Fallbacks: GetInteger64vAPPLE
    pub GetInteger64v: FnPtr,
    /// Fallbacks: GetIntegerIndexedvEXT
    pub GetIntegeri_v: FnPtr,
    pub GetIntegerv: FnPtr,
    /// Fallbacks: GetMultisamplefvNV
    pub GetMultisamplefv: FnPtr,
    pub GetProgramInfoLog: FnPtr,
    pub GetProgramiv: FnPtr,
    /// Fallbacks: GetQueryObjecti64vEXT
    pub GetQueryObjecti64v: FnPtr,
    /// Fallbacks: GetQueryObjectivARB, GetQueryObjectivEXT
    pub GetQueryObjectiv: FnPtr,
    /// Fallbacks: GetQueryObjectui64vEXT
    pub GetQueryObjectui64v: FnPtr,
    /// Fallbacks: GetQueryObjectuivARB
    pub GetQueryObjectuiv: FnPtr,
    /// Fallbacks: GetQueryivARB
    pub GetQueryiv: FnPtr,
    /// Fallbacks: GetRenderbufferParameterivEXT
    pub GetRenderbufferParameteriv: FnPtr,
    /// Fallbacks: GetSamplerParameterIivEXT, GetSamplerParameterIivOES
    pub GetSamplerParameterIiv: FnPtr,
    /// Fallbacks: GetSamplerParameterIuivEXT, GetSamplerParameterIuivOES
    pub GetSamplerParameterIuiv: FnPtr,
    pub GetSamplerParameterfv: FnPtr,
    pub GetSamplerParameteriv: FnPtr,
    pub GetShaderInfoLog: FnPtr,
    /// Fallbacks: GetShaderSourceARB
    pub GetShaderSource: FnPtr,
    pub GetShaderiv: FnPtr,
    pub GetStageIndexNV: FnPtr,
    pub GetString: FnPtr,
    pub GetStringi: FnPtr,
    /// Fallbacks: GetSyncivAPPLE
    pub GetSynciv: FnPtr,
    pub GetTexImage: FnPtr,
    pub GetTexLevelParameterfv: FnPtr,
    pub GetTexLevelParameteriv: FnPtr,
    /// Fallbacks: GetTexParameterIivEXT, GetTexParameterIivOES
    pub GetTexParameterIiv: FnPtr,
    /// Fallbacks: GetTexParameterIuivEXT, GetTexParameterIuivOES
    pub GetTexParameterIuiv: FnPtr,
    pub GetTexParameterfv: FnPtr,
    pub GetTexParameteriv: FnPtr,
    /// Fallbacks: GetTransformFeedbackVaryingEXT
    pub GetTransformFeedbackVarying: FnPtr,
    pub GetUniformBlockIndex: FnPtr,
    pub GetUniformIndices: FnPtr,
    /// Fallbacks: GetUniformLocationARB
    pub GetUniformLocation: FnPtr,
    /// Fallbacks: GetUniformfvARB
    pub GetUniformfv: FnPtr,
    /// Fallbacks: GetUniformivARB
    pub GetUniformiv: FnPtr,
    /// Fallbacks: GetUniformuivEXT
    pub GetUniformuiv: FnPtr,
    /// Fallbacks: GetVertexAttribIivEXT
    pub GetVertexAttribIiv: FnPtr,
    /// Fallbacks: GetVertexAttribIuivEXT
    pub GetVertexAttribIuiv: FnPtr,
    /// Fallbacks: GetVertexAttribPointervARB, GetVertexAttribPointervNV
    pub GetVertexAttribPointerv: FnPtr,
    /// Fallbacks: GetVertexAttribdvARB, GetVertexAttribdvNV
    pub GetVertexAttribdv: FnPtr,
    /// Fallbacks: GetVertexAttribfvARB, GetVertexAttribfvNV
    pub GetVertexAttribfv: FnPtr,
    /// Fallbacks: GetVertexAttribivARB, GetVertexAttribivNV
    pub GetVertexAttribiv: FnPtr,
    pub Hint: FnPtr,
    /// Fallbacks: IsBufferARB
    pub IsBuffer: FnPtr,
    pub IsCommandListNV: FnPtr,
    pub IsEnabled: FnPtr,
    /// Fallbacks: IsEnabledIndexedEXT, IsEnablediEXT, IsEnablediNV, IsEnablediOES
    pub IsEnabledi: FnPtr,
    /// Fallbacks: IsFramebufferEXT
    pub IsFramebuffer: FnPtr,
    pub IsProgram: FnPtr,
    /// Fallbacks: IsQueryARB
    pub IsQuery: FnPtr,
    /// Fallbacks: IsRenderbufferEXT
    pub IsRenderbuffer: FnPtr,
    pub IsSampler: FnPtr,
    pub IsShader: FnPtr,
    pub IsStateNV: FnPtr,
    /// Fallbacks: IsSyncAPPLE
    pub IsSync: FnPtr,
    pub IsTexture: FnPtr,
    /// Fallbacks: IsVertexArrayAPPLE, IsVertexArrayOES
    pub IsVertexArray: FnPtr,
    pub LineWidth: FnPtr,
    /// Fallbacks: LinkProgramARB
    pub LinkProgram: FnPtr,
    pub ListDrawCommandsStatesClientNV: FnPtr,
    pub LogicOp: FnPtr,
    /// Fallbacks: MapBufferARB, MapBufferOES
    pub MapBuffer: FnPtr,
    /// Fallbacks: MapBufferRangeEXT
    pub MapBufferRange: FnPtr,
    /// Fallbacks: MultiDrawArraysEXT
    pub MultiDrawArrays: FnPtr,
    /// Fallbacks: MultiDrawElementsEXT
    pub MultiDrawElements: FnPtr,
    /// Fallbacks: MultiDrawElementsBaseVertexEXT
    pub MultiDrawElementsBaseVertex: FnPtr,
    pub MultiTexCoordP1ui: FnPtr,
    pub MultiTexCoordP1uiv: FnPtr,
    pub MultiTexCoordP2ui: FnPtr,
    pub MultiTexCoordP2uiv: FnPtr,
    pub MultiTexCoordP3ui: FnPtr,
    pub MultiTexCoordP3uiv: FnPtr,
    pub MultiTexCoordP4ui: FnPtr,
    pub MultiTexCoordP4uiv: FnPtr,
    pub NormalP3ui: FnPtr,
    pub NormalP3uiv: FnPtr,
    pub PixelStoref: FnPtr,
    pub PixelStorei: FnPtr,
    /// Fallbacks: PointParameterfARB, PointParameterfEXT, PointParameterfSGIS
    pub PointParameterf: FnPtr,
    /// Fallbacks: PointParameterfvARB, PointParameterfvEXT, PointParameterfvSGIS
    pub PointParameterfv: FnPtr,
    /// Fallbacks: PointParameteriNV
    pub PointParameteri: FnPtr,
    /// Fallbacks: PointParameterivNV
    pub PointParameteriv: FnPtr,
    pub PointSize: FnPtr,
    /// Fallbacks: PolygonModeNV
    pub PolygonMode: FnPtr,
    pub PolygonOffset: FnPtr,
    pub PrimitiveRestartIndex: FnPtr,
    /// Fallbacks: ProvokingVertexEXT
    pub ProvokingVertex: FnPtr,
    /// Fallbacks: QueryCounterEXT
    pub QueryCounter: FnPtr,
    pub ReadBuffer: FnPtr,
    pub ReadPixels: FnPtr,
    /// Fallbacks: RenderbufferStorageEXT
    pub RenderbufferStorage: FnPtr,
    /// Fallbacks: RenderbufferStorageMultisampleEXT, RenderbufferStorageMultisampleNV
    pub RenderbufferStorageMultisample: FnPtr,
    /// Fallbacks: SampleCoverageARB
    pub SampleCoverage: FnPtr,
    pub SampleMaski: FnPtr,
    /// Fallbacks: SamplerParameterIivEXT, SamplerParameterIivOES
    pub SamplerParameterIiv: FnPtr,
    /// Fallbacks: SamplerParameterIuivEXT, SamplerParameterIuivOES
    pub SamplerParameterIuiv: FnPtr,
    pub SamplerParameterf: FnPtr,
    pub SamplerParameterfv: FnPtr,
    pub SamplerParameteri: FnPtr,
    pub SamplerParameteriv: FnPtr,
    pub Scissor: FnPtr,
    pub SecondaryColorP3ui: FnPtr,
    pub SecondaryColorP3uiv: FnPtr,
    /// Fallbacks: ShaderSourceARB
    pub ShaderSource: FnPtr,
    pub StateCaptureNV: FnPtr,
    pub StencilFunc: FnPtr,
    pub StencilFuncSeparate: FnPtr,
    pub StencilMask: FnPtr,
    pub StencilMaskSeparate: FnPtr,
    pub StencilOp: FnPtr,
    /// Fallbacks: StencilOpSeparateATI
    pub StencilOpSeparate: FnPtr,
    /// Fallbacks: TexBufferARB, TexBufferEXT, TexBufferOES
    pub TexBuffer: FnPtr,
    pub TexCoordP1ui: FnPtr,
    pub TexCoordP1uiv: FnPtr,
    pub TexCoordP2ui: FnPtr,
    pub TexCoordP2uiv: FnPtr,
    pub TexCoordP3ui: FnPtr,
    pub TexCoordP3uiv: FnPtr,
    pub TexCoordP4ui: FnPtr,
    pub TexCoordP4uiv: FnPtr,
    pub TexImage1D: FnPtr,
    pub TexImage2D: FnPtr,
    pub TexImage2DMultisample: FnPtr,
    /// Fallbacks: TexImage3DEXT
    pub TexImage3D: FnPtr,
    pub TexImage3DMultisample: FnPtr,
    /// Fallbacks: TexParameterIivEXT, TexParameterIivOES
    pub TexParameterIiv: FnPtr,
    /// Fallbacks: TexParameterIuivEXT, TexParameterIuivOES
    pub TexParameterIuiv: FnPtr,
    pub TexParameterf: FnPtr,
    pub TexParameterfv: FnPtr,
    pub TexParameteri: FnPtr,
    pub TexParameteriv: FnPtr,
    /// Fallbacks: TexSubImage1DEXT
    pub TexSubImage1D: FnPtr,
    /// Fallbacks: TexSubImage2DEXT
    pub TexSubImage2D: FnPtr,
    /// Fallbacks: TexSubImage3DEXT
    pub TexSubImage3D: FnPtr,
    /// Fallbacks: TransformFeedbackVaryingsEXT
    pub TransformFeedbackVaryings: FnPtr,
    /// Fallbacks: Uniform1fARB
    pub Uniform1f: FnPtr,
    /// Fallbacks: Uniform1fvARB
    pub Uniform1fv: FnPtr,
    /// Fallbacks: Uniform1iARB
    pub Uniform1i: FnPtr,
    /// Fallbacks: Uniform1ivARB
    pub Uniform1iv: FnPtr,
    /// Fallbacks: Uniform1uiEXT
    pub Uniform1ui: FnPtr,
    /// Fallbacks: Uniform1uivEXT
    pub Uniform1uiv: FnPtr,
    /// Fallbacks: Uniform2fARB
    pub Uniform2f: FnPtr,
    /// Fallbacks: Uniform2fvARB
    pub Uniform2fv: FnPtr,
    /// Fallbacks: Uniform2iARB
    pub Uniform2i: FnPtr,
    /// Fallbacks: Uniform2ivARB
    pub Uniform2iv: FnPtr,
    /// Fallbacks: Uniform2uiEXT
    pub Uniform2ui: FnPtr,
    /// Fallbacks: Uniform2uivEXT
    pub Uniform2uiv: FnPtr,
    /// Fallbacks: Uniform3fARB
    pub Uniform3f: FnPtr,
    /// Fallbacks: Uniform3fvARB
    pub Uniform3fv: FnPtr,
    /// Fallbacks: Uniform3iARB
    pub Uniform3i: FnPtr,
    /// Fallbacks: Uniform3ivARB
    pub Uniform3iv: FnPtr,
    /// Fallbacks: Uniform3uiEXT
    pub Uniform3ui: FnPtr,
    /// Fallbacks: Uniform3uivEXT
    pub Uniform3uiv: FnPtr,
    /// Fallbacks: Uniform4fARB
    pub Uniform4f: FnPtr,
    /// Fallbacks: Uniform4fvARB
    pub Uniform4fv: FnPtr,
    /// Fallbacks: Uniform4iARB
    pub Uniform4i: FnPtr,
    /// Fallbacks: Uniform4ivARB
    pub Uniform4iv: FnPtr,
    /// Fallbacks: Uniform4uiEXT
    pub Uniform4ui: FnPtr,
    /// Fallbacks: Uniform4uivEXT
    pub Uniform4uiv: FnPtr,
    pub UniformBlockBinding: FnPtr,
    /// Fallbacks: UniformMatrix2fvARB
    pub UniformMatrix2fv: FnPtr,
    /// Fallbacks: UniformMatrix2x3fvNV
    pub UniformMatrix2x3fv: FnPtr,
    /// Fallbacks: UniformMatrix2x4fvNV
    pub UniformMatrix2x4fv: FnPtr,
    /// Fallbacks: UniformMatrix3fvARB
    pub UniformMatrix3fv: FnPtr,
    /// Fallbacks: UniformMatrix3x2fvNV
    pub UniformMatrix3x2fv: FnPtr,
    /// Fallbacks: UniformMatrix3x4fvNV
    pub UniformMatrix3x4fv: FnPtr,
    /// Fallbacks: UniformMatrix4fvARB
    pub UniformMatrix4fv: FnPtr,
    /// Fallbacks: UniformMatrix4x2fvNV
    pub UniformMatrix4x2fv: FnPtr,
    /// Fallbacks: UniformMatrix4x3fvNV
    pub UniformMatrix4x3fv: FnPtr,
    /// Fallbacks: UnmapBufferARB, UnmapBufferOES
    pub UnmapBuffer: FnPtr,
    /// Fallbacks: UseProgramObjectARB
    pub UseProgram: FnPtr,
    /// Fallbacks: ValidateProgramARB
    pub ValidateProgram: FnPtr,
    /// Fallbacks: VertexAttrib1dARB, VertexAttrib1dNV
    pub VertexAttrib1d: FnPtr,
    /// Fallbacks: VertexAttrib1dvARB, VertexAttrib1dvNV
    pub VertexAttrib1dv: FnPtr,
    /// Fallbacks: VertexAttrib1fARB, VertexAttrib1fNV
    pub VertexAttrib1f: FnPtr,
    /// Fallbacks: VertexAttrib1fvARB, VertexAttrib1fvNV
    pub VertexAttrib1fv: FnPtr,
    /// Fallbacks: VertexAttrib1sARB, VertexAttrib1sNV
    pub VertexAttrib1s: FnPtr,
    /// Fallbacks: VertexAttrib1svARB, VertexAttrib1svNV
    pub VertexAttrib1sv: FnPtr,
    /// Fallbacks: VertexAttrib2dARB, VertexAttrib2dNV
    pub VertexAttrib2d: FnPtr,
    /// Fallbacks: VertexAttrib2dvARB, VertexAttrib2dvNV
    pub VertexAttrib2dv: FnPtr,
    /// Fallbacks: VertexAttrib2fARB, VertexAttrib2fNV
    pub VertexAttrib2f: FnPtr,
    /// Fallbacks: VertexAttrib2fvARB, VertexAttrib2fvNV
    pub VertexAttrib2fv: FnPtr,
    /// Fallbacks: VertexAttrib2sARB, VertexAttrib2sNV
    pub VertexAttrib2s: FnPtr,
    /// Fallbacks: VertexAttrib2svARB, VertexAttrib2svNV
    pub VertexAttrib2sv: FnPtr,
    /// Fallbacks: VertexAttrib3dARB, VertexAttrib3dNV
    pub VertexAttrib3d: FnPtr,
    /// Fallbacks: VertexAttrib3dvARB, VertexAttrib3dvNV
    pub VertexAttrib3dv: FnPtr,
    /// Fallbacks: VertexAttrib3fARB, VertexAttrib3fNV
    pub VertexAttrib3f: FnPtr,
    /// Fallbacks: VertexAttrib3fvARB, VertexAttrib3fvNV
    pub VertexAttrib3fv: FnPtr,
    /// Fallbacks: VertexAttrib3sARB, VertexAttrib3sNV
    pub VertexAttrib3s: FnPtr,
    /// Fallbacks: VertexAttrib3svARB, VertexAttrib3svNV
    pub VertexAttrib3sv: FnPtr,
    /// Fallbacks: VertexAttrib4NbvARB
    pub VertexAttrib4Nbv: FnPtr,
    /// Fallbacks: VertexAttrib4NivARB
    pub VertexAttrib4Niv: FnPtr,
    /// Fallbacks: VertexAttrib4NsvARB
    pub VertexAttrib4Nsv: FnPtr,
    /// Fallbacks: VertexAttrib4NubARB, VertexAttrib4ubNV
    pub VertexAttrib4Nub: FnPtr,
    /// Fallbacks: VertexAttrib4NubvARB, VertexAttrib4ubvNV
    pub VertexAttrib4Nubv: FnPtr,
    /// Fallbacks: VertexAttrib4NuivARB
    pub VertexAttrib4Nuiv: FnPtr,
    /// Fallbacks: VertexAttrib4NusvARB
    pub VertexAttrib4Nusv: FnPtr,
    /// Fallbacks: VertexAttrib4bvARB
    pub VertexAttrib4bv: FnPtr,
    /// Fallbacks: VertexAttrib4dARB, VertexAttrib4dNV
    pub VertexAttrib4d: FnPtr,
    /// Fallbacks: VertexAttrib4dvARB, VertexAttrib4dvNV
    pub VertexAttrib4dv: FnPtr,
    /// Fallbacks: VertexAttrib4fARB, VertexAttrib4fNV
    pub VertexAttrib4f: FnPtr,
    /// Fallbacks: VertexAttrib4fvARB, VertexAttrib4fvNV
    pub VertexAttrib4fv: FnPtr,
    /// Fallbacks: VertexAttrib4ivARB
    pub VertexAttrib4iv: FnPtr,
    /// Fallbacks: VertexAttrib4sARB, VertexAttrib4sNV
    pub VertexAttrib4s: FnPtr,
    /// Fallbacks: VertexAttrib4svARB, VertexAttrib4svNV
    pub VertexAttrib4sv: FnPtr,
    /// Fallbacks: VertexAttrib4ubvARB
    pub VertexAttrib4ubv: FnPtr,
    /// Fallbacks: VertexAttrib4uivARB
    pub VertexAttrib4uiv: FnPtr,
    /// Fallbacks: VertexAttrib4usvARB
    pub VertexAttrib4usv: FnPtr,
    /// Fallbacks: VertexAttribDivisorANGLE, VertexAttribDivisorARB, VertexAttribDivisorEXT, VertexAttribDivisorNV
    pub VertexAttribDivisor: FnPtr,
    /// Fallbacks: VertexAttribI1iEXT
    pub VertexAttribI1i: FnPtr,
    /// Fallbacks: VertexAttribI1ivEXT
    pub VertexAttribI1iv: FnPtr,
    /// Fallbacks: VertexAttribI1uiEXT
    pub VertexAttribI1ui: FnPtr,
    /// Fallbacks: VertexAttribI1uivEXT
    pub VertexAttribI1uiv: FnPtr,
    /// Fallbacks: VertexAttribI2iEXT
    pub VertexAttribI2i: FnPtr,
    /// Fallbacks: VertexAttribI2ivEXT
    pub VertexAttribI2iv: FnPtr,
    /// Fallbacks: VertexAttribI2uiEXT
    pub VertexAttribI2ui: FnPtr,
    /// Fallbacks: VertexAttribI2uivEXT
    pub VertexAttribI2uiv: FnPtr,
    /// Fallbacks: VertexAttribI3iEXT
    pub VertexAttribI3i: FnPtr,
    /// Fallbacks: VertexAttribI3ivEXT
    pub VertexAttribI3iv: FnPtr,
    /// Fallbacks: VertexAttribI3uiEXT
    pub VertexAttribI3ui: FnPtr,
    /// Fallbacks: VertexAttribI3uivEXT
    pub VertexAttribI3uiv: FnPtr,
    /// Fallbacks: VertexAttribI4bvEXT
    pub VertexAttribI4bv: FnPtr,
    /// Fallbacks: VertexAttribI4iEXT
    pub VertexAttribI4i: FnPtr,
    /// Fallbacks: VertexAttribI4ivEXT
    pub VertexAttribI4iv: FnPtr,
    /// Fallbacks: VertexAttribI4svEXT
    pub VertexAttribI4sv: FnPtr,
    /// Fallbacks: VertexAttribI4ubvEXT
    pub VertexAttribI4ubv: FnPtr,
    /// Fallbacks: VertexAttribI4uiEXT
    pub VertexAttribI4ui: FnPtr,
    /// Fallbacks: VertexAttribI4uivEXT
    pub VertexAttribI4uiv: FnPtr,
    /// Fallbacks: VertexAttribI4usvEXT
    pub VertexAttribI4usv: FnPtr,
    /// Fallbacks: VertexAttribIPointerEXT
    pub VertexAttribIPointer: FnPtr,
    pub VertexAttribP1ui: FnPtr,
    pub VertexAttribP1uiv: FnPtr,
    pub VertexAttribP2ui: FnPtr,
    pub VertexAttribP2uiv: FnPtr,
    pub VertexAttribP3ui: FnPtr,
    pub VertexAttribP3uiv: FnPtr,
    pub VertexAttribP4ui: FnPtr,
    pub VertexAttribP4uiv: FnPtr,
    /// Fallbacks: VertexAttribPointerARB
    pub VertexAttribPointer: FnPtr,
    pub VertexP2ui: FnPtr,
    pub VertexP2uiv: FnPtr,
    pub VertexP3ui: FnPtr,
    pub VertexP3uiv: FnPtr,
    pub VertexP4ui: FnPtr,
    pub VertexP4uiv: FnPtr,
    pub Viewport: FnPtr,
    /// Fallbacks: WaitSyncAPPLE
    pub WaitSync: FnPtr,
    _priv: (),
}
impl Gl {
    /// Load each OpenGL symbol using a custom load function. This allows for the
    /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
    ///
    /// ~~~ignore
    /// let gl = Gl::load_with(|s| glfw.get_proc_address(s));
    /// ~~~
    #[allow(dead_code, unused_variables)]
    pub fn load_with<F>(mut loadfn: F) -> Gl
    where
        F: FnMut(&'static str) -> *const __gl_imports::raw::c_void,
    {
        #[inline(never)]
        fn do_metaloadfn(
            loadfn: &mut dyn FnMut(&'static str) -> *const __gl_imports::raw::c_void,
            symbol: &'static str,
            symbols: &[&'static str],
        ) -> *const __gl_imports::raw::c_void {
            let mut ptr = loadfn(symbol);
            if ptr.is_null() {
                for &sym in symbols {
                    ptr = loadfn(sym);
                    if !ptr.is_null() {
                        break;
                    }
                }
            }
            ptr
        }
        let mut metaloadfn = |symbol: &'static str, symbols: &[&'static str]| {
            do_metaloadfn(&mut loadfn, symbol, symbols)
        };
        Gl {
            ActiveTexture: FnPtr::new(metaloadfn("glActiveTexture", &["glActiveTextureARB"])),
            AttachShader: FnPtr::new(metaloadfn("glAttachShader", &["glAttachObjectARB"])),
            BeginConditionalRender: FnPtr::new(metaloadfn(
                "glBeginConditionalRender",
                &["glBeginConditionalRenderNV"],
            )),
            BeginQuery: FnPtr::new(metaloadfn("glBeginQuery", &["glBeginQueryARB"])),
            BeginTransformFeedback: FnPtr::new(metaloadfn(
                "glBeginTransformFeedback",
                &["glBeginTransformFeedbackEXT", "glBeginTransformFeedbackNV"],
            )),
            BindAttribLocation: FnPtr::new(metaloadfn(
                "glBindAttribLocation",
                &["glBindAttribLocationARB"],
            )),
            BindBuffer: FnPtr::new(metaloadfn("glBindBuffer", &["glBindBufferARB"])),
            BindBufferBase: FnPtr::new(metaloadfn(
                "glBindBufferBase",
                &["glBindBufferBaseEXT", "glBindBufferBaseNV"],
            )),
            BindBufferRange: FnPtr::new(metaloadfn(
                "glBindBufferRange",
                &["glBindBufferRangeEXT", "glBindBufferRangeNV"],
            )),
            BindFragDataLocation: FnPtr::new(metaloadfn(
                "glBindFragDataLocation",
                &["glBindFragDataLocationEXT"],
            )),
            BindFragDataLocationIndexed: FnPtr::new(metaloadfn(
                "glBindFragDataLocationIndexed",
                &["glBindFragDataLocationIndexedEXT"],
            )),
            BindFramebuffer: FnPtr::new(metaloadfn("glBindFramebuffer", &[])),
            BindRenderbuffer: FnPtr::new(metaloadfn("glBindRenderbuffer", &[])),
            BindSampler: FnPtr::new(metaloadfn("glBindSampler", &[])),
            BindTexture: FnPtr::new(metaloadfn("glBindTexture", &["glBindTextureEXT"])),
            BindVertexArray: FnPtr::new(metaloadfn("glBindVertexArray", &["glBindVertexArrayOES"])),
            BlendColor: FnPtr::new(metaloadfn("glBlendColor", &["glBlendColorEXT"])),
            BlendEquation: FnPtr::new(metaloadfn("glBlendEquation", &["glBlendEquationEXT"])),
            BlendEquationSeparate: FnPtr::new(metaloadfn(
                "glBlendEquationSeparate",
                &["glBlendEquationSeparateEXT"],
            )),
            BlendFunc: FnPtr::new(metaloadfn("glBlendFunc", &[])),
            BlendFuncSeparate: FnPtr::new(metaloadfn(
                "glBlendFuncSeparate",
                &["glBlendFuncSeparateEXT", "glBlendFuncSeparateINGR"],
            )),
            BlitFramebuffer: FnPtr::new(metaloadfn(
                "glBlitFramebuffer",
                &["glBlitFramebufferEXT", "glBlitFramebufferNV"],
            )),
            BufferData: FnPtr::new(metaloadfn("glBufferData", &["glBufferDataARB"])),
            BufferSubData: FnPtr::new(metaloadfn("glBufferSubData", &["glBufferSubDataARB"])),
            CallCommandListNV: FnPtr::new(metaloadfn("glCallCommandListNV", &[])),
            CheckFramebufferStatus: FnPtr::new(metaloadfn(
                "glCheckFramebufferStatus",
                &["glCheckFramebufferStatusEXT"],
            )),
            ClampColor: FnPtr::new(metaloadfn("glClampColor", &["glClampColorARB"])),
            Clear: FnPtr::new(metaloadfn("glClear", &[])),
            ClearBufferfi: FnPtr::new(metaloadfn("glClearBufferfi", &[])),
            ClearBufferfv: FnPtr::new(metaloadfn("glClearBufferfv", &[])),
            ClearBufferiv: FnPtr::new(metaloadfn("glClearBufferiv", &[])),
            ClearBufferuiv: FnPtr::new(metaloadfn("glClearBufferuiv", &[])),
            ClearColor: FnPtr::new(metaloadfn("glClearColor", &[])),
            ClearDepth: FnPtr::new(metaloadfn("glClearDepth", &[])),
            ClearStencil: FnPtr::new(metaloadfn("glClearStencil", &[])),
            ClientWaitSync: FnPtr::new(metaloadfn("glClientWaitSync", &["glClientWaitSyncAPPLE"])),
            ColorMask: FnPtr::new(metaloadfn("glColorMask", &[])),
            ColorMaski: FnPtr::new(metaloadfn(
                "glColorMaski",
                &[
                    "glColorMaskIndexedEXT",
                    "glColorMaskiEXT",
                    "glColorMaskiOES",
                ],
            )),
            ColorP3ui: FnPtr::new(metaloadfn("glColorP3ui", &[])),
            ColorP3uiv: FnPtr::new(metaloadfn("glColorP3uiv", &[])),
            ColorP4ui: FnPtr::new(metaloadfn("glColorP4ui", &[])),
            ColorP4uiv: FnPtr::new(metaloadfn("glColorP4uiv", &[])),
            CommandListSegmentsNV: FnPtr::new(metaloadfn("glCommandListSegmentsNV", &[])),
            CompileCommandListNV: FnPtr::new(metaloadfn("glCompileCommandListNV", &[])),
            CompileShader: FnPtr::new(metaloadfn("glCompileShader", &["glCompileShaderARB"])),
            CompressedTexImage1D: FnPtr::new(metaloadfn(
                "glCompressedTexImage1D",
                &["glCompressedTexImage1DARB"],
            )),
            CompressedTexImage2D: FnPtr::new(metaloadfn(
                "glCompressedTexImage2D",
                &["glCompressedTexImage2DARB"],
            )),
            CompressedTexImage3D: FnPtr::new(metaloadfn(
                "glCompressedTexImage3D",
                &["glCompressedTexImage3DARB"],
            )),
            CompressedTexSubImage1D: FnPtr::new(metaloadfn(
                "glCompressedTexSubImage1D",
                &["glCompressedTexSubImage1DARB"],
            )),
            CompressedTexSubImage2D: FnPtr::new(metaloadfn(
                "glCompressedTexSubImage2D",
                &["glCompressedTexSubImage2DARB"],
            )),
            CompressedTexSubImage3D: FnPtr::new(metaloadfn(
                "glCompressedTexSubImage3D",
                &["glCompressedTexSubImage3DARB"],
            )),
            CopyBufferSubData: FnPtr::new(metaloadfn(
                "glCopyBufferSubData",
                &["glCopyBufferSubDataNV"],
            )),
            CopyTexImage1D: FnPtr::new(metaloadfn("glCopyTexImage1D", &["glCopyTexImage1DEXT"])),
            CopyTexImage2D: FnPtr::new(metaloadfn("glCopyTexImage2D", &["glCopyTexImage2DEXT"])),
            CopyTexSubImage1D: FnPtr::new(metaloadfn(
                "glCopyTexSubImage1D",
                &["glCopyTexSubImage1DEXT"],
            )),
            CopyTexSubImage2D: FnPtr::new(metaloadfn(
                "glCopyTexSubImage2D",
                &["glCopyTexSubImage2DEXT"],
            )),
            CopyTexSubImage3D: FnPtr::new(metaloadfn(
                "glCopyTexSubImage3D",
                &["glCopyTexSubImage3DEXT"],
            )),
            CreateCommandListsNV: FnPtr::new(metaloadfn("glCreateCommandListsNV", &[])),
            CreateProgram: FnPtr::new(metaloadfn("glCreateProgram", &["glCreateProgramObjectARB"])),
            CreateShader: FnPtr::new(metaloadfn("glCreateShader", &["glCreateShaderObjectARB"])),
            CreateStatesNV: FnPtr::new(metaloadfn("glCreateStatesNV", &[])),
            CullFace: FnPtr::new(metaloadfn("glCullFace", &[])),
            DeleteBuffers: FnPtr::new(metaloadfn("glDeleteBuffers", &["glDeleteBuffersARB"])),
            DeleteCommandListsNV: FnPtr::new(metaloadfn("glDeleteCommandListsNV", &[])),
            DeleteFramebuffers: FnPtr::new(metaloadfn(
                "glDeleteFramebuffers",
                &["glDeleteFramebuffersEXT"],
            )),
            DeleteProgram: FnPtr::new(metaloadfn("glDeleteProgram", &[])),
            DeleteQueries: FnPtr::new(metaloadfn("glDeleteQueries", &["glDeleteQueriesARB"])),
            DeleteRenderbuffers: FnPtr::new(metaloadfn(
                "glDeleteRenderbuffers",
                &["glDeleteRenderbuffersEXT"],
            )),
            DeleteSamplers: FnPtr::new(metaloadfn("glDeleteSamplers", &[])),
            DeleteShader: FnPtr::new(metaloadfn("glDeleteShader", &[])),
            DeleteStatesNV: FnPtr::new(metaloadfn("glDeleteStatesNV", &[])),
            DeleteSync: FnPtr::new(metaloadfn("glDeleteSync", &["glDeleteSyncAPPLE"])),
            DeleteTextures: FnPtr::new(metaloadfn("glDeleteTextures", &[])),
            DeleteVertexArrays: FnPtr::new(metaloadfn(
                "glDeleteVertexArrays",
                &["glDeleteVertexArraysAPPLE", "glDeleteVertexArraysOES"],
            )),
            DepthFunc: FnPtr::new(metaloadfn("glDepthFunc", &[])),
            DepthMask: FnPtr::new(metaloadfn("glDepthMask", &[])),
            DepthRange: FnPtr::new(metaloadfn("glDepthRange", &[])),
            DetachShader: FnPtr::new(metaloadfn("glDetachShader", &["glDetachObjectARB"])),
            Disable: FnPtr::new(metaloadfn("glDisable", &[])),
            DisableVertexAttribArray: FnPtr::new(metaloadfn(
                "glDisableVertexAttribArray",
                &["glDisableVertexAttribArrayARB"],
            )),
            Disablei: FnPtr::new(metaloadfn(
                "glDisablei",
                &[
                    "glDisableIndexedEXT",
                    "glDisableiEXT",
                    "glDisableiNV",
                    "glDisableiOES",
                ],
            )),
            DrawArrays: FnPtr::new(metaloadfn("glDrawArrays", &["glDrawArraysEXT"])),
            DrawArraysInstanced: FnPtr::new(metaloadfn(
                "glDrawArraysInstanced",
                &[
                    "glDrawArraysInstancedANGLE",
                    "glDrawArraysInstancedARB",
                    "glDrawArraysInstancedEXT",
                    "glDrawArraysInstancedNV",
                ],
            )),
            DrawBuffer: FnPtr::new(metaloadfn("glDrawBuffer", &[])),
            DrawBuffers: FnPtr::new(metaloadfn(
                "glDrawBuffers",
                &["glDrawBuffersARB", "glDrawBuffersATI", "glDrawBuffersEXT"],
            )),
            DrawCommandsAddressNV: FnPtr::new(metaloadfn("glDrawCommandsAddressNV", &[])),
            DrawCommandsNV: FnPtr::new(metaloadfn("glDrawCommandsNV", &[])),
            DrawCommandsStatesAddressNV: FnPtr::new(metaloadfn(
                "glDrawCommandsStatesAddressNV",
                &[],
            )),
            DrawCommandsStatesNV: FnPtr::new(metaloadfn("glDrawCommandsStatesNV", &[])),
            DrawElements: FnPtr::new(metaloadfn("glDrawElements", &[])),
            DrawElementsBaseVertex: FnPtr::new(metaloadfn(
                "glDrawElementsBaseVertex",
                &["glDrawElementsBaseVertexEXT", "glDrawElementsBaseVertexOES"],
            )),
            DrawElementsInstanced: FnPtr::new(metaloadfn(
                "glDrawElementsInstanced",
                &[
                    "glDrawElementsInstancedANGLE",
                    "glDrawElementsInstancedARB",
                    "glDrawElementsInstancedEXT",
                    "glDrawElementsInstancedNV",
                ],
            )),
            DrawElementsInstancedBaseVertex: FnPtr::new(metaloadfn(
                "glDrawElementsInstancedBaseVertex",
                &[
                    "glDrawElementsInstancedBaseVertexEXT",
                    "glDrawElementsInstancedBaseVertexOES",
                ],
            )),
            DrawRangeElements: FnPtr::new(metaloadfn(
                "glDrawRangeElements",
                &["glDrawRangeElementsEXT"],
            )),
            DrawRangeElementsBaseVertex: FnPtr::new(metaloadfn(
                "glDrawRangeElementsBaseVertex",
                &[
                    "glDrawRangeElementsBaseVertexEXT",
                    "glDrawRangeElementsBaseVertexOES",
                ],
            )),
            Enable: FnPtr::new(metaloadfn("glEnable", &[])),
            EnableVertexAttribArray: FnPtr::new(metaloadfn(
                "glEnableVertexAttribArray",
                &["glEnableVertexAttribArrayARB"],
            )),
            Enablei: FnPtr::new(metaloadfn(
                "glEnablei",
                &[
                    "glEnableIndexedEXT",
                    "glEnableiEXT",
                    "glEnableiNV",
                    "glEnableiOES",
                ],
            )),
            EndConditionalRender: FnPtr::new(metaloadfn(
                "glEndConditionalRender",
                &["glEndConditionalRenderNV", "glEndConditionalRenderNVX"],
            )),
            EndQuery: FnPtr::new(metaloadfn("glEndQuery", &["glEndQueryARB"])),
            EndTransformFeedback: FnPtr::new(metaloadfn(
                "glEndTransformFeedback",
                &["glEndTransformFeedbackEXT", "glEndTransformFeedbackNV"],
            )),
            FenceSync: FnPtr::new(metaloadfn("glFenceSync", &["glFenceSyncAPPLE"])),
            Finish: FnPtr::new(metaloadfn("glFinish", &[])),
            Flush: FnPtr::new(metaloadfn("glFlush", &[])),
            FlushMappedBufferRange: FnPtr::new(metaloadfn(
                "glFlushMappedBufferRange",
                &[
                    "glFlushMappedBufferRangeAPPLE",
                    "glFlushMappedBufferRangeEXT",
                ],
            )),
            FramebufferRenderbuffer: FnPtr::new(metaloadfn(
                "glFramebufferRenderbuffer",
                &["glFramebufferRenderbufferEXT"],
            )),
            FramebufferTexture: FnPtr::new(metaloadfn(
                "glFramebufferTexture",
                &[
                    "glFramebufferTextureARB",
                    "glFramebufferTextureEXT",
                    "glFramebufferTextureOES",
                ],
            )),
            FramebufferTexture1D: FnPtr::new(metaloadfn(
                "glFramebufferTexture1D",
                &["glFramebufferTexture1DEXT"],
            )),
            FramebufferTexture2D: FnPtr::new(metaloadfn(
                "glFramebufferTexture2D",
                &["glFramebufferTexture2DEXT"],
            )),
            FramebufferTexture3D: FnPtr::new(metaloadfn(
                "glFramebufferTexture3D",
                &["glFramebufferTexture3DEXT"],
            )),
            FramebufferTextureLayer: FnPtr::new(metaloadfn(
                "glFramebufferTextureLayer",
                &[
                    "glFramebufferTextureLayerARB",
                    "glFramebufferTextureLayerEXT",
                ],
            )),
            FrontFace: FnPtr::new(metaloadfn("glFrontFace", &[])),
            GenBuffers: FnPtr::new(metaloadfn("glGenBuffers", &["glGenBuffersARB"])),
            GenFramebuffers: FnPtr::new(metaloadfn("glGenFramebuffers", &["glGenFramebuffersEXT"])),
            GenQueries: FnPtr::new(metaloadfn("glGenQueries", &["glGenQueriesARB"])),
            GenRenderbuffers: FnPtr::new(metaloadfn(
                "glGenRenderbuffers",
                &["glGenRenderbuffersEXT"],
            )),
            GenSamplers: FnPtr::new(metaloadfn("glGenSamplers", &[])),
            GenTextures: FnPtr::new(metaloadfn("glGenTextures", &[])),
            GenVertexArrays: FnPtr::new(metaloadfn(
                "glGenVertexArrays",
                &["glGenVertexArraysAPPLE", "glGenVertexArraysOES"],
            )),
            GenerateMipmap: FnPtr::new(metaloadfn("glGenerateMipmap", &["glGenerateMipmapEXT"])),
            GetActiveAttrib: FnPtr::new(metaloadfn("glGetActiveAttrib", &["glGetActiveAttribARB"])),
            GetActiveUniform: FnPtr::new(metaloadfn(
                "glGetActiveUniform",
                &["glGetActiveUniformARB"],
            )),
            GetActiveUniformBlockName: FnPtr::new(metaloadfn("glGetActiveUniformBlockName", &[])),
            GetActiveUniformBlockiv: FnPtr::new(metaloadfn("glGetActiveUniformBlockiv", &[])),
            GetActiveUniformName: FnPtr::new(metaloadfn("glGetActiveUniformName", &[])),
            GetActiveUniformsiv: FnPtr::new(metaloadfn("glGetActiveUniformsiv", &[])),
            GetAttachedShaders: FnPtr::new(metaloadfn("glGetAttachedShaders", &[])),
            GetAttribLocation: FnPtr::new(metaloadfn(
                "glGetAttribLocation",
                &["glGetAttribLocationARB"],
            )),
            GetBooleani_v: FnPtr::new(metaloadfn("glGetBooleani_v", &["glGetBooleanIndexedvEXT"])),
            GetBooleanv: FnPtr::new(metaloadfn("glGetBooleanv", &[])),
            GetBufferParameteri64v: FnPtr::new(metaloadfn("glGetBufferParameteri64v", &[])),
            GetBufferParameteriv: FnPtr::new(metaloadfn(
                "glGetBufferParameteriv",
                &["glGetBufferParameterivARB"],
            )),
            GetBufferPointerv: FnPtr::new(metaloadfn(
                "glGetBufferPointerv",
                &["glGetBufferPointervARB", "glGetBufferPointervOES"],
            )),
            GetBufferSubData: FnPtr::new(metaloadfn(
                "glGetBufferSubData",
                &["glGetBufferSubDataARB"],
            )),
            GetCommandHeaderNV: FnPtr::new(metaloadfn("glGetCommandHeaderNV", &[])),
            GetCompressedTexImage: FnPtr::new(metaloadfn(
                "glGetCompressedTexImage",
                &["glGetCompressedTexImageARB"],
            )),
            GetDoublev: FnPtr::new(metaloadfn("glGetDoublev", &[])),
            GetError: FnPtr::new(metaloadfn("glGetError", &[])),
            GetFloatv: FnPtr::new(metaloadfn("glGetFloatv", &[])),
            GetFragDataIndex: FnPtr::new(metaloadfn(
                "glGetFragDataIndex",
                &["glGetFragDataIndexEXT"],
            )),
            GetFragDataLocation: FnPtr::new(metaloadfn(
                "glGetFragDataLocation",
                &["glGetFragDataLocationEXT"],
            )),
            GetFramebufferAttachmentParameteriv: FnPtr::new(metaloadfn(
                "glGetFramebufferAttachmentParameteriv",
                &["glGetFramebufferAttachmentParameterivEXT"],
            )),
            GetInteger64i_v: FnPtr::new(metaloadfn("glGetInteger64i_v", &[])),
            GetInteger64v: FnPtr::new(metaloadfn("glGetInteger64v", &["glGetInteger64vAPPLE"])),
            GetIntegeri_v: FnPtr::new(metaloadfn("glGetIntegeri_v", &["glGetIntegerIndexedvEXT"])),
            GetIntegerv: FnPtr::new(metaloadfn("glGetIntegerv", &[])),
            GetMultisamplefv: FnPtr::new(metaloadfn(
                "glGetMultisamplefv",
                &["glGetMultisamplefvNV"],
            )),
            GetProgramInfoLog: FnPtr::new(metaloadfn("glGetProgramInfoLog", &[])),
            GetProgramiv: FnPtr::new(metaloadfn("glGetProgramiv", &[])),
            GetQueryObjecti64v: FnPtr::new(metaloadfn(
                "glGetQueryObjecti64v",
                &["glGetQueryObjecti64vEXT"],
            )),
            GetQueryObjectiv: FnPtr::new(metaloadfn(
                "glGetQueryObjectiv",
                &["glGetQueryObjectivARB", "glGetQueryObjectivEXT"],
            )),
            GetQueryObjectui64v: FnPtr::new(metaloadfn(
                "glGetQueryObjectui64v",
                &["glGetQueryObjectui64vEXT"],
            )),
            GetQueryObjectuiv: FnPtr::new(metaloadfn(
                "glGetQueryObjectuiv",
                &["glGetQueryObjectuivARB"],
            )),
            GetQueryiv: FnPtr::new(metaloadfn("glGetQueryiv", &["glGetQueryivARB"])),
            GetRenderbufferParameteriv: FnPtr::new(metaloadfn(
                "glGetRenderbufferParameteriv",
                &["glGetRenderbufferParameterivEXT"],
            )),
            GetSamplerParameterIiv: FnPtr::new(metaloadfn(
                "glGetSamplerParameterIiv",
                &["glGetSamplerParameterIivEXT", "glGetSamplerParameterIivOES"],
            )),
            GetSamplerParameterIuiv: FnPtr::new(metaloadfn(
                "glGetSamplerParameterIuiv",
                &[
                    "glGetSamplerParameterIuivEXT",
                    "glGetSamplerParameterIuivOES",
                ],
            )),
            GetSamplerParameterfv: FnPtr::new(metaloadfn("glGetSamplerParameterfv", &[])),
            GetSamplerParameteriv: FnPtr::new(metaloadfn("glGetSamplerParameteriv", &[])),
            GetShaderInfoLog: FnPtr::new(metaloadfn("glGetShaderInfoLog", &[])),
            GetShaderSource: FnPtr::new(metaloadfn("glGetShaderSource", &["glGetShaderSourceARB"])),
            GetShaderiv: FnPtr::new(metaloadfn("glGetShaderiv", &[])),
            GetStageIndexNV: FnPtr::new(metaloadfn("glGetStageIndexNV", &[])),
            GetString: FnPtr::new(metaloadfn("glGetString", &[])),
            GetStringi: FnPtr::new(metaloadfn("glGetStringi", &[])),
            GetSynciv: FnPtr::new(metaloadfn("glGetSynciv", &["glGetSyncivAPPLE"])),
            GetTexImage: FnPtr::new(metaloadfn("glGetTexImage", &[])),
            GetTexLevelParameterfv: FnPtr::new(metaloadfn("glGetTexLevelParameterfv", &[])),
            GetTexLevelParameteriv: FnPtr::new(metaloadfn("glGetTexLevelParameteriv", &[])),
            GetTexParameterIiv: FnPtr::new(metaloadfn(
                "glGetTexParameterIiv",
                &["glGetTexParameterIivEXT", "glGetTexParameterIivOES"],
            )),
            GetTexParameterIuiv: FnPtr::new(metaloadfn(
                "glGetTexParameterIuiv",
                &["glGetTexParameterIuivEXT", "glGetTexParameterIuivOES"],
            )),
            GetTexParameterfv: FnPtr::new(metaloadfn("glGetTexParameterfv", &[])),
            GetTexParameteriv: FnPtr::new(metaloadfn("glGetTexParameteriv", &[])),
            GetTransformFeedbackVarying: FnPtr::new(metaloadfn(
                "glGetTransformFeedbackVarying",
                &["glGetTransformFeedbackVaryingEXT"],
            )),
            GetUniformBlockIndex: FnPtr::new(metaloadfn("glGetUniformBlockIndex", &[])),
            GetUniformIndices: FnPtr::new(metaloadfn("glGetUniformIndices", &[])),
            GetUniformLocation: FnPtr::new(metaloadfn(
                "glGetUniformLocation",
                &["glGetUniformLocationARB"],
            )),
            GetUniformfv: FnPtr::new(metaloadfn("glGetUniformfv", &["glGetUniformfvARB"])),
            GetUniformiv: FnPtr::new(metaloadfn("glGetUniformiv", &["glGetUniformivARB"])),
            GetUniformuiv: FnPtr::new(metaloadfn("glGetUniformuiv", &["glGetUniformuivEXT"])),
            GetVertexAttribIiv: FnPtr::new(metaloadfn(
                "glGetVertexAttribIiv",
                &["glGetVertexAttribIivEXT"],
            )),
            GetVertexAttribIuiv: FnPtr::new(metaloadfn(
                "glGetVertexAttribIuiv",
                &["glGetVertexAttribIuivEXT"],
            )),
            GetVertexAttribPointerv: FnPtr::new(metaloadfn(
                "glGetVertexAttribPointerv",
                &[
                    "glGetVertexAttribPointervARB",
                    "glGetVertexAttribPointervNV",
                ],
            )),
            GetVertexAttribdv: FnPtr::new(metaloadfn(
                "glGetVertexAttribdv",
                &["glGetVertexAttribdvARB", "glGetVertexAttribdvNV"],
            )),
            GetVertexAttribfv: FnPtr::new(metaloadfn(
                "glGetVertexAttribfv",
                &["glGetVertexAttribfvARB", "glGetVertexAttribfvNV"],
            )),
            GetVertexAttribiv: FnPtr::new(metaloadfn(
                "glGetVertexAttribiv",
                &["glGetVertexAttribivARB", "glGetVertexAttribivNV"],
            )),
            Hint: FnPtr::new(metaloadfn("glHint", &[])),
            IsBuffer: FnPtr::new(metaloadfn("glIsBuffer", &["glIsBufferARB"])),
            IsCommandListNV: FnPtr::new(metaloadfn("glIsCommandListNV", &[])),
            IsEnabled: FnPtr::new(metaloadfn("glIsEnabled", &[])),
            IsEnabledi: FnPtr::new(metaloadfn(
                "glIsEnabledi",
                &[
                    "glIsEnabledIndexedEXT",
                    "glIsEnablediEXT",
                    "glIsEnablediNV",
                    "glIsEnablediOES",
                ],
            )),
            IsFramebuffer: FnPtr::new(metaloadfn("glIsFramebuffer", &["glIsFramebufferEXT"])),
            IsProgram: FnPtr::new(metaloadfn("glIsProgram", &[])),
            IsQuery: FnPtr::new(metaloadfn("glIsQuery", &["glIsQueryARB"])),
            IsRenderbuffer: FnPtr::new(metaloadfn("glIsRenderbuffer", &["glIsRenderbufferEXT"])),
            IsSampler: FnPtr::new(metaloadfn("glIsSampler", &[])),
            IsShader: FnPtr::new(metaloadfn("glIsShader", &[])),
            IsStateNV: FnPtr::new(metaloadfn("glIsStateNV", &[])),
            IsSync: FnPtr::new(metaloadfn("glIsSync", &["glIsSyncAPPLE"])),
            IsTexture: FnPtr::new(metaloadfn("glIsTexture", &[])),
            IsVertexArray: FnPtr::new(metaloadfn(
                "glIsVertexArray",
                &["glIsVertexArrayAPPLE", "glIsVertexArrayOES"],
            )),
            LineWidth: FnPtr::new(metaloadfn("glLineWidth", &[])),
            LinkProgram: FnPtr::new(metaloadfn("glLinkProgram", &["glLinkProgramARB"])),
            ListDrawCommandsStatesClientNV: FnPtr::new(metaloadfn(
                "glListDrawCommandsStatesClientNV",
                &[],
            )),
            LogicOp: FnPtr::new(metaloadfn("glLogicOp", &[])),
            MapBuffer: FnPtr::new(metaloadfn(
                "glMapBuffer",
                &["glMapBufferARB", "glMapBufferOES"],
            )),
            MapBufferRange: FnPtr::new(metaloadfn("glMapBufferRange", &["glMapBufferRangeEXT"])),
            MultiDrawArrays: FnPtr::new(metaloadfn("glMultiDrawArrays", &["glMultiDrawArraysEXT"])),
            MultiDrawElements: FnPtr::new(metaloadfn(
                "glMultiDrawElements",
                &["glMultiDrawElementsEXT"],
            )),
            MultiDrawElementsBaseVertex: FnPtr::new(metaloadfn(
                "glMultiDrawElementsBaseVertex",
                &["glMultiDrawElementsBaseVertexEXT"],
            )),
            MultiTexCoordP1ui: FnPtr::new(metaloadfn("glMultiTexCoordP1ui", &[])),
            MultiTexCoordP1uiv: FnPtr::new(metaloadfn("glMultiTexCoordP1uiv", &[])),
            MultiTexCoordP2ui: FnPtr::new(metaloadfn("glMultiTexCoordP2ui", &[])),
            MultiTexCoordP2uiv: FnPtr::new(metaloadfn("glMultiTexCoordP2uiv", &[])),
            MultiTexCoordP3ui: FnPtr::new(metaloadfn("glMultiTexCoordP3ui", &[])),
            MultiTexCoordP3uiv: FnPtr::new(metaloadfn("glMultiTexCoordP3uiv", &[])),
            MultiTexCoordP4ui: FnPtr::new(metaloadfn("glMultiTexCoordP4ui", &[])),
            MultiTexCoordP4uiv: FnPtr::new(metaloadfn("glMultiTexCoordP4uiv", &[])),
            NormalP3ui: FnPtr::new(metaloadfn("glNormalP3ui", &[])),
            NormalP3uiv: FnPtr::new(metaloadfn("glNormalP3uiv", &[])),
            PixelStoref: FnPtr::new(metaloadfn("glPixelStoref", &[])),
            PixelStorei: FnPtr::new(metaloadfn("glPixelStorei", &[])),
            PointParameterf: FnPtr::new(metaloadfn(
                "glPointParameterf",
                &[
                    "glPointParameterfARB",
                    "glPointParameterfEXT",
                    "glPointParameterfSGIS",
                ],
            )),
            PointParameterfv: FnPtr::new(metaloadfn(
                "glPointParameterfv",
                &[
                    "glPointParameterfvARB",
                    "glPointParameterfvEXT",
                    "glPointParameterfvSGIS",
                ],
            )),
            PointParameteri: FnPtr::new(metaloadfn("glPointParameteri", &["glPointParameteriNV"])),
            PointParameteriv: FnPtr::new(metaloadfn(
                "glPointParameteriv",
                &["glPointParameterivNV"],
            )),
            PointSize: FnPtr::new(metaloadfn("glPointSize", &[])),
            PolygonMode: FnPtr::new(metaloadfn("glPolygonMode", &["glPolygonModeNV"])),
            PolygonOffset: FnPtr::new(metaloadfn("glPolygonOffset", &[])),
            PrimitiveRestartIndex: FnPtr::new(metaloadfn("glPrimitiveRestartIndex", &[])),
            ProvokingVertex: FnPtr::new(metaloadfn("glProvokingVertex", &["glProvokingVertexEXT"])),
            QueryCounter: FnPtr::new(metaloadfn("glQueryCounter", &["glQueryCounterEXT"])),
            ReadBuffer: FnPtr::new(metaloadfn("glReadBuffer", &[])),
            ReadPixels: FnPtr::new(metaloadfn("glReadPixels", &[])),
            RenderbufferStorage: FnPtr::new(metaloadfn(
                "glRenderbufferStorage",
                &["glRenderbufferStorageEXT"],
            )),
            RenderbufferStorageMultisample: FnPtr::new(metaloadfn(
                "glRenderbufferStorageMultisample",
                &[
                    "glRenderbufferStorageMultisampleEXT",
                    "glRenderbufferStorageMultisampleNV",
                ],
            )),
            SampleCoverage: FnPtr::new(metaloadfn("glSampleCoverage", &["glSampleCoverageARB"])),
            SampleMaski: FnPtr::new(metaloadfn("glSampleMaski", &[])),
            SamplerParameterIiv: FnPtr::new(metaloadfn(
                "glSamplerParameterIiv",
                &["glSamplerParameterIivEXT", "glSamplerParameterIivOES"],
            )),
            SamplerParameterIuiv: FnPtr::new(metaloadfn(
                "glSamplerParameterIuiv",
                &["glSamplerParameterIuivEXT", "glSamplerParameterIuivOES"],
            )),
            SamplerParameterf: FnPtr::new(metaloadfn("glSamplerParameterf", &[])),
            SamplerParameterfv: FnPtr::new(metaloadfn("glSamplerParameterfv", &[])),
            SamplerParameteri: FnPtr::new(metaloadfn("glSamplerParameteri", &[])),
            SamplerParameteriv: FnPtr::new(metaloadfn("glSamplerParameteriv", &[])),
            Scissor: FnPtr::new(metaloadfn("glScissor", &[])),
            SecondaryColorP3ui: FnPtr::new(metaloadfn("glSecondaryColorP3ui", &[])),
            SecondaryColorP3uiv: FnPtr::new(metaloadfn("glSecondaryColorP3uiv", &[])),
            ShaderSource: FnPtr::new(metaloadfn("glShaderSource", &["glShaderSourceARB"])),
            StateCaptureNV: FnPtr::new(metaloadfn("glStateCaptureNV", &[])),
            StencilFunc: FnPtr::new(metaloadfn("glStencilFunc", &[])),
            StencilFuncSeparate: FnPtr::new(metaloadfn("glStencilFuncSeparate", &[])),
            StencilMask: FnPtr::new(metaloadfn("glStencilMask", &[])),
            StencilMaskSeparate: FnPtr::new(metaloadfn("glStencilMaskSeparate", &[])),
            StencilOp: FnPtr::new(metaloadfn("glStencilOp", &[])),
            StencilOpSeparate: FnPtr::new(metaloadfn(
                "glStencilOpSeparate",
                &["glStencilOpSeparateATI"],
            )),
            TexBuffer: FnPtr::new(metaloadfn(
                "glTexBuffer",
                &["glTexBufferARB", "glTexBufferEXT", "glTexBufferOES"],
            )),
            TexCoordP1ui: FnPtr::new(metaloadfn("glTexCoordP1ui", &[])),
            TexCoordP1uiv: FnPtr::new(metaloadfn("glTexCoordP1uiv", &[])),
            TexCoordP2ui: FnPtr::new(metaloadfn("glTexCoordP2ui", &[])),
            TexCoordP2uiv: FnPtr::new(metaloadfn("glTexCoordP2uiv", &[])),
            TexCoordP3ui: FnPtr::new(metaloadfn("glTexCoordP3ui", &[])),
            TexCoordP3uiv: FnPtr::new(metaloadfn("glTexCoordP3uiv", &[])),
            TexCoordP4ui: FnPtr::new(metaloadfn("glTexCoordP4ui", &[])),
            TexCoordP4uiv: FnPtr::new(metaloadfn("glTexCoordP4uiv", &[])),
            TexImage1D: FnPtr::new(metaloadfn("glTexImage1D", &[])),
            TexImage2D: FnPtr::new(metaloadfn("glTexImage2D", &[])),
            TexImage2DMultisample: FnPtr::new(metaloadfn("glTexImage2DMultisample", &[])),
            TexImage3D: FnPtr::new(metaloadfn("glTexImage3D", &["glTexImage3DEXT"])),
            TexImage3DMultisample: FnPtr::new(metaloadfn("glTexImage3DMultisample", &[])),
            TexParameterIiv: FnPtr::new(metaloadfn(
                "glTexParameterIiv",
                &["glTexParameterIivEXT", "glTexParameterIivOES"],
            )),
            TexParameterIuiv: FnPtr::new(metaloadfn(
                "glTexParameterIuiv",
                &["glTexParameterIuivEXT", "glTexParameterIuivOES"],
            )),
            TexParameterf: FnPtr::new(metaloadfn("glTexParameterf", &[])),
            TexParameterfv: FnPtr::new(metaloadfn("glTexParameterfv", &[])),
            TexParameteri: FnPtr::new(metaloadfn("glTexParameteri", &[])),
            TexParameteriv: FnPtr::new(metaloadfn("glTexParameteriv", &[])),
            TexSubImage1D: FnPtr::new(metaloadfn("glTexSubImage1D", &["glTexSubImage1DEXT"])),
            TexSubImage2D: FnPtr::new(metaloadfn("glTexSubImage2D", &["glTexSubImage2DEXT"])),
            TexSubImage3D: FnPtr::new(metaloadfn("glTexSubImage3D", &["glTexSubImage3DEXT"])),
            TransformFeedbackVaryings: FnPtr::new(metaloadfn(
                "glTransformFeedbackVaryings",
                &["glTransformFeedbackVaryingsEXT"],
            )),
            Uniform1f: FnPtr::new(metaloadfn("glUniform1f", &["glUniform1fARB"])),
            Uniform1fv: FnPtr::new(metaloadfn("glUniform1fv", &["glUniform1fvARB"])),
            Uniform1i: FnPtr::new(metaloadfn("glUniform1i", &["glUniform1iARB"])),
            Uniform1iv: FnPtr::new(metaloadfn("glUniform1iv", &["glUniform1ivARB"])),
            Uniform1ui: FnPtr::new(metaloadfn("glUniform1ui", &["glUniform1uiEXT"])),
            Uniform1uiv: FnPtr::new(metaloadfn("glUniform1uiv", &["glUniform1uivEXT"])),
            Uniform2f: FnPtr::new(metaloadfn("glUniform2f", &["glUniform2fARB"])),
            Uniform2fv: FnPtr::new(metaloadfn("glUniform2fv", &["glUniform2fvARB"])),
            Uniform2i: FnPtr::new(metaloadfn("glUniform2i", &["glUniform2iARB"])),
            Uniform2iv: FnPtr::new(metaloadfn("glUniform2iv", &["glUniform2ivARB"])),
            Uniform2ui: FnPtr::new(metaloadfn("glUniform2ui", &["glUniform2uiEXT"])),
            Uniform2uiv: FnPtr::new(metaloadfn("glUniform2uiv", &["glUniform2uivEXT"])),
            Uniform3f: FnPtr::new(metaloadfn("glUniform3f", &["glUniform3fARB"])),
            Uniform3fv: FnPtr::new(metaloadfn("glUniform3fv", &["glUniform3fvARB"])),
            Uniform3i: FnPtr::new(metaloadfn("glUniform3i", &["glUniform3iARB"])),
            Uniform3iv: FnPtr::new(metaloadfn("glUniform3iv", &["glUniform3ivARB"])),
            Uniform3ui: FnPtr::new(metaloadfn("glUniform3ui", &["glUniform3uiEXT"])),
            Uniform3uiv: FnPtr::new(metaloadfn("glUniform3uiv", &["glUniform3uivEXT"])),
            Uniform4f: FnPtr::new(metaloadfn("glUniform4f", &["glUniform4fARB"])),
            Uniform4fv: FnPtr::new(metaloadfn("glUniform4fv", &["glUniform4fvARB"])),
            Uniform4i: FnPtr::new(metaloadfn("glUniform4i", &["glUniform4iARB"])),
            Uniform4iv: FnPtr::new(metaloadfn("glUniform4iv", &["glUniform4ivARB"])),
            Uniform4ui: FnPtr::new(metaloadfn("glUniform4ui", &["glUniform4uiEXT"])),
            Uniform4uiv: FnPtr::new(metaloadfn("glUniform4uiv", &["glUniform4uivEXT"])),
            UniformBlockBinding: FnPtr::new(metaloadfn("glUniformBlockBinding", &[])),
            UniformMatrix2fv: FnPtr::new(metaloadfn(
                "glUniformMatrix2fv",
                &["glUniformMatrix2fvARB"],
            )),
            UniformMatrix2x3fv: FnPtr::new(metaloadfn(
                "glUniformMatrix2x3fv",
                &["glUniformMatrix2x3fvNV"],
            )),
            UniformMatrix2x4fv: FnPtr::new(metaloadfn(
                "glUniformMatrix2x4fv",
                &["glUniformMatrix2x4fvNV"],
            )),
            UniformMatrix3fv: FnPtr::new(metaloadfn(
                "glUniformMatrix3fv",
                &["glUniformMatrix3fvARB"],
            )),
            UniformMatrix3x2fv: FnPtr::new(metaloadfn(
                "glUniformMatrix3x2fv",
                &["glUniformMatrix3x2fvNV"],
            )),
            UniformMatrix3x4fv: FnPtr::new(metaloadfn(
                "glUniformMatrix3x4fv",
                &["glUniformMatrix3x4fvNV"],
            )),
            UniformMatrix4fv: FnPtr::new(metaloadfn(
                "glUniformMatrix4fv",
                &["glUniformMatrix4fvARB"],
            )),
            UniformMatrix4x2fv: FnPtr::new(metaloadfn(
                "glUniformMatrix4x2fv",
                &["glUniformMatrix4x2fvNV"],
            )),
            UniformMatrix4x3fv: FnPtr::new(metaloadfn(
                "glUniformMatrix4x3fv",
                &["glUniformMatrix4x3fvNV"],
            )),
            UnmapBuffer: FnPtr::new(metaloadfn(
                "glUnmapBuffer",
                &["glUnmapBufferARB", "glUnmapBufferOES"],
            )),
            UseProgram: FnPtr::new(metaloadfn("glUseProgram", &["glUseProgramObjectARB"])),
            ValidateProgram: FnPtr::new(metaloadfn("glValidateProgram", &["glValidateProgramARB"])),
            VertexAttrib1d: FnPtr::new(metaloadfn(
                "glVertexAttrib1d",
                &["glVertexAttrib1dARB", "glVertexAttrib1dNV"],
            )),
            VertexAttrib1dv: FnPtr::new(metaloadfn(
                "glVertexAttrib1dv",
                &["glVertexAttrib1dvARB", "glVertexAttrib1dvNV"],
            )),
            VertexAttrib1f: FnPtr::new(metaloadfn(
                "glVertexAttrib1f",
                &["glVertexAttrib1fARB", "glVertexAttrib1fNV"],
            )),
            VertexAttrib1fv: FnPtr::new(metaloadfn(
                "glVertexAttrib1fv",
                &["glVertexAttrib1fvARB", "glVertexAttrib1fvNV"],
            )),
            VertexAttrib1s: FnPtr::new(metaloadfn(
                "glVertexAttrib1s",
                &["glVertexAttrib1sARB", "glVertexAttrib1sNV"],
            )),
            VertexAttrib1sv: FnPtr::new(metaloadfn(
                "glVertexAttrib1sv",
                &["glVertexAttrib1svARB", "glVertexAttrib1svNV"],
            )),
            VertexAttrib2d: FnPtr::new(metaloadfn(
                "glVertexAttrib2d",
                &["glVertexAttrib2dARB", "glVertexAttrib2dNV"],
            )),
            VertexAttrib2dv: FnPtr::new(metaloadfn(
                "glVertexAttrib2dv",
                &["glVertexAttrib2dvARB", "glVertexAttrib2dvNV"],
            )),
            VertexAttrib2f: FnPtr::new(metaloadfn(
                "glVertexAttrib2f",
                &["glVertexAttrib2fARB", "glVertexAttrib2fNV"],
            )),
            VertexAttrib2fv: FnPtr::new(metaloadfn(
                "glVertexAttrib2fv",
                &["glVertexAttrib2fvARB", "glVertexAttrib2fvNV"],
            )),
            VertexAttrib2s: FnPtr::new(metaloadfn(
                "glVertexAttrib2s",
                &["glVertexAttrib2sARB", "glVertexAttrib2sNV"],
            )),
            VertexAttrib2sv: FnPtr::new(metaloadfn(
                "glVertexAttrib2sv",
                &["glVertexAttrib2svARB", "glVertexAttrib2svNV"],
            )),
            VertexAttrib3d: FnPtr::new(metaloadfn(
                "glVertexAttrib3d",
                &["glVertexAttrib3dARB", "glVertexAttrib3dNV"],
            )),
            VertexAttrib3dv: FnPtr::new(metaloadfn(
                "glVertexAttrib3dv",
                &["glVertexAttrib3dvARB", "glVertexAttrib3dvNV"],
            )),
            VertexAttrib3f: FnPtr::new(metaloadfn(
                "glVertexAttrib3f",
                &["glVertexAttrib3fARB", "glVertexAttrib3fNV"],
            )),
            VertexAttrib3fv: FnPtr::new(metaloadfn(
                "glVertexAttrib3fv",
                &["glVertexAttrib3fvARB", "glVertexAttrib3fvNV"],
            )),
            VertexAttrib3s: FnPtr::new(metaloadfn(
                "glVertexAttrib3s",
                &["glVertexAttrib3sARB", "glVertexAttrib3sNV"],
            )),
            VertexAttrib3sv: FnPtr::new(metaloadfn(
                "glVertexAttrib3sv",
                &["glVertexAttrib3svARB", "glVertexAttrib3svNV"],
            )),
            VertexAttrib4Nbv: FnPtr::new(metaloadfn(
                "glVertexAttrib4Nbv",
                &["glVertexAttrib4NbvARB"],
            )),
            VertexAttrib4Niv: FnPtr::new(metaloadfn(
                "glVertexAttrib4Niv",
                &["glVertexAttrib4NivARB"],
            )),
            VertexAttrib4Nsv: FnPtr::new(metaloadfn(
                "glVertexAttrib4Nsv",
                &["glVertexAttrib4NsvARB"],
            )),
            VertexAttrib4Nub: FnPtr::new(metaloadfn(
                "glVertexAttrib4Nub",
                &["glVertexAttrib4NubARB", "glVertexAttrib4ubNV"],
            )),
            VertexAttrib4Nubv: FnPtr::new(metaloadfn(
                "glVertexAttrib4Nubv",
                &["glVertexAttrib4NubvARB", "glVertexAttrib4ubvNV"],
            )),
            VertexAttrib4Nuiv: FnPtr::new(metaloadfn(
                "glVertexAttrib4Nuiv",
                &["glVertexAttrib4NuivARB"],
            )),
            VertexAttrib4Nusv: FnPtr::new(metaloadfn(
                "glVertexAttrib4Nusv",
                &["glVertexAttrib4NusvARB"],
            )),
            VertexAttrib4bv: FnPtr::new(metaloadfn("glVertexAttrib4bv", &["glVertexAttrib4bvARB"])),
            VertexAttrib4d: FnPtr::new(metaloadfn(
                "glVertexAttrib4d",
                &["glVertexAttrib4dARB", "glVertexAttrib4dNV"],
            )),
            VertexAttrib4dv: FnPtr::new(metaloadfn(
                "glVertexAttrib4dv",
                &["glVertexAttrib4dvARB", "glVertexAttrib4dvNV"],
            )),
            VertexAttrib4f: FnPtr::new(metaloadfn(
                "glVertexAttrib4f",
                &["glVertexAttrib4fARB", "glVertexAttrib4fNV"],
            )),
            VertexAttrib4fv: FnPtr::new(metaloadfn(
                "glVertexAttrib4fv",
                &["glVertexAttrib4fvARB", "glVertexAttrib4fvNV"],
            )),
            VertexAttrib4iv: FnPtr::new(metaloadfn("glVertexAttrib4iv", &["glVertexAttrib4ivARB"])),
            VertexAttrib4s: FnPtr::new(metaloadfn(
                "glVertexAttrib4s",
                &["glVertexAttrib4sARB", "glVertexAttrib4sNV"],
            )),
            VertexAttrib4sv: FnPtr::new(metaloadfn(
                "glVertexAttrib4sv",
                &["glVertexAttrib4svARB", "glVertexAttrib4svNV"],
            )),
            VertexAttrib4ubv: FnPtr::new(metaloadfn(
                "glVertexAttrib4ubv",
                &["glVertexAttrib4ubvARB"],
            )),
            VertexAttrib4uiv: FnPtr::new(metaloadfn(
                "glVertexAttrib4uiv",
                &["glVertexAttrib4uivARB"],
            )),
            VertexAttrib4usv: FnPtr::new(metaloadfn(
                "glVertexAttrib4usv",
                &["glVertexAttrib4usvARB"],
            )),
            VertexAttribDivisor: FnPtr::new(metaloadfn(
                "glVertexAttribDivisor",
                &[
                    "glVertexAttribDivisorANGLE",
                    "glVertexAttribDivisorARB",
                    "glVertexAttribDivisorEXT",
                    "glVertexAttribDivisorNV",
                ],
            )),
            VertexAttribI1i: FnPtr::new(metaloadfn("glVertexAttribI1i", &["glVertexAttribI1iEXT"])),
            VertexAttribI1iv: FnPtr::new(metaloadfn(
                "glVertexAttribI1iv",
                &["glVertexAttribI1ivEXT"],
            )),
            VertexAttribI1ui: FnPtr::new(metaloadfn(
                "glVertexAttribI1ui",
                &["glVertexAttribI1uiEXT"],
            )),
            VertexAttribI1uiv: FnPtr::new(metaloadfn(
                "glVertexAttribI1uiv",
                &["glVertexAttribI1uivEXT"],
            )),
            VertexAttribI2i: FnPtr::new(metaloadfn("glVertexAttribI2i", &["glVertexAttribI2iEXT"])),
            VertexAttribI2iv: FnPtr::new(metaloadfn(
                "glVertexAttribI2iv",
                &["glVertexAttribI2ivEXT"],
            )),
            VertexAttribI2ui: FnPtr::new(metaloadfn(
                "glVertexAttribI2ui",
                &["glVertexAttribI2uiEXT"],
            )),
            VertexAttribI2uiv: FnPtr::new(metaloadfn(
                "glVertexAttribI2uiv",
                &["glVertexAttribI2uivEXT"],
            )),
            VertexAttribI3i: FnPtr::new(metaloadfn("glVertexAttribI3i", &["glVertexAttribI3iEXT"])),
            VertexAttribI3iv: FnPtr::new(metaloadfn(
                "glVertexAttribI3iv",
                &["glVertexAttribI3ivEXT"],
            )),
            VertexAttribI3ui: FnPtr::new(metaloadfn(
                "glVertexAttribI3ui",
                &["glVertexAttribI3uiEXT"],
            )),
            VertexAttribI3uiv: FnPtr::new(metaloadfn(
                "glVertexAttribI3uiv",
                &["glVertexAttribI3uivEXT"],
            )),
            VertexAttribI4bv: FnPtr::new(metaloadfn(
                "glVertexAttribI4bv",
                &["glVertexAttribI4bvEXT"],
            )),
            VertexAttribI4i: FnPtr::new(metaloadfn("glVertexAttribI4i", &["glVertexAttribI4iEXT"])),
            VertexAttribI4iv: FnPtr::new(metaloadfn(
                "glVertexAttribI4iv",
                &["glVertexAttribI4ivEXT"],
            )),
            VertexAttribI4sv: FnPtr::new(metaloadfn(
                "glVertexAttribI4sv",
                &["glVertexAttribI4svEXT"],
            )),
            VertexAttribI4ubv: FnPtr::new(metaloadfn(
                "glVertexAttribI4ubv",
                &["glVertexAttribI4ubvEXT"],
            )),
            VertexAttribI4ui: FnPtr::new(metaloadfn(
                "glVertexAttribI4ui",
                &["glVertexAttribI4uiEXT"],
            )),
            VertexAttribI4uiv: FnPtr::new(metaloadfn(
                "glVertexAttribI4uiv",
                &["glVertexAttribI4uivEXT"],
            )),
            VertexAttribI4usv: FnPtr::new(metaloadfn(
                "glVertexAttribI4usv",
                &["glVertexAttribI4usvEXT"],
            )),
            VertexAttribIPointer: FnPtr::new(metaloadfn(
                "glVertexAttribIPointer",
                &["glVertexAttribIPointerEXT"],
            )),
            VertexAttribP1ui: FnPtr::new(metaloadfn("glVertexAttribP1ui", &[])),
            VertexAttribP1uiv: FnPtr::new(metaloadfn("glVertexAttribP1uiv", &[])),
            VertexAttribP2ui: FnPtr::new(metaloadfn("glVertexAttribP2ui", &[])),
            VertexAttribP2uiv: FnPtr::new(metaloadfn("glVertexAttribP2uiv", &[])),
            VertexAttribP3ui: FnPtr::new(metaloadfn("glVertexAttribP3ui", &[])),
            VertexAttribP3uiv: FnPtr::new(metaloadfn("glVertexAttribP3uiv", &[])),
            VertexAttribP4ui: FnPtr::new(metaloadfn("glVertexAttribP4ui", &[])),
            VertexAttribP4uiv: FnPtr::new(metaloadfn("glVertexAttribP4uiv", &[])),
            VertexAttribPointer: FnPtr::new(metaloadfn(
                "glVertexAttribPointer",
                &["glVertexAttribPointerARB"],
            )),
            VertexP2ui: FnPtr::new(metaloadfn("glVertexP2ui", &[])),
            VertexP2uiv: FnPtr::new(metaloadfn("glVertexP2uiv", &[])),
            VertexP3ui: FnPtr::new(metaloadfn("glVertexP3ui", &[])),
            VertexP3uiv: FnPtr::new(metaloadfn("glVertexP3uiv", &[])),
            VertexP4ui: FnPtr::new(metaloadfn("glVertexP4ui", &[])),
            VertexP4uiv: FnPtr::new(metaloadfn("glVertexP4uiv", &[])),
            Viewport: FnPtr::new(metaloadfn("glViewport", &[])),
            WaitSync: FnPtr::new(metaloadfn("glWaitSync", &["glWaitSyncAPPLE"])),
            _priv: (),
        }
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ActiveTexture(&self, texture: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(
            self.ActiveTexture.f,
        )(texture)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn AttachShader(&self, program: types::GLuint, shader: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(
            self.AttachShader.f,
        )(program, shader)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BeginConditionalRender(&self, id: types::GLuint, mode: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(
            self.BeginConditionalRender.f,
        )(id, mode)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BeginQuery(&self, target: types::GLenum, id: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.BeginQuery.f,
        )(target, id)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BeginTransformFeedback(&self, primitiveMode: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(
            self.BeginTransformFeedback.f,
        )(primitiveMode)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BindAttribLocation(
        &self,
        program: types::GLuint,
        index: types::GLuint,
        name: *const types::GLchar,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLuint, *const types::GLchar) -> (),
        >(self.BindAttribLocation.f)(program, index, name)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BindBuffer(&self, target: types::GLenum, buffer: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.BindBuffer.f,
        )(target, buffer)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BindBufferBase(
        &self,
        target: types::GLenum,
        index: types::GLuint,
        buffer: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLuint, types::GLuint) -> (),
        >(self.BindBufferBase.f)(target, index, buffer)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BindBufferRange(
        &self,
        target: types::GLenum,
        index: types::GLuint,
        buffer: types::GLuint,
        offset: types::GLintptr,
        size: types::GLsizeiptr,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLuint,
                types::GLuint,
                types::GLintptr,
                types::GLsizeiptr,
            ) -> (),
        >(self.BindBufferRange.f)(target, index, buffer, offset, size)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BindFragDataLocation(
        &self,
        program: types::GLuint,
        color: types::GLuint,
        name: *const types::GLchar,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLuint, *const types::GLchar) -> (),
        >(self.BindFragDataLocation.f)(program, color, name)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BindFragDataLocationIndexed(
        &self,
        program: types::GLuint,
        colorNumber: types::GLuint,
        index: types::GLuint,
        name: *const types::GLchar,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLuint,
                types::GLuint,
                *const types::GLchar,
            ) -> (),
        >(self.BindFragDataLocationIndexed.f)(program, colorNumber, index, name)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BindFramebuffer(&self, target: types::GLenum, framebuffer: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.BindFramebuffer.f,
        )(target, framebuffer)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BindRenderbuffer(
        &self,
        target: types::GLenum,
        renderbuffer: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.BindRenderbuffer.f,
        )(target, renderbuffer)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BindSampler(&self, unit: types::GLuint, sampler: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(
            self.BindSampler.f,
        )(unit, sampler)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BindTexture(&self, target: types::GLenum, texture: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.BindTexture.f,
        )(target, texture)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BindVertexArray(&self, array: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.BindVertexArray.f,
        )(array)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BlendColor(
        &self,
        red: types::GLfloat,
        green: types::GLfloat,
        blue: types::GLfloat,
        alpha: types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLfloat,
                types::GLfloat,
                types::GLfloat,
                types::GLfloat,
            ) -> (),
        >(self.BlendColor.f)(red, green, blue, alpha)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BlendEquation(&self, mode: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(
            self.BlendEquation.f,
        )(mode)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BlendEquationSeparate(
        &self,
        modeRGB: types::GLenum,
        modeAlpha: types::GLenum,
    ) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(
            self.BlendEquationSeparate.f,
        )(modeRGB, modeAlpha)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BlendFunc(&self, sfactor: types::GLenum, dfactor: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(
            self.BlendFunc.f,
        )(sfactor, dfactor)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BlendFuncSeparate(
        &self,
        sfactorRGB: types::GLenum,
        dfactorRGB: types::GLenum,
        sfactorAlpha: types::GLenum,
        dfactorAlpha: types::GLenum,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> (),
        >(self.BlendFuncSeparate.f)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BlitFramebuffer(
        &self,
        srcX0: types::GLint,
        srcY0: types::GLint,
        srcX1: types::GLint,
        srcY1: types::GLint,
        dstX0: types::GLint,
        dstY0: types::GLint,
        dstX1: types::GLint,
        dstY1: types::GLint,
        mask: types::GLbitfield,
        filter: types::GLenum,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLbitfield,
                types::GLenum,
            ) -> (),
        >(self.BlitFramebuffer.f)(
            srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BufferData(
        &self,
        target: types::GLenum,
        size: types::GLsizeiptr,
        data: *const __gl_imports::raw::c_void,
        usage: types::GLenum,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLsizeiptr,
                *const __gl_imports::raw::c_void,
                types::GLenum,
            ) -> (),
        >(self.BufferData.f)(target, size, data, usage)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn BufferSubData(
        &self,
        target: types::GLenum,
        offset: types::GLintptr,
        size: types::GLsizeiptr,
        data: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLintptr,
                types::GLsizeiptr,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.BufferSubData.f)(target, offset, size, data)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CallCommandListNV(&self, list: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.CallCommandListNV.f,
        )(list)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CheckFramebufferStatus(&self, target: types::GLenum) -> types::GLenum {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLenum>(
            self.CheckFramebufferStatus.f,
        )(target)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ClampColor(&self, target: types::GLenum, clamp: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(
            self.ClampColor.f,
        )(target, clamp)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Clear(&self, mask: types::GLbitfield) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(self.Clear.f)(
            mask,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ClearBufferfi(
        &self,
        buffer: types::GLenum,
        drawbuffer: types::GLint,
        depth: types::GLfloat,
        stencil: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLint, types::GLfloat, types::GLint) -> (),
        >(self.ClearBufferfi.f)(buffer, drawbuffer, depth, stencil)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ClearBufferfv(
        &self,
        buffer: types::GLenum,
        drawbuffer: types::GLint,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLint, *const types::GLfloat) -> (),
        >(self.ClearBufferfv.f)(buffer, drawbuffer, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ClearBufferiv(
        &self,
        buffer: types::GLenum,
        drawbuffer: types::GLint,
        value: *const types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLint, *const types::GLint) -> (),
        >(self.ClearBufferiv.f)(buffer, drawbuffer, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ClearBufferuiv(
        &self,
        buffer: types::GLenum,
        drawbuffer: types::GLint,
        value: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLint, *const types::GLuint) -> (),
        >(self.ClearBufferuiv.f)(buffer, drawbuffer, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ClearColor(
        &self,
        red: types::GLfloat,
        green: types::GLfloat,
        blue: types::GLfloat,
        alpha: types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLfloat,
                types::GLfloat,
                types::GLfloat,
                types::GLfloat,
            ) -> (),
        >(self.ClearColor.f)(red, green, blue, alpha)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ClearDepth(&self, depth: types::GLdouble) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble) -> ()>(
            self.ClearDepth.f,
        )(depth)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ClearStencil(&self, s: types::GLint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(
            self.ClearStencil.f,
        )(s)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ClientWaitSync(
        &self,
        sync: types::GLsync,
        flags: types::GLbitfield,
        timeout: types::GLuint64,
    ) -> types::GLenum {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsync, types::GLbitfield, types::GLuint64) -> types::GLenum,
        >(self.ClientWaitSync.f)(sync, flags, timeout)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ColorMask(
        &self,
        red: types::GLboolean,
        green: types::GLboolean,
        blue: types::GLboolean,
        alpha: types::GLboolean,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLboolean,
                types::GLboolean,
                types::GLboolean,
                types::GLboolean,
            ) -> (),
        >(self.ColorMask.f)(red, green, blue, alpha)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ColorMaski(
        &self,
        index: types::GLuint,
        r: types::GLboolean,
        g: types::GLboolean,
        b: types::GLboolean,
        a: types::GLboolean,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLboolean,
                types::GLboolean,
                types::GLboolean,
                types::GLboolean,
            ) -> (),
        >(self.ColorMaski.f)(index, r, g, b, a)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ColorP3ui(&self, type_: types::GLenum, color: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.ColorP3ui.f,
        )(type_, color)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ColorP3uiv(&self, type_: types::GLenum, color: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLuint) -> (),
        >(self.ColorP3uiv.f)(type_, color)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ColorP4ui(&self, type_: types::GLenum, color: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.ColorP4ui.f,
        )(type_, color)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ColorP4uiv(&self, type_: types::GLenum, color: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLuint) -> (),
        >(self.ColorP4uiv.f)(type_, color)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CommandListSegmentsNV(&self, list: types::GLuint, segments: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(
            self.CommandListSegmentsNV.f,
        )(list, segments)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CompileCommandListNV(&self, list: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.CompileCommandListNV.f,
        )(list)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CompileShader(&self, shader: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.CompileShader.f,
        )(shader)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CompressedTexImage1D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        internalformat: types::GLenum,
        width: types::GLsizei,
        border: types::GLint,
        imageSize: types::GLsizei,
        data: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLenum,
                types::GLsizei,
                types::GLint,
                types::GLsizei,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.CompressedTexImage1D.f)(
            target,
            level,
            internalformat,
            width,
            border,
            imageSize,
            data,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CompressedTexImage2D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        internalformat: types::GLenum,
        width: types::GLsizei,
        height: types::GLsizei,
        border: types::GLint,
        imageSize: types::GLsizei,
        data: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLenum,
                types::GLsizei,
                types::GLsizei,
                types::GLint,
                types::GLsizei,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.CompressedTexImage2D.f)(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            imageSize,
            data,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CompressedTexImage3D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        internalformat: types::GLenum,
        width: types::GLsizei,
        height: types::GLsizei,
        depth: types::GLsizei,
        border: types::GLint,
        imageSize: types::GLsizei,
        data: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLenum,
                types::GLsizei,
                types::GLsizei,
                types::GLsizei,
                types::GLint,
                types::GLsizei,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.CompressedTexImage3D.f)(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            imageSize,
            data,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CompressedTexSubImage1D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        xoffset: types::GLint,
        width: types::GLsizei,
        format: types::GLenum,
        imageSize: types::GLsizei,
        data: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLenum,
                types::GLsizei,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.CompressedTexSubImage1D.f)(
            target, level, xoffset, width, format, imageSize, data
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CompressedTexSubImage2D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        xoffset: types::GLint,
        yoffset: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
        format: types::GLenum,
        imageSize: types::GLsizei,
        data: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLsizei,
                types::GLenum,
                types::GLsizei,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.CompressedTexSubImage2D.f)(
            target, level, xoffset, yoffset, width, height, format, imageSize, data,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CompressedTexSubImage3D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        xoffset: types::GLint,
        yoffset: types::GLint,
        zoffset: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
        depth: types::GLsizei,
        format: types::GLenum,
        imageSize: types::GLsizei,
        data: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLsizei,
                types::GLsizei,
                types::GLenum,
                types::GLsizei,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.CompressedTexSubImage3D.f)(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CopyBufferSubData(
        &self,
        readTarget: types::GLenum,
        writeTarget: types::GLenum,
        readOffset: types::GLintptr,
        writeOffset: types::GLintptr,
        size: types::GLsizeiptr,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLenum,
                types::GLintptr,
                types::GLintptr,
                types::GLsizeiptr,
            ) -> (),
        >(self.CopyBufferSubData.f)(readTarget, writeTarget, readOffset, writeOffset, size)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CopyTexImage1D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        internalformat: types::GLenum,
        x: types::GLint,
        y: types::GLint,
        width: types::GLsizei,
        border: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLint,
            ) -> (),
        >(self.CopyTexImage1D.f)(target, level, internalformat, x, y, width, border)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CopyTexImage2D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        internalformat: types::GLenum,
        x: types::GLint,
        y: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
        border: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLsizei,
                types::GLint,
            ) -> (),
        >(self.CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CopyTexSubImage1D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        xoffset: types::GLint,
        x: types::GLint,
        y: types::GLint,
        width: types::GLsizei,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLsizei,
            ) -> (),
        >(self.CopyTexSubImage1D.f)(target, level, xoffset, x, y, width)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CopyTexSubImage2D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        xoffset: types::GLint,
        yoffset: types::GLint,
        x: types::GLint,
        y: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLsizei,
            ) -> (),
        >(self.CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CopyTexSubImage3D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        xoffset: types::GLint,
        yoffset: types::GLint,
        zoffset: types::GLint,
        x: types::GLint,
        y: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLsizei,
            ) -> (),
        >(self.CopyTexSubImage3D.f)(
            target, level, xoffset, yoffset, zoffset, x, y, width, height,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CreateCommandListsNV(&self, n: types::GLsizei, lists: *mut types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *mut types::GLuint) -> (),
        >(self.CreateCommandListsNV.f)(n, lists)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CreateProgram(&self) -> types::GLuint {
        __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLuint>(
            self.CreateProgram.f,
        )()
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CreateShader(&self, type_: types::GLenum) -> types::GLuint {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLuint>(
            self.CreateShader.f,
        )(type_)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CreateStatesNV(&self, n: types::GLsizei, states: *mut types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *mut types::GLuint) -> (),
        >(self.CreateStatesNV.f)(n, states)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CullFace(&self, mode: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.CullFace.f)(
            mode,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DeleteBuffers(&self, n: types::GLsizei, buffers: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *const types::GLuint) -> (),
        >(self.DeleteBuffers.f)(n, buffers)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DeleteCommandListsNV(
        &self,
        n: types::GLsizei,
        lists: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *const types::GLuint) -> (),
        >(self.DeleteCommandListsNV.f)(n, lists)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DeleteFramebuffers(
        &self,
        n: types::GLsizei,
        framebuffers: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *const types::GLuint) -> (),
        >(self.DeleteFramebuffers.f)(n, framebuffers)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DeleteProgram(&self, program: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.DeleteProgram.f,
        )(program)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DeleteQueries(&self, n: types::GLsizei, ids: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *const types::GLuint) -> (),
        >(self.DeleteQueries.f)(n, ids)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DeleteRenderbuffers(
        &self,
        n: types::GLsizei,
        renderbuffers: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *const types::GLuint) -> (),
        >(self.DeleteRenderbuffers.f)(n, renderbuffers)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DeleteSamplers(
        &self,
        count: types::GLsizei,
        samplers: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *const types::GLuint) -> (),
        >(self.DeleteSamplers.f)(count, samplers)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DeleteShader(&self, shader: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.DeleteShader.f,
        )(shader)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DeleteStatesNV(&self, n: types::GLsizei, states: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *const types::GLuint) -> (),
        >(self.DeleteStatesNV.f)(n, states)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DeleteSync(&self, sync: types::GLsync) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync) -> ()>(
            self.DeleteSync.f,
        )(sync)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DeleteTextures(&self, n: types::GLsizei, textures: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *const types::GLuint) -> (),
        >(self.DeleteTextures.f)(n, textures)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DeleteVertexArrays(&self, n: types::GLsizei, arrays: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *const types::GLuint) -> (),
        >(self.DeleteVertexArrays.f)(n, arrays)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DepthFunc(&self, func: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.DepthFunc.f)(
            func,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DepthMask(&self, flag: types::GLboolean) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean) -> ()>(
            self.DepthMask.f,
        )(flag)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DepthRange(&self, n: types::GLdouble, f: types::GLdouble) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(
            self.DepthRange.f,
        )(n, f)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DetachShader(&self, program: types::GLuint, shader: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(
            self.DetachShader.f,
        )(program, shader)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Disable(&self, cap: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.Disable.f)(
            cap,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DisableVertexAttribArray(&self, index: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.DisableVertexAttribArray.f,
        )(index)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Disablei(&self, target: types::GLenum, index: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.Disablei.f,
        )(target, index)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawArrays(
        &self,
        mode: types::GLenum,
        first: types::GLint,
        count: types::GLsizei,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLint, types::GLsizei) -> (),
        >(self.DrawArrays.f)(mode, first, count)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawArraysInstanced(
        &self,
        mode: types::GLenum,
        first: types::GLint,
        count: types::GLsizei,
        instancecount: types::GLsizei,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLint, types::GLsizei, types::GLsizei) -> (),
        >(self.DrawArraysInstanced.f)(mode, first, count, instancecount)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawBuffer(&self, buf: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(
            self.DrawBuffer.f,
        )(buf)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawBuffers(&self, n: types::GLsizei, bufs: *const types::GLenum) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *const types::GLenum) -> (),
        >(self.DrawBuffers.f)(n, bufs)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawCommandsAddressNV(
        &self,
        primitiveMode: types::GLenum,
        indirects: *const types::GLuint64,
        sizes: *const types::GLsizei,
        count: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                *const types::GLuint64,
                *const types::GLsizei,
                types::GLuint,
            ) -> (),
        >(self.DrawCommandsAddressNV.f)(primitiveMode, indirects, sizes, count)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawCommandsNV(
        &self,
        primitiveMode: types::GLenum,
        buffer: types::GLuint,
        indirects: *const types::GLintptr,
        sizes: *const types::GLsizei,
        count: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLuint,
                *const types::GLintptr,
                *const types::GLsizei,
                types::GLuint,
            ) -> (),
        >(self.DrawCommandsNV.f)(primitiveMode, buffer, indirects, sizes, count)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawCommandsStatesAddressNV(
        &self,
        indirects: *const types::GLuint64,
        sizes: *const types::GLsizei,
        states: *const types::GLuint,
        fbos: *const types::GLuint,
        count: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                *const types::GLuint64,
                *const types::GLsizei,
                *const types::GLuint,
                *const types::GLuint,
                types::GLuint,
            ) -> (),
        >(self.DrawCommandsStatesAddressNV.f)(indirects, sizes, states, fbos, count)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawCommandsStatesNV(
        &self,
        buffer: types::GLuint,
        indirects: *const types::GLintptr,
        sizes: *const types::GLsizei,
        states: *const types::GLuint,
        fbos: *const types::GLuint,
        count: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                *const types::GLintptr,
                *const types::GLsizei,
                *const types::GLuint,
                *const types::GLuint,
                types::GLuint,
            ) -> (),
        >(self.DrawCommandsStatesNV.f)(buffer, indirects, sizes, states, fbos, count)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawElements(
        &self,
        mode: types::GLenum,
        count: types::GLsizei,
        type_: types::GLenum,
        indices: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLsizei,
                types::GLenum,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.DrawElements.f)(mode, count, type_, indices)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawElementsBaseVertex(
        &self,
        mode: types::GLenum,
        count: types::GLsizei,
        type_: types::GLenum,
        indices: *const __gl_imports::raw::c_void,
        basevertex: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLsizei,
                types::GLenum,
                *const __gl_imports::raw::c_void,
                types::GLint,
            ) -> (),
        >(self.DrawElementsBaseVertex.f)(mode, count, type_, indices, basevertex)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawElementsInstanced(
        &self,
        mode: types::GLenum,
        count: types::GLsizei,
        type_: types::GLenum,
        indices: *const __gl_imports::raw::c_void,
        instancecount: types::GLsizei,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLsizei,
                types::GLenum,
                *const __gl_imports::raw::c_void,
                types::GLsizei,
            ) -> (),
        >(self.DrawElementsInstanced.f)(mode, count, type_, indices, instancecount)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawElementsInstancedBaseVertex(
        &self,
        mode: types::GLenum,
        count: types::GLsizei,
        type_: types::GLenum,
        indices: *const __gl_imports::raw::c_void,
        instancecount: types::GLsizei,
        basevertex: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLsizei,
                types::GLenum,
                *const __gl_imports::raw::c_void,
                types::GLsizei,
                types::GLint,
            ) -> (),
        >(self.DrawElementsInstancedBaseVertex.f)(
            mode,
            count,
            type_,
            indices,
            instancecount,
            basevertex,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawRangeElements(
        &self,
        mode: types::GLenum,
        start: types::GLuint,
        end: types::GLuint,
        count: types::GLsizei,
        type_: types::GLenum,
        indices: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLuint,
                types::GLuint,
                types::GLsizei,
                types::GLenum,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.DrawRangeElements.f)(mode, start, end, count, type_, indices)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DrawRangeElementsBaseVertex(
        &self,
        mode: types::GLenum,
        start: types::GLuint,
        end: types::GLuint,
        count: types::GLsizei,
        type_: types::GLenum,
        indices: *const __gl_imports::raw::c_void,
        basevertex: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLuint,
                types::GLuint,
                types::GLsizei,
                types::GLenum,
                *const __gl_imports::raw::c_void,
                types::GLint,
            ) -> (),
        >(self.DrawRangeElementsBaseVertex.f)(
            mode, start, end, count, type_, indices, basevertex
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Enable(&self, cap: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.Enable.f)(
            cap,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn EnableVertexAttribArray(&self, index: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.EnableVertexAttribArray.f,
        )(index)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Enablei(&self, target: types::GLenum, index: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.Enablei.f,
        )(target, index)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn EndConditionalRender(&self) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.EndConditionalRender.f)()
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn EndQuery(&self, target: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.EndQuery.f)(
            target,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn EndTransformFeedback(&self) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.EndTransformFeedback.f)()
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn FenceSync(
        &self,
        condition: types::GLenum,
        flags: types::GLbitfield,
    ) -> types::GLsync {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLbitfield) -> types::GLsync,
        >(self.FenceSync.f)(condition, flags)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Finish(&self) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.Finish.f)()
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Flush(&self) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.Flush.f)()
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn FlushMappedBufferRange(
        &self,
        target: types::GLenum,
        offset: types::GLintptr,
        length: types::GLsizeiptr,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr) -> (),
        >(self.FlushMappedBufferRange.f)(target, offset, length)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn FramebufferRenderbuffer(
        &self,
        target: types::GLenum,
        attachment: types::GLenum,
        renderbuffertarget: types::GLenum,
        renderbuffer: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint) -> (),
        >(self.FramebufferRenderbuffer.f)(
            target, attachment, renderbuffertarget, renderbuffer
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn FramebufferTexture(
        &self,
        target: types::GLenum,
        attachment: types::GLenum,
        texture: types::GLuint,
        level: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLint) -> (),
        >(self.FramebufferTexture.f)(target, attachment, texture, level)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn FramebufferTexture1D(
        &self,
        target: types::GLenum,
        attachment: types::GLenum,
        textarget: types::GLenum,
        texture: types::GLuint,
        level: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLenum,
                types::GLenum,
                types::GLuint,
                types::GLint,
            ) -> (),
        >(self.FramebufferTexture1D.f)(target, attachment, textarget, texture, level)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn FramebufferTexture2D(
        &self,
        target: types::GLenum,
        attachment: types::GLenum,
        textarget: types::GLenum,
        texture: types::GLuint,
        level: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLenum,
                types::GLenum,
                types::GLuint,
                types::GLint,
            ) -> (),
        >(self.FramebufferTexture2D.f)(target, attachment, textarget, texture, level)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn FramebufferTexture3D(
        &self,
        target: types::GLenum,
        attachment: types::GLenum,
        textarget: types::GLenum,
        texture: types::GLuint,
        level: types::GLint,
        zoffset: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLenum,
                types::GLenum,
                types::GLuint,
                types::GLint,
                types::GLint,
            ) -> (),
        >(self.FramebufferTexture3D.f)(
            target, attachment, textarget, texture, level, zoffset
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn FramebufferTextureLayer(
        &self,
        target: types::GLenum,
        attachment: types::GLenum,
        texture: types::GLuint,
        level: types::GLint,
        layer: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLenum,
                types::GLuint,
                types::GLint,
                types::GLint,
            ) -> (),
        >(self.FramebufferTextureLayer.f)(target, attachment, texture, level, layer)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn FrontFace(&self, mode: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.FrontFace.f)(
            mode,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GenBuffers(&self, n: types::GLsizei, buffers: *mut types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *mut types::GLuint) -> (),
        >(self.GenBuffers.f)(n, buffers)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GenFramebuffers(
        &self,
        n: types::GLsizei,
        framebuffers: *mut types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *mut types::GLuint) -> (),
        >(self.GenFramebuffers.f)(n, framebuffers)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GenQueries(&self, n: types::GLsizei, ids: *mut types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *mut types::GLuint) -> (),
        >(self.GenQueries.f)(n, ids)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GenRenderbuffers(
        &self,
        n: types::GLsizei,
        renderbuffers: *mut types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *mut types::GLuint) -> (),
        >(self.GenRenderbuffers.f)(n, renderbuffers)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GenSamplers(&self, count: types::GLsizei, samplers: *mut types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *mut types::GLuint) -> (),
        >(self.GenSamplers.f)(count, samplers)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GenTextures(&self, n: types::GLsizei, textures: *mut types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *mut types::GLuint) -> (),
        >(self.GenTextures.f)(n, textures)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GenVertexArrays(&self, n: types::GLsizei, arrays: *mut types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsizei, *mut types::GLuint) -> (),
        >(self.GenVertexArrays.f)(n, arrays)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GenerateMipmap(&self, target: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(
            self.GenerateMipmap.f,
        )(target)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetActiveAttrib(
        &self,
        program: types::GLuint,
        index: types::GLuint,
        bufSize: types::GLsizei,
        length: *mut types::GLsizei,
        size: *mut types::GLint,
        type_: *mut types::GLenum,
        name: *mut types::GLchar,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLuint,
                types::GLsizei,
                *mut types::GLsizei,
                *mut types::GLint,
                *mut types::GLenum,
                *mut types::GLchar,
            ) -> (),
        >(self.GetActiveAttrib.f)(program, index, bufSize, length, size, type_, name)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetActiveUniform(
        &self,
        program: types::GLuint,
        index: types::GLuint,
        bufSize: types::GLsizei,
        length: *mut types::GLsizei,
        size: *mut types::GLint,
        type_: *mut types::GLenum,
        name: *mut types::GLchar,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLuint,
                types::GLsizei,
                *mut types::GLsizei,
                *mut types::GLint,
                *mut types::GLenum,
                *mut types::GLchar,
            ) -> (),
        >(self.GetActiveUniform.f)(program, index, bufSize, length, size, type_, name)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetActiveUniformBlockName(
        &self,
        program: types::GLuint,
        uniformBlockIndex: types::GLuint,
        bufSize: types::GLsizei,
        length: *mut types::GLsizei,
        uniformBlockName: *mut types::GLchar,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLuint,
                types::GLsizei,
                *mut types::GLsizei,
                *mut types::GLchar,
            ) -> (),
        >(self.GetActiveUniformBlockName.f)(
            program,
            uniformBlockIndex,
            bufSize,
            length,
            uniformBlockName,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetActiveUniformBlockiv(
        &self,
        program: types::GLuint,
        uniformBlockIndex: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLuint,
                types::GLenum,
                *mut types::GLint,
            ) -> (),
        >(self.GetActiveUniformBlockiv.f)(program, uniformBlockIndex, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetActiveUniformName(
        &self,
        program: types::GLuint,
        uniformIndex: types::GLuint,
        bufSize: types::GLsizei,
        length: *mut types::GLsizei,
        uniformName: *mut types::GLchar,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLuint,
                types::GLsizei,
                *mut types::GLsizei,
                *mut types::GLchar,
            ) -> (),
        >(self.GetActiveUniformName.f)(program, uniformIndex, bufSize, length, uniformName)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetActiveUniformsiv(
        &self,
        program: types::GLuint,
        uniformCount: types::GLsizei,
        uniformIndices: *const types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLsizei,
                *const types::GLuint,
                types::GLenum,
                *mut types::GLint,
            ) -> (),
        >(self.GetActiveUniformsiv.f)(program, uniformCount, uniformIndices, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetAttachedShaders(
        &self,
        program: types::GLuint,
        maxCount: types::GLsizei,
        count: *mut types::GLsizei,
        shaders: *mut types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLsizei,
                *mut types::GLsizei,
                *mut types::GLuint,
            ) -> (),
        >(self.GetAttachedShaders.f)(program, maxCount, count, shaders)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetAttribLocation(
        &self,
        program: types::GLuint,
        name: *const types::GLchar,
    ) -> types::GLint {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint,
        >(self.GetAttribLocation.f)(program, name)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetBooleani_v(
        &self,
        target: types::GLenum,
        index: types::GLuint,
        data: *mut types::GLboolean,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLuint, *mut types::GLboolean) -> (),
        >(self.GetBooleani_v.f)(target, index, data)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetBooleanv(&self, pname: types::GLenum, data: *mut types::GLboolean) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *mut types::GLboolean) -> (),
        >(self.GetBooleanv.f)(pname, data)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetBufferParameteri64v(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *mut types::GLint64,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint64) -> (),
        >(self.GetBufferParameteri64v.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetBufferParameteriv(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> (),
        >(self.GetBufferParameteriv.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetBufferPointerv(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *const *mut __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLenum,
                *const *mut __gl_imports::raw::c_void,
            ) -> (),
        >(self.GetBufferPointerv.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetBufferSubData(
        &self,
        target: types::GLenum,
        offset: types::GLintptr,
        size: types::GLsizeiptr,
        data: *mut __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLintptr,
                types::GLsizeiptr,
                *mut __gl_imports::raw::c_void,
            ) -> (),
        >(self.GetBufferSubData.f)(target, offset, size, data)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetCommandHeaderNV(
        &self,
        tokenID: types::GLenum,
        size: types::GLuint,
    ) -> types::GLuint {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLuint) -> types::GLuint,
        >(self.GetCommandHeaderNV.f)(tokenID, size)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetCompressedTexImage(
        &self,
        target: types::GLenum,
        level: types::GLint,
        img: *mut __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLint, *mut __gl_imports::raw::c_void) -> (),
        >(self.GetCompressedTexImage.f)(target, level, img)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetDoublev(&self, pname: types::GLenum, data: *mut types::GLdouble) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *mut types::GLdouble) -> (),
        >(self.GetDoublev.f)(pname, data)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetError(&self) -> types::GLenum {
        __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLenum>(self.GetError.f)()
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetFloatv(&self, pname: types::GLenum, data: *mut types::GLfloat) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *mut types::GLfloat) -> (),
        >(self.GetFloatv.f)(pname, data)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetFragDataIndex(
        &self,
        program: types::GLuint,
        name: *const types::GLchar,
    ) -> types::GLint {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint,
        >(self.GetFragDataIndex.f)(program, name)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetFragDataLocation(
        &self,
        program: types::GLuint,
        name: *const types::GLchar,
    ) -> types::GLint {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint,
        >(self.GetFragDataLocation.f)(program, name)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetFramebufferAttachmentParameteriv(
        &self,
        target: types::GLenum,
        attachment: types::GLenum,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLenum,
                types::GLenum,
                *mut types::GLint,
            ) -> (),
        >(self.GetFramebufferAttachmentParameteriv.f)(target, attachment, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetInteger64i_v(
        &self,
        target: types::GLenum,
        index: types::GLuint,
        data: *mut types::GLint64,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLuint, *mut types::GLint64) -> (),
        >(self.GetInteger64i_v.f)(target, index, data)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetInteger64v(&self, pname: types::GLenum, data: *mut types::GLint64) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *mut types::GLint64) -> (),
        >(self.GetInteger64v.f)(pname, data)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetIntegeri_v(
        &self,
        target: types::GLenum,
        index: types::GLuint,
        data: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLuint, *mut types::GLint) -> (),
        >(self.GetIntegeri_v.f)(target, index, data)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetIntegerv(&self, pname: types::GLenum, data: *mut types::GLint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLint) -> ()>(
            self.GetIntegerv.f,
        )(pname, data)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetMultisamplefv(
        &self,
        pname: types::GLenum,
        index: types::GLuint,
        val: *mut types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLuint, *mut types::GLfloat) -> (),
        >(self.GetMultisamplefv.f)(pname, index, val)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetProgramInfoLog(
        &self,
        program: types::GLuint,
        bufSize: types::GLsizei,
        length: *mut types::GLsizei,
        infoLog: *mut types::GLchar,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLsizei,
                *mut types::GLsizei,
                *mut types::GLchar,
            ) -> (),
        >(self.GetProgramInfoLog.f)(program, bufSize, length, infoLog)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetProgramiv(
        &self,
        program: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> (),
        >(self.GetProgramiv.f)(program, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetQueryObjecti64v(
        &self,
        id: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLint64,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint64) -> (),
        >(self.GetQueryObjecti64v.f)(id, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetQueryObjectiv(
        &self,
        id: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> (),
        >(self.GetQueryObjectiv.f)(id, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetQueryObjectui64v(
        &self,
        id: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLuint64,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint64) -> (),
        >(self.GetQueryObjectui64v.f)(id, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetQueryObjectuiv(
        &self,
        id: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> (),
        >(self.GetQueryObjectuiv.f)(id, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetQueryiv(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> (),
        >(self.GetQueryiv.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetRenderbufferParameteriv(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> (),
        >(self.GetRenderbufferParameteriv.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetSamplerParameterIiv(
        &self,
        sampler: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> (),
        >(self.GetSamplerParameterIiv.f)(sampler, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetSamplerParameterIuiv(
        &self,
        sampler: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> (),
        >(self.GetSamplerParameterIuiv.f)(sampler, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetSamplerParameterfv(
        &self,
        sampler: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLfloat) -> (),
        >(self.GetSamplerParameterfv.f)(sampler, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetSamplerParameteriv(
        &self,
        sampler: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> (),
        >(self.GetSamplerParameteriv.f)(sampler, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetShaderInfoLog(
        &self,
        shader: types::GLuint,
        bufSize: types::GLsizei,
        length: *mut types::GLsizei,
        infoLog: *mut types::GLchar,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLsizei,
                *mut types::GLsizei,
                *mut types::GLchar,
            ) -> (),
        >(self.GetShaderInfoLog.f)(shader, bufSize, length, infoLog)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetShaderSource(
        &self,
        shader: types::GLuint,
        bufSize: types::GLsizei,
        length: *mut types::GLsizei,
        source: *mut types::GLchar,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLsizei,
                *mut types::GLsizei,
                *mut types::GLchar,
            ) -> (),
        >(self.GetShaderSource.f)(shader, bufSize, length, source)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetShaderiv(
        &self,
        shader: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> (),
        >(self.GetShaderiv.f)(shader, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetStageIndexNV(&self, shadertype: types::GLenum) -> types::GLushort {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLushort>(
            self.GetStageIndexNV.f,
        )(shadertype)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetString(&self, name: types::GLenum) -> *const types::GLubyte {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> *const types::GLubyte>(
            self.GetString.f,
        )(name)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetStringi(
        &self,
        name: types::GLenum,
        index: types::GLuint,
    ) -> *const types::GLubyte {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLuint) -> *const types::GLubyte,
        >(self.GetStringi.f)(name, index)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetSynciv(
        &self,
        sync: types::GLsync,
        pname: types::GLenum,
        bufSize: types::GLsizei,
        length: *mut types::GLsizei,
        values: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLsync,
                types::GLenum,
                types::GLsizei,
                *mut types::GLsizei,
                *mut types::GLint,
            ) -> (),
        >(self.GetSynciv.f)(sync, pname, bufSize, length, values)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetTexImage(
        &self,
        target: types::GLenum,
        level: types::GLint,
        format: types::GLenum,
        type_: types::GLenum,
        pixels: *mut __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLenum,
                types::GLenum,
                *mut __gl_imports::raw::c_void,
            ) -> (),
        >(self.GetTexImage.f)(target, level, format, type_, pixels)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetTexLevelParameterfv(
        &self,
        target: types::GLenum,
        level: types::GLint,
        pname: types::GLenum,
        params: *mut types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLenum,
                *mut types::GLfloat,
            ) -> (),
        >(self.GetTexLevelParameterfv.f)(target, level, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetTexLevelParameteriv(
        &self,
        target: types::GLenum,
        level: types::GLint,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLint, types::GLenum, *mut types::GLint) -> (),
        >(self.GetTexLevelParameteriv.f)(target, level, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetTexParameterIiv(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> (),
        >(self.GetTexParameterIiv.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetTexParameterIuiv(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *mut types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *mut types::GLuint) -> (),
        >(self.GetTexParameterIuiv.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetTexParameterfv(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *mut types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> (),
        >(self.GetTexParameterfv.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetTexParameteriv(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> (),
        >(self.GetTexParameteriv.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetTransformFeedbackVarying(
        &self,
        program: types::GLuint,
        index: types::GLuint,
        bufSize: types::GLsizei,
        length: *mut types::GLsizei,
        size: *mut types::GLsizei,
        type_: *mut types::GLenum,
        name: *mut types::GLchar,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLuint,
                types::GLsizei,
                *mut types::GLsizei,
                *mut types::GLsizei,
                *mut types::GLenum,
                *mut types::GLchar,
            ) -> (),
        >(self.GetTransformFeedbackVarying.f)(
            program, index, bufSize, length, size, type_, name
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetUniformBlockIndex(
        &self,
        program: types::GLuint,
        uniformBlockName: *const types::GLchar,
    ) -> types::GLuint {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLuint,
        >(self.GetUniformBlockIndex.f)(program, uniformBlockName)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetUniformIndices(
        &self,
        program: types::GLuint,
        uniformCount: types::GLsizei,
        uniformNames: *const *const types::GLchar,
        uniformIndices: *mut types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLsizei,
                *const *const types::GLchar,
                *mut types::GLuint,
            ) -> (),
        >(self.GetUniformIndices.f)(program, uniformCount, uniformNames, uniformIndices)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetUniformLocation(
        &self,
        program: types::GLuint,
        name: *const types::GLchar,
    ) -> types::GLint {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint,
        >(self.GetUniformLocation.f)(program, name)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetUniformfv(
        &self,
        program: types::GLuint,
        location: types::GLint,
        params: *mut types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLint, *mut types::GLfloat) -> (),
        >(self.GetUniformfv.f)(program, location, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetUniformiv(
        &self,
        program: types::GLuint,
        location: types::GLint,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLint, *mut types::GLint) -> (),
        >(self.GetUniformiv.f)(program, location, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetUniformuiv(
        &self,
        program: types::GLuint,
        location: types::GLint,
        params: *mut types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLint, *mut types::GLuint) -> (),
        >(self.GetUniformuiv.f)(program, location, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetVertexAttribIiv(
        &self,
        index: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> (),
        >(self.GetVertexAttribIiv.f)(index, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetVertexAttribIuiv(
        &self,
        index: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> (),
        >(self.GetVertexAttribIuiv.f)(index, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetVertexAttribPointerv(
        &self,
        index: types::GLuint,
        pname: types::GLenum,
        pointer: *const *mut __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLenum,
                *const *mut __gl_imports::raw::c_void,
            ) -> (),
        >(self.GetVertexAttribPointerv.f)(index, pname, pointer)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetVertexAttribdv(
        &self,
        index: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLdouble,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLdouble) -> (),
        >(self.GetVertexAttribdv.f)(index, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetVertexAttribfv(
        &self,
        index: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLfloat) -> (),
        >(self.GetVertexAttribfv.f)(index, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetVertexAttribiv(
        &self,
        index: types::GLuint,
        pname: types::GLenum,
        params: *mut types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> (),
        >(self.GetVertexAttribiv.f)(index, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Hint(&self, target: types::GLenum, mode: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(
            self.Hint.f,
        )(target, mode)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsBuffer(&self, buffer: types::GLuint) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(
            self.IsBuffer.f,
        )(buffer)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsCommandListNV(&self, list: types::GLuint) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(
            self.IsCommandListNV.f,
        )(list)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsEnabled(&self, cap: types::GLenum) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(
            self.IsEnabled.f,
        )(cap)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsEnabledi(
        &self,
        target: types::GLenum,
        index: types::GLuint,
    ) -> types::GLboolean {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLuint) -> types::GLboolean,
        >(self.IsEnabledi.f)(target, index)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsFramebuffer(&self, framebuffer: types::GLuint) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(
            self.IsFramebuffer.f,
        )(framebuffer)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsProgram(&self, program: types::GLuint) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(
            self.IsProgram.f,
        )(program)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsQuery(&self, id: types::GLuint) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(
            self.IsQuery.f,
        )(id)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsRenderbuffer(&self, renderbuffer: types::GLuint) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(
            self.IsRenderbuffer.f,
        )(renderbuffer)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsSampler(&self, sampler: types::GLuint) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(
            self.IsSampler.f,
        )(sampler)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsShader(&self, shader: types::GLuint) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(
            self.IsShader.f,
        )(shader)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsStateNV(&self, state: types::GLuint) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(
            self.IsStateNV.f,
        )(state)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsSync(&self, sync: types::GLsync) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync) -> types::GLboolean>(
            self.IsSync.f,
        )(sync)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsTexture(&self, texture: types::GLuint) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(
            self.IsTexture.f,
        )(texture)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsVertexArray(&self, array: types::GLuint) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(
            self.IsVertexArray.f,
        )(array)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn LineWidth(&self, width: types::GLfloat) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(
            self.LineWidth.f,
        )(width)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn LinkProgram(&self, program: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.LinkProgram.f,
        )(program)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ListDrawCommandsStatesClientNV(
        &self,
        list: types::GLuint,
        segment: types::GLuint,
        indirects: *const *const __gl_imports::raw::c_void,
        sizes: *const types::GLsizei,
        states: *const types::GLuint,
        fbos: *const types::GLuint,
        count: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLuint,
                *const *const __gl_imports::raw::c_void,
                *const types::GLsizei,
                *const types::GLuint,
                *const types::GLuint,
                types::GLuint,
            ) -> (),
        >(self.ListDrawCommandsStatesClientNV.f)(
            list, segment, indirects, sizes, states, fbos, count,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn LogicOp(&self, opcode: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.LogicOp.f)(
            opcode,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MapBuffer(
        &self,
        target: types::GLenum,
        access: types::GLenum,
    ) -> *mut __gl_imports::raw::c_void {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum) -> *mut __gl_imports::raw::c_void,
        >(self.MapBuffer.f)(target, access)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MapBufferRange(
        &self,
        target: types::GLenum,
        offset: types::GLintptr,
        length: types::GLsizeiptr,
        access: types::GLbitfield,
    ) -> *mut __gl_imports::raw::c_void {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLintptr,
                types::GLsizeiptr,
                types::GLbitfield,
            ) -> *mut __gl_imports::raw::c_void,
        >(self.MapBufferRange.f)(target, offset, length, access)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MultiDrawArrays(
        &self,
        mode: types::GLenum,
        first: *const types::GLint,
        count: *const types::GLsizei,
        drawcount: types::GLsizei,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                *const types::GLint,
                *const types::GLsizei,
                types::GLsizei,
            ) -> (),
        >(self.MultiDrawArrays.f)(mode, first, count, drawcount)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MultiDrawElements(
        &self,
        mode: types::GLenum,
        count: *const types::GLsizei,
        type_: types::GLenum,
        indices: *const *const __gl_imports::raw::c_void,
        drawcount: types::GLsizei,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                *const types::GLsizei,
                types::GLenum,
                *const *const __gl_imports::raw::c_void,
                types::GLsizei,
            ) -> (),
        >(self.MultiDrawElements.f)(mode, count, type_, indices, drawcount)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MultiDrawElementsBaseVertex(
        &self,
        mode: types::GLenum,
        count: *const types::GLsizei,
        type_: types::GLenum,
        indices: *const *const __gl_imports::raw::c_void,
        drawcount: types::GLsizei,
        basevertex: *const types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                *const types::GLsizei,
                types::GLenum,
                *const *const __gl_imports::raw::c_void,
                types::GLsizei,
                *const types::GLint,
            ) -> (),
        >(self.MultiDrawElementsBaseVertex.f)(
            mode, count, type_, indices, drawcount, basevertex
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MultiTexCoordP1ui(
        &self,
        texture: types::GLenum,
        type_: types::GLenum,
        coords: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> (),
        >(self.MultiTexCoordP1ui.f)(texture, type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MultiTexCoordP1uiv(
        &self,
        texture: types::GLenum,
        type_: types::GLenum,
        coords: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> (),
        >(self.MultiTexCoordP1uiv.f)(texture, type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MultiTexCoordP2ui(
        &self,
        texture: types::GLenum,
        type_: types::GLenum,
        coords: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> (),
        >(self.MultiTexCoordP2ui.f)(texture, type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MultiTexCoordP2uiv(
        &self,
        texture: types::GLenum,
        type_: types::GLenum,
        coords: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> (),
        >(self.MultiTexCoordP2uiv.f)(texture, type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MultiTexCoordP3ui(
        &self,
        texture: types::GLenum,
        type_: types::GLenum,
        coords: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> (),
        >(self.MultiTexCoordP3ui.f)(texture, type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MultiTexCoordP3uiv(
        &self,
        texture: types::GLenum,
        type_: types::GLenum,
        coords: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> (),
        >(self.MultiTexCoordP3uiv.f)(texture, type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MultiTexCoordP4ui(
        &self,
        texture: types::GLenum,
        type_: types::GLenum,
        coords: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> (),
        >(self.MultiTexCoordP4ui.f)(texture, type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MultiTexCoordP4uiv(
        &self,
        texture: types::GLenum,
        type_: types::GLenum,
        coords: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> (),
        >(self.MultiTexCoordP4uiv.f)(texture, type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn NormalP3ui(&self, type_: types::GLenum, coords: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.NormalP3ui.f,
        )(type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn NormalP3uiv(&self, type_: types::GLenum, coords: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLuint) -> (),
        >(self.NormalP3uiv.f)(type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn PixelStoref(&self, pname: types::GLenum, param: types::GLfloat) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(
            self.PixelStoref.f,
        )(pname, param)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn PixelStorei(&self, pname: types::GLenum, param: types::GLint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(
            self.PixelStorei.f,
        )(pname, param)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn PointParameterf(&self, pname: types::GLenum, param: types::GLfloat) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(
            self.PointParameterf.f,
        )(pname, param)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn PointParameterfv(
        &self,
        pname: types::GLenum,
        params: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLfloat) -> (),
        >(self.PointParameterfv.f)(pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn PointParameteri(&self, pname: types::GLenum, param: types::GLint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(
            self.PointParameteri.f,
        )(pname, param)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn PointParameteriv(&self, pname: types::GLenum, params: *const types::GLint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLint) -> (),
        >(self.PointParameteriv.f)(pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn PointSize(&self, size: types::GLfloat) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(
            self.PointSize.f,
        )(size)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn PolygonMode(&self, face: types::GLenum, mode: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(
            self.PolygonMode.f,
        )(face, mode)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn PolygonOffset(&self, factor: types::GLfloat, units: types::GLfloat) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(
            self.PolygonOffset.f,
        )(factor, units)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn PrimitiveRestartIndex(&self, index: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.PrimitiveRestartIndex.f,
        )(index)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ProvokingVertex(&self, mode: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(
            self.ProvokingVertex.f,
        )(mode)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn QueryCounter(&self, id: types::GLuint, target: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(
            self.QueryCounter.f,
        )(id, target)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ReadBuffer(&self, src: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(
            self.ReadBuffer.f,
        )(src)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ReadPixels(
        &self,
        x: types::GLint,
        y: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
        format: types::GLenum,
        type_: types::GLenum,
        pixels: *mut __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLsizei,
                types::GLenum,
                types::GLenum,
                *mut __gl_imports::raw::c_void,
            ) -> (),
        >(self.ReadPixels.f)(x, y, width, height, format, type_, pixels)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn RenderbufferStorage(
        &self,
        target: types::GLenum,
        internalformat: types::GLenum,
        width: types::GLsizei,
        height: types::GLsizei,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLsizei, types::GLsizei) -> (),
        >(self.RenderbufferStorage.f)(target, internalformat, width, height)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn RenderbufferStorageMultisample(
        &self,
        target: types::GLenum,
        samples: types::GLsizei,
        internalformat: types::GLenum,
        width: types::GLsizei,
        height: types::GLsizei,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLsizei,
                types::GLenum,
                types::GLsizei,
                types::GLsizei,
            ) -> (),
        >(self.RenderbufferStorageMultisample.f)(
            target, samples, internalformat, width, height
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn SampleCoverage(&self, value: types::GLfloat, invert: types::GLboolean) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLboolean) -> ()>(
            self.SampleCoverage.f,
        )(value, invert)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn SampleMaski(&self, maskNumber: types::GLuint, mask: types::GLbitfield) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLbitfield) -> ()>(
            self.SampleMaski.f,
        )(maskNumber, mask)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn SamplerParameterIiv(
        &self,
        sampler: types::GLuint,
        pname: types::GLenum,
        param: *const types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *const types::GLint) -> (),
        >(self.SamplerParameterIiv.f)(sampler, pname, param)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn SamplerParameterIuiv(
        &self,
        sampler: types::GLuint,
        pname: types::GLenum,
        param: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *const types::GLuint) -> (),
        >(self.SamplerParameterIuiv.f)(sampler, pname, param)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn SamplerParameterf(
        &self,
        sampler: types::GLuint,
        pname: types::GLenum,
        param: types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, types::GLfloat) -> (),
        >(self.SamplerParameterf.f)(sampler, pname, param)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn SamplerParameterfv(
        &self,
        sampler: types::GLuint,
        pname: types::GLenum,
        param: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *const types::GLfloat) -> (),
        >(self.SamplerParameterfv.f)(sampler, pname, param)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn SamplerParameteri(
        &self,
        sampler: types::GLuint,
        pname: types::GLenum,
        param: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, types::GLint) -> (),
        >(self.SamplerParameteri.f)(sampler, pname, param)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn SamplerParameteriv(
        &self,
        sampler: types::GLuint,
        pname: types::GLenum,
        param: *const types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, *const types::GLint) -> (),
        >(self.SamplerParameteriv.f)(sampler, pname, param)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Scissor(
        &self,
        x: types::GLint,
        y: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> (),
        >(self.Scissor.f)(x, y, width, height)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn SecondaryColorP3ui(&self, type_: types::GLenum, color: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.SecondaryColorP3ui.f,
        )(type_, color)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn SecondaryColorP3uiv(
        &self,
        type_: types::GLenum,
        color: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLuint) -> (),
        >(self.SecondaryColorP3uiv.f)(type_, color)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ShaderSource(
        &self,
        shader: types::GLuint,
        count: types::GLsizei,
        string: *const *const types::GLchar,
        length: *const types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLsizei,
                *const *const types::GLchar,
                *const types::GLint,
            ) -> (),
        >(self.ShaderSource.f)(shader, count, string, length)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn StateCaptureNV(&self, state: types::GLuint, mode: types::GLenum) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(
            self.StateCaptureNV.f,
        )(state, mode)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn StencilFunc(
        &self,
        func: types::GLenum,
        ref_: types::GLint,
        mask: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLint, types::GLuint) -> (),
        >(self.StencilFunc.f)(func, ref_, mask)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn StencilFuncSeparate(
        &self,
        face: types::GLenum,
        func: types::GLenum,
        ref_: types::GLint,
        mask: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLint, types::GLuint) -> (),
        >(self.StencilFuncSeparate.f)(face, func, ref_, mask)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn StencilMask(&self, mask: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.StencilMask.f,
        )(mask)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn StencilMaskSeparate(&self, face: types::GLenum, mask: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.StencilMaskSeparate.f,
        )(face, mask)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn StencilOp(
        &self,
        fail: types::GLenum,
        zfail: types::GLenum,
        zpass: types::GLenum,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLenum) -> (),
        >(self.StencilOp.f)(fail, zfail, zpass)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn StencilOpSeparate(
        &self,
        face: types::GLenum,
        sfail: types::GLenum,
        dpfail: types::GLenum,
        dppass: types::GLenum,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> (),
        >(self.StencilOpSeparate.f)(face, sfail, dpfail, dppass)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexBuffer(
        &self,
        target: types::GLenum,
        internalformat: types::GLenum,
        buffer: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> (),
        >(self.TexBuffer.f)(target, internalformat, buffer)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexCoordP1ui(&self, type_: types::GLenum, coords: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.TexCoordP1ui.f,
        )(type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexCoordP1uiv(&self, type_: types::GLenum, coords: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLuint) -> (),
        >(self.TexCoordP1uiv.f)(type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexCoordP2ui(&self, type_: types::GLenum, coords: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.TexCoordP2ui.f,
        )(type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexCoordP2uiv(&self, type_: types::GLenum, coords: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLuint) -> (),
        >(self.TexCoordP2uiv.f)(type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexCoordP3ui(&self, type_: types::GLenum, coords: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.TexCoordP3ui.f,
        )(type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexCoordP3uiv(&self, type_: types::GLenum, coords: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLuint) -> (),
        >(self.TexCoordP3uiv.f)(type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexCoordP4ui(&self, type_: types::GLenum, coords: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.TexCoordP4ui.f,
        )(type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexCoordP4uiv(&self, type_: types::GLenum, coords: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLuint) -> (),
        >(self.TexCoordP4uiv.f)(type_, coords)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexImage1D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        internalformat: types::GLint,
        width: types::GLsizei,
        border: types::GLint,
        format: types::GLenum,
        type_: types::GLenum,
        pixels: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLint,
                types::GLenum,
                types::GLenum,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.TexImage1D.f)(
            target,
            level,
            internalformat,
            width,
            border,
            format,
            type_,
            pixels,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexImage2D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        internalformat: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
        border: types::GLint,
        format: types::GLenum,
        type_: types::GLenum,
        pixels: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLsizei,
                types::GLint,
                types::GLenum,
                types::GLenum,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.TexImage2D.f)(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            type_,
            pixels,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexImage2DMultisample(
        &self,
        target: types::GLenum,
        samples: types::GLsizei,
        internalformat: types::GLenum,
        width: types::GLsizei,
        height: types::GLsizei,
        fixedsamplelocations: types::GLboolean,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLsizei,
                types::GLenum,
                types::GLsizei,
                types::GLsizei,
                types::GLboolean,
            ) -> (),
        >(self.TexImage2DMultisample.f)(
            target,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexImage3D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        internalformat: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
        depth: types::GLsizei,
        border: types::GLint,
        format: types::GLenum,
        type_: types::GLenum,
        pixels: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLsizei,
                types::GLsizei,
                types::GLint,
                types::GLenum,
                types::GLenum,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.TexImage3D.f)(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format,
            type_,
            pixels,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexImage3DMultisample(
        &self,
        target: types::GLenum,
        samples: types::GLsizei,
        internalformat: types::GLenum,
        width: types::GLsizei,
        height: types::GLsizei,
        depth: types::GLsizei,
        fixedsamplelocations: types::GLboolean,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLsizei,
                types::GLenum,
                types::GLsizei,
                types::GLsizei,
                types::GLsizei,
                types::GLboolean,
            ) -> (),
        >(self.TexImage3DMultisample.f)(
            target,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexParameterIiv(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *const types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> (),
        >(self.TexParameterIiv.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexParameterIuiv(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> (),
        >(self.TexParameterIuiv.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexParameterf(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        param: types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> (),
        >(self.TexParameterf.f)(target, pname, param)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexParameterfv(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> (),
        >(self.TexParameterfv.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexParameteri(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        param: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> (),
        >(self.TexParameteri.f)(target, pname, param)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexParameteriv(
        &self,
        target: types::GLenum,
        pname: types::GLenum,
        params: *const types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> (),
        >(self.TexParameteriv.f)(target, pname, params)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexSubImage1D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        xoffset: types::GLint,
        width: types::GLsizei,
        format: types::GLenum,
        type_: types::GLenum,
        pixels: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLenum,
                types::GLenum,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.TexSubImage1D.f)(target, level, xoffset, width, format, type_, pixels)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexSubImage2D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        xoffset: types::GLint,
        yoffset: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
        format: types::GLenum,
        type_: types::GLenum,
        pixels: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLsizei,
                types::GLenum,
                types::GLenum,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.TexSubImage2D.f)(
            target, level, xoffset, yoffset, width, height, format, type_, pixels,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TexSubImage3D(
        &self,
        target: types::GLenum,
        level: types::GLint,
        xoffset: types::GLint,
        yoffset: types::GLint,
        zoffset: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
        depth: types::GLsizei,
        format: types::GLenum,
        type_: types::GLenum,
        pixels: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLenum,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLsizei,
                types::GLsizei,
                types::GLsizei,
                types::GLenum,
                types::GLenum,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.TexSubImage3D.f)(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels,
        )
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn TransformFeedbackVaryings(
        &self,
        program: types::GLuint,
        count: types::GLsizei,
        varyings: *const *const types::GLchar,
        bufferMode: types::GLenum,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLsizei,
                *const *const types::GLchar,
                types::GLenum,
            ) -> (),
        >(self.TransformFeedbackVaryings.f)(program, count, varyings, bufferMode)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform1f(&self, location: types::GLint, v0: types::GLfloat) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat) -> ()>(
            self.Uniform1f.f,
        )(location, v0)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform1fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> (),
        >(self.Uniform1fv.f)(location, count, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform1i(&self, location: types::GLint, v0: types::GLint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(
            self.Uniform1i.f,
        )(location, v0)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform1iv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        value: *const types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> (),
        >(self.Uniform1iv.f)(location, count, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform1ui(&self, location: types::GLint, v0: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint) -> ()>(
            self.Uniform1ui.f,
        )(location, v0)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform1uiv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        value: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> (),
        >(self.Uniform1uiv.f)(location, count, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform2f(
        &self,
        location: types::GLint,
        v0: types::GLfloat,
        v1: types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLfloat, types::GLfloat) -> (),
        >(self.Uniform2f.f)(location, v0, v1)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform2fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> (),
        >(self.Uniform2fv.f)(location, count, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform2i(
        &self,
        location: types::GLint,
        v0: types::GLint,
        v1: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLint, types::GLint) -> (),
        >(self.Uniform2i.f)(location, v0, v1)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform2iv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        value: *const types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> (),
        >(self.Uniform2iv.f)(location, count, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform2ui(
        &self,
        location: types::GLint,
        v0: types::GLuint,
        v1: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLuint, types::GLuint) -> (),
        >(self.Uniform2ui.f)(location, v0, v1)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform2uiv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        value: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> (),
        >(self.Uniform2uiv.f)(location, count, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform3f(
        &self,
        location: types::GLint,
        v0: types::GLfloat,
        v1: types::GLfloat,
        v2: types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLfloat) -> (),
        >(self.Uniform3f.f)(location, v0, v1, v2)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform3fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> (),
        >(self.Uniform3fv.f)(location, count, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform3i(
        &self,
        location: types::GLint,
        v0: types::GLint,
        v1: types::GLint,
        v2: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> (),
        >(self.Uniform3i.f)(location, v0, v1, v2)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform3iv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        value: *const types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> (),
        >(self.Uniform3iv.f)(location, count, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform3ui(
        &self,
        location: types::GLint,
        v0: types::GLuint,
        v1: types::GLuint,
        v2: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLuint, types::GLuint, types::GLuint) -> (),
        >(self.Uniform3ui.f)(location, v0, v1, v2)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform3uiv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        value: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> (),
        >(self.Uniform3uiv.f)(location, count, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform4f(
        &self,
        location: types::GLint,
        v0: types::GLfloat,
        v1: types::GLfloat,
        v2: types::GLfloat,
        v3: types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLfloat,
                types::GLfloat,
                types::GLfloat,
                types::GLfloat,
            ) -> (),
        >(self.Uniform4f.f)(location, v0, v1, v2, v3)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform4fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> (),
        >(self.Uniform4fv.f)(location, count, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform4i(
        &self,
        location: types::GLint,
        v0: types::GLint,
        v1: types::GLint,
        v2: types::GLint,
        v3: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
            ) -> (),
        >(self.Uniform4i.f)(location, v0, v1, v2, v3)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform4iv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        value: *const types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> (),
        >(self.Uniform4iv.f)(location, count, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform4ui(
        &self,
        location: types::GLint,
        v0: types::GLuint,
        v1: types::GLuint,
        v2: types::GLuint,
        v3: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLuint,
                types::GLuint,
                types::GLuint,
                types::GLuint,
            ) -> (),
        >(self.Uniform4ui.f)(location, v0, v1, v2, v3)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Uniform4uiv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        value: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> (),
        >(self.Uniform4uiv.f)(location, count, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UniformBlockBinding(
        &self,
        program: types::GLuint,
        uniformBlockIndex: types::GLuint,
        uniformBlockBinding: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> (),
        >(self.UniformBlockBinding.f)(program, uniformBlockIndex, uniformBlockBinding)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UniformMatrix2fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        transpose: types::GLboolean,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLsizei,
                types::GLboolean,
                *const types::GLfloat,
            ) -> (),
        >(self.UniformMatrix2fv.f)(location, count, transpose, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UniformMatrix2x3fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        transpose: types::GLboolean,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLsizei,
                types::GLboolean,
                *const types::GLfloat,
            ) -> (),
        >(self.UniformMatrix2x3fv.f)(location, count, transpose, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UniformMatrix2x4fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        transpose: types::GLboolean,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLsizei,
                types::GLboolean,
                *const types::GLfloat,
            ) -> (),
        >(self.UniformMatrix2x4fv.f)(location, count, transpose, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UniformMatrix3fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        transpose: types::GLboolean,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLsizei,
                types::GLboolean,
                *const types::GLfloat,
            ) -> (),
        >(self.UniformMatrix3fv.f)(location, count, transpose, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UniformMatrix3x2fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        transpose: types::GLboolean,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLsizei,
                types::GLboolean,
                *const types::GLfloat,
            ) -> (),
        >(self.UniformMatrix3x2fv.f)(location, count, transpose, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UniformMatrix3x4fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        transpose: types::GLboolean,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLsizei,
                types::GLboolean,
                *const types::GLfloat,
            ) -> (),
        >(self.UniformMatrix3x4fv.f)(location, count, transpose, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UniformMatrix4fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        transpose: types::GLboolean,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLsizei,
                types::GLboolean,
                *const types::GLfloat,
            ) -> (),
        >(self.UniformMatrix4fv.f)(location, count, transpose, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UniformMatrix4x2fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        transpose: types::GLboolean,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLsizei,
                types::GLboolean,
                *const types::GLfloat,
            ) -> (),
        >(self.UniformMatrix4x2fv.f)(location, count, transpose, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UniformMatrix4x3fv(
        &self,
        location: types::GLint,
        count: types::GLsizei,
        transpose: types::GLboolean,
        value: *const types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLint,
                types::GLsizei,
                types::GLboolean,
                *const types::GLfloat,
            ) -> (),
        >(self.UniformMatrix4x3fv.f)(location, count, transpose, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UnmapBuffer(&self, target: types::GLenum) -> types::GLboolean {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(
            self.UnmapBuffer.f,
        )(target)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UseProgram(&self, program: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.UseProgram.f,
        )(program)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ValidateProgram(&self, program: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(
            self.ValidateProgram.f,
        )(program)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib1d(&self, index: types::GLuint, x: types::GLdouble) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble) -> ()>(
            self.VertexAttrib1d.f,
        )(index, x)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib1dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLdouble) -> (),
        >(self.VertexAttrib1dv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib1f(&self, index: types::GLuint, x: types::GLfloat) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat) -> ()>(
            self.VertexAttrib1f.f,
        )(index, x)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib1fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLfloat) -> (),
        >(self.VertexAttrib1fv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib1s(&self, index: types::GLuint, x: types::GLshort) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort) -> ()>(
            self.VertexAttrib1s.f,
        )(index, x)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib1sv(&self, index: types::GLuint, v: *const types::GLshort) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLshort) -> (),
        >(self.VertexAttrib1sv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib2d(
        &self,
        index: types::GLuint,
        x: types::GLdouble,
        y: types::GLdouble,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble) -> (),
        >(self.VertexAttrib2d.f)(index, x, y)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib2dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLdouble) -> (),
        >(self.VertexAttrib2dv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib2f(
        &self,
        index: types::GLuint,
        x: types::GLfloat,
        y: types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat) -> (),
        >(self.VertexAttrib2f.f)(index, x, y)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib2fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLfloat) -> (),
        >(self.VertexAttrib2fv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib2s(
        &self,
        index: types::GLuint,
        x: types::GLshort,
        y: types::GLshort,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLshort, types::GLshort) -> (),
        >(self.VertexAttrib2s.f)(index, x, y)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib2sv(&self, index: types::GLuint, v: *const types::GLshort) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLshort) -> (),
        >(self.VertexAttrib2sv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib3d(
        &self,
        index: types::GLuint,
        x: types::GLdouble,
        y: types::GLdouble,
        z: types::GLdouble,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLdouble,
                types::GLdouble,
                types::GLdouble,
            ) -> (),
        >(self.VertexAttrib3d.f)(index, x, y, z)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib3dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLdouble) -> (),
        >(self.VertexAttrib3dv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib3f(
        &self,
        index: types::GLuint,
        x: types::GLfloat,
        y: types::GLfloat,
        z: types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat) -> (),
        >(self.VertexAttrib3f.f)(index, x, y, z)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib3fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLfloat) -> (),
        >(self.VertexAttrib3fv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib3s(
        &self,
        index: types::GLuint,
        x: types::GLshort,
        y: types::GLshort,
        z: types::GLshort,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLshort, types::GLshort, types::GLshort) -> (),
        >(self.VertexAttrib3s.f)(index, x, y, z)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib3sv(&self, index: types::GLuint, v: *const types::GLshort) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLshort) -> (),
        >(self.VertexAttrib3sv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4Nbv(&self, index: types::GLuint, v: *const types::GLbyte) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLbyte) -> (),
        >(self.VertexAttrib4Nbv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4Niv(&self, index: types::GLuint, v: *const types::GLint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLint) -> (),
        >(self.VertexAttrib4Niv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4Nsv(&self, index: types::GLuint, v: *const types::GLshort) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLshort) -> (),
        >(self.VertexAttrib4Nsv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4Nub(
        &self,
        index: types::GLuint,
        x: types::GLubyte,
        y: types::GLubyte,
        z: types::GLubyte,
        w: types::GLubyte,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLubyte,
                types::GLubyte,
                types::GLubyte,
                types::GLubyte,
            ) -> (),
        >(self.VertexAttrib4Nub.f)(index, x, y, z, w)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4Nubv(&self, index: types::GLuint, v: *const types::GLubyte) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLubyte) -> (),
        >(self.VertexAttrib4Nubv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4Nuiv(&self, index: types::GLuint, v: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLuint) -> (),
        >(self.VertexAttrib4Nuiv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4Nusv(&self, index: types::GLuint, v: *const types::GLushort) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLushort) -> (),
        >(self.VertexAttrib4Nusv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4bv(&self, index: types::GLuint, v: *const types::GLbyte) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLbyte) -> (),
        >(self.VertexAttrib4bv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4d(
        &self,
        index: types::GLuint,
        x: types::GLdouble,
        y: types::GLdouble,
        z: types::GLdouble,
        w: types::GLdouble,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLdouble,
                types::GLdouble,
                types::GLdouble,
                types::GLdouble,
            ) -> (),
        >(self.VertexAttrib4d.f)(index, x, y, z, w)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLdouble) -> (),
        >(self.VertexAttrib4dv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4f(
        &self,
        index: types::GLuint,
        x: types::GLfloat,
        y: types::GLfloat,
        z: types::GLfloat,
        w: types::GLfloat,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLfloat,
                types::GLfloat,
                types::GLfloat,
                types::GLfloat,
            ) -> (),
        >(self.VertexAttrib4f.f)(index, x, y, z, w)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLfloat) -> (),
        >(self.VertexAttrib4fv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4iv(&self, index: types::GLuint, v: *const types::GLint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLint) -> (),
        >(self.VertexAttrib4iv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4s(
        &self,
        index: types::GLuint,
        x: types::GLshort,
        y: types::GLshort,
        z: types::GLshort,
        w: types::GLshort,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLshort,
                types::GLshort,
                types::GLshort,
                types::GLshort,
            ) -> (),
        >(self.VertexAttrib4s.f)(index, x, y, z, w)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4sv(&self, index: types::GLuint, v: *const types::GLshort) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLshort) -> (),
        >(self.VertexAttrib4sv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4ubv(&self, index: types::GLuint, v: *const types::GLubyte) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLubyte) -> (),
        >(self.VertexAttrib4ubv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLuint) -> (),
        >(self.VertexAttrib4uiv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttrib4usv(&self, index: types::GLuint, v: *const types::GLushort) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLushort) -> (),
        >(self.VertexAttrib4usv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribDivisor(&self, index: types::GLuint, divisor: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(
            self.VertexAttribDivisor.f,
        )(index, divisor)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI1i(&self, index: types::GLuint, x: types::GLint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint) -> ()>(
            self.VertexAttribI1i.f,
        )(index, x)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI1iv(&self, index: types::GLuint, v: *const types::GLint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLint) -> (),
        >(self.VertexAttribI1iv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI1ui(&self, index: types::GLuint, x: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(
            self.VertexAttribI1ui.f,
        )(index, x)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI1uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLuint) -> (),
        >(self.VertexAttribI1uiv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI2i(
        &self,
        index: types::GLuint,
        x: types::GLint,
        y: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLint, types::GLint) -> (),
        >(self.VertexAttribI2i.f)(index, x, y)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI2iv(&self, index: types::GLuint, v: *const types::GLint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLint) -> (),
        >(self.VertexAttribI2iv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI2ui(
        &self,
        index: types::GLuint,
        x: types::GLuint,
        y: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> (),
        >(self.VertexAttribI2ui.f)(index, x, y)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI2uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLuint) -> (),
        >(self.VertexAttribI2uiv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI3i(
        &self,
        index: types::GLuint,
        x: types::GLint,
        y: types::GLint,
        z: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint) -> (),
        >(self.VertexAttribI3i.f)(index, x, y, z)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI3iv(&self, index: types::GLuint, v: *const types::GLint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLint) -> (),
        >(self.VertexAttribI3iv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI3ui(
        &self,
        index: types::GLuint,
        x: types::GLuint,
        y: types::GLuint,
        z: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> (),
        >(self.VertexAttribI3ui.f)(index, x, y, z)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI3uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLuint) -> (),
        >(self.VertexAttribI3uiv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI4bv(&self, index: types::GLuint, v: *const types::GLbyte) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLbyte) -> (),
        >(self.VertexAttribI4bv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI4i(
        &self,
        index: types::GLuint,
        x: types::GLint,
        y: types::GLint,
        z: types::GLint,
        w: types::GLint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLint,
                types::GLint,
                types::GLint,
                types::GLint,
            ) -> (),
        >(self.VertexAttribI4i.f)(index, x, y, z, w)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI4iv(&self, index: types::GLuint, v: *const types::GLint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLint) -> (),
        >(self.VertexAttribI4iv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI4sv(&self, index: types::GLuint, v: *const types::GLshort) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLshort) -> (),
        >(self.VertexAttribI4sv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI4ubv(&self, index: types::GLuint, v: *const types::GLubyte) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLubyte) -> (),
        >(self.VertexAttribI4ubv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI4ui(
        &self,
        index: types::GLuint,
        x: types::GLuint,
        y: types::GLuint,
        z: types::GLuint,
        w: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLuint,
                types::GLuint,
                types::GLuint,
                types::GLuint,
            ) -> (),
        >(self.VertexAttribI4ui.f)(index, x, y, z, w)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI4uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLuint) -> (),
        >(self.VertexAttribI4uiv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribI4usv(&self, index: types::GLuint, v: *const types::GLushort) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, *const types::GLushort) -> (),
        >(self.VertexAttribI4usv.f)(index, v)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribIPointer(
        &self,
        index: types::GLuint,
        size: types::GLint,
        type_: types::GLenum,
        stride: types::GLsizei,
        pointer: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLint,
                types::GLenum,
                types::GLsizei,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.VertexAttribIPointer.f)(index, size, type_, stride, pointer)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribP1ui(
        &self,
        index: types::GLuint,
        type_: types::GLenum,
        normalized: types::GLboolean,
        value: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, types::GLuint) -> (),
        >(self.VertexAttribP1ui.f)(index, type_, normalized, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribP1uiv(
        &self,
        index: types::GLuint,
        type_: types::GLenum,
        normalized: types::GLboolean,
        value: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLenum,
                types::GLboolean,
                *const types::GLuint,
            ) -> (),
        >(self.VertexAttribP1uiv.f)(index, type_, normalized, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribP2ui(
        &self,
        index: types::GLuint,
        type_: types::GLenum,
        normalized: types::GLboolean,
        value: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, types::GLuint) -> (),
        >(self.VertexAttribP2ui.f)(index, type_, normalized, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribP2uiv(
        &self,
        index: types::GLuint,
        type_: types::GLenum,
        normalized: types::GLboolean,
        value: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLenum,
                types::GLboolean,
                *const types::GLuint,
            ) -> (),
        >(self.VertexAttribP2uiv.f)(index, type_, normalized, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribP3ui(
        &self,
        index: types::GLuint,
        type_: types::GLenum,
        normalized: types::GLboolean,
        value: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, types::GLuint) -> (),
        >(self.VertexAttribP3ui.f)(index, type_, normalized, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribP3uiv(
        &self,
        index: types::GLuint,
        type_: types::GLenum,
        normalized: types::GLboolean,
        value: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLenum,
                types::GLboolean,
                *const types::GLuint,
            ) -> (),
        >(self.VertexAttribP3uiv.f)(index, type_, normalized, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribP4ui(
        &self,
        index: types::GLuint,
        type_: types::GLenum,
        normalized: types::GLboolean,
        value: types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, types::GLuint) -> (),
        >(self.VertexAttribP4ui.f)(index, type_, normalized, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribP4uiv(
        &self,
        index: types::GLuint,
        type_: types::GLenum,
        normalized: types::GLboolean,
        value: *const types::GLuint,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLenum,
                types::GLboolean,
                *const types::GLuint,
            ) -> (),
        >(self.VertexAttribP4uiv.f)(index, type_, normalized, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexAttribPointer(
        &self,
        index: types::GLuint,
        size: types::GLint,
        type_: types::GLenum,
        normalized: types::GLboolean,
        stride: types::GLsizei,
        pointer: *const __gl_imports::raw::c_void,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::GLuint,
                types::GLint,
                types::GLenum,
                types::GLboolean,
                types::GLsizei,
                *const __gl_imports::raw::c_void,
            ) -> (),
        >(self.VertexAttribPointer.f)(index, size, type_, normalized, stride, pointer)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexP2ui(&self, type_: types::GLenum, value: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.VertexP2ui.f,
        )(type_, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexP2uiv(&self, type_: types::GLenum, value: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLuint) -> (),
        >(self.VertexP2uiv.f)(type_, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexP3ui(&self, type_: types::GLenum, value: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.VertexP3ui.f,
        )(type_, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexP3uiv(&self, type_: types::GLenum, value: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLuint) -> (),
        >(self.VertexP3uiv.f)(type_, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexP4ui(&self, type_: types::GLenum, value: types::GLuint) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(
            self.VertexP4ui.f,
        )(type_, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn VertexP4uiv(&self, type_: types::GLenum, value: *const types::GLuint) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLenum, *const types::GLuint) -> (),
        >(self.VertexP4uiv.f)(type_, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn Viewport(
        &self,
        x: types::GLint,
        y: types::GLint,
        width: types::GLsizei,
        height: types::GLsizei,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> (),
        >(self.Viewport.f)(x, y, width, height)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn WaitSync(
        &self,
        sync: types::GLsync,
        flags: types::GLbitfield,
        timeout: types::GLuint64,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(types::GLsync, types::GLbitfield, types::GLuint64) -> (),
        >(self.WaitSync.f)(sync, flags, timeout)
    }
}

unsafe impl __gl_imports::Send for Gl {}
