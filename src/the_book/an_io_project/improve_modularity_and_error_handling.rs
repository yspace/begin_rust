use std::env;
use std::fs;
use std::process;
use std::error::Error;

pub fn action_main2(){

    //
//    Calling the command line parsing logic with the argument values
    let args: Vec<String> = env::args().collect();
    println!("all raw args: {:?}", &args) ;

    // NOTE 这是一个反模式：原生类型迷惑
//    let (query, filename) = parse_config(&args[2..]) ;
//    println!("query: {}, filename: {}", query, filename);

//    let config = parse_config2(&args[2..]) ;
//    println!("config: {:?} ", config) ;
//    println!("Searching for {}", config.query);
//    println!("In file {}", config.filename);

    //
//    let config = Config::new(&args[2..]);
//    println!("{:?}", config);

    let config = Config::new(&args[2..]).unwrap_or_else(|err|{
        println!("Problem parsing arguments:{}", err);
        process::exit(1);
    });

//    let contents =  fs::read_to_string(config.filename)
//        .expect("Something went wrong reading the file");

//    Setting up any other configuration
//    Calling a run function in lib.rs

//        run(config) ; // 这个是忽略Result出现错误时的用法

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
//    Handling the error if run returns an error

}

#[derive(Debug)]
struct Config{
    query: String,
    filename: String,
}

impl Config{
    fn new1(args: &[String])-> Config{
        if args.len() < 2 {
            // a call to panic! is more appropriate for a programming problem than a usage problem
            panic!("not enough arguments");
         }

        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }

    fn new(args: &[String])-> Result<Config, &'static  str>{
        //  &'static str is the type of string literals
        if args.len() < 2 {
            return Err("not enough argument");
        }

        let query = args[0].clone() ;
        let filename = args[1].clone() ;

        Ok(
            Config{
                query,
                filename,
            }
        )
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}