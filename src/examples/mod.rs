////  相当于nodejs中的index文件
//
//// 声明子模块
//mod destruct;
//
//pub mod funcs;
//
////  导出
//pub use self::destruct::*;

pub mod hello_world ;
pub mod primitives ;
pub mod custom_types ;
pub mod variable_bindings ;
pub mod types ;
pub mod conversion ;
pub mod expressions ;
pub mod flow_of_control ;

use seahorse::{App, Command, Context, Flag, FlagType};


pub fn mod_init(app: App) -> App {
    // build commands
    app.command(
        Command::new()
            .name("example")
            .action(|c: &Context| {
                print!("hi example")
            })
    )
        .command(Command::new()
            .name("rbe-primitive")
            .usage("cargo run ebe-primitive")
            .action(|c: &Context| {
                primitives::action_main() ;

                primitives::literals_and_operators::action_main() ;
                primitives::arrays_and_slices::action_main() ;

            }))
        .command(Command::new()
            .name("rbe-custom-type")
            .usage("cargo run ebe-custom-type")
            .action(|c: &Context| {
                custom_types::structures::action_main() ;
                custom_types::enums::action_main() ;
                custom_types::constants::action_main() ;
            }))
        .command(Command::new()
            .name("rbe-var-binding")
            .usage("cargo run ebe-var-binding")
            .action(|c: &Context| {
                variable_bindings::main() ;
                variable_bindings::mutability::action_main() ;
                variable_bindings::scope_and_shadowing::action_main() ;
                variable_bindings::declare_first::action_main() ;
                variable_bindings::freezing::action_main() ;
            }))

        .command(Command::new()
            .name("rbe-types")
            .usage("cargo run ebe-types")
            .action(|c: &Context| {
                types::casting::action_main() ;
                types::literals::action_main() ;
                types::inference::action_main() ;
                types::aliasing::action_main() ;
            }))
        .command(Command::new()
            .name("rbe-conversion")
            .usage("cargo run ebe-conversion")
            .action(|c: &Context| {
                conversion::from_and_into::act_main()   ;
                conversion::try_from_and_into::act_main() ;
                conversion::to_and_from_strings::act_main() ;
            }))
        .command(Command::new()
            .name("rbe-expressions")
            .usage("cargo run ebe-expressions")
            .action(|c: &Context| {
                expressions::action_main() ;
            }))
        .command(Command::new()
            .name("rbe-flow-of-control")
            .usage("cargo run ebe-flow-of-control")
            .action(|c: &Context| {
                flow_of_control::if_else::act_main() ;

                flow_of_control::loops::act_main() ;
            }))
}

