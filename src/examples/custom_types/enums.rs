
enum WebEvent{
    PageLoad,
    PageUnload,
    // 类似元组结构
    KeyPress(char),
    Paste(String),
    // 或者c风格的结构
    Click{x: i64, y: i64},
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

pub fn action_main(){
        let pressed = WebEvent::KeyPress('x');
        // `to_owned()` creates an owned `String` from a string slice.
        let pasted  = WebEvent::Paste("my text".to_owned());
        let click   = WebEvent::Click { x: 20, y: 80 };
        let load    = WebEvent::PageLoad;
        let unload  = WebEvent::PageUnload;

        inspect(pressed);
        inspect(pasted);
        inspect(click);
        inspect(load);
        inspect(unload);

        //
        main2() ;

        uses::main() ;

        c_like::action_main() ;
        testcases::action_main() ;
}



// 类型别名
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;


}

//enum VeryVerboseEnumOfThingsToDoWithNumbers {
//    Add,
//    Subtract,
//}

// Self 别名
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    pub  fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type NumOps = VeryVerboseEnumOfThingsToDoWithNumbers ;

fn main2() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = VeryVerboseEnumOfThingsToDoWithNumbers::Add;

    let result = x.run(1,2);
    println!("result: {}",result) ;

    let a = NumOps::Subtract ;
    println!("substract result: {}",  a.run(1,2) );
}


//
pub mod uses{
// An attribute to hide warnings for unused code.
    #![allow(dead_code)]

   pub enum Status {
        Rich,
        Poor,
    }

    enum Work {
        Civilian,
        Soldier,
    }

    pub fn main(){
        // Explicitly `use` each name so they are available without
        // manual scoping.
//        use crate::examples::custom_types::enums::uses::Status::{Poor, Rich} ; // 这是用crate的路径
//        use crate::Status::{Poor, Rich};
//         Automatically `use` each name inside `Work`.
//        use crate::Work::*;
        // 使用self 表示当前模块 有共同前缀的use 可以用下面这种方式 减少输入
        use self::{
          Status::{Poor, Rich},
            Work::* ,
        };

        // Equivalent to `Status::Poor`.
        let status = Poor;
        // Equivalent to `Work::Civilian`.
        let work = Civilian;

        match status {
            // Note the lack of scoping because of the explicit `use` above.
            Rich => println!("The rich have lots of money!"),
            Poor => println!("The poor have no money..."),
        }

        match work {
            // Note again the lack of scoping.
            Civilian => println!("Civilians work!"),
            Soldier  => println!("Soldiers fight!"),
        }
    }
}

pub mod c_like{
    #![allow(dead_code)]

    // 隐式鉴别器(始于0)
    enum Number{
        Zero,
        One,
        Two,
    }

    // 显式鉴别器
    enum Color{
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue= 0x0000ff,
    }

    pub fn action_main(){
        // 转型为integers
        println!("zero is {}", Number::Zero as i32);
        println!("one is {}", Number::One as i32);

        println!("roses are #{:06x}", Color::Red as i32);
        println!("roses are #{:06x}", Color::Blue as i32);
    }
}

pub mod testcases{
    use self::List::* ;

    enum List{
        Cons(u32, Box<List>),
        Nil,
    }
    // 方法可以关联到枚举
    impl List{
        // 创建一个空枚举
        fn new() -> List{
            Nil
        }

        fn prepend(self, elem: u32) -> List{
            Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32{
            match *self {
                Cons(_, ref tail) => 1+ tail.len(),
                Nil => 0
            }
        }

        fn stringify(&self) -> String{
            match *self {
                Cons(head, ref tail)=> {
                    format!("{}, {}", head, tail.stringify())
                },
                Nil =>{
                    format!("Nil")
                }
            }
        }
    }

    pub fn action_main(){
        let mut list = List::new() ;

       list = list.prepend(1);
       list = list.prepend(2);
       list = list.prepend(3);

        // Show the final state of the list
        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }

}