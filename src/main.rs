use std::env::args;

mod catout;

fn env() -> Vec<String> {
    let mut command_env: Vec<String> = Vec::new();
    for argument in args() {
        command_env.push(argument);
    }
    return command_env;
}

fn main() {
    let arguments: Vec<String> = env();
    let mut svr = catout::CatOut::new();
    println!("===Starting CatOut Server===");
    svr.start_program_monitor("cat", arguments);
    svr.listen("127.0.0.1:9001");

    println!("===Stopping CatOut Server===");
}
