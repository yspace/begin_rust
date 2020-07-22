
use std::env ;

pub fn action_minigrep(){
//    reading_the_argument_values() ;
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args) ;

    // ### Saving the Argument Values in Variables
    assert!(args.len() >= 4 ,"参数数目不对");
    let query = &args[2];
    let filename = &args[3];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}

fn reading_the_argument_values(){
    let args: Vec<String> = env::args().collect();
    print!("{:?}", args) ;
}