#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
extern crate libc;

use std::os::raw::*;
use self::libc::FILE;

#[cfg(not(windows))]
use self::libc::tm;
#[cfg(windows)]
pub type tm = c_void;

pub type jmp_buf = c_void;
use self::libc::time_t;

pub const PNG_LIBPNG_VER_SONUM: c_uint = 16;
pub const PNG_LIBPNG_VER_DLLNUM: c_uint = 16;
pub const PNG_LIBPNG_VER_MAJOR: c_uint = 1;
pub const PNG_LIBPNG_VER_MINOR: c_uint = 6;
pub const PNG_LIBPNG_VER_RELEASE: c_uint = 34;
pub const PNG_LIBPNG_VER_BUILD: c_uint = 0;
pub const PNG_LIBPNG_BUILD_ALPHA: c_uint = 1;
pub const PNG_LIBPNG_BUILD_BETA: c_uint = 2;
pub const PNG_LIBPNG_BUILD_RC: c_uint = 3;
pub const PNG_LIBPNG_BUILD_STABLE: c_uint = 4;
pub const PNG_LIBPNG_BUILD_RELEASE_STATUS_MASK: c_uint = 7;
pub const PNG_LIBPNG_BUILD_PATCH: c_uint = 8;
pub const PNG_LIBPNG_BUILD_PRIVATE: c_uint = 16;
pub const PNG_LIBPNG_BUILD_SPECIAL: c_uint = 32;
pub const PNG_LIBPNG_BUILD_BASE_TYPE: c_uint = 4;
pub const PNG_LIBPNG_VER: c_uint = 10634;
pub const PNG_API_RULE: c_uint = 0;
pub const PNG_DEFAULT_READ_MACROS: c_uint = 1;
pub const PNG_GAMMA_THRESHOLD_FIXED: c_uint = 5000;
pub const PNG_INFLATE_BUF_SIZE: c_uint = 1024;
pub const PNG_MAX_GAMMA_8: c_uint = 11;
pub const PNG_QUANTIZE_BLUE_BITS: c_uint = 5;
pub const PNG_QUANTIZE_GREEN_BITS: c_uint = 5;
pub const PNG_QUANTIZE_RED_BITS: c_uint = 5;
pub const PNG_TEXT_Z_DEFAULT_COMPRESSION: c_int = -1;
pub const PNG_TEXT_Z_DEFAULT_STRATEGY: c_uint = 0;
pub const PNG_USER_CHUNK_CACHE_MAX: c_uint = 1000;
pub const PNG_USER_CHUNK_MALLOC_MAX: c_uint = 8000000;
pub const PNG_USER_HEIGHT_MAX: c_uint = 1000000;
pub const PNG_USER_WIDTH_MAX: c_uint = 1000000;
pub const PNG_ZBUF_SIZE: c_uint = 8192;
pub const PNG_Z_DEFAULT_COMPRESSION: c_int = -1;
pub const PNG_Z_DEFAULT_NOFILTER_STRATEGY: c_uint = 0;
pub const PNG_Z_DEFAULT_STRATEGY: c_uint = 1;
pub const PNG_sCAL_PRECISION: c_uint = 5;
pub const PNG_sRGB_PROFILE_CHECKS: c_uint = 2;
pub const PNG_LIBPNG_BUILD_TYPE: c_uint = 4;
pub const PNG_TEXT_COMPRESSION_NONE_WR: c_int = -3;
pub const PNG_TEXT_COMPRESSION_zTXt_WR: c_int = -2;
pub const PNG_TEXT_COMPRESSION_NONE: c_int = -1;
pub const PNG_TEXT_COMPRESSION_zTXt: c_uint = 0;
pub const PNG_ITXT_COMPRESSION_NONE: c_uint = 1;
pub const PNG_ITXT_COMPRESSION_zTXt: c_uint = 2;
pub const PNG_TEXT_COMPRESSION_LAST: c_uint = 3;
pub const PNG_HAVE_IHDR: c_uint = 1;
pub const PNG_HAVE_PLTE: c_uint = 2;
pub const PNG_AFTER_IDAT: c_uint = 8;
pub const PNG_FP_1: c_uint = 100000;
pub const PNG_FP_HALF: c_uint = 50000;
pub const PNG_COLOR_MASK_PALETTE: c_uint = 1;
pub const PNG_COLOR_MASK_COLOR: c_uint = 2;
pub const PNG_COLOR_MASK_ALPHA: c_uint = 4;
pub const PNG_COLOR_TYPE_GRAY: c_uint = 0;
pub const PNG_COLOR_TYPE_PALETTE: c_uint = 3;
pub const PNG_COLOR_TYPE_RGB: c_uint = 2;
pub const PNG_COLOR_TYPE_RGB_ALPHA: c_uint = 6;
pub const PNG_COLOR_TYPE_GRAY_ALPHA: c_uint = 4;
pub const PNG_COLOR_TYPE_RGBA: c_uint = 6;
pub const PNG_COLOR_TYPE_GA: c_uint = 4;
pub const PNG_COMPRESSION_TYPE_BASE: c_uint = 0;
pub const PNG_COMPRESSION_TYPE_DEFAULT: c_uint = 0;
pub const PNG_FILTER_TYPE_BASE: c_uint = 0;
pub const PNG_INTRAPIXEL_DIFFERENCING: c_uint = 64;
pub const PNG_FILTER_TYPE_DEFAULT: c_uint = 0;
pub const PNG_INTERLACE_NONE: c_uint = 0;
pub const PNG_INTERLACE_ADAM7: c_uint = 1;
pub const PNG_INTERLACE_LAST: c_uint = 2;
pub const PNG_OFFSET_PIXEL: c_uint = 0;
pub const PNG_OFFSET_MICROMETER: c_uint = 1;
pub const PNG_OFFSET_LAST: c_uint = 2;
pub const PNG_EQUATION_LINEAR: c_uint = 0;
pub const PNG_EQUATION_BASE_E: c_uint = 1;
pub const PNG_EQUATION_ARBITRARY: c_uint = 2;
pub const PNG_EQUATION_HYPERBOLIC: c_uint = 3;
pub const PNG_EQUATION_LAST: c_uint = 4;
pub const PNG_SCALE_UNKNOWN: c_uint = 0;
pub const PNG_SCALE_METER: c_uint = 1;
pub const PNG_SCALE_RADIAN: c_uint = 2;
pub const PNG_SCALE_LAST: c_uint = 3;
pub const PNG_RESOLUTION_UNKNOWN: c_uint = 0;
pub const PNG_RESOLUTION_METER: c_uint = 1;
pub const PNG_RESOLUTION_LAST: c_uint = 2;
pub const PNG_sRGB_INTENT_PERCEPTUAL: c_uint = 0;
pub const PNG_sRGB_INTENT_RELATIVE: c_uint = 1;
pub const PNG_sRGB_INTENT_SATURATION: c_uint = 2;
pub const PNG_sRGB_INTENT_ABSOLUTE: c_uint = 3;
pub const PNG_sRGB_INTENT_LAST: c_uint = 4;
pub const PNG_KEYWORD_MAX_LENGTH: c_uint = 79;
pub const PNG_MAX_PALETTE_LENGTH: c_uint = 256;
pub const PNG_INFO_gAMA: c_uint = 1;
pub const PNG_INFO_sBIT: c_uint = 2;
pub const PNG_INFO_cHRM: c_uint = 4;
pub const PNG_INFO_PLTE: c_uint = 8;
pub const PNG_INFO_tRNS: c_uint = 16;
pub const PNG_INFO_bKGD: c_uint = 32;
pub const PNG_INFO_hIST: c_uint = 64;
pub const PNG_INFO_pHYs: c_uint = 128;
pub const PNG_INFO_oFFs: c_uint = 256;
pub const PNG_INFO_tIME: c_uint = 512;
pub const PNG_INFO_pCAL: c_uint = 1024;
pub const PNG_INFO_sRGB: c_uint = 2048;
pub const PNG_INFO_iCCP: c_uint = 4096;
pub const PNG_INFO_sPLT: c_uint = 8192;
pub const PNG_INFO_sCAL: c_uint = 16384;
pub const PNG_INFO_IDAT: c_uint = 32768;
pub const PNG_INFO_eXIf: c_uint = 65536;
pub const PNG_TRANSFORM_IDENTITY: c_uint = 0;
pub const PNG_TRANSFORM_STRIP_16: c_uint = 1;
pub const PNG_TRANSFORM_STRIP_ALPHA: c_uint = 2;
pub const PNG_TRANSFORM_PACKING: c_uint = 4;
pub const PNG_TRANSFORM_PACKSWAP: c_uint = 8;
pub const PNG_TRANSFORM_EXPAND: c_uint = 16;
pub const PNG_TRANSFORM_INVERT_MONO: c_uint = 32;
pub const PNG_TRANSFORM_SHIFT: c_uint = 64;
pub const PNG_TRANSFORM_BGR: c_uint = 128;
pub const PNG_TRANSFORM_SWAP_ALPHA: c_uint = 256;
pub const PNG_TRANSFORM_SWAP_ENDIAN: c_uint = 512;
pub const PNG_TRANSFORM_INVERT_ALPHA: c_uint = 1024;
pub const PNG_TRANSFORM_STRIP_FILLER: c_uint = 2048;
pub const PNG_TRANSFORM_STRIP_FILLER_BEFORE: c_uint = 2048;
pub const PNG_TRANSFORM_STRIP_FILLER_AFTER: c_uint = 4096;
pub const PNG_TRANSFORM_GRAY_TO_RGB: c_uint = 8192;
pub const PNG_TRANSFORM_EXPAND_16: c_uint = 16384;
pub const PNG_TRANSFORM_SCALE_16: c_uint = 32768;
pub const PNG_FLAG_MNG_EMPTY_PLTE: c_uint = 1;
pub const PNG_FLAG_MNG_FILTER_64: c_uint = 4;
pub const PNG_ALL_MNG_FEATURES: c_uint = 5;
pub const PNG_ERROR_ACTION_NONE: c_uint = 1;
pub const PNG_ERROR_ACTION_WARN: c_uint = 2;
pub const PNG_ERROR_ACTION_ERROR: c_uint = 3;
pub const PNG_RGB_TO_GRAY_DEFAULT: c_int = -1;
pub const PNG_ALPHA_PNG: c_uint = 0;
pub const PNG_ALPHA_STANDARD: c_uint = 1;
pub const PNG_ALPHA_ASSOCIATED: c_uint = 1;
pub const PNG_ALPHA_PREMULTIPLIED: c_uint = 1;
pub const PNG_ALPHA_OPTIMIZED: c_uint = 2;
pub const PNG_ALPHA_BROKEN: c_uint = 3;
pub const PNG_DEFAULT_sRGB: c_int = -1;
pub const PNG_GAMMA_MAC_18: c_int = -2;
pub const PNG_GAMMA_sRGB: c_uint = 220000;
pub const PNG_GAMMA_LINEAR: c_uint = 100000;
pub const PNG_FILLER_BEFORE: c_uint = 0;
pub const PNG_FILLER_AFTER: c_uint = 1;
pub const PNG_BACKGROUND_GAMMA_UNKNOWN: c_uint = 0;
pub const PNG_BACKGROUND_GAMMA_SCREEN: c_uint = 1;
pub const PNG_BACKGROUND_GAMMA_FILE: c_uint = 2;
pub const PNG_BACKGROUND_GAMMA_UNIQUE: c_uint = 3;
pub const PNG_GAMMA_THRESHOLD: f64 = 0.05;
pub const PNG_CRC_DEFAULT: c_uint = 0;
pub const PNG_CRC_ERROR_QUIT: c_uint = 1;
pub const PNG_CRC_WARN_DISCARD: c_uint = 2;
pub const PNG_CRC_WARN_USE: c_uint = 3;
pub const PNG_CRC_QUIET_USE: c_uint = 4;
pub const PNG_CRC_NO_CHANGE: c_uint = 5;
pub const PNG_NO_FILTERS: c_uint = 0;
pub const PNG_FILTER_NONE: c_uint = 8;
pub const PNG_FILTER_SUB: c_uint = 16;
pub const PNG_FILTER_UP: c_uint = 32;
pub const PNG_FILTER_AVG: c_uint = 64;
pub const PNG_FILTER_PAETH: c_uint = 128;
pub const PNG_FAST_FILTERS: c_uint = 56;
pub const PNG_ALL_FILTERS: c_uint = 248;
pub const PNG_FILTER_VALUE_NONE: c_uint = 0;
pub const PNG_FILTER_VALUE_SUB: c_uint = 1;
pub const PNG_FILTER_VALUE_UP: c_uint = 2;
pub const PNG_FILTER_VALUE_AVG: c_uint = 3;
pub const PNG_FILTER_VALUE_PAETH: c_uint = 4;
pub const PNG_FILTER_VALUE_LAST: c_uint = 5;
pub const PNG_FILTER_HEURISTIC_DEFAULT: c_uint = 0;
pub const PNG_FILTER_HEURISTIC_UNWEIGHTED: c_uint = 1;
pub const PNG_FILTER_HEURISTIC_WEIGHTED: c_uint = 2;
pub const PNG_FILTER_HEURISTIC_LAST: c_uint = 3;
pub const PNG_DESTROY_WILL_FREE_DATA: c_uint = 1;
pub const PNG_SET_WILL_FREE_DATA: c_uint = 1;
pub const PNG_USER_WILL_FREE_DATA: c_uint = 2;
pub const PNG_FREE_HIST: c_uint = 8;
pub const PNG_FREE_ICCP: c_uint = 16;
pub const PNG_FREE_SPLT: c_uint = 32;
pub const PNG_FREE_ROWS: c_uint = 64;
pub const PNG_FREE_PCAL: c_uint = 128;
pub const PNG_FREE_SCAL: c_uint = 256;
pub const PNG_FREE_UNKN: c_uint = 512;
pub const PNG_FREE_PLTE: c_uint = 4096;
pub const PNG_FREE_TRNS: c_uint = 8192;
pub const PNG_FREE_TEXT: c_uint = 16384;
pub const PNG_FREE_EXIF: c_uint = 32768;
pub const PNG_FREE_ALL: c_uint = 65535;
pub const PNG_FREE_MUL: c_uint = 16928;
pub const PNG_HANDLE_CHUNK_AS_DEFAULT: c_uint = 0;
pub const PNG_HANDLE_CHUNK_NEVER: c_uint = 1;
pub const PNG_HANDLE_CHUNK_IF_SAFE: c_uint = 2;
pub const PNG_HANDLE_CHUNK_ALWAYS: c_uint = 3;
pub const PNG_HANDLE_CHUNK_LAST: c_uint = 4;
pub const PNG_IO_NONE: c_uint = 0;
pub const PNG_IO_READING: c_uint = 1;
pub const PNG_IO_WRITING: c_uint = 2;
pub const PNG_IO_SIGNATURE: c_uint = 16;
pub const PNG_IO_CHUNK_HDR: c_uint = 32;
pub const PNG_IO_CHUNK_DATA: c_uint = 64;
pub const PNG_IO_CHUNK_CRC: c_uint = 128;
pub const PNG_IO_MASK_OP: c_uint = 15;
pub const PNG_IO_MASK_LOC: c_uint = 240;
pub const PNG_INTERLACE_ADAM7_PASSES: c_uint = 7;
pub const PNG_IMAGE_VERSION: c_uint = 1;
pub const PNG_IMAGE_WARNING: c_uint = 1;
pub const PNG_IMAGE_ERROR: c_uint = 2;
pub const PNG_FORMAT_FLAG_ALPHA: c_uint = 1;
pub const PNG_FORMAT_FLAG_COLOR: c_uint = 2;
pub const PNG_FORMAT_FLAG_LINEAR: c_uint = 4;
pub const PNG_FORMAT_FLAG_COLORMAP: c_uint = 8;
pub const PNG_FORMAT_FLAG_BGR: c_uint = 16;
pub const PNG_FORMAT_FLAG_AFIRST: c_uint = 32;
pub const PNG_FORMAT_FLAG_ASSOCIATED_ALPHA: c_uint = 64;
pub const PNG_FORMAT_GRAY: c_uint = 0;
pub const PNG_FORMAT_GA: c_uint = 1;
pub const PNG_FORMAT_AG: c_uint = 33;
pub const PNG_FORMAT_RGB: c_uint = 2;
pub const PNG_FORMAT_BGR: c_uint = 18;
pub const PNG_FORMAT_RGBA: c_uint = 3;
pub const PNG_FORMAT_ARGB: c_uint = 35;
pub const PNG_FORMAT_BGRA: c_uint = 19;
pub const PNG_FORMAT_ABGR: c_uint = 51;
pub const PNG_FORMAT_LINEAR_Y: c_uint = 4;
pub const PNG_FORMAT_LINEAR_Y_ALPHA: c_uint = 5;
pub const PNG_FORMAT_LINEAR_RGB: c_uint = 6;
pub const PNG_FORMAT_LINEAR_RGB_ALPHA: c_uint = 7;
pub const PNG_FORMAT_RGB_COLORMAP: c_uint = 10;
pub const PNG_FORMAT_BGR_COLORMAP: c_uint = 26;
pub const PNG_FORMAT_RGBA_COLORMAP: c_uint = 11;
pub const PNG_FORMAT_ARGB_COLORMAP: c_uint = 43;
pub const PNG_FORMAT_BGRA_COLORMAP: c_uint = 27;
pub const PNG_FORMAT_ABGR_COLORMAP: c_uint = 59;
pub const PNG_IMAGE_FLAG_COLORSPACE_NOT_sRGB: c_uint = 1;
pub const PNG_IMAGE_FLAG_FAST: c_uint = 2;
pub const PNG_IMAGE_FLAG_16BIT_sRGB: c_uint = 4;
pub const PNG_MAXIMUM_INFLATE_WINDOW: c_uint = 2;
pub const PNG_SKIP_sRGB_CHECK_PROFILE: c_uint = 4;
pub const PNG_IGNORE_ADLER32: c_uint = 8;
pub const PNG_OPTION_NEXT: c_uint = 12;
pub const PNG_OPTION_UNSET: c_uint = 0;
pub const PNG_OPTION_INVALID: c_uint = 1;
pub const PNG_OPTION_OFF: c_uint = 2;
pub const PNG_OPTION_ON: c_uint = 3;
pub type png_fixed_point = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct png_struct_def {
    _unused: [u8; 0],
}
pub type png_struct = png_struct_def;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct png_info_def {
    _unused: [u8; 0],
}
pub type png_info = png_info_def;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct png_color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
#[test]
#[cfg(target_pointer_width = "64")]
fn bindgen_test_layout_png_color_struct() {
    assert_eq!(::std::mem::size_of::<png_color>(), 3usize, concat!("Size of: ", stringify!(png_color)));
    assert_eq!(::std::mem::align_of::<png_color>(), 1usize, concat!("Alignment of ", stringify!(png_color)));
    assert_eq!(unsafe { &(*(0 as *const png_color)).red as *const _ as usize }, 0usize, concat!("Alignment of field: ", stringify!(png_color), "::", stringify!(red)));
    assert_eq!(unsafe { &(*(0 as *const png_color)).green as *const _ as usize }, 1usize, concat!("Alignment of field: ", stringify!(png_color), "::", stringify!(green)));
    assert_eq!(unsafe { &(*(0 as *const png_color)).blue as *const _ as usize }, 2usize, concat!("Alignment of field: ", stringify!(png_color), "::", stringify!(blue)));
}
impl Clone for png_color {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct png_color_16 {
    pub index: u8,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub gray: u16,
}
#[test]
#[cfg(target_pointer_width = "64")]
fn bindgen_test_layout_png_color_16_struct() {
    assert_eq!(::std::mem::size_of::<png_color_16>(), 10usize, concat!("Size of: ", stringify!(png_color_16)));
    assert_eq!(::std::mem::align_of::<png_color_16>(), 2usize, concat!("Alignment of ", stringify!(png_color_16)));
    assert_eq!(unsafe { &(*(0 as *const png_color_16)).index as *const _ as usize }, 0usize, concat!("Alignment of field: ", stringify!(png_color_16), "::", stringify!(index)));
    assert_eq!(unsafe { &(*(0 as *const png_color_16)).red as *const _ as usize }, 2usize, concat!("Alignment of field: ", stringify!(png_color_16), "::", stringify!(red)));
    assert_eq!(unsafe { &(*(0 as *const png_color_16)).green as *const _ as usize }, 4usize, concat!("Alignment of field: ", stringify!(png_color_16), "::", stringify!(green)));
    assert_eq!(unsafe { &(*(0 as *const png_color_16)).blue as *const _ as usize }, 6usize, concat!("Alignment of field: ", stringify!(png_color_16), "::", stringify!(blue)));
    assert_eq!(unsafe { &(*(0 as *const png_color_16)).gray as *const _ as usize }, 8usize, concat!("Alignment of field: ", stringify!(png_color_16), "::", stringify!(gray)));
}
impl Clone for png_color_16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct png_color_8 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub gray: u8,
    pub alpha: u8,
}
#[test]
#[cfg(target_pointer_width = "64")]
fn bindgen_test_layout_png_color_8_struct() {
    assert_eq!(::std::mem::size_of::<png_color_8>(), 5usize, concat!("Size of: ", stringify!(png_color_8)));
    assert_eq!(::std::mem::align_of::<png_color_8>(), 1usize, concat!("Alignment of ", stringify!(png_color_8)));
    assert_eq!(unsafe { &(*(0 as *const png_color_8)).red as *const _ as usize }, 0usize, concat!("Alignment of field: ", stringify!(png_color_8), "::", stringify!(red)));
    assert_eq!(unsafe { &(*(0 as *const png_color_8)).green as *const _ as usize }, 1usize, concat!("Alignment of field: ", stringify!(png_color_8), "::", stringify!(green)));
    assert_eq!(unsafe { &(*(0 as *const png_color_8)).blue as *const _ as usize }, 2usize, concat!("Alignment of field: ", stringify!(png_color_8), "::", stringify!(blue)));
    assert_eq!(unsafe { &(*(0 as *const png_color_8)).gray as *const _ as usize }, 3usize, concat!("Alignment of field: ", stringify!(png_color_8), "::", stringify!(gray)));
    assert_eq!(unsafe { &(*(0 as *const png_color_8)).alpha as *const _ as usize }, 4usize, concat!("Alignment of field: ", stringify!(png_color_8), "::", stringify!(alpha)));
}
impl Clone for png_color_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct png_sPLT_entry {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub alpha: u16,
    pub frequency: u16,
}
#[test]
#[cfg(target_pointer_width = "64")]
fn bindgen_test_layout_png_sPLT_entry_struct() {
    assert_eq!(::std::mem::size_of::<png_sPLT_entry>(), 10usize, concat!("Size of: ", stringify!(png_sPLT_entry)));
    assert_eq!(::std::mem::align_of::<png_sPLT_entry>(), 2usize, concat!("Alignment of ", stringify!(png_sPLT_entry)));
    assert_eq!(unsafe { &(*(0 as *const png_sPLT_entry)).red as *const _ as usize }, 0usize, concat!("Alignment of field: ", stringify!(png_sPLT_entry), "::", stringify!(red)));
    assert_eq!(unsafe { &(*(0 as *const png_sPLT_entry)).green as *const _ as usize }, 2usize, concat!("Alignment of field: ", stringify!(png_sPLT_entry), "::", stringify!(green)));
    assert_eq!(unsafe { &(*(0 as *const png_sPLT_entry)).blue as *const _ as usize }, 4usize, concat!("Alignment of field: ", stringify!(png_sPLT_entry), "::", stringify!(blue)));
    assert_eq!(unsafe { &(*(0 as *const png_sPLT_entry)).alpha as *const _ as usize }, 6usize, concat!("Alignment of field: ", stringify!(png_sPLT_entry), "::", stringify!(alpha)));
    assert_eq!(unsafe { &(*(0 as *const png_sPLT_entry)).frequency as *const _ as usize }, 8usize, concat!("Alignment of field: ", stringify!(png_sPLT_entry), "::", stringify!(frequency)));
}
impl Clone for png_sPLT_entry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct png_sPLT {
    pub name: *mut c_char,
    pub depth: u8,
    pub entries: *mut png_sPLT_entry,
    pub nentries: i32,
}
#[test]
#[cfg(target_pointer_width = "64")]
fn bindgen_test_layout_png_sPLT_struct() {
    assert_eq!(::std::mem::size_of::<png_sPLT>(), 32usize, concat!("Size of: ", stringify!(png_sPLT)));
    assert_eq!(::std::mem::align_of::<png_sPLT>(), 8usize, concat!("Alignment of ", stringify!(png_sPLT)));
    assert_eq!(unsafe { &(*(0 as *const png_sPLT)).name as *const _ as usize }, 0usize, concat!("Alignment of field: ", stringify!(png_sPLT), "::", stringify!(name)));
    assert_eq!(unsafe { &(*(0 as *const png_sPLT)).depth as *const _ as usize }, 8usize, concat!("Alignment of field: ", stringify!(png_sPLT), "::", stringify!(depth)));
    assert_eq!(unsafe { &(*(0 as *const png_sPLT)).entries as *const _ as usize }, 16usize, concat!("Alignment of field: ", stringify!(png_sPLT), "::", stringify!(entries)));
    assert_eq!(unsafe { &(*(0 as *const png_sPLT)).nentries as *const _ as usize }, 24usize, concat!("Alignment of field: ", stringify!(png_sPLT), "::", stringify!(nentries)));
}
impl Clone for png_sPLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct png_text {
    pub compression: c_int,
    pub key: *mut c_char,
    pub text: *mut c_char,
    pub text_length: usize,
    pub itxt_length: usize,
    pub lang: *mut c_char,
    pub lang_key: *mut c_char,
}
#[test]
#[cfg(target_pointer_width = "64")]
fn bindgen_test_layout_png_text_struct() {
    assert_eq!(::std::mem::size_of::<png_text>(), 56usize, concat!("Size of: ", stringify!(png_text)));
    assert_eq!(::std::mem::align_of::<png_text>(), 8usize, concat!("Alignment of ", stringify!(png_text)));
    assert_eq!(unsafe { &(*(0 as *const png_text)).compression as *const _ as usize }, 0usize, concat!("Alignment of field: ", stringify!(png_text), "::", stringify!(compression)));
    assert_eq!(unsafe { &(*(0 as *const png_text)).key as *const _ as usize }, 8usize, concat!("Alignment of field: ", stringify!(png_text), "::", stringify!(key)));
    assert_eq!(unsafe { &(*(0 as *const png_text)).text as *const _ as usize }, 16usize, concat!("Alignment of field: ", stringify!(png_text), "::", stringify!(text)));
    assert_eq!(unsafe { &(*(0 as *const png_text)).text_length as *const _ as usize }, 24usize, concat!("Alignment of field: ", stringify!(png_text), "::", stringify!(text_length)));
    assert_eq!(unsafe { &(*(0 as *const png_text)).itxt_length as *const _ as usize }, 32usize, concat!("Alignment of field: ", stringify!(png_text), "::", stringify!(itxt_length)));
    assert_eq!(unsafe { &(*(0 as *const png_text)).lang as *const _ as usize }, 40usize, concat!("Alignment of field: ", stringify!(png_text), "::", stringify!(lang)));
    assert_eq!(unsafe { &(*(0 as *const png_text)).lang_key as *const _ as usize }, 48usize, concat!("Alignment of field: ", stringify!(png_text), "::", stringify!(lang_key)));
}
impl Clone for png_text {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct png_time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}
#[test]
#[cfg(target_pointer_width = "64")]
fn bindgen_test_layout_png_time_struct() {
    assert_eq!(::std::mem::size_of::<png_time>(), 8usize, concat!("Size of: ", stringify!(png_time)));
    assert_eq!(::std::mem::align_of::<png_time>(), 2usize, concat!("Alignment of ", stringify!(png_time)));
    assert_eq!(unsafe { &(*(0 as *const png_time)).year as *const _ as usize }, 0usize, concat!("Alignment of field: ", stringify!(png_time), "::", stringify!(year)));
    assert_eq!(unsafe { &(*(0 as *const png_time)).month as *const _ as usize }, 2usize, concat!("Alignment of field: ", stringify!(png_time), "::", stringify!(month)));
    assert_eq!(unsafe { &(*(0 as *const png_time)).day as *const _ as usize }, 3usize, concat!("Alignment of field: ", stringify!(png_time), "::", stringify!(day)));
    assert_eq!(unsafe { &(*(0 as *const png_time)).hour as *const _ as usize }, 4usize, concat!("Alignment of field: ", stringify!(png_time), "::", stringify!(hour)));
    assert_eq!(unsafe { &(*(0 as *const png_time)).minute as *const _ as usize }, 5usize, concat!("Alignment of field: ", stringify!(png_time), "::", stringify!(minute)));
    assert_eq!(unsafe { &(*(0 as *const png_time)).second as *const _ as usize }, 6usize, concat!("Alignment of field: ", stringify!(png_time), "::", stringify!(second)));
}
impl Clone for png_time {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct png_unknown_chunk {
    pub name: [u8; 5usize],
    pub data: *mut u8,
    pub size: usize,
    pub location: u8,
}
#[test]
#[cfg(target_pointer_width = "64")]
fn bindgen_test_layout_png_unknown_chunk_t() {
    assert_eq!(::std::mem::size_of::<png_unknown_chunk>(), 32usize, concat!("Size of: ", stringify!(png_unknown_chunk)));
    assert_eq!(::std::mem::align_of::<png_unknown_chunk>(), 8usize, concat!("Alignment of ", stringify!(png_unknown_chunk)));
    assert_eq!(unsafe { &(*(0 as *const png_unknown_chunk)).name as *const _ as usize }, 0usize, concat!("Alignment of field: ", stringify!(png_unknown_chunk), "::", stringify!(name)));
    assert_eq!(unsafe { &(*(0 as *const png_unknown_chunk)).data as *const _ as usize }, 8usize, concat!("Alignment of field: ", stringify!(png_unknown_chunk), "::", stringify!(data)));
    assert_eq!(unsafe { &(*(0 as *const png_unknown_chunk)).size as *const _ as usize }, 16usize, concat!("Alignment of field: ", stringify!(png_unknown_chunk), "::", stringify!(size)));
    assert_eq!(unsafe { &(*(0 as *const png_unknown_chunk)).location as *const _ as usize }, 24usize, concat!("Alignment of field: ", stringify!(png_unknown_chunk), "::", stringify!(location)));
}
impl Clone for png_unknown_chunk {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct png_row_info {
    pub width: u32,
    pub rowbytes: usize,
    pub color_type: u8,
    pub bit_depth: u8,
    pub channels: u8,
    pub pixel_depth: u8,
}
#[test]
#[cfg(target_pointer_width = "64")]
fn bindgen_test_layout_png_row_info_struct() {
    assert_eq!(::std::mem::size_of::<png_row_info>(), 24usize, concat!("Size of: ", stringify!(png_row_info)));
    assert_eq!(::std::mem::align_of::<png_row_info>(), 8usize, concat!("Alignment of ", stringify!(png_row_info)));
    assert_eq!(unsafe { &(*(0 as *const png_row_info)).width as *const _ as usize }, 0usize, concat!("Alignment of field: ", stringify!(png_row_info), "::", stringify!(width)));
    assert_eq!(unsafe { &(*(0 as *const png_row_info)).rowbytes as *const _ as usize }, 8usize, concat!("Alignment of field: ", stringify!(png_row_info), "::", stringify!(rowbytes)));
    assert_eq!(unsafe { &(*(0 as *const png_row_info)).color_type as *const _ as usize }, 16usize, concat!("Alignment of field: ", stringify!(png_row_info), "::", stringify!(color_type)));
    assert_eq!(unsafe { &(*(0 as *const png_row_info)).bit_depth as *const _ as usize }, 17usize, concat!("Alignment of field: ", stringify!(png_row_info), "::", stringify!(bit_depth)));
    assert_eq!(unsafe { &(*(0 as *const png_row_info)).channels as *const _ as usize }, 18usize, concat!("Alignment of field: ", stringify!(png_row_info), "::", stringify!(channels)));
    assert_eq!(unsafe { &(*(0 as *const png_row_info)).pixel_depth as *const _ as usize }, 19usize, concat!("Alignment of field: ", stringify!(png_row_info), "::", stringify!(pixel_depth)));
}
impl Clone for png_row_info {
    fn clone(&self) -> Self {
        *self
    }
}
pub type png_error_ptr = Option<unsafe extern "C" fn(arg1: *mut png_struct, arg2: *const c_char)>;
pub type png_rw_ptr = Option<unsafe extern "C" fn(arg1: *mut png_struct, arg2: *mut u8, arg3: usize)>;
pub type png_flush_ptr = Option<unsafe extern "C" fn(arg1: *mut png_struct)>;
pub type png_read_status_ptr = Option<unsafe extern "C" fn(arg1: *mut png_struct, arg2: u32, arg3: c_int)>;
pub type png_write_status_ptr = Option<unsafe extern "C" fn(arg1: *mut png_struct, arg2: u32, arg3: c_int)>;
pub type png_progressive_info_ptr = Option<unsafe extern "C" fn(arg1: *mut png_struct, arg2: *mut png_info)>;
pub type png_progressive_end_ptr = Option<unsafe extern "C" fn(arg1: *mut png_struct, arg2: *mut png_info)>;
pub type png_progressive_row_ptr = Option<unsafe extern "C" fn(arg1: *mut png_struct, arg2: *mut u8, arg3: u32, arg4: c_int)>;
pub type png_user_transform_ptr = Option<unsafe extern "C" fn(arg1: *mut png_struct, arg2: *mut png_row_info, arg3: *mut u8)>;
pub type png_user_chunk_ptr = Option<unsafe extern "C" fn(arg1: *mut png_struct, arg2: *mut png_unknown_chunk) -> c_int>;
pub type png_longjmp_ptr = Option<unsafe extern "C" fn(arg1: *mut c_int, arg2: c_int)>;
pub type png_malloc_ptr = Option<unsafe extern "C" fn(arg1: *mut png_struct, arg2: usize) -> *mut c_void>;
pub type png_free_ptr = Option<unsafe extern "C" fn(arg1: *mut png_struct, arg2: *mut c_void)>;
extern "C" {
    pub fn png_access_version_number() -> u32;
    pub fn png_set_sig_bytes(png_ptr: &mut png_struct, num_bytes: c_int);
    pub fn png_sig_cmp(sig: *const u8, start: usize, num_to_check: usize) -> c_int;
    pub fn png_create_read(user_png_ver: *const c_char, error_ptr: *mut c_void, error_fn: png_error_ptr, warn_fn: png_error_ptr) -> *mut png_struct;
    pub fn png_create_write(user_png_ver: *const c_char, error_ptr: *mut c_void, error_fn: png_error_ptr, warn_fn: png_error_ptr) -> *mut png_struct;
    pub fn png_get_compression_buffer_size(png_ptr: &png_struct) -> usize;
    pub fn png_set_compression_buffer_size(png_ptr: &mut png_struct, size: usize);
    pub fn png_set_longjmp_fn(png_ptr: &mut png_struct, longjmp_fn: png_longjmp_ptr, jmp_buf_size: usize) -> *mut jmp_buf;
    pub fn png_longjmp(png_ptr: &png_struct, val: c_int);
    pub fn png_reset_zstream(png_ptr: &mut png_struct) -> c_int;
    pub fn png_create_read_struct_2(
        user_png_ver: *const c_char, error_ptr: *mut c_void, error_fn: png_error_ptr, warn_fn: png_error_ptr, mem_ptr: *mut c_void, malloc_fn: png_malloc_ptr, free_fn: png_free_ptr
    ) -> *mut png_struct;
    pub fn png_create_write_struct_2(
        user_png_ver: *const c_char, error_ptr: *mut c_void, error_fn: png_error_ptr, warn_fn: png_error_ptr, mem_ptr: *mut c_void, malloc_fn: png_malloc_ptr, free_fn: png_free_ptr
    ) -> *mut png_struct;
    pub fn png_write_sig(png_ptr: &mut png_struct);
    pub fn png_write_chunk(png_ptr: &mut png_struct, chunk_name: *const u8, data: *const u8, length: usize);
    pub fn png_write_chunk_start(png_ptr: &mut png_struct, chunk_name: *const u8, length: u32);
    pub fn png_write_chunk_data(png_ptr: &mut png_struct, data: *const u8, length: usize);
    pub fn png_write_chunk_end(png_ptr: &mut png_struct);
    pub fn png_create_info(png_ptr: &png_struct) -> *mut png_info;
    pub fn png_info_init_3(info_ptr: *mut *mut png_info, png_info_struct_size: usize);
    pub fn png_write_info_before_PLTE(png_ptr: &mut png_struct, info_ptr: &png_info);
    pub fn png_write_info(png_ptr: &mut png_struct, info_ptr: &png_info);
    pub fn png_read_info(png_ptr: &mut png_struct, info_ptr: &mut png_info);
    pub fn png_convert_to_rfc1123(png_ptr: &mut png_struct, ptime: *const png_time) -> *const c_char;
    pub fn png_convert_to_rfc1123_buffer(out: *mut c_char, ptime: *const png_time) -> c_int;
    pub fn png_convert_from_struct_tm(ptime: *mut png_time, ttime: *const tm);
    pub fn png_convert_from_time_t(ptime: *mut png_time, ttime: time_t);
    pub fn png_set_expand(png_ptr: &mut png_struct);
    pub fn png_set_expand_gray_1_2_4_to_8(png_ptr: &mut png_struct);
    pub fn png_set_palette_to_rgb(png_ptr: &mut png_struct);
    pub fn png_set_tRNS_to_alpha(png_ptr: &mut png_struct);
    pub fn png_set_expand_16(png_ptr: &mut png_struct);
    pub fn png_set_bgr(png_ptr: &mut png_struct);
    pub fn png_set_gray_to_rgb(png_ptr: &mut png_struct);
    pub fn png_set_rgb_to_gray(png_ptr: &mut png_struct, error_action: c_int, red: f64, green: f64);
    pub fn png_set_rgb_to_gray_fixed(png_ptr: &mut png_struct, error_action: c_int, red: png_fixed_point, green: png_fixed_point);
    pub fn png_get_rgb_to_gray_status(png_ptr: &png_struct) -> u8;
    pub fn png_build_grayscale_palette(bit_depth: c_int, palette: *mut png_color);
    pub fn png_set_alpha_mode(png_ptr: &mut png_struct, mode: c_int, output_gamma: f64);
    pub fn png_set_alpha_mode_fixed(png_ptr: &mut png_struct, mode: c_int, output_gamma: png_fixed_point);
    pub fn png_set_strip_alpha(png_ptr: &mut png_struct);
    pub fn png_set_swap_alpha(png_ptr: &mut png_struct);
    pub fn png_set_invert_alpha(png_ptr: &mut png_struct);
    pub fn png_set_filler(png_ptr: &mut png_struct, filler: u32, flags: c_int);
    pub fn png_set_add_alpha(png_ptr: &mut png_struct, filler: u32, flags: c_int);
    pub fn png_set_swap(png_ptr: &mut png_struct);
    pub fn png_set_packing(png_ptr: &mut png_struct);
    pub fn png_set_packswap(png_ptr: &mut png_struct);
    pub fn png_set_shift(png_ptr: &mut png_struct, true_bits: *const png_color_8);
    pub fn png_set_interlace_handling(png_ptr: &mut png_struct) -> c_int;
    pub fn png_set_invert_mono(png_ptr: &mut png_struct);
    pub fn png_set_background(png_ptr: &mut png_struct, background_color: *const png_color_16, background_gamma_code: c_int, need_expand: c_int, background_gamma: f64);
    pub fn png_set_background_fixed(png_ptr: &mut png_struct, background_color: *const png_color_16, background_gamma_code: c_int, need_expand: c_int, background_gamma: png_fixed_point);
    pub fn png_set_scale_16(png_ptr: &mut png_struct);
    pub fn png_set_strip_16(png_ptr: &mut png_struct);
    pub fn png_set_quantize(png_ptr: &mut png_struct, palette: *mut png_color, num_palette: c_int, maximum_colors: c_int, histogram: *const u16, full_quantize: c_int);
    pub fn png_set_gamma(png_ptr: &mut png_struct, screen_gamma: f64, override_file_gamma: f64);
    pub fn png_set_gamma_fixed(png_ptr: &mut png_struct, screen_gamma: png_fixed_point, override_file_gamma: png_fixed_point);
    pub fn png_set_flush(png_ptr: &mut png_struct, nrows: c_int);
    pub fn png_write_flush(png_ptr: &mut png_struct);
    pub fn png_start_read_image(png_ptr: &mut png_struct);
    pub fn png_read_update_info(png_ptr: &mut png_struct, info_ptr: &mut png_info);
    pub fn png_read_rows(png_ptr: &mut png_struct, row: *mut *mut u8, display_row: *mut *mut u8, num_rows: u32);
    pub fn png_read_row(png_ptr: &mut png_struct, row: *mut u8, display_row: *mut u8);
    pub fn png_read_image(png_ptr: &mut png_struct, image: *mut *mut u8);
    pub fn png_write_row(png_ptr: &mut png_struct, row: *const u8);
    pub fn png_write_rows(png_ptr: &mut png_struct, row: *mut *mut u8, num_rows: u32);
    pub fn png_write_image(png_ptr: &mut png_struct, image: *mut *mut u8);
    pub fn png_write_end(png_ptr: &mut png_struct, info_ptr: &mut png_info);
    pub fn png_read_end(png_ptr: &mut png_struct, info_ptr: &mut png_info);
    pub fn png_destroy_info(png_ptr: &png_struct, info_ptr_ptr: *mut *mut png_info);
    pub fn png_destroy_read(png_ptr_ptr: *mut *mut png_struct, info_ptr_ptr: *mut *mut png_info, end_info_ptr_ptr: *mut *mut png_info);
    pub fn png_destroy_write(png_ptr_ptr: *mut *mut png_struct, info_ptr_ptr: *mut *mut png_info);
    pub fn png_set_crc_action(png_ptr: &mut png_struct, crit_action: c_int, ancil_action: c_int);
    pub fn png_set_filter(png_ptr: &mut png_struct, method: c_int, filters: c_int);
    pub fn png_set_filter_heuristics(png_ptr: &mut png_struct, heuristic_method: c_int, num_weights: c_int, filter_weights: *const f64, filter_costs: *const f64);
    pub fn png_set_filter_heuristics_fixed(png_ptr: &mut png_struct, heuristic_method: c_int, num_weights: c_int, filter_weights: *const png_fixed_point, filter_costs: *const png_fixed_point);
    pub fn png_set_compression_level(png_ptr: &mut png_struct, level: c_int);
    pub fn png_set_compression_mem_level(png_ptr: &mut png_struct, mem_level: c_int);
    pub fn png_set_compression_strategy(png_ptr: &mut png_struct, strategy: c_int);
    pub fn png_set_compression_window_bits(png_ptr: &mut png_struct, window_bits: c_int);
    pub fn png_set_compression_method(png_ptr: &mut png_struct, method: c_int);
    pub fn png_set_text_compression_level(png_ptr: &mut png_struct, level: c_int);
    pub fn png_set_text_compression_mem_level(png_ptr: &mut png_struct, mem_level: c_int);
    pub fn png_set_text_compression_strategy(png_ptr: &mut png_struct, strategy: c_int);
    pub fn png_set_text_compression_window_bits(png_ptr: &mut png_struct, window_bits: c_int);
    pub fn png_set_text_compression_method(png_ptr: &mut png_struct, method: c_int);
    pub fn png_init_io(png_ptr: &mut png_struct, fp: *mut FILE);
    pub fn png_set_error_fn(png_ptr: &mut png_struct, error_ptr: *mut c_void, error_fn: png_error_ptr, warning_fn: png_error_ptr);
    pub fn png_get_error_ptr(png_ptr: &png_struct) -> *mut c_void;
    pub fn png_set_write_fn(png_ptr: &mut png_struct, io_ptr: *mut c_void, write_data_fn: png_rw_ptr, output_flush_fn: png_flush_ptr);
    pub fn png_set_read_fn(png_ptr: &mut png_struct, io_ptr: *mut c_void, read_data_fn: png_rw_ptr);
    pub fn png_get_io_ptr(png_ptr: &png_struct) -> *mut c_void;
    pub fn png_set_read_status_fn(png_ptr: &mut png_struct, read_row_fn: png_read_status_ptr);
    pub fn png_set_write_status_fn(png_ptr: &mut png_struct, write_row_fn: png_write_status_ptr);
    pub fn png_set_mem_fn(png_ptr: &mut png_struct, mem_ptr: *mut c_void, malloc_fn: png_malloc_ptr, free_fn: png_free_ptr);
    pub fn png_get_mem_ptr(png_ptr: &png_struct) -> *mut c_void;
    pub fn png_set_read_user_transform_fn(png_ptr: &mut png_struct, read_user_transform_fn: png_user_transform_ptr);
    pub fn png_set_write_user_transform_fn(png_ptr: &mut png_struct, write_user_transform_fn: png_user_transform_ptr);
    pub fn png_set_user_transform_info(png_ptr: &mut png_struct, user_transform_ptr: *mut c_void, user_transform_depth: c_int, user_transform_channels: c_int);
    pub fn png_get_user_transform_ptr(png_ptr: &png_struct) -> *mut c_void;
    pub fn png_get_current_row_number(arg1: *const png_struct) -> u32;
    pub fn png_get_current_pass_number(arg1: *const png_struct) -> u8;
    pub fn png_set_read_user_chunk_fn(png_ptr: &mut png_struct, user_chunk_ptr: *mut c_void, read_user_chunk_fn: png_user_chunk_ptr);
    pub fn png_get_user_chunk_ptr(png_ptr: &png_struct) -> *mut c_void;
    pub fn png_set_progressive_read_fn(png_ptr: &mut png_struct, progressive_ptr: *mut c_void, info_fn: png_progressive_info_ptr, row_fn: png_progressive_row_ptr, end_fn: png_progressive_end_ptr);
    pub fn png_get_progressive_ptr(png_ptr: &png_struct) -> *mut c_void;
    pub fn png_process_data(png_ptr: &mut png_struct, info_ptr: &mut png_info, buffer: *mut u8, buffer_size: usize);
    pub fn png_process_data_pause(arg1: *mut png_struct, save: c_int) -> usize;
    pub fn png_process_data_skip(arg1: *mut png_struct) -> u32;
    pub fn png_progressive_combine_row(png_ptr: &png_struct, old_row: *mut u8, new_row: *const u8);
    pub fn png_malloc(png_ptr: &png_struct, size: usize) -> *mut c_void;
    pub fn png_calloc(png_ptr: &png_struct, size: usize) -> *mut c_void;
    pub fn png_malloc_warn(png_ptr: &png_struct, size: usize) -> *mut c_void;
    pub fn png_free(png_ptr: &png_struct, ptr: *mut c_void);
    pub fn png_free_data(png_ptr: &png_struct, info_ptr: &mut png_info, free_me: u32, num: c_int);
    pub fn png_data_freer(png_ptr: &png_struct, info_ptr: &mut png_info, freer: c_int, mask: u32);
    pub fn png_malloc_default(png_ptr: &png_struct, size: usize) -> *mut c_void;
    pub fn png_free_default(png_ptr: &png_struct, ptr: *mut c_void);
    pub fn png_error(png_ptr: &png_struct, error_message: *const c_char);
    pub fn png_chunk_error(png_ptr: &png_struct, error_message: *const c_char);
    pub fn png_warning(png_ptr: &png_struct, warning_message: *const c_char);
    pub fn png_chunk_warning(png_ptr: &png_struct, warning_message: *const c_char);
    pub fn png_benign_error(png_ptr: &png_struct, warning_message: *const c_char);
    pub fn png_chunk_benign_error(png_ptr: &png_struct, warning_message: *const c_char);
    pub fn png_set_benign_errors(png_ptr: &mut png_struct, allowed: c_int);
    pub fn png_get_valid(png_ptr: &png_struct, info_ptr: &png_info, flag: u32) -> u32;
    pub fn png_get_rowbytes(png_ptr: &png_struct, info_ptr: &png_info) -> usize;
    pub fn png_get_rows(png_ptr: &png_struct, info_ptr: &png_info) -> *mut *mut u8;
    pub fn png_set_rows(png_ptr: &png_struct, info_ptr: &mut png_info, row_pointers: *mut *mut u8);
    pub fn png_get_channels(png_ptr: &png_struct, info_ptr: &png_info) -> u8;
    pub fn png_get_image_width(png_ptr: &png_struct, info_ptr: &png_info) -> u32;
    pub fn png_get_image_height(png_ptr: &png_struct, info_ptr: &png_info) -> u32;
    pub fn png_get_bit_depth(png_ptr: &png_struct, info_ptr: &png_info) -> u8;
    pub fn png_get_color_type(png_ptr: &png_struct, info_ptr: &png_info) -> u8;
    pub fn png_get_filter_type(png_ptr: &png_struct, info_ptr: &png_info) -> u8;
    pub fn png_get_interlace_type(png_ptr: &png_struct, info_ptr: &png_info) -> u8;
    pub fn png_get_compression_type(png_ptr: &png_struct, info_ptr: &png_info) -> u8;
    pub fn png_get_pixels_per_meter(png_ptr: &png_struct, info_ptr: &png_info) -> u32;
    pub fn png_get_x_pixels_per_meter(png_ptr: &png_struct, info_ptr: &png_info) -> u32;
    pub fn png_get_y_pixels_per_meter(png_ptr: &png_struct, info_ptr: &png_info) -> u32;
    pub fn png_get_pixel_aspect_ratio(png_ptr: &png_struct, info_ptr: &png_info) -> f32;
    pub fn png_get_pixel_aspect_ratio_fixed(png_ptr: &png_struct, info_ptr: &png_info) -> png_fixed_point;
    pub fn png_get_x_offset_pixels(png_ptr: &png_struct, info_ptr: &png_info) -> i32;
    pub fn png_get_y_offset_pixels(png_ptr: &png_struct, info_ptr: &png_info) -> i32;
    pub fn png_get_x_offset_microns(png_ptr: &png_struct, info_ptr: &png_info) -> i32;
    pub fn png_get_y_offset_microns(png_ptr: &png_struct, info_ptr: &png_info) -> i32;
    pub fn png_get_signature(png_ptr: &png_struct, info_ptr: &png_info) -> *const u8;
    pub fn png_get_bKGD(png_ptr: &png_struct, info_ptr: &mut png_info, background: *mut *mut png_color_16) -> u32;
    pub fn png_set_bKGD(png_ptr: &png_struct, info_ptr: &mut png_info, background: *const png_color_16);
    pub fn png_get_cHRM(
        png_ptr: &png_struct, info_ptr: &png_info, white_x: &mut f64, white_y: &mut f64, red_x: &mut f64, red_y: &mut f64, green_x: &mut f64, green_y: &mut f64, blue_x: &mut f64, blue_y: &mut f64
    ) -> u32;
    pub fn png_get_cHRM_XYZ(
        png_ptr: &png_struct, info_ptr: &png_info, red_X: &mut f64, red_Y: &mut f64, red_Z: &mut f64, green_X: &mut f64, green_Y: &mut f64, green_Z: &mut f64, blue_X: &mut f64, blue_Y: &mut f64,
        blue_Z: &mut f64,
    ) -> u32;
    pub fn png_get_cHRM_fixed(
        png_ptr: &png_struct, info_ptr: &png_info, int_white_x: &mut png_fixed_point, int_white_y: &mut png_fixed_point, int_red_x: &mut png_fixed_point, int_red_y: &mut png_fixed_point,
        int_green_x: &mut png_fixed_point, int_green_y: &mut png_fixed_point, int_blue_x: &mut png_fixed_point, int_blue_y: &mut png_fixed_point,
    ) -> u32;
    pub fn png_get_cHRM_XYZ_fixed(
        png_ptr: &png_struct, info_ptr: &png_info, int_red_X: &mut png_fixed_point, int_red_Y: &mut png_fixed_point, int_red_Z: &mut png_fixed_point, int_green_X: &mut png_fixed_point,
        int_green_Y: &mut png_fixed_point, int_green_Z: &mut png_fixed_point, int_blue_X: &mut png_fixed_point, int_blue_Y: &mut png_fixed_point, int_blue_Z: &mut png_fixed_point,
    ) -> u32;
    pub fn png_set_cHRM(png_ptr: &png_struct, info_ptr: &mut png_info, white_x: f64, white_y: f64, red_x: f64, red_y: f64, green_x: f64, green_y: f64, blue_x: f64, blue_y: f64);
    pub fn png_set_cHRM_XYZ(png_ptr: &png_struct, info_ptr: &mut png_info, red_X: f64, red_Y: f64, red_Z: f64, green_X: f64, green_Y: f64, green_Z: f64, blue_X: f64, blue_Y: f64, blue_Z: f64);
    pub fn png_set_cHRM_fixed(
        png_ptr: &png_struct, info_ptr: &mut png_info, int_white_x: png_fixed_point, int_white_y: png_fixed_point, int_red_x: png_fixed_point, int_red_y: png_fixed_point,
        int_green_x: png_fixed_point, int_green_y: png_fixed_point, int_blue_x: png_fixed_point, int_blue_y: png_fixed_point,
    );
    pub fn png_set_cHRM_XYZ_fixed(
        png_ptr: &png_struct, info_ptr: &mut png_info, int_red_X: png_fixed_point, int_red_Y: png_fixed_point, int_red_Z: png_fixed_point, int_green_X: png_fixed_point, int_green_Y: png_fixed_point,
        int_green_Z: png_fixed_point, int_blue_X: png_fixed_point, int_blue_Y: png_fixed_point, int_blue_Z: png_fixed_point,
    );
    pub fn png_get_gAMA(png_ptr: &png_struct, info_ptr: &png_info, file_gamma: &mut f64) -> u32;
    pub fn png_get_gAMA_fixed(png_ptr: &png_struct, info_ptr: &png_info, int_file_gamma: *mut png_fixed_point) -> u32;
    pub fn png_set_gAMA(png_ptr: &png_struct, info_ptr: &mut png_info, file_gamma: f64);
    pub fn png_set_gAMA_fixed(png_ptr: &png_struct, info_ptr: &mut png_info, int_file_gamma: png_fixed_point);
    pub fn png_get_hIST(png_ptr: &png_struct, info_ptr: &mut png_info, hist: *mut *mut u16) -> u32;
    pub fn png_set_hIST(png_ptr: &png_struct, info_ptr: &mut png_info, hist: *const u16);
    pub fn png_get_IHDR(
        png_ptr: &png_struct, info_ptr: &png_info, width: &mut u32, height: &mut u32, bit_depth: &mut c_int, color_type: &mut c_int, interlace_method: &mut c_int, compression_method: &mut c_int,
        filter_method: &mut c_int,
    ) -> u32;
    pub fn png_set_IHDR(
        png_ptr: &png_struct, info_ptr: &mut png_info, width: u32, height: u32, bit_depth: c_int, color_type: c_int, interlace_method: c_int, compression_method: c_int, filter_method: c_int
    );
    pub fn png_get_oFFs(png_ptr: &png_struct, info_ptr: &png_info, offset_x: &mut i32, offset_y: &mut i32, unit_type: &mut c_int) -> u32;
    pub fn png_set_oFFs(png_ptr: &png_struct, info_ptr: &mut png_info, offset_x: i32, offset_y: i32, unit_type: c_int);
    pub fn png_get_pCAL(
        png_ptr: &png_struct, info_ptr: &mut png_info, purpose: *mut *mut c_char, X0: &mut i32, X1: &mut i32, type_: &mut c_int, nparams: &mut c_int, units: *mut *mut c_char,
        params: *mut *mut *mut c_char,
    ) -> u32;
    pub fn png_set_pCAL(png_ptr: &png_struct, info_ptr: &mut png_info, purpose: *const c_char, X0: i32, X1: i32, type_: c_int, nparams: c_int, units: *const c_char, params: *mut *mut c_char);
    pub fn png_get_pHYs(png_ptr: &png_struct, info_ptr: &png_info, res_x: &mut u32, res_y: &mut u32, unit_type: &mut c_int) -> u32;
    pub fn png_set_pHYs(png_ptr: &png_struct, info_ptr: &mut png_info, res_x: u32, res_y: u32, unit_type: c_int);
    pub fn png_get_PLTE(png_ptr: &png_struct, info_ptr: &mut png_info, palette: *mut *mut png_color, num_palette: &mut c_int) -> u32;
    pub fn png_set_PLTE(png_ptr: &mut png_struct, info_ptr: &mut png_info, palette: *const png_color, num_palette: c_int);
    pub fn png_get_sBIT(png_ptr: &png_struct, info_ptr: &mut png_info, sig_bit: *mut *mut png_color_8) -> u32;
    pub fn png_set_sBIT(png_ptr: &png_struct, info_ptr: &mut png_info, sig_bit: &png_color_8);
    pub fn png_get_sRGB(png_ptr: &png_struct, info_ptr: &png_info, file_srgb_intent: &mut c_int) -> u32;
    pub fn png_set_sRGB(png_ptr: &png_struct, info_ptr: &mut png_info, srgb_intent: c_int);
    pub fn png_set_sRGB_gAMA_and_cHRM(png_ptr: &png_struct, info_ptr: &mut png_info, srgb_intent: c_int);
    pub fn png_get_iCCP(png_ptr: &png_struct, info_ptr: &mut png_info, name: *mut *mut c_char, compression_type: &mut c_int, profile: *mut *mut u8, proflen: &mut u32) -> u32;
    pub fn png_set_iCCP(png_ptr: &png_struct, info_ptr: &mut png_info, name: *const c_char, compression_type: c_int, profile: *const u8, proflen: u32);
    pub fn png_get_sPLT(png_ptr: &png_struct, info_ptr: &mut png_info, entries: *mut *mut png_sPLT) -> c_int;
    pub fn png_set_sPLT(png_ptr: &png_struct, info_ptr: &mut png_info, entries: *const png_sPLT, nentries: c_int);
    pub fn png_get_text(png_ptr: &png_struct, info_ptr: &mut png_info, text_ptr: *mut *mut png_text, num_text: &mut c_int) -> c_int;
    pub fn png_set_text(png_ptr: &png_struct, info_ptr: &mut png_info, text_ptr: *const png_text, num_text: c_int);
    pub fn png_get_tIME(png_ptr: &png_struct, info_ptr: &mut png_info, mod_time: *mut *mut png_time) -> u32;
    pub fn png_set_tIME(png_ptr: &png_struct, info_ptr: &mut png_info, mod_time: &png_time);
    pub fn png_get_tRNS(png_ptr: &png_struct, info_ptr: &mut png_info, trans_alpha: *mut *mut u8, num_trans: &mut c_int, trans_color: *mut *mut png_color_16) -> u32;
    pub fn png_set_tRNS(png_ptr: &mut png_struct, info_ptr: &mut png_info, trans_alpha: *const u8, num_trans: c_int, trans_color: *const png_color_16);
    pub fn png_get_sCAL(png_ptr: &png_struct, info_ptr: &png_info, unit: &mut c_int, width: &mut f64, height: &mut f64) -> u32;
    pub fn png_get_sCAL_fixed(png_ptr: &png_struct, info_ptr: &png_info, unit: &mut c_int, width: *mut png_fixed_point, height: *mut png_fixed_point) -> u32;
    pub fn png_get_sCAL_s(png_ptr: &png_struct, info_ptr: &png_info, unit: &mut c_int, swidth: *mut *mut c_char, sheight: *mut *mut c_char) -> u32;
    pub fn png_set_sCAL(png_ptr: &png_struct, info_ptr: &mut png_info, unit: c_int, width: f64, height: f64);
    pub fn png_set_sCAL_fixed(png_ptr: &png_struct, info_ptr: &mut png_info, unit: c_int, width: png_fixed_point, height: png_fixed_point);
    pub fn png_set_sCAL_s(png_ptr: &png_struct, info_ptr: &mut png_info, unit: c_int, swidth: *const c_char, sheight: *const c_char);
    pub fn png_set_keep_unknown_chunks(png_ptr: &mut png_struct, keep: c_int, chunk_list: *const u8, num_chunks: c_int);
    pub fn png_handle_as_unknown(png_ptr: &png_struct, chunk_name: *const u8) -> c_int;
    pub fn png_set_unknown_chunks(png_ptr: &png_struct, info_ptr: &mut png_info, unknowns: *const png_unknown_chunk, num_unknowns: c_int);
    pub fn png_set_unknown_chunk_location(png_ptr: &png_struct, info_ptr: &mut png_info, chunk: c_int, location: c_int);
    pub fn png_get_unknown_chunks(png_ptr: &png_struct, info_ptr: &mut png_info, entries: *mut *mut png_unknown_chunk) -> c_int;
    pub fn png_set_invalid(png_ptr: &png_struct, info_ptr: &mut png_info, mask: c_int);
    pub fn png_read_png(png_ptr: &mut png_struct, info_ptr: &mut png_info, transforms: c_int, params: *mut c_void);
    pub fn png_write_png(png_ptr: &mut png_struct, info_ptr: &mut png_info, transforms: c_int, params: *mut c_void);
    pub fn png_get_copyright(png_ptr: &png_struct) -> *const c_char;
    pub fn png_get_header_ver(png_ptr: &png_struct) -> *const c_char;
    pub fn png_get_header_version(png_ptr: &png_struct) -> *const c_char;
    pub fn png_get_libpng_ver(png_ptr: &png_struct) -> *const c_char;
    pub fn png_permit_mng_features(png_ptr: &mut png_struct, mng_features_permitted: u32) -> u32;
    pub fn png_set_user_limits(png_ptr: &mut png_struct, user_width_max: u32, user_height_max: u32);
    pub fn png_get_user_width_max(png_ptr: &png_struct) -> u32;
    pub fn png_get_user_height_max(png_ptr: &png_struct) -> u32;
    pub fn png_set_chunk_cache_max(png_ptr: &mut png_struct, user_chunk_cache_max: u32);
    pub fn png_get_chunk_cache_max(png_ptr: &png_struct) -> u32;
    pub fn png_set_chunk_malloc_max(png_ptr: &mut png_struct, user_chunk_cache_max: usize);
    pub fn png_get_chunk_malloc_max(png_ptr: &png_struct) -> usize;
    pub fn png_get_pixels_per_inch(png_ptr: &png_struct, info_ptr: &png_info) -> u32;
    pub fn png_get_x_pixels_per_inch(png_ptr: &png_struct, info_ptr: &png_info) -> u32;
    pub fn png_get_y_pixels_per_inch(png_ptr: &png_struct, info_ptr: &png_info) -> u32;
    pub fn png_get_x_offset_inches(png_ptr: &png_struct, info_ptr: &png_info) -> f32;
    pub fn png_get_x_offset_inches_fixed(png_ptr: &png_struct, info_ptr: &png_info) -> png_fixed_point;
    pub fn png_get_y_offset_inches(png_ptr: &png_struct, info_ptr: &png_info) -> f32;
    pub fn png_get_y_offset_inches_fixed(png_ptr: &png_struct, info_ptr: &png_info) -> png_fixed_point;
    pub fn png_get_pHYs_dpi(png_ptr: &png_struct, info_ptr: &png_info, res_x: &mut u32, res_y: &mut u32, unit_type: &mut c_int) -> u32;
    pub fn png_get_io_state(png_ptr: &png_struct) -> u32;
    pub fn png_get_io_chunk_type(png_ptr: &png_struct) -> u32;
    pub fn png_get_uint_32(buf: *const u8) -> u32;
    pub fn png_get_uint_16(buf: *const u8) -> u16;
    pub fn png_get_int_32(buf: *const u8) -> i32;
    pub fn png_get_uint_31(png_ptr: &png_struct, buf: *const u8) -> u32;
    pub fn png_save_uint_32(buf: *mut u8, i: u32);
    pub fn png_save_int_32(buf: *mut u8, i: i32);
    pub fn png_save_uint_16(buf: *mut u8, i: c_uint);
    pub fn png_set_check_for_invalid_index(png_ptr: &mut png_struct, allowed: c_int);
    pub fn png_get_palette_max(png_ptr: &png_struct, info_ptr: &png_info) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct png_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct png_image {
    pub opaque: *mut png_control,
    pub version: u32,
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub flags: u32,
    pub colormap_entries: u32,
    pub warning_or_error: u32,
    pub message: [c_char; 64usize],
}
#[test]
#[cfg(target_pointer_width = "64")]
fn bindgen_test_layout_png_image() {
    assert_eq!(::std::mem::size_of::<png_image>(), 104usize, concat!("Size of: ", stringify!(png_image)));
    assert_eq!(::std::mem::align_of::<png_image>(), 8usize, concat!("Alignment of ", stringify!(png_image)));
    assert_eq!(unsafe { &(*(0 as *const png_image)).opaque as *const _ as usize }, 0usize, concat!("Alignment of field: ", stringify!(png_image), "::", stringify!(opaque)));
    assert_eq!(unsafe { &(*(0 as *const png_image)).version as *const _ as usize }, 8usize, concat!("Alignment of field: ", stringify!(png_image), "::", stringify!(version)));
    assert_eq!(unsafe { &(*(0 as *const png_image)).width as *const _ as usize }, 12usize, concat!("Alignment of field: ", stringify!(png_image), "::", stringify!(width)));
    assert_eq!(unsafe { &(*(0 as *const png_image)).height as *const _ as usize }, 16usize, concat!("Alignment of field: ", stringify!(png_image), "::", stringify!(height)));
    assert_eq!(unsafe { &(*(0 as *const png_image)).format as *const _ as usize }, 20usize, concat!("Alignment of field: ", stringify!(png_image), "::", stringify!(format)));
    assert_eq!(unsafe { &(*(0 as *const png_image)).flags as *const _ as usize }, 24usize, concat!("Alignment of field: ", stringify!(png_image), "::", stringify!(flags)));
    assert_eq!(unsafe { &(*(0 as *const png_image)).colormap_entries as *const _ as usize }, 28usize, concat!("Alignment of field: ", stringify!(png_image), "::", stringify!(colormap_entries)));
    assert_eq!(unsafe { &(*(0 as *const png_image)).warning_or_error as *const _ as usize }, 32usize, concat!("Alignment of field: ", stringify!(png_image), "::", stringify!(warning_or_error)));
    assert_eq!(unsafe { &(*(0 as *const png_image)).message as *const _ as usize }, 36usize, concat!("Alignment of field: ", stringify!(png_image), "::", stringify!(message)));
}
extern "C" {
    pub fn png_image_begin_read_from_file(image: *mut png_image, file_name: *const c_char) -> c_int;
    pub fn png_image_begin_read_from_stdio(image: *mut png_image, file: *mut FILE) -> c_int;
    pub fn png_image_begin_read_from_memory(image: *mut png_image, memory: *const c_void, size: usize) -> c_int;
    pub fn png_image_finish_read(image: *mut png_image, background: *const png_color, buffer: *mut c_void, row_stride: i32, colormap: *mut c_void) -> c_int;
    pub fn png_image_free(image: *mut png_image);
    pub fn png_image_write_to_file(image: *mut png_image, file: *const c_char, convert_to_8bit: c_int, buffer: *const c_void, row_stride: i32, colormap: *const c_void) -> c_int;
    pub fn png_image_write_to_stdio(image: *mut png_image, file: *mut FILE, convert_to_8_bit: c_int, buffer: *const c_void, row_stride: i32, colormap: *const c_void) -> c_int;
    pub fn png_image_write_to_memory(
        image: *mut png_image, memory: *mut c_void, memory_bytes: *mut usize, convert_to_8_bit: c_int, buffer: *const c_void, row_stride: i32, colormap: *const c_void
    ) -> c_int;
    pub fn png_set_option(png_ptr: &mut png_struct, option: c_int, onoff: c_int) -> c_int;
}

#[test]
#[cfg(target_pointer_width = "32")]
fn bindgen_test_32_layout_png_color() {
    assert_eq!(::std::mem::size_of::<png_color>(), 3usize, concat!("Size of: ", stringify!(png_color)));
    assert_eq!(::std::mem::align_of::<png_color>(), 1usize, concat!("Alignment of ", stringify!(png_color)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color>())).red as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(png_color), "::", stringify!(red)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color>())).green as *const _ as usize }, 1usize, concat!("Offset of field: ", stringify!(png_color), "::", stringify!(green)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color>())).blue as *const _ as usize }, 2usize, concat!("Offset of field: ", stringify!(png_color), "::", stringify!(blue)));
}

