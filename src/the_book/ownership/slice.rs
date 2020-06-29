fn first_word(s: &String) ->usize {
    let bytes = s.as_bytes() ;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return  i ;
        }
    }
    s.len()
}

pub fn action_string_slices(){
    let s = String::from("hello world") ;

    let hello = &s[0..5] ;
    let world = &s[6..11] ;

    println!("{} , {}", hello, world) ;


    let s = String::from("hello");

    // 截头
    let slice = &s[0..2];
    println!("slice is {}", slice) ;
    let slice = &s[..2];
    println!("slice is {}", slice) ;
    // 去尾
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
    println!("slice is {}", slice) ;
    // 整个string
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
    println!("slice is {}", slice) ;

    // String Literals Are Slices
    {
        let my_string = String::from("hello world");

        // first_word works on slices of `String`s
        let word = first_word3(&my_string[..]);

        println!("the first world is :{}", world) ;

        let my_string_literal = "hello world";

        // first_word works on slices of string literals
        let word = first_word3(&my_string_literal[..]);

        println!("the first world is :{}", world) ;

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word3(my_string_literal);
        println!("the first world is :{}", world) ;
    }
}

fn first_word2(s: &String) -> &str{
    let bytes = s.as_bytes() ;

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i] ;
        }
    }
    &s[..]
}

pub fn action_first_world(){
//    the borrowing rules that if we have an immutable reference to something,
//    we cannot also take a mutable reference.

    let mut s = String::from("hello world");
    let word = first_word2(&s) ;
//    s.clear() ; // 这里执行了 mut  更改操作  编译器不会通过的！

    print!("the first word is : {}", word) ;
}

fn first_word3(s: &str) -> &str {

    for(i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' '{
            return &s[0..i] ;
        }
    }
    &s[..]
}

pub fn action_other_slices(){

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    // This slice has the type &[i32]. It works the same way as string slices do,
    // by storing a reference to the first element and a length. You’ll use this kind of slice
    // for all sorts of other collections.
}