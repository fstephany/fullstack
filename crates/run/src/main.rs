use wasm_run::prelude::*;

use structopt::StructOpt;
use wasm_run::DefaultServeArgs;

#[wasm_run::main(
    "frontend",
    run_server = run_server
)]
#[derive(StructOpt, Debug)]
enum Cli {}

fn run_server(_args: DefaultServeArgs) -> anyhow::Result<()> {
    println!("Woaw");

    Ok(())
}
