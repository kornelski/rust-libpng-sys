# Rust bindings for libpng 1.6

libpng is likely to bring sorrow and regret. I strongly recommend to use a native Rust PNG library instead (e.g. [LodePNG](https://lib.rs/crates/lodepng)).

This crate bundles libpng 1.6.44 as a fallback. Please [check](http://www.libpng.org/pub/png/libpng.html) whether this version is still secure before using this package.

If you add it as a build dependency, Cargo will make `DEP_PNG_INCLUDE` env var available, containing a path to a directory with `png.h`.

You may need `use libpng_sys as _;` in your Rust program to ensure it's "used" and actually linked.