#[test]
#[cfg(target_pointer_width = "32")]
fn bindgen_test_32_layout_png_color_16() {
    assert_eq!(::std::mem::size_of::<png_color_16>(), 10usize, concat!("Size of: ", stringify!(png_color_16)));
    assert_eq!(::std::mem::align_of::<png_color_16>(), 2usize, concat!("Alignment of ", stringify!(png_color_16)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color_16>())).index as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(png_color_16), "::", stringify!(index)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color_16>())).red as *const _ as usize }, 2usize, concat!("Offset of field: ", stringify!(png_color_16), "::", stringify!(red)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color_16>())).green as *const _ as usize }, 4usize, concat!("Offset of field: ", stringify!(png_color_16), "::", stringify!(green)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color_16>())).blue as *const _ as usize }, 6usize, concat!("Offset of field: ", stringify!(png_color_16), "::", stringify!(blue)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color_16>())).gray as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(png_color_16), "::", stringify!(gray)));
}

#[test]
#[cfg(target_pointer_width = "32")]
fn bindgen_test_32_layout_png_color_8() {
    assert_eq!(::std::mem::size_of::<png_color_8>(), 5usize, concat!("Size of: ", stringify!(png_color_8)));
    assert_eq!(::std::mem::align_of::<png_color_8>(), 1usize, concat!("Alignment of ", stringify!(png_color_8)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color_8>())).red as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(png_color_8), "::", stringify!(red)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color_8>())).green as *const _ as usize }, 1usize, concat!("Offset of field: ", stringify!(png_color_8), "::", stringify!(green)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color_8>())).blue as *const _ as usize }, 2usize, concat!("Offset of field: ", stringify!(png_color_8), "::", stringify!(blue)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color_8>())).gray as *const _ as usize }, 3usize, concat!("Offset of field: ", stringify!(png_color_8), "::", stringify!(gray)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_color_8>())).alpha as *const _ as usize }, 4usize, concat!("Offset of field: ", stringify!(png_color_8), "::", stringify!(alpha)));
}

