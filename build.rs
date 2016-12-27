use std::process::Command;
use std::{env, fs};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

const GMP_NAME: &'static str = "libgmp";
const GMP_VERSION: &'static str = "6.0.0";

#[cfg(unix)]
fn check_library(name: &str) -> bool {
    // First check whether ldconfig utility is available (if we're on linux)
    if let Ok(po) = Command::new("ldconfig").arg("-p").output() {
        let target = env::var("TARGET").unwrap();
        let is_64bit = target.contains("x86_64");
        let pattern = format!("{}.so (libc6{})", name, if is_64bit { ",x86-64" } else { "" });
        if !po.stdout.is_empty() {
            let br = BufReader::new(&*po.stdout);
            return br.lines().map(|l| l.unwrap())
                .any(|l| l.contains(&*pattern))
        }
    }

    // If it fails, then check common system libraries directories
    for &dir in &["/lib", "/usr/lib", "/usr/local/lib"] {
        let p = Path::new(dir).join(format!("{}.so", name));
        if p.exists() { return true; }
    }

    // Nothing found, build the lib from scratch
    false
}

// Windows does not have predefined locations with libraries, sorry
#[cfg(windows)]
fn check_library(name: &str) -> bool {
    false
}

fn main() {
    // GMP does not support pkg-config :(
    // Try to guess its presence manually
    if check_library(GMP_NAME) {
        println!("cargo:rustc-link-lib=static=gmp");
        return;
    }

    // Bind some useful paths

    let project_src_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let gmp_src_root = project_src_root.join::<String>([GMP_NAME, "-", GMP_VERSION].concat());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

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
    let target = env::var("TARGET").unwrap();

    let mut cflags = env::var("CFLAGS").unwrap_or_else(|_| String::new());
    cflags.push_str(" -ffunction-sections -fdata-sections");
    if target.contains("i686") {
        cflags.push_str(" -m32");
    } else if target.contains("x86_64") {
        cflags.push_str(" -m64");
    }
    if !target.contains("i686") {
        cflags.push_str(" -fPIC");
    }

    let _ = fs::remove_dir_all(gmp_build_dir);
    let _ = fs::remove_dir_all(gmp_out_dir);

    let _ = fs::create_dir_all(gmp_out_lib_dir);
    let _ = fs::create_dir_all(gmp_out_include_dir);
    fs::create_dir(gmp_build_dir).unwrap();

    let config_opts = vec![
        "--enable-shared=no".to_string() // TODO: why?
    ];

    // Run configure
    run(Command::new("sh")
                .env("CFLAGS", cflags)
                .current_dir(gmp_build_dir)
                .arg("-c")
                .arg(format!(
                    "{} {}",
                    gmp_src_root.join("configure").display(),
                    config_opts.join(" ")
                ).replace("C:\\", "/c/").replace("\\", "/")));

    // Run make
    run(Command::new(make())
       .arg(format!("-j{}", env::var("NUM_JOBS").unwrap()))
       .current_dir(gmp_build_dir));

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
    println!("cargo:rustc-flags=-L {}", lib_dir.display());
    println!("cargo:libdir={}", lib_dir.display());
    println!("cargo:include={}", include_dir.display());
    println!("cargo:rustc-link-lib=static=gmp");
}

fn make() -> &'static str {
    if cfg!(target_os = "freebsd") {"gmake"} else {"make"}
}

fn run(cmd: &mut Command) {
    use std::process::Stdio;

    println!("running: {:?}", cmd);
    assert!(cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success());
}
