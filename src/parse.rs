use crate::common::Res;
use crate::params::Arguments;

static PY3_HEADER: &'static str = include_str!("py3_header.txt");

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

pub fn parse_script(mut script: String, arguments: &Arguments) -> Res<(String, Vec<Arg>)> {
    let mut py3 = String::with_capacity(script.len() * 3 + PY3_HEADER.len());
    py3.push_str(PY3_HEADER);
    Ok((py3, vec![]))
}