#[test]
#[cfg(target_pointer_width = "32")]
fn bindgen_test_32_layout_png_sPLT_entry() {
    assert_eq!(::std::mem::size_of::<png_sPLT_entry>(), 10usize, concat!("Size of: ", stringify!(png_sPLT_entry)));
    assert_eq!(::std::mem::align_of::<png_sPLT_entry>(), 2usize, concat!("Alignment of ", stringify!(png_sPLT_entry)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_sPLT_entry>())).red as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(png_sPLT_entry), "::", stringify!(red)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_sPLT_entry>())).green as *const _ as usize }, 2usize, concat!("Offset of field: ", stringify!(png_sPLT_entry), "::", stringify!(green)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_sPLT_entry>())).blue as *const _ as usize }, 4usize, concat!("Offset of field: ", stringify!(png_sPLT_entry), "::", stringify!(blue)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_sPLT_entry>())).alpha as *const _ as usize }, 6usize, concat!("Offset of field: ", stringify!(png_sPLT_entry), "::", stringify!(alpha)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_sPLT_entry>())).frequency as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(png_sPLT_entry), "::", stringify!(frequency)));
}

#[test]
#[cfg(target_pointer_width = "32")]
fn bindgen_test_32_layout_png_sPLT() {
    assert_eq!(::std::mem::size_of::<png_sPLT>(), 16usize, concat!("Size of: ", stringify!(png_sPLT)));
    assert_eq!(::std::mem::align_of::<png_sPLT>(), 4usize, concat!("Alignment of ", stringify!(png_sPLT)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_sPLT>())).name as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(png_sPLT), "::", stringify!(name)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_sPLT>())).depth as *const _ as usize }, 4usize, concat!("Offset of field: ", stringify!(png_sPLT), "::", stringify!(depth)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_sPLT>())).entries as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(png_sPLT), "::", stringify!(entries)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_sPLT>())).nentries as *const _ as usize }, 12usize, concat!("Offset of field: ", stringify!(png_sPLT), "::", stringify!(nentries)));
}

