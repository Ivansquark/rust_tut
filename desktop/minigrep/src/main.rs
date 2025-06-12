use std::env;
use std::process;

use minigrep::Config;

fn main() {
    println!("minigrep");
    let args: Vec<String> = env::args().collect();
    //let config = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //run(config).unwrap();
    if let Err(e) = minigrep::run(config) {
        println!("Problem in read file: {e}");
        process::exit(1);
    }
    //run(config).unwrap_or_else(|err| {
    //    println!("Problem in read file: {err}");
    //    process::exit(1);
    //})

    //dbg!(args);
}
