
pub fn act_main(){
    let mut count = 0u32 ;

    println!("Let's count until infinity!") ;

    //
    loop{
        count += 1 ;

        if count == 3{
            println!("three");

            // 跳过本次迭代的其他情形
            continue ;
        }

        println!("{}", count) ;

        if count == 5 {
            println!("OK, that's enough");

            // 退出本次循环
            break ;
        }

    }
}