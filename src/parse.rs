use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::common::Res;
use crate::params::Arguments;

static PY3_HEADER: &'static str = include_str!("py3_header.txt");

lazy_static! {
    static ref IS_NOOP_RE: Regex = Regex::new(r"^\s*([#\?].*)?$").unwrap();
    static ref IS_ARG_RE: Regex = Regex::new(r"^\s*\?").unwrap();
}

#[derive(Debug)]
pub enum ArgType {
    Text,
    File,
    Int,
    Real,
    Bool,
    JSON,
}

#[derive(Debug)]
pub struct Arg {
    name: String,
    typ: ArgType,
    is_required: bool,
    default: Option<String>,
}

pub fn parse_script(mut psh_script: String, arguments: &Arguments) -> Res<(String, Vec<Arg>)> {
    let mut py3 = String::with_capacity(psh_script.len() * 3 + PY3_HEADER.len());
    py3.push_str(PY3_HEADER);
    let mut code_started = false;
    for src_line in psh_script.lines() {
        if ! code_started && ! IS_NOOP_RE.is_match(src_line) {
            code_started = true;
            println!("START CODE");  //TODO @mark: TEMPORARY! REMOVE THIS!
        }
        dbg!(src_line);  //TODO @mark: TEMPORARY! REMOVE THIS!
        if IS_ARG_RE.is_match(src_line) {
            if code_started {
                return Err(format!("found argument line after start of code: {}", src_line))
            }
        }
    }
    Ok((py3, vec![]))
}
