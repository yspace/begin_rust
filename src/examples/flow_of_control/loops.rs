
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

    //
    nesting_and_labels::act_main() ;
    returning_from_loops::act_main() ;
}

pub mod nesting_and_labels{
    #![allow(unreachable_code)]

    pub fn act_main(){
        'outer: loop {
            println!("Entered the outer loop") ;

            'inner: loop{
                println!("Entered the inner loop");

                // 这个只会终结内部循环
                // break ;

                // 这个就会跳出外部循环
                break 'outer ;
            }

            println!("This point will never be reached ");
        }

        println!("Exited the outer loop") ;
    }
}

pub mod returning_from_loops{
    pub fn act_main(){
        let mut counter = 0 ;

        let result = loop {
            counter += 1 ;

            if counter == 10{
                // 会作为loop的值返回出去的
                break counter * 2 ;
            }
        };

        assert_eq!(result , 20) ;
    }
}