pub fn action_if_let(){
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // 下面更简单的等价写法：
    // The syntax if let takes a pattern and an expression separated by an equal sign.
    // It works the same way as a match, where the expression is given to the match and the pattern is its first arm.

    if let Some(3) = some_u8_value {
        println!("three") ;
    }
    //  you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.

    if let Some(3) = some_u8_value {
        print!("three");
    }else{
        println!("others") ;
    }
}