extern crate bump_alloc;

use ::std::env::args;
use ::std::process::exit;

use ::bump_alloc::BumpAlloc;

use crate::common::Res;
use crate::params::parse_params;

#[global_allocator]
static A : BumpAlloc = BumpAlloc::new();


mod common;
mod params;

fn main() {
    if let Err(err_msg) = run(args().collect()) {
        eprintln!("Error! {}", err_msg);
        exit(1);
    };
    println!("Hello, world!");
}

fn run(arguments: Vec<String>) -> Res<()> {
    let args = parse_params(arguments)?;
    Ok(())  //TODO @mark: TEMPORARY! REMOVE THIS!
}
