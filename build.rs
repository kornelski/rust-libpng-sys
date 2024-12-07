
use std::env;
use std::process;
use std::path::PathBuf;
use std::fs;

fn main() {
    let build = cfg!(feature = "build");
    let std_zlib = cfg!(feature = "libz-sys");
    let wants_static = build || cfg!(feature = "static") || env::var("PNG_STATIC").is_ok();

    // we don't trust libpng-config or pkg-config to supply valid static library,
    // especially that Rust only accepts ones compiled with -fPIC
    if build || wants_static {
        build_static(std_zlib);
        return;
    }

    if !try_libpng_config(wants_static, std_zlib) && !try_pkgconfig(wants_static) {
        build_static(std_zlib);
    }
}

fn try_libpng_config(wants_static: bool, std_zlib: bool) -> bool {
    let cross_compile = env::var("TARGET") != env::var("HOST");
    if cross_compile {
        return false; // libpng-config is not aware of platform differences
    }

    if let Some(ver) = libpng_config(wants_static, "--version") {
        // require >= 1.6
        let mut ver = ver.split('.');
        let major = ver.next().and_then(|n| n.parse().ok()).unwrap_or(0);
        let minor = ver.next().and_then(|n| n.parse().ok()).unwrap_or(0);
        if major == 1 && minor < 6 {
            return false;
        }
    } else {
        return false;
    }

    if let Some(libdir) = libpng_config(wants_static, "--libdir") {
        if let Some(args) = libpng_config(wants_static, "--libs") {
            let libdir = libdir.trim_end();
            println!("cargo:rustc-link-search=native={}", libdir);
            println!("cargo:root={}", libdir);
            libs_from_args(&args, wants_static, std_zlib);
        } else {
            return false;
        }
    } else {
        return false;
    }

    if let Some(iopts) = libpng_config(wants_static, "--I_opts") {
        let dirs: Vec<_> = iopts.split(" -I").map(|opt| if opt.starts_with("-I") {&opt[2..]} else {opt}).collect();
        println!("cargo:include={}", env::join_paths(dirs).unwrap().to_string_lossy());
    } else {
        fallback_include_path();
    }
    true
}

fn fallback_include_path() {
    // fallback to bundled headers. It will probably cause problems on version mismatch,
    // but brokenness of libpng-config can't be helped
    println!("cargo:include={}", dunce::canonicalize("vendor").unwrap().display());
}

fn libs_from_args(libs: &str, wants_static: bool, std_zlib: bool) {
    let mut args = libs.trim_end().split_whitespace();
    while let Some(lib) = args.next() {
        if lib.starts_with("-l") {
            let lib_name = if lib.len() == 2 {
                args.next().expect("-l with argument")
            } else {
                &lib[2..]
            };
            if !std_zlib && "z" == lib_name {
                continue;
            }
            let link_static = if lib_name.contains("png") {wants_static} else {
                let lib_name = lib_name.to_uppercase();
                env::var_os(format!("{}_STATIC", lib_name)).is_some() ||
                env::var_os(format!("LIB{}_STATIC", lib_name)).is_some()
            };
            println!("cargo:rustc-link-lib={}{}", if link_static {"static="} else {""}, lib_name);
        }
    }
}

fn libpng_config(wants_static: bool, arg: &str) -> Option<String> {
    let mut cmd = process::Command::new("libpng-config");
    if wants_static {
        cmd.arg("--static");
    }
    cmd.arg(arg);
    if let Ok(out) = cmd.output() {
        String::from_utf8(out.stdout).ok()
    } else {
        None
    }
}

fn try_pkgconfig(wants_static: bool) -> bool {
    let mut pkg = pkg_config::Config::new();
    pkg.statik(wants_static);
    pkg.atleast_version("1.6"); // don't use old versions as they cause C API pains
    if let Ok(lib) = pkg.probe("libpng") {
        if let Some(path) = lib.include_paths.get(0) {
            println!("cargo:include={}", path.display());
        } else {
            fallback_include_path();
        }
        if let Some(path) = lib.link_paths.get(0) {
            println!("cargo:root={}", path.display());
        }
        return true;
    }
    false
}

fn build_static(std_zlib: bool) {
    let mut cc = cc::Build::new();
    cc.warnings(false);

    let vendor = dunce::canonicalize("vendor").unwrap();
    let prebuild_conf = vendor.join("scripts/pnglibconf.h.prebuilt");

    if !vendor.exists() || !prebuild_conf.exists() {
        panic!("libpng-sys: vendor/ dir is missing. Try running `git submodule update --init`");
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy(prebuild_conf, out_dir.join("pnglibconf.h")).unwrap();

    let mut includes = vec![vendor];

    if let Some(inc) = env::var_os("DEP_Z_INCLUDE") {
        includes.push(PathBuf::from(inc));
    } else if std_zlib {
        if let Ok(libz) = pkg_config::probe_library("z") {
            for path in libz.include_paths {
                includes.push(path);
            }
        } else {
            println!("cargo:warning=libpng-sys crate could not find libz headers");
            println!("cargo:rustc-link-lib=z");
        }
    }

    println!("cargo:root={}", out_dir.display());

    includes.push(out_dir);
    println!("cargo:include={}", env::join_paths(&includes).unwrap().to_string_lossy());

    for path in includes {
        cc.include(path);
    }

    cc
        .file("vendor/png.c")
        .file("vendor/pngerror.c")
        .file("vendor/pngget.c")
        .file("vendor/pngmem.c")
        .file("vendor/pngpread.c")
        .file("vendor/pngread.c")
        .file("vendor/pngrio.c")
        .file("vendor/pngrtran.c")
        .file("vendor/pngrutil.c")
        .file("vendor/pngset.c")
        .file("vendor/pngtrans.c")
        .file("vendor/pngwio.c")
        .file("vendor/pngwrite.c")
        .file("vendor/pngwtran.c")
        .file("vendor/pngwutil.c");

    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if arch == "arm" || arch == "aarch64" {
        cc
            .file("vendor/arm/arm_init.c")
            .file("vendor/arm/filter_neon_intrinsics.c")
            .file("vendor/arm/filter_neon.S")
            .file("vendor/arm/palette_neon_intrinsics.c");
    }

    cc.compile("libpng.a");
}
