extern crate pkg_config;
extern crate cc;
extern crate dunce;

use std::env;
use std::process;
use std::path::PathBuf;

fn main() {
    let wants_static = cfg!(feature = "static") || env::var("PNG_STATIC").is_ok();

    if !try_libpng_config(wants_static) && !try_pkgconfig(wants_static) {
        build_static();
    }
}

fn try_libpng_config(wants_static: bool) -> bool {
    if let Some(libdir) = libpng_config(wants_static, "--libdir") {
        let libdir = libdir.trim_right();
        println!("cargo:rustc-link-search=native={}", libdir.trim_right());
        println!("cargo:root={}", libdir);
    } else {
        return false;
    }

    if let Some(iopts) = libpng_config(wants_static, "--I_opts") {
        let dirs: Vec<_> = iopts.split(" -I").map(|opt| if opt.starts_with("-I") {&opt[2..]} else {opt}).collect();
        println!("cargo:include={}", env::join_paths(dirs).unwrap().to_string_lossy());
    }

    if let Some(libs) = libpng_config(wants_static, "--libs") {
        for lib in libs.trim_right().split_whitespace() {
            if lib.starts_with("-l") {
                let lib_name = &lib[2..];
                let link_static = if lib_name.contains("png") {wants_static} else {
                    let lib_name = lib_name.to_uppercase();
                    env::var_os(format!("{}_STATIC", lib_name)).is_some() ||
                    env::var_os(format!("LIB{}_STATIC", lib_name)).is_some()
                };
                println!("cargo:rustc-link-lib={}{}", if link_static {"static="} else {""}, lib_name);
            }
        }
    } else {
        return false;
    }
    true
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
    pkg.atleast_version("1.4"); // 1.2 is shite
    if let Ok(lib) = pkg.probe("libpng") {
        if let Some(path) = lib.include_paths.get(0) {
            println!("cargo:include={}", path.display());
        }
        if let Some(path) = lib.link_paths.get(0) {
            println!("cargo:root={}", path.display());
        }
        return true;
    }
    false
}

fn build_static() {
    let mut cc = cc::Build::new();
    cc.warnings(false);

    let vendor = dunce::canonicalize("vendor").unwrap();
    let mut includes = vec![vendor];

    if let Some(inc) = env::var_os("DEP_Z_INCLUDE") {
        includes.push(PathBuf::from(inc));
        if let Ok(lib) = env::var("DEP_Z_ROOT") {
            println!("cargo:rustc-link-search=native={}", lib);
            println!("cargo:rustc-link-lib=static=zlib");
        } else {
            println!("cargo:rustc-link-lib=zlib");
        }
    } else if let Ok(libz) = pkg_config::probe_library("z") {
        for path in libz.include_paths {
            includes.push(PathBuf::from(path));
        }
    } else {
        println!("cargo:rustc-link-lib=z");
    }

    println!("cargo:include={}", env::join_paths(&includes).unwrap().to_string_lossy());
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());

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
        .file("vendor/pngwutil.c")
        .compile("libpng.a");
}
