
use std::env;
use std::fs;



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

    // ## Reading a File
    // --snip--
//    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn reading_the_argument_values(){
    let args: Vec<String> = env::args().collect();
    print!("{:?}", args) ;
}