use futures::{self, executor} ;
use std::thread::sleep;
use std::time::Duration;

async fn learn_sing(){
    sleep(Duration::from_secs(5)) ;
    println!("学唱歌");
}

async fn sing(){
    println!("唱歌");
}

async fn learn_and_sing_song(){
    learn_sing().await;
    sing().await;
}

async fn dance(){
    println!("跳舞");
}

async fn async_main(){
    let f1 = learn_and_sing_song() ;
    let f2 = dance() ;

//    futures::join!( f2,f1) ;
    futures::join!( f1,f2) ;
}

pub fn action_main(){
    executor::block_on(async_main());
}