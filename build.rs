extern crate pkg_config;

use std::collections::HashSet;
use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();

    let lib_dir = env::var_os("LIBUSB_LIB_DIR").map(PathBuf::from);
    let include_dir = env::var_os("LIBUSB_INCLUDE_DIR").map(PathBuf::from);

    let (lib_dir, include_dir) = if lib_dir.is_none() || include_dir.is_none() {
        let libusb_dir = env::var_os("LIBUSB_DIR").unwrap_or_else(|| {
            find_libusb_dir(&target)
        });
        let libusb_dir = Path::new(&libusb_dir);
        let lib_dir = lib_dir.unwrap_or_else(|| libusb_dir.join("lib"));
        let include_dir = include_dir.unwrap_or_else(|| libusb_dir.join("include"));
        (lib_dir, include_dir)
    } else {
        (lib_dir.unwrap(), include_dir.unwrap())
    };

    if !Path::new(&lib_dir).exists() {
        panic!("libusb-1.0 library directory does not exist: {}",
               lib_dir.to_string_lossy());
    }
    if !Path::new(&include_dir).exists() {
        panic!("libusb-1.0 include directory does not exist: {}",
               include_dir.to_string_lossy());
    }

    println!("cargo:rustc-link-search=native={}", lib_dir.to_string_lossy());
    println!("cargo:include={}", include_dir.to_string_lossy());

    let libs = vec!["libusb-1.0"];

    let kind = determine_mode(Path::new(&lib_dir), &libs);
    for lib in libs.iter() {
        println!("cargo:rustc-link-lib={}={}", kind, lib);
    }
}

fn find_libusb_dir(target: &str) -> OsString {
    let host = env::var("HOST").unwrap();

    try_pkg_config();

    let mut msg = format!("
Could not find directory of libusb-1.0 installation, and this `-sys` crate cannot
proceed without this knowledge. If libusb-1.0 is installed and this crate had
trouble finding it,  you can set the `LIBUSB_DIR` environment variable for the
compilation process.
    $HOST = {}
    $TARGET = {}
    openssl-sys = {}
",
    host, target, env!("CARGO_PKG_VERSION"));

    if host.contains("unknown-linux") && target.contains("unknown-linux-gnu") {
        if Command::new("pkg-config").output().is_err() {
            msg.push_str(&format!("
It looks like you're compiling on Linux and also targeting Linux. Currently this
requires the `pkg-config` utility to find libusb-1.0 but unfortunately `pkg-config`
could not be found.
"));
        }
    }

    if host.contains("windows") && target.contains("windows-gnu") {
        msg.push_str(&format!("
It looks like you're compiling for MinGW but you may not have either libusb-1.0 or
pkg-config installed.
"
));
    }

    if host.contains("windows") && target.contains("windows-msvc") {
        msg.push_str(&format!("
It looks like you're compiling for MSVC but we couldn't detect a libusb-1.0
installation.
"
));
    }

    panic!(msg);
}

/// Attempt to find libusb through pkg-config.
///
/// Note that if this succeeds then the function does not return as pkg-config
/// typically tells us all the information that we need.
fn try_pkg_config() {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();

    // If we're going to windows-gnu we can use pkg-config, but only so long as
    // we're coming from a windows host.
    //
    // Otherwise if we're going to windows we probably can't use pkg-config.
    if target.contains("windows-gnu") && host.contains("windows") {
        env::set_var("PKG_CONFIG_ALLOW_CROSS", "1");
    } else if target.contains("windows") {
        return
    }

    let lib = pkg_config::Config::new()
        .print_system_libs(false)
        .find("libusb-1.0")
        .unwrap();

    for include in lib.include_paths.iter() {
        println!("cargo:include={}", include.display());
    }

    std::process::exit(0);
}

/// Given a libdir for libusb (where artifacts are located) as well as the name
/// of the libraries we're linking to, figure out whether we should link them
/// statically or dynamically.
fn determine_mode(libdir: &Path, libs: &[&str]) -> &'static str {
    // First see if a mode was explicitly requested
    let kind = env::var("LIBUSB_STATIC").ok();
    match kind.as_ref().map(|s| &s[..]) {
        Some("0") => return "dylib",
        Some(_) => return "static",
        None => {}
    }

    // Next, see what files we actually have to link against, and see what our
    // possibilities even are.
    let files = libdir.read_dir().unwrap()
                      .map(|e| e.unwrap())
                      .map(|e| e.file_name())
                      .filter_map(|e| e.into_string().ok())
                      .collect::<HashSet<_>>();
    let can_static = libs.iter().all(|l| {
        files.contains(&format!("lib{}.a", l)) ||
            files.contains(&format!("{}.lib", l))
    });
    let can_dylib = libs.iter().all(|l| {
        files.contains(&format!("lib{}.so", l)) ||
            files.contains(&format!("{}.dll", l)) ||
            files.contains(&format!("lib{}.dylib", l))
    });
    match (can_static, can_dylib) {
        (true, false) => return "static",
        (false, true) => return "dylib",
        (false, false) => {
            panic!("libusb-1.0 libdir at `{}` does not contain the required files \
                    to either statically or dynamically link libusb-1.0",
                   libdir.display());
        }
        (true, true) => {}
    }

    // Ok, we've got not explicit preference and can *either* link statically or
    // link dynamically. In the interest of "security upgrades" and/or "best
    // practices with security libs", let's link dynamically.
    "dylib"
}