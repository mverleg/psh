
use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::common::Res;
use crate::params::Arguments;

static PY3_HEADER: &'static str = include_str!("py3_header.txt");

lazy_static! {
    static ref IS_NOOP_RE: Regex = Regex::new(r"^\s*([#\?].*)?$").unwrap();
    static ref IS_ARG_RE: Regex = Regex::new(r"^\s*\?").unwrap();
    static ref PARSE_ARG_RE: Regex = Regex::new(r"^\s*\?\s*(?P<name>\w+)(?:\s*:\s*(?P<type>\w+))?(?:\s*=\s*(?P<default>\w+))?\s*(?:#.*)?$").unwrap();
}

#[derive(Debug)]
pub enum ParamType {
    Text,
    File,
    Int,
    Real,
    Bool,
    Toggle,
    Secret,
    //Json,
}

#[derive(Debug)]
pub struct Param {
    name: String,
    typ: ParamType,
    is_required: bool,
    default: Option<String>,
}

pub fn parse_script(mut psh_script: String, arguments: &Arguments) -> Res<(String, Vec<Param>)> {
    let mut py3 = String::with_capacity(psh_script.len() * 3 + PY3_HEADER.len());
    py3.push_str(PY3_HEADER);
    let mut params = vec![];
    let mut code_started = false;
    for src_line in psh_script.lines() {
        if ! code_started && ! IS_NOOP_RE.is_match(src_line) {
            code_started = true;
            println!("START CODE");  //TODO @mark: TEMPORARY! REMOVE THIS!
        }
        dbg!(src_line);  //TODO @mark: TEMPORARY! REMOVE THIS!
        if IS_ARG_RE.is_match(src_line) {
            if code_started {
                return Err(format!("found parameter line after start of code (should be at the top, before any code): {}", src_line))
            }
            let param = match PARSE_ARG_RE.captures(src_line) {
                Some(parts) => {
                    let typ = parts.name("type").map(|t| parse_type(t.as_str())?);
                    Param {
                        name: parts.name("name").unwrap().as_str().to_owned(),
                        typ: typ.map(|t| t.0),
                        is_required: typ.map(|t| t.1),
                        default: parts.name("default"),
                    }
                },
                None => return Err(format!("found parameter line but could not parse the format: {}\nexamples of valid formats:\n  '? name'\n  '? name: type'\n  '? name: type? = default  # comment'", src_line)),
            };
            params.push(param);
        }
    }
    Ok((py3, params))
}

fn parse_type(mut type_name: &str) -> Res<(ParamType, bool)> {
    use ParamType::*;

    let mut is_required = false;
    if type_name.ends_with("?") {
        is_required = true;
        type_name = &type_name[1..]
    }
    let typ = match type_name {
        "string" => Text,
        "text" => Text,
        "file" => File,
        "path" => File,
        "int" => Int,
        "integer" => Int,
        "real" => Real,
        "double" => Real,
        "float" => Real,
        "bool" => Bool,
        "boolean" => Bool,
        "toggle" => Toggle,
        "secret" => Secret,
        _ => return Err(format!("found a parameter with type '{}', which is invalid\nallowed types are: text, file, int, real, bool, toggle, secret (postfix '?' for optional, e.g. 'int?')", type_name))
    };
    Ok((typ, is_required))
}
