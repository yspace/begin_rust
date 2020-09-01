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
            }))
}

