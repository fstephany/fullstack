use std::{fs, process::Command};
use structopt::StructOpt;
use wasm_run::{
    prelude::anyhow::{self, bail, Context},
    BuildArgs, BuildProfile, DefaultBuildArgs, DefaultServeArgs,
};

#[wasm_run::main(
    "frontend",
    run_server = run_server,
    post_build = post_build
)]
#[derive(StructOpt, Debug)]
enum Cli {}

fn post_build(
    args: &DefaultBuildArgs,
    profile: BuildProfile,
    wasm_js: String,
    wasm_bin: Vec<u8>,
) -> anyhow::Result<()> {
    // Write WASM output to disk
    let build_path = args.build_path();
    let wasm_js_path = build_path.join("app.js");
    let wasm_bin_path = build_path.join("app_bg.wasm");

    fs::write(&wasm_js_path, wasm_js)
        .with_context(|| format!("could not write JS file to `{}`", wasm_js_path.display()))?;
    fs::write(&wasm_bin_path, wasm_bin)
        .with_context(|| format!("could not write WASM file to `{}`", wasm_bin_path.display()))?;

    // Building SASS
    build_sass(args, profile)?;

    // Build Backend
    build_backend(args, profile)?;

    Ok(())
}

fn run_server(_args: DefaultServeArgs) -> anyhow::Result<()> {
    println!("Woaw");
    Ok(())
}

fn build_sass(args: &DefaultBuildArgs, profile: BuildProfile) -> anyhow::Result<()> {
    println!("Building Sass");
    let options = args.sass_options(profile);
    let sass_dirs = args.sass_lookup_directories(profile);

    println!("dirs: {:?}", sass_dirs);

    for style_path in sass_dirs {
        println!(
            "Build Sass from {}",
            style_path.to_str().unwrap_or("unrenderable path")
        );
        args.build_sass_from_dir(&style_path, options.clone())?;
    }

    println!("Sass built");
    Ok(())
}

fn build_backend(_args: &DefaultBuildArgs, profile: BuildProfile) -> anyhow::Result<()> {
    let mut command = Command::new("cargo");

    command
        .args(&["build", "--bin", "backend"])
        .args(match profile {
            BuildProfile::Profiling => &["--release"] as &[&str],
            BuildProfile::Release => &["--release"],
            BuildProfile::Dev => &[],
        });

    println!("Building backend");
    let status = command.status().context("could not start build process")?;

    if status.success() {
        println!("Backend built")
    } else {
        if let Some(code) = status.code() {
            bail!("build process exit with code {}", code);
        } else {
            bail!("build process has been terminated by a signal");
        }
    }

    Ok(())
}
