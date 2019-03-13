//! # This is not the documentation you're looking for
//! See [libpng documentation](http://libpng.org/pub/png/libpng-manual.txt) instead.


#[cfg(feature="cloudflare-zlib-sys")]
extern crate cloudflare_zlib_sys;

pub mod ffi;

#[test]
fn links() {
    use std::ptr::null_mut;
    unsafe {
        assert!(ffi::png_access_version_number() > 10000);

        let res = ffi::png_create_read_struct_2("testing".as_ptr() as *const _, null_mut(), None, None, null_mut(), None, None);
        assert!(res.is_null());
    }
}
