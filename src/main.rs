use ::std::env::args;

use crate::common::Res;
use crate::params::parse_params;

mod common;
mod params;

fn main() {
    if let Err(err_msg) = run(args().collect()) {
        eprintln!("Error! {}", err_msg);
    };
    println!("Hello, world!");
}

fn run(arguments: Vec<String>) -> Res<()> {
    let args = parse_params(arguments)?;
    Ok(())  //TODO @mark: TEMPORARY! REMOVE THIS!
}
