
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

//use crate::List::{Cons, Nil};
use self::List::{Cons, Nil};
use std::rc::Rc;

pub fn action_rc(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
   // Rc::clone only increments the reference count
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    //
    increases_the_reference_count() ;
}

fn increases_the_reference_count(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));

        // the implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
