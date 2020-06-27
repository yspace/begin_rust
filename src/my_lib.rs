use seahorse::{App, Command, Context, Flag, FlagType};

pub fn info() -> String {
    String::from("info from my_lib")
}


pub fn mod_init(app: App) -> App {
    // build commands
    app.command(
        Command::new()
            .name("my_lib/info")
            .action(|c: &Context| {
                 action_info();
            })
    )
}

pub fn action_info(){
    println!("[my_lib]: hi this is info from my_lib module ");
}