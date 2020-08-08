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
        eprintln!("Error! {}", err_msg);
        exit(1);
    };
    println!("Hello, world!");
}

fn run(arguments: Vec<String>) -> Res<()> {
    let args = parse_params(arguments)?;
    let script = fs::read_to_string(&args.script)
        .map_err(|e| format!("failed to read script file; reason: {}", e))?;
    parse_script(script, &args);
    if args.options.verbose {
        println!("psh will run: {}", args);
    }
    Ok(())  //TODO @mark: TEMPORARY! REMOVE THIS!
}
