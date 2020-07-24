use std::env;
use std::fs;

pub fn action_main2(){

    //
//    Calling the command line parsing logic with the argument values
    let args: Vec<String> = env::args().collect();
    println!("all raw args: {:?}", &args) ;

    // NOTE 这是一个反模式：原生类型迷惑
    let (query, filename) = parse_config(&args[2..]) ;
    println!("query: {}, filename: {}", query, filename);

    let config = parse_config2(&args[2..]) ;
    println!("config: {:?} ", config) ;
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    //
    let config = Config::new(&args[2..]);
    println!("{:?}", config);

    let contents =  fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

//    Setting up any other configuration
//    Calling a run function in lib.rs
//    Handling the error if run returns an error

}

#[derive(Debug)]
struct Config{
    query: String,
    filename: String,
}

impl Config{
    fn new(args: &[String])-> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

// ## lib logic
fn parse_config(args: &[String]) ->(&str, &str){
    // Note: Using primitive values when a complex type would be more appropriate is an anti-pattern known as primitive obsession.
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

fn parse_config2(args: &[String]) -> Config{
    let query = args[1].clone() ;
    let filename = args[2].clone() ;

    Config{
        query,
        filename,
    }
}