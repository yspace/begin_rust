pub fn action_string(){
// This type is allocated on the heap and as such is able to
// store an amount of text that is unknown to us at compile time.
    let s = String::from("hello") ; // 从字符串字面量创建String

    // ## 可变的String
    let mut s = String::from("hello") ;
    s.push_str(", world!") ;

    println!("{}", s) ;

    // ## MOVE
    let x = 5 ;
    let y = x ; // copy     深浅拷贝是无区别的 只发生在栈上
    println!("x: {} y: {}", x, y) ;
    // The reason is that types such as integers that have a known size at compile time are stored entirely on the stack


    let s1 = String::from("hello") ;
    let s2 = s1 ; // 所有权转移了 不同于简单类型  浅拷贝+无效s1           rust术语：move

//    println!("{}, world!", s1); // 不能再使用s1了

    // rust 从来不会为我们的数据创建深拷贝的

    let  s1 = String::from("hello");
    let s2 = s1.clone() ; // 手动调用深拷贝  堆上复制一份
    println!("s1={} , s2 = {}", s1, s2);

    // ## 拷贝类型
    //
//   All the integer types, such as u32.
//   The Boolean type, bool, with values true and false.
//   All the floating point types, such as f64.
//   The character type, char.
//   Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.


}

pub fn action_ownership_func(){
    // 给函数参数传值很类似赋值行为 给函数传变量会同赋值一样发生move 或者copy的

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.