#[test]
#[cfg(target_pointer_width = "32")]
fn bindgen_test_32_layout_png_text() {
    assert_eq!(::std::mem::size_of::<png_text>(), 28usize, concat!("Size of: ", stringify!(png_text)));
    assert_eq!(::std::mem::align_of::<png_text>(), 4usize, concat!("Alignment of ", stringify!(png_text)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_text>())).compression as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(png_text), "::", stringify!(compression)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_text>())).key as *const _ as usize }, 4usize, concat!("Offset of field: ", stringify!(png_text), "::", stringify!(key)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_text>())).text as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(png_text), "::", stringify!(text)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_text>())).text_length as *const _ as usize }, 12usize, concat!("Offset of field: ", stringify!(png_text), "::", stringify!(text_length)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_text>())).itxt_length as *const _ as usize }, 16usize, concat!("Offset of field: ", stringify!(png_text), "::", stringify!(itxt_length)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_text>())).lang as *const _ as usize }, 20usize, concat!("Offset of field: ", stringify!(png_text), "::", stringify!(lang)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_text>())).lang_key as *const _ as usize }, 24usize, concat!("Offset of field: ", stringify!(png_text), "::", stringify!(lang_key)));
}

#[test]
#[cfg(target_pointer_width = "32")]
fn bindgen_test_32_layout_png_time() {
    assert_eq!(::std::mem::size_of::<png_time>(), 8usize, concat!("Size of: ", stringify!(png_time)));
    assert_eq!(::std::mem::align_of::<png_time>(), 2usize, concat!("Alignment of ", stringify!(png_time)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_time>())).year as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(png_time), "::", stringify!(year)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_time>())).month as *const _ as usize }, 2usize, concat!("Offset of field: ", stringify!(png_time), "::", stringify!(month)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_time>())).day as *const _ as usize }, 3usize, concat!("Offset of field: ", stringify!(png_time), "::", stringify!(day)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_time>())).hour as *const _ as usize }, 4usize, concat!("Offset of field: ", stringify!(png_time), "::", stringify!(hour)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_time>())).minute as *const _ as usize }, 5usize, concat!("Offset of field: ", stringify!(png_time), "::", stringify!(minute)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_time>())).second as *const _ as usize }, 6usize, concat!("Offset of field: ", stringify!(png_time), "::", stringify!(second)));
}

