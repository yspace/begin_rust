
pub fn act_main(){
    // 记数变量
    let mut n = 1 ;

    while n < 101 {
        if n % 15 == 0{
            println!("fizzbuzz") ;
        }else if n % 3 == 0 {
            println!("fizz");
        }else if n %5 == 0 {
            println!("buzz") ;
        }else{
            println!("{}", n) ;
        }
        // 递增计数器
        n += 1 ;
    }
}