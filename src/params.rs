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

struct ArgStack {
    index: usize,
    arguments: Vec<String>,
}

impl ArgStack {
    fn new(arguments: Vec<String>) -> Self {
        ArgStack { index: 0, arguments }
    }

    fn pop(&mut self) -> Option<&String> {
        let arg = self.arguments.get(self.index)?;
        self.index += 1;
        Some(arg)
    }
}

pub fn parse_params(arg_vec: Vec<String>) -> Res<Params> {
    let mut arg_stack = ArgStack::new(arg_vec);
    let command = parse_command(&mut arg_stack)?;
    let script = parse_script(&mut arg_stack)?;
    for i in 2 .. arg_stack.len() {
        let arg = &arg_stack[i];
        if ! arg.starts_with("--") {
            return Err(format!("expected a parameter name, but got '{}'", arg))
        }
        if arg.contains('=') {

        }
        dbg!(&arg_stack[i]);  //TODO @mark: TEMPORARY! REMOVE THIS!
    }
    Ok(Params {
        command,
        script,
        bindings: HashMap::new(),
    })
}

fn parse_command(arguments: &mut ArgStack) -> Res<String> {
    match arguments.pop() {
        Some(cmd) => Ok(cmd.to_owned()),
        None => return Err("did not find the command 'psy' was invoked with".to_owned()),
    }
}

fn parse_script(arguments: &mut ArgStack) -> Res<PathBuf> {
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
    Ok(script)
}
