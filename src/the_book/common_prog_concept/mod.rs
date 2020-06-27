

pub const IS_DEBUG : bool= true ;

// 导出模块
pub mod data_types ;

pub mod functions ;
pub mod control_flow ;


pub fn action_variables(){
    let mut x = 5 ;
    println!("The value of x is {} ", x) ;
    x = 6;
    println!("The value of x is {} ", x) ;

    // ## 常量
    const MAX_POINTS : u32 = 100_100 ;
    println!("The const is {}" , MAX_POINTS) ;
    println!("Is this mod in debug mode: {}" , IS_DEBUG) ;

//    ## 遮盖
    {
        let x = 5 ;
        let x = x + 1 ;
        let x = x * 2 ;
        println!("x is {} ", x) ;
        // 不同类型
        let spaces = "  ";
        let spaces = spaces.len() ; // 避免变量名称不断膨胀
        println!("the len of spaces is {}", spaces) ;
    }
}

