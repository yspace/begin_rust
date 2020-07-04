#![allow(dead_code)]

//extern crate restaurant ;

use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;

use std::process::Command as StcCmd;

mod my_lib;
mod my_mod;
mod action;

// rust by examples ==》 RBE
mod examples;

// the book
mod the_book ;

fn main() {

    restaurant::eat_at_restaurant() ;

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
        .name("book-ownership-funcs")
        .usage("ownership: scope ")
        .action(|c: &Context| {
            the_book::ownership::scope::action_ownership_func();
        }))
        .command(Command::new()
        .name("book-ref-borrow")
        .usage("ownership: scope ")
        .action(|c: &Context| {
            the_book::ownership::ref_borrow::action_ref_as_param();
        }))
        .command(Command::new()
        .name("book-mut-ref")
        .usage("ownership: scope ")
        .action(|c: &Context| {
            the_book::ownership::ref_borrow::action_mut_ref() ;
        }))
        .command(Command::new()
        .name("book-slice")
        .usage("slice ： string ")
        .action(|c: &Context| {
            the_book::ownership::slice::action_string_slices() ;
        }))
        .command(Command::new()
        .name("book-struct")
        .usage("struct ： declare ")
        .action(|c: &Context| {
            the_book::structs::define_instantiate::action_def();
        }))
        .command(Command::new()
        .name("book-struct2")
        .usage("struct ： example program ")
        .action(|c: &Context| {
            the_book::structs::example_prog::action_rectangles();
        }))
        .command(Command::new()
        .name("book-method")
        .usage("struct ： method ")
        .action(|c: &Context| {
            the_book::structs::methods::action_method();
        }))
        .command(Command::new()
        .name("book-if-let")
        .usage("enum ： if-let ")
        .action(|c: &Context| {
            the_book::enums_pattern_match::if_let::action_if_let();
        }))
        // ...
        ;
    // TODO 今天学了所有权的传递 移动 返回  可以籍此重新改动下上面的command方法 太长了
     // let app = xxx_mod::init_mod(app) ; // 传入传出
     let app = my_lib::mod_init(app) ;

    app.run(args);
}