#[test]
#[cfg(target_pointer_width = "32")]
fn bindgen_test_32_layout_png_unknown_chunk_t() {
    assert_eq!(::std::mem::size_of::<png_unknown_chunk>(), 20usize, concat!("Size of: ", stringify!(png_unknown_chunk_t)));
    assert_eq!(::std::mem::align_of::<png_unknown_chunk>(), 4usize, concat!("Alignment of ", stringify!(png_unknown_chunk_t)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_unknown_chunk>())).name as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(png_unknown_chunk_t), "::", stringify!(name)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_unknown_chunk>())).data as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(png_unknown_chunk_t), "::", stringify!(data)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_unknown_chunk>())).size as *const _ as usize }, 12usize, concat!("Offset of field: ", stringify!(png_unknown_chunk_t), "::", stringify!(size)));
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<png_unknown_chunk>())).location as *const _ as usize },
        16usize,
        concat!("Offset of field: ", stringify!(png_unknown_chunk_t), "::", stringify!(location))
    );
}

#[test]
#[cfg(target_pointer_width = "32")]
fn bindgen_test_32_layout_png_row_info() {
    assert_eq!(::std::mem::size_of::<png_row_info>(), 12usize, concat!("Size of: ", stringify!(png_row_info)));
    assert_eq!(::std::mem::align_of::<png_row_info>(), 4usize, concat!("Alignment of ", stringify!(png_row_info)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_row_info>())).width as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(png_row_info), "::", stringify!(width)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_row_info>())).rowbytes as *const _ as usize }, 4usize, concat!("Offset of field: ", stringify!(png_row_info), "::", stringify!(rowbytes)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_row_info>())).color_type as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(png_row_info), "::", stringify!(color_type)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_row_info>())).bit_depth as *const _ as usize }, 9usize, concat!("Offset of field: ", stringify!(png_row_info), "::", stringify!(bit_depth)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_row_info>())).channels as *const _ as usize }, 10usize, concat!("Offset of field: ", stringify!(png_row_info), "::", stringify!(channels)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_row_info>())).pixel_depth as *const _ as usize }, 11usize, concat!("Offset of field: ", stringify!(png_row_info), "::", stringify!(pixel_depth)));
}

