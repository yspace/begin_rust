
use futures::executor ;

async fn hello(){
    println!("sync hello");
}

//block_on
//await
pub fn action_main() {
    let f = hello();
    executor::block_on(f);

    my_function();
    // executor::block_on(hello());
    println!("Hello, world!");
}

fn my_function() {
    println!("my function!");
}