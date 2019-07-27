//! # This is not the documentation you're looking for
//! See [libpng documentation](http://libpng.org/pub/png/libpng-manual.txt) instead.


#[cfg(feature="cloudflare-zlib-sys")]
extern crate cloudflare_zlib_sys;

#[cfg(all(feature="libz-sys", not(feature="cloudflare-zlib-sys")))]
extern crate libz_sys;

pub mod ffi;

#[test]
fn links() {
    unsafe {
        assert!(ffi::png_access_version_number() > 10000);
    }
}

#[test]
fn z() {
    extern "C" {
        fn deflateEnd(x: *mut u8) -> std::os::raw::c_int;
    }
    unsafe {
        deflateEnd(std::ptr::null_mut());
    }
}

#[test]
fn creates() {
    use std::ptr::null_mut;
    unsafe {
        assert!(ffi::png_access_version_number() > 10600);

        let mut res = ffi::png_create_read_struct_2(ffi::PNG_LIBPNG_VER_STRING, null_mut(), None, None, null_mut(), None, None);
        assert!(!res.is_null());
        ffi::png_destroy_read_struct(&mut res, null_mut(), null_mut());
    }
}
