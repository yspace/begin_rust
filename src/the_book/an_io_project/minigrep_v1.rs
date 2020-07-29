use std::env;
use std::process;

use minigrep::Config;

pub fn action_main(){

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args[2..]).unwrap_or_else(|err|{
        println!("Problem parsing arguments:{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}