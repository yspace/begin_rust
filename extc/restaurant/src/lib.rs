mod front_of_house {
    pub mod hosting {
      pub  fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    println!("fn eat_at_restaurant called!") ;
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // 使用枚举
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

//super 用法
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // This is like starting a filesystem path with the .. syntax
    }

    fn cook_order() {}

    // 结构体暴露为公有构造
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 暴露枚举 // 不过枚举成员（variant--变体）不用显示pub  为了减少麻烦 枚举变体默认就是public的
    //  Structs are often useful without their fields being public,
    //  so struct fields follow the general rule of everything being private by default unless annotated with pub.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
// ## use  bring the crate::front_of_house::hosting module into the scope of the eat_at_restaurant function
use crate::front_of_house::hosting ;
// use self::front_of_house::hosting; // 等同上面的效果 不过此行使用了相对路径
// Adding use and a path in a scope is similar to creating a symbolic link in the filesystem

// 使用短路径
pub fn eat_at_restaurant2() {

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//pub use crate::front_of_house::hosting; // 重新暴露
pub use crate::front_of_house::hosting as my_hosting; // 重新暴露 并使用as 给予一个别名

//  ## use 引入合并
//use std::{cmp::Ordering, io}; // Specifying a nested path to bring multiple items with the same prefix into scope
//use std::io::{self, Write};  // This line brings std::io and std::io::Write into scope.
//use std::collections::*;     // * glob 引入特殊情况下才使用 比如测试