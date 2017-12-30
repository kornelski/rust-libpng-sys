//! # This is not the documentation you're looking for
//! See [libpng documentation](http://libpng.org/pub/png/libpng-manual.txt) instead.
extern crate libz_sys;

pub mod ffi;

#[test]
fn links() {
    unsafe {
        assert!(ffi::png_access_version_number() > 10000);
    }
}
