
pub fn action_main(){
    // 带后缀的字面量 其类型在初始化时已知
    let x = 1u8 ;
    let y = 2u32 ;
    let z = 3f32 ;

    // 没有后缀的字面量，其类型依赖于他们怎样被使用 如果不存在约束 那么编译器会分别为整数和浮点数使用i32和f64类型的
    let i = 1 ;
    let f = 1.0 ;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

}