

pub fn action_box(){
    using_box() ;
    using_list_type();

}

/// Using a Box<T> to Store Data on the Heap
fn using_box(){
    // Putting a single value on the heap isn’t very useful, so you won’t use boxes by themselves in this way very often.
    // 把i32放到堆上 是一个 不甚有用的例子
    let b = Box::new(5);
    println!("b = {}", b);
}

//  Enabling Recursive Types with Boxes

enum List {
    Cons(i32, List),
    Nil ,
}

fn using_list_type(){
//    use List::{Cons, Nil} ;
use self::List::{Cons, Nil} ;

    let list = Cons(1,
        Cons(2,
            Cons(3, Nil)
        )
    );

    println!("{:?}", list) ;
}