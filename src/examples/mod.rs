////  相当于nodejs中的index文件
//
//// 声明子模块
//mod destruct;
//
//pub mod funcs;
//
////  导出
//pub use self::destruct::*;
use seahorse::{App, Command, Context, Flag, FlagType};

pub mod hello_world ;


pub fn mod_init(app: App) -> App {
    // build commands
    app.command(
        Command::new()
            .name("example")
            .action(|c: &Context| {
                print!("hi example")
            })
    )
}