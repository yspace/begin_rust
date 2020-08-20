use std::thread ;
use std::time::Duration ;


pub fn action_run_simultaneously(){
    creating_thread() ;
    using_join_jandles() ;
    using_move_closures() ;
}


/// creating_thread create a New thread with spawn
fn creating_thread(){
    thread::spawn(||{
       for i in 1..10 {
           println!("hi number {} from  the spawned thread !", i) ;
           thread::sleep(  Duration::from_millis(1)) ;
       }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1)) ;
    }
}

// Waiting for All Threads to Finish Using join Handles
fn using_join_jandles(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

//    handle.join().unwrap();
}

fn using_move_closures(){
    let v = vec![1, 2, 3];

//    let handle = thread::spawn(|| {
    let handle = thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
    });
//    drop(v); // oh no!
    handle.join().unwrap();
}