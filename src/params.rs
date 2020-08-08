use ::std::collections::HashMap;
use ::std::path::PathBuf;
use ::std::process::exit;
use ::std::str::FromStr;

use crate::common::Res;

static HELP_TEXT: &'static str = include_str!("help.txt");


#[derive(Debug)]
pub struct Options {
    verbose: bool,
}

#[derive(Debug)]
pub struct Params {
    pub command: String,
    pub script: PathBuf,
    pub options: Options,
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

    fn peek(&mut self) -> Option<&String> {
        self.arguments.get(self.index)
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
    let options = parse_psh_options(&mut arg_stack)?;
    let script = parse_script(&mut arg_stack)?;
    let bindings = parse_bindings(&mut arg_stack)?;
    Ok(Params {
        command,
        options,
        script,
        bindings,
    })
}

/// Extract the name of the command, i.e. 'psh'.
fn parse_command(arguments: &mut ArgStack) -> Res<String> {
    match arguments.pop() {
        Some(cmd) => Ok(cmd.to_owned()),
        None => return Err("did not find the command 'psy' was invoked with".to_owned()),
    }
}

/// Extract options to 'psh' itself (before the .psh filename).
fn parse_psh_options(arguments: &mut ArgStack) -> Res<Options> {
    let mut verbose = false;
    while let Some(opt) = arguments.peek() {
        if !opt.starts_with("-") {
            break
        }
        let opt = arguments.pop().unwrap();
        if opt == "-h" || opt == "--help" {
            println!("{}", HELP_TEXT);
            exit(0)
        }
        if opt == "-v" {
            if verbose == true {
                return Err("verbose (-v) cannot be supplied more than once".to_owned())
            }
            verbose = true;
            continue;
        }
        return Err(format!("did not recognize option '{}' (if you want to pass to the .psh script, supply the option after the .psh filename)", opt))
    }
    Ok(Options {
        verbose,
    })
}

/// Extract the .psh filename, failing if the file is not found.
fn parse_script(arguments: &mut ArgStack) -> Res<PathBuf> {
    let script_name = match arguments.pop() {
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

/// Extract variables for the .psh script (after the .psh filename).
fn parse_bindings(arguments: &mut ArgStack) -> Res<HashMap<String, String>> {
    let mut bindings = HashMap::new();
    while let Some(opt) = arguments.pop() {
        if ! opt.starts_with("--") {
            if opt.starts_with("-") {
                return Err(format!("environment values should have two dashes (found '{}')", opt))
            }
            return Err(format!("expected environment value, starting with --, but found '{}'", opt))
        }
        if let Some(eq_pos) = opt.find('=') {
            let name = opt[2 .. eq_pos].to_owned();
            let value = opt[eq_pos+1 ..].to_owned();
            bindings.insert(name, value);
        } else {
            let name = opt[2..].to_owned();
            let value = match arguments.pop() {
                Some(val) => val.to_owned(),
                None => return Err(format!("")),
            };
            bindings.insert(name, value);
        }
    }
    Ok(bindings)
}