#[test]
#[cfg(target_pointer_width = "32")]
fn bindgen_test_32_layout_png_image() {
    assert_eq!(::std::mem::size_of::<png_image>(), 96usize, concat!("Size of: ", stringify!(png_image)));
    assert_eq!(::std::mem::align_of::<png_image>(), 4usize, concat!("Alignment of ", stringify!(png_image)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_image>())).opaque as *const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(png_image), "::", stringify!(opaque)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_image>())).version as *const _ as usize }, 4usize, concat!("Offset of field: ", stringify!(png_image), "::", stringify!(version)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_image>())).width as *const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(png_image), "::", stringify!(width)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_image>())).height as *const _ as usize }, 12usize, concat!("Offset of field: ", stringify!(png_image), "::", stringify!(height)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_image>())).format as *const _ as usize }, 16usize, concat!("Offset of field: ", stringify!(png_image), "::", stringify!(format)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_image>())).flags as *const _ as usize }, 20usize, concat!("Offset of field: ", stringify!(png_image), "::", stringify!(flags)));
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<png_image>())).colormap_entries as *const _ as usize },
        24usize,
        concat!("Offset of field: ", stringify!(png_image), "::", stringify!(colormap_entries))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<png_image>())).warning_or_error as *const _ as usize },
        28usize,
        concat!("Offset of field: ", stringify!(png_image), "::", stringify!(warning_or_error))
    );
    assert_eq!(unsafe { &(*(::std::ptr::null::<png_image>())).message as *const _ as usize }, 32usize, concat!("Offset of field: ", stringify!(png_image), "::", stringify!(message)));
}
