use std::sync::mpsc ;
use  std::thread ;
use std::time::Duration;


pub fn action_using_message_passing(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    //
    sending_multiple_values();
    //Creating Multiple Producers by Cloning the Transmitter
    creating_multiple_producers() ;
}

fn use_val_after_sent(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
      //   println!("val is {}", val); // NOTE here you can't use the val ,which has been sent
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn sending_multiple_values(){
    // 故事类似 妻子在家等待丈夫发送回来的消息
    let (tx, rx) = mpsc::channel() ;

    thread::spawn(move ||{
       let vals = vec![
            String::from("hi"),
           String::from("from"),
           String::from("the"),
           String::from("thread"),
       ];

        // treating rx as an iterator.
        for val in vals {
            tx.send(val).unwrap() ;
            thread::sleep(Duration::from_secs(1)) ;
        }

    });

    for received in rx {
        println!("Got: {}", received) ;
    }
}

// Sending multiple messages from multiple producers
fn creating_multiple_producers(){
    // 故事： 类似父母有多个子女在外打工 可以不定期的发送消息给父母
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}