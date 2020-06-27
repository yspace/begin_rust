#![allow(dead_code)]

use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;

use std::process::Command as StcCmd;

mod my_lib;
mod my_mod;
mod action;

// rust by examples
mod examples;

// the book
mod the_book ;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|c| println!("Hello, {:?}", c.args))
        // add more command here!
        .command(Command::new().name("add").action(|c: &Context| {
            println!("hi  this is add sub command");
        }))
        .command(
            Command::new()
                .name("struct/destruct")
                .action(|c: &Context| {
                    action::action_struct_match();
                }),
        )
        // ----------------------------------
        .command(Command::new()
        .name("funcs:fn-ptr")
        .usage("... funcs:fn-ptr")
        .action(|c: &Context| {
            action::funcs::action_fn_ptr();
        }))
        //
        .command(Command::new()
        .name("comments")
        .usage("comments")
        .action(|c: &Context| {
            examples::hello_world::Comments();
        }))
        //
        .command(Command::new()
        .name("fmt-print")
        .usage("formatted print")
        .action(|c: &Context| {
            examples::hello_world::Formatted_print();
        }))
        .command(Command::new()
        .name("fmt-debug")
        .usage("debug trait")
        .action(|c: &Context| {
            examples::hello_world::DebugDemo();
        }))
        .command(Command::new()
        .name("fmt-display")
        .usage("display trait")
        .action(|c: &Context| {
            examples::hello_world::Display();
        }))
        .command(Command::new()
        .name("fmt-display-list")
        .usage("display trait")
        .action(|c: &Context| {
            examples::hello_world::DisplayList();
        }))
        .command(Command::new()
        .name("fmt-formatting")
        .usage("fmt ")
        .action(|c: &Context| {
            examples::hello_world::Formatting();
        }))
        .command(Command::new()
        .name("book-game")
        .usage("fmt ")
        .action(|c: &Context| {
            the_book::guessing_game::Run();
        }))
        //
        .command(Command::new()
        .name("book-var")
        .usage("fmt ")
        .action(|c: &Context| {
            the_book::common_prog_concept::action_variables();
        }))
        .command(Command::new()
        .name("book-types")
        .usage("fmt ")
        .action(|c: &Context| {
            the_book::common_prog_concept::data_types::action_data_types();
        }))
        .command(Command::new()
        .name("book-fn")
        .usage("functions ")
        .action(|c: &Context| {
            the_book::common_prog_concept::functions::action_basic();
        }))
        .command(Command::new()
        .name("book-if")
        .usage("constrol flow ")
        .action(|c: &Context| {
            the_book::common_prog_concept::control_flow::action_if_expr();
        }))
        .command(Command::new()
        .name("book-loop")
        .usage("constrol flow ")
        .action(|c: &Context| {
            the_book::common_prog_concept::control_flow::action_loops() ;
        }))
        .command(Command::new()
        .name("book-string")
        .usage("ownership: scope ")
        .action(|c: &Context| {
            the_book::ownership::scope::action_string() ;
        }))
        .command(Command::new()
        .name("book-string")
        .usage("ownership: scope ")
        .action(|c: &Context| {
            the_book::ownership::scope::action_string() ;
        }))
        // ...
        ;

    app.run(args);
}
