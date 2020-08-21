//use std::sync::Mutex;
use std::thread;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub fn action_share_state(){
    using_mutex() ;

    using_arc() ;
}

fn using_mutex(){
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn sharing_a_mutex(){
//    let counter = Mutex::new(0);
//    let mut handles = vec![];
//
//    for _ in 0..10 {
//        let handle = thread::spawn(move || {
//            let mut num = counter.lock().unwrap();
//
//            *num += 1;
//        });
//        handles.push(handle);
//    }
//
//    for handle in handles {
//        handle.join().unwrap();
//    }
//
//    println!("Result: {}", *counter.lock().unwrap());
}

fn multiple_ownership(){
//    let counter = Rc::new(Mutex::new(0));
//    let mut handles = vec![];
//
//    for _ in 0..10 {
//        let counter = Rc::clone(&counter);
//        let handle = thread::spawn(move || {
//            let mut num = counter.lock().unwrap();
//
//            *num += 1;
//        });
//        handles.push(handle);
//    }
//
//    for handle in handles {
//        handle.join().unwrap();
//    }
//
//    println!("Result: {}", *counter.lock().unwrap());
}

/// Using an Arc<T> to wrap the Mutex<T> to be able to share ownership across multiple threads
fn using_arc(){
    // atomics work like primitive types but are safe to share across threads.

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}