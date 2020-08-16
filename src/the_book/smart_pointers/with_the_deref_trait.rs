
pub fn action_deref_trait(){
    ref_to_i32();
    using_box_like_ref();
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
    type Target = T;

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