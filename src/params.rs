use ::std::collections::HashMap;
use ::std::path::PathBuf;
use ::std::str::FromStr;

use crate::common::Res;

#[derive(Debug)]
pub struct Params {
    pub command: String,
    pub script: PathBuf,
    pub bindings: HashMap<String, String>,
}

pub fn parse_params(arguments: Vec<String>) -> Res<Params> {
    let command = match arguments.get(0) {
        Some(cmd) => cmd.to_owned(),
        None => return Err("did not find the command 'psy' was invoked with".to_owned()),
    };
    let script_name = match arguments.get(1) {
        Some(cmd) => cmd,
        None => return Err("provide the .psy script as the first argument to 'psy'".to_owned()),
    };
    if script_name.starts_with("-") {
        return Err(format!("found a flag instead of a .psh script path: '{}'", script_name))
    }
    let script = match PathBuf::from_str(script_name) {
        Ok(pth) => pth,
        Err(_) => return Err(format!("the script name '{}' is not a valid path syntactically", script_name)),
    };
    if !script.exists() {
        return Err(format!("could not find script '{}'", script_name))
    }
    if !script.is_file() {
        return Err(format!("script '{}' is not a file", script_name))
    }
    for i in 2 .. arguments.len() {
        let arg = &arguments[i];
        if ! arg.starts_with("--") {
            return Err(format!("expected a parameter name, but got '{}'", arg))
        }
        if arg.contains('=') {

        }
        dbg!(&arguments[i]);  //TODO @mark: TEMPORARY! REMOVE THIS!
    }
    Ok(Params {
        command,
        script,
        bindings: HashMap::new(),
    })
}
