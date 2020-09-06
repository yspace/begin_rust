#![allow(dead_code)]

//extern crate restaurant ; // 不需要声明也可以直接用呀

use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;

//use std::process::Command as StcCmd;

mod activities ;



fn main() {


    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|c| println!("Hello rust async , {:?}", c.args))
        // add more command here!
//        .command(Command::new().name("add").action(|c: &Context| {
//            println!("hi  this is add sub command");
//        }))
        .command(Command::new()
            .name("simple-async")
            .usage("... ")
            .action(|c: &Context| {


                activities::simple_async::action_main() ;
                activities::simple_async2::action_main() ;

            }))
        ;

    app.run(args);
}

