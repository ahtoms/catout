mod cat;

use std::env::args;
use std::collections::HashMap;

use cat::catout::CatOut;


fn argument_parse(arg: String) -> Option<(String, String)> {
    let mut result = None;

    match arg.find("=") {
        Some(index) => {
            let split = arg.split_at(index);
            let property = String::from(split.0);
            let value = String::from(split.1).chars().skip(1).collect();
            result = Some((property, value));
        }
        None => {
            //Print out error
        }
    }
    return result;
}


fn parse_program_args(prog_args: &String) -> Vec<String> {
    let mut prog_args_str: String = prog_args.chars().skip(1).collect();
    let mut args: Vec<String> = Vec::new();
    prog_args_str.pop();
    let matches = prog_args.split_whitespace();
    for s in matches {
        args.push(String::from(s));
    }
    return args;
}


fn env() -> HashMap<String, String> {
    let mut command_env: HashMap<String, String> = HashMap::new();
    for argument in args() {
        match argument_parse(argument) {
            Some(tuple) => {
                //match tuple.0 {
                command_env.insert(tuple.0, tuple.1);
                //}
            }
            None => { /*Error with parsing*/ }
        }
    }
    return command_env;
}


fn main() {
    let arguments: HashMap<String, String> = env();
    let mut svr = CatOut::new();
    let mut listen_address = String::from("127.0.0.1:");
    listen_address.push_str(arguments.get("port").unwrap().as_str());
    println!("===Starting CatOut Server===");
    svr.start_program(arguments.get("program").unwrap().as_str(),
        parse_program_args(arguments.get("arguments").unwrap()));
    svr.listen(listen_address.as_str());
    println!("===Stopping CatOut Server===");
}
