extern crate bump_alloc;

use ::std::env::args;
use ::std::fs;
use ::std::process::exit;

use ::bump_alloc::BumpAlloc;

use crate::common::Res;
use crate::params::parse_params;
use crate::parse::parse_script;

#[global_allocator]
static A : BumpAlloc = BumpAlloc::new();

mod common;
mod params;
mod parse;

fn main() {
    if let Err(err_msg) = run(args().collect()) {
        eprintln!("Error: {}", err_msg);
        exit(1);
    };
    println!("Hello, world!");
}

fn run(arguments: Vec<String>) -> Res<()> {
    let cli_args = parse_params(arguments)?;
    let script_psh = fs::read_to_string(&cli_args.script)
        .map_err(|e| format!("failed to read script file; reason: {}", e))?;
    let (script_py3, psh_parms) = parse_script(script_psh, &cli_args)?;
    if cli_args.options.verbose {
        println!("psh will run: {}", &cli_args);
    }
    Ok(())  //TODO @mark: TEMPORARY! REMOVE THIS!
}
