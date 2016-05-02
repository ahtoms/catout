use std::env::args;

mod catout;

fn env() -> Vec<String> {
    let mut command_env: Vec<String> = Vec::new();
    for argument in args() {
        command_env.push(argument);
    }
    command_env.remove(0);
    return command_env;
}

fn print_out_arguments(env: &Vec<String>) {
    for a in env {
        println!("{}", a);
    }
}

fn main() {
    let arguments: Vec<String> = env();
    let mut svr = catout::CatOut::new();
    println!("===Starting CatOut Server===");
    print_out_arguments(&arguments);
    svr.start_program_monitor("yes", arguments);
    svr.listen("127.0.0.1:9001");

    println!("===Stopping CatOut Server===");
}
