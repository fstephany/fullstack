use wasm_run::prelude::*;

use structopt::StructOpt;
use wasm_run::DefaultServeArgs;

#[wasm_run::main(
    "frontend",
    run_server = run_server,
    post_build = post_build
)]
#[derive(StructOpt, Debug)]
enum Cli {}

fn post_build(
    args: &DefaultBuildArgs,
    _profile: BuildProfile,
    wasm_js: String,
    wasm_bin: Vec<u8>,
) -> anyhow::Result<()> {
    Ok(())
}

fn run_server(_args: DefaultServeArgs) -> anyhow::Result<()> {
    println!("Woaw");

    Ok(())
}
