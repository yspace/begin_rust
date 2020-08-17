
pub fn action_deref_trait(){
    ref_to_i32();
    using_box_like_ref();

    deref_coercion_in_action() ;
}

fn ref_to_i32(){
    let x = 5 ;
    let y  = &x ;

    assert_eq!(5 , x);
    assert_eq!(5 , *y); // 解引用
}

fn using_box_like_ref(){
    let x = 5 ;
    let y = Box::new(x) ;

    assert_eq!(5, x) ;
    assert_eq!(5, *y) ;
}

// ## Defining Our Own Smart Pointer
struct MyBox<T>(T) ;

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// Treating a Type Like a Reference by Implementing the Deref Trait
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    // associated type for the Deref trait
    type Target = T;

    // we implemented the Deref trait
    fn deref(&self) -> &T {
        &self.0
    }
}

fn deref_mybox(){
    let x = 5 ;
    let y = MyBox::new(x) ;

    assert_eq!(5, x) ;
    assert_eq!(5, *y) ;
}

// Deref coercion happens automatically when we pass a reference to a particular type’s value
// as an argument to a function or method that doesn’t match the parameter type in the function or method definition. A sequence of calls to the deref method converts the type we provided into the type the parameter needs.
pub fn deref_coercion_in_action(){
    hello("Rust") ;

    let m = MyBox::new(String::from("Rust2")) ;
    // Rust can turn &MyBox<String> into &String by calling deref.
    // The standard library provides an implementation of Deref on String that returns a string slice,
    // and this is in the API documentation for Deref. Rust calls deref again to turn the &String into &str,
    // which matches the hello function’s definition.
    hello(&m) ;

    // The code we would have to write if Rust didn’t have deref coercion
    let m = MyBox::new(String::from("Rust3"));
    // The (*m) dereferences the MyBox<String> into a String.
    // Then the & and [..] take a string slice of the String that is equal to the whole string
    // to match the signature of hello.
    hello(&(*m)[..]);

}
fn hello(name: &str){
    println!("Hello, {}!", name ) ;
}