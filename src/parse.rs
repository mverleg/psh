use ::std::fmt;

use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::common::Res;
use crate::params::Arguments;
use std::fmt::Formatter;

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

impl fmt::Display for ParamType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use ParamType::*;

        write!(f, "{}", match self {
            Text => "text",
            File => "file",
            Int => "int",
            Real => "real",
            Bool => "bool",
            Toggle => "toggle",
            Secret => "secret",
        })
    }
}

#[derive(Debug)]
pub struct Param {
    name: String,
    typ: ParamType,
    is_required: bool,
    default: Option<String>,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}{})", self.name, self.typ, if self.is_required { "" } else { "?" })
    }
}

pub fn parse_script(mut psh_script: String, arguments: &Arguments) -> Res<(String, Vec<Param>)> {
    let mut py3 = String::with_capacity(psh_script.len() * 3 + PY3_HEADER.len());
    py3.push_str(PY3_HEADER);
    let mut params = vec![];
    let mut code_started = false;
    for src_line in psh_script.lines() {
        if ! code_started && ! IS_NOOP_RE.is_match(src_line) {
            code_started = true;
        }
        if IS_ARG_RE.is_match(src_line) {
            if code_started {
                return Err(format!("found parameter line after start of code (should be at the top, before any code): {}", src_line))
            }
            let param = parse_param(src_line)?;
            if arguments.options.verbose {
                print!("expecting argument '{}' of type {}", &param.name, &param.typ);
                print!("{}", if param.is_required { " (required)" } else { " (optional)" });
                param.default.as_ref().map(|ref def| print!(", otherwise using {} as default", def));
                println!();
            }
            params.push(param);
        }
    }
    Ok((py3, params))
}

/// Parse a parameter line. The line is already known to be a parameter, and at the top of the script.
fn parse_param(src_line: &str) -> Res<Param> {
    match PARSE_ARG_RE.captures(src_line) {
        Some(parts) => {
            let name = parts.name("name").unwrap().as_str().to_owned();
            let typ = match parts.name("type") {
                Some(type_name) => parse_type(type_name.as_str())?,
                None => (ParamType::Text, true),
            };
            let default = parts.name("default").map(|def| def.as_str().to_owned());
            if default.is_some() && typ.1 {
                return Err(format!("parameter '{}' is optional (type has '?' postfix), but it also has a default; this is invalid, parameters with a default value can never be empty", &name))
            }
            Ok(Param {
                name,
                typ: typ.0,
                is_required: typ.1,
                default,
            })
        },
        None => return Err(format!("found parameter line but could not parse the format: {}\nexamples of valid formats:\n  '? name'\n  '? name: type'\n  '? name: type? = default  # comment'", src_line)),
    }
}

/// Parse the type of a parameter, including '?' postfix.
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
        _ => return Err(format!("found a parameter with type '{}', which is invalid\nallowed types are: text (=default), file, int, real, bool, toggle, secret (postfix '?' for optional, e.g. 'int?')", type_name))
    };
    Ok((typ, is_required))
}
