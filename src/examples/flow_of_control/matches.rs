
// rust match 类似c中的 switch

pub fn act_main(){
    let number  = 13 ;
    println!("Tell me about {}", number);

    match number {
        // 匹配单个值
        1 => println!("One"),
        // 匹配多值
        2|3|5|7|11 => println!("this is a prime"),
        // 匹配一个包含区间（开区间)
        13 ..= 19 =>println!("A teen"),
        // 处理其他情形
        _ => println!("Ain't special"),
    }

    let boolean = true ;
    // match 也是一个表达式
    let binary = match boolean {
        // match的 臂 必须覆盖所有可能的值
        false => 0 ,
        true =>1,

    };
    println!("{} -> {}", boolean, binary) ;
}