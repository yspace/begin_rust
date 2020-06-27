
pub fn action_if_expr(){
    let number = 3 ;

    // conditions in if expressions are sometimes called arms, just like the arms in match expressions
    if number < 5{
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // 作为右值
    {
        let cond = true ;
        let number = if cond { 5 } else { 6 } ;
        println!("The value of number is {} ", number) ;
        // Remember that blocks of code evaluate to the last expression in them, and numbers
        // by themselves are also expressions.
    }
}

pub fn action_loops() {
    // ## loop
    {
        let mut i = 1 ;
        loop {
            i = i+1 ;
            println!("agan!---> {}", i) ;
            if i == 10 {
                break ;
            }
        }

        // Returning Values from Loops
        {
            let mut cnt = 0 ;
            let result = loop {
                cnt  += 1 ;
                if cnt == 10 {
                    break cnt * 2 ;
                }
            };

            println!("The result is {}", result) ;
        }

        // Conditional Loops with while
        {
            let mut number = 3 ;
            while number != 0  {
                println!("{}!", number) ;

                number -= 1 ;
            }
            println!("LIFTOFF!!!");
        }
        // Looping Through a Collection with for
        {
            let a = [10, 20, 30, 40, 50];
            let mut index = 0 ;

            while index < 5 {
                println!("the value is: {}", a[index]) ;

                index += 1 ;
            }
            // 上面迭代集合的方法容易出问题(索引容易写错而越界) 且比较低效（） 应该使用for来应对这种情况

            let a = [10, 20, 30, 40, 50];
            for ele in  a.iter() {
                println!("the value is : {}", ele) ;
            }

            // ###
            for number in (1..4).rev() {
                println!("{}!", number) ;
            }
            println!("LIFTOFF!!!") ;
        }
    }

}