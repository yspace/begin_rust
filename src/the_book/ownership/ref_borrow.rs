pub fn action_ref_as_param(){
 let s1 = String::from("hello");
    let len = calc_len(&s1) ;

    println!("The length of '{}' is {} ", s1, len);
}
fn calc_len(s: &String) -> usize {
    s.len()
}

// We call having references as function parameters borrowing.
// As in real life, if a person owns something, you can borrow it from them.
// When you’re done, you have to give it back.

pub fn action_mut_ref(){
    // 可变借用最多只能一次     处于数据竞争考虑  即同时刻不允许多个写引用持有者 以防竞争条件发生
    let mut s = String::from("hello");
    change(&mut s) ;
    println!("The s is {}", s) ;

    // 用大括号创建scope 允许多次可变引用的创建
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}