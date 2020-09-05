use std::convert::From ;

pub fn act_main(){
    action_from() ;
    intos::action_main() ;
}

// From 是扇入式
// The From trait allows for a type to define how to create itself
// from another type, hence providing a very simple mechanism
// for converting between several types.

fn convert_str_into_string(){
    let my_str = "hello";
    let my_string = String::from(my_str) ;
}

// 定义我们自己的conversion功能

#[derive(Debug)]
struct Number{
    value: i32 ,
}

impl From<i32> for Number{
    fn from(item: i32) -> Self {
        Number{value: item}
    }
}

pub fn action_from(){
    let num = Number::from(30) ;
    println!("My number is {:?}", num);
}

mod intos{
    use std::convert::From;

    #[derive(Debug)]
    struct Number{
        value: i32 ,
    }

    impl From<i32> for Number{
        fn from(item: i32) -> Self {
            Number{value:item}
        }
    }

    pub fn action_main(){
        let int = 5 ;
        //  试着移除类型定义
        let num: Number = int.into();
        // 这里有小小的trade-off 需要我们手动标记类型 不然编译器无法推断into到何种类型

        println!("My number is {:?}", num) ;
    }

}