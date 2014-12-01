extern crate "pkg-config" as pkg_config;

use std::os;
use std::io::{mod, fs, Command};
use std::io::process::InheritFd;
use std::io::fs::PathExtensions;

const GMP_NAME: &'static str = "libgmp";
const GMP_VERSION: &'static str = "6.0.0";

fn main() {
    let mut opts = pkg_config::default_options(GMP_NAME);
    opts.atleast_version = Some(GMP_VERSION.into_string());
    match pkg_config::find_library_opts(GMP_NAME, &opts) {
        Ok(_) => return,
        Err(..) => {}
    }

    // Bind some useful paths

    let project_src_root = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let gmp_src_root = project_src_root.join([GMP_NAME, "-", GMP_VERSION].concat());

    let out_dir = Path::new(os::getenv("OUT_DIR").unwrap());

    let gmp_build_dir = out_dir.join("build");

    let gmp_out_dir = out_dir.join("out");
    let gmp_out_lib_dir = gmp_out_dir.join("lib");
    let gmp_out_include_dir = gmp_out_dir.join("include");

    // Do not rebuild libgmp if it had already been built
    
    if !(gmp_out_lib_dir.exists() && gmp_out_lib_dir.join("libgmp.a").exists() &&
         gmp_out_include_dir.exists() && gmp_out_include_dir.join("gmp.h").exists()) {
        run_build(&gmp_src_root, &gmp_build_dir, 
                  &gmp_out_dir, &gmp_out_lib_dir, &gmp_out_include_dir);
    }

    // TODO: Regenerate and update source file if we have bindgen, otherwise copy prebuilt source

    // Emit cargo config
    emit_cargo_config(&gmp_out_lib_dir, &gmp_out_include_dir);
}

fn run_build(gmp_src_root: &Path,
             gmp_build_dir: &Path,
             gmp_out_dir: &Path,
             gmp_out_lib_dir: &Path,
             gmp_out_include_dir: &Path) {
    // let windows = target.contains("windows") || target.contains("mingw");
    //
    let target = os::getenv("TARGET").unwrap();

    let mut cflags = os::getenv("CFLAGS").unwrap_or(String::new());
    cflags.push_str(" -ffunction-sections -fdata-sections");
    if target.contains("i686") {
        cflags.push_str(" -m32");
    } else if target.as_slice().contains("x86_64") {
        cflags.push_str(" -m64");
    }
    if !target.contains("i686") {
        cflags.push_str(" -fPIC");
    }    

    let _ = fs::rmdir_recursive(gmp_build_dir);
    let _ = fs::rmdir_recursive(gmp_out_dir);

    let _ = fs::mkdir_recursive(gmp_out_lib_dir, io::USER_DIR);
    let _ = fs::mkdir_recursive(gmp_out_include_dir, io::USER_DIR);
    fs::mkdir(gmp_build_dir, io::USER_DIR).unwrap();

    let config_opts = vec![
        "--enable-shared=no".into_string() // TODO: why?
    ];

    // Run configure
    run(Command::new("sh")
                .env("CFLAGS", cflags)
                .cwd(gmp_build_dir)
                .arg("-c")
                .arg(format!(
                    "{} {}", 
                    gmp_src_root.join("configure").display(),
                    config_opts.connect(" ")
                ).replace("C:\\", "/c/").replace("\\", "/")));

    // Run make
    run(Command::new(make())
       .arg(format!("-j{}", os::getenv("NUM_JOBS").unwrap()))
       .cwd(gmp_build_dir));

    // Copy the library archive file
    let p1 = gmp_build_dir.join(".libs/libgmp.a");
    let p2 = gmp_build_dir.join(".libs/libgmp.lib");
    if p1.exists() {
        fs::rename(&p1, &gmp_out_lib_dir.join("libgmp.a")).unwrap();
    } else {
        fs::rename(&p2, &gmp_out_lib_dir.join("libgmp.a")).unwrap();
    }
    
    // Copy the single include file
    fs::copy(&gmp_build_dir.join("gmp.h"), &gmp_out_include_dir.join("gmp.h")).unwrap();
}

fn emit_cargo_config(lib_dir: &Path, include_dir: &Path) {
    println!("cargo:rustc-flags=-L {} -l gmp:static", lib_dir.display());
    println!("cargo:libdir={}", lib_dir.display());
    println!("cargo:include={}", include_dir.display());
}

fn make() -> &'static str {
    if cfg!(target_os = "freebsd") {"gmake"} else {"make"}
}

fn run(cmd: &mut Command) {
    println!("running: {}", cmd);
    assert!(cmd.stdout(InheritFd(1))
            .stderr(InheritFd(2))
            .status()
            .unwrap()
            .success());
}


