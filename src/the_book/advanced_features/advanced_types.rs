pub fn action_main(){
    declare_type_alias() ;
}

fn declare_type_alias(){
    // The main use case for type synonyms is to reduce repetition.

    type Kilometer = i32 ;

    let x: i32 = 5 ;
    let y: Kilometer = 5 ;

    println!("x+y = {}", x + y);

    // 比如下面这个长的类型
    {
        let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

        fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
            // --snip--
        }
//
//        fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
//            // --snip--
//        }
    }
    // Introducing a type alias Thunk to reduce repetition
    {
        type Thunk = Box<dyn Fn() + Send + 'static>;

        let f: Thunk = Box::new(|| println!("hi"));

        fn takes_long_type(f: Thunk) {
            // --snip--
        }

//        fn returns_long_type() -> Thunk {
//            // --snip--
//        }
    }

    // Type aliases are also commonly used with the Result<T, E> type for reducing repetition.
}

fn the_never_type(){
    fn bar() -> ! {
        // --snip--
        loop {

        }
    }
    // 例子
    {
//        impl<T> Option<T> {
//            pub fn unwrap(self) -> T {
//                match self {
//                    Some(val) => val,
//                    None => panic!("called `Option::unwrap()` on a `None` value"),
//                }
//            }
//        }
        //  Rust sees that val has the type T and panic! has the type !, so the result of the overall match expression is T.



        if false {
            // Here, the loop never ends, so ! is the value of the expression
            print!("forever ");
            loop {
                print!("and ever ");
            }
        }

    }
}