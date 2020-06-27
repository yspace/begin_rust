use std::fmt;
use std::fmt::{Error, Formatter, Display};

pub fn Comments() {
    // This is an example of a line comment
    // There are two slashes at the beginning of the line
    // And nothing written inside these will be read by the compiler

    // println!("Hello, world!");

    // Run it. See? Now try deleting the two slashes, and run it again.

    /*
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But
     * block comments are extremely useful for temporarily disabling
     * chunks of code. /* Block comments can be /* nested, */ */
     * so it takes only a few keystrokes to comment out everything
     * in this main() function. /*/*/* Try it yourself! */*/*/
     */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */

    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

// ## Formatted print

pub fn Formatted_print(){
//    默认是i32类型 可以添加后缀
    println!("{} days ", 31);
    println!("{} days ", 31i64);


    // 使用位置参数
    println!("{0} , this is {1}, {1}, this is {0}", "Alice", "Bob");

    // 使用命名参数
    println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
        verb="jumps over"  );

    // 特殊格式 可以在冒号 : 后指定
    println!("{} of {:} people know binary , the other half doesn't ", 1, 2);

    // 右对齐指定宽度
    println!("{number:>width$}", number=1, width=6);

    // 用零补齐数字
    println!("{number:>0width$}", number=1, width=6);

    // rust会检测参数数量是否正确 缺失参数会报错的哦！
    println!("My name is {0} , {1} {0}", "Bond" , "James");

    // 创建一个含有i32成员的结构
    #[allow(dead_code)]
    struct Structure(i32) ;
    // 结构体需要更复杂的处理
//    println!("This struct '{}' won't print... ", Structure(3));

    /*
    std::fmt 包含很多traits 他们统治了文本的显示 两个很重要的基本形式如下：
    - fmt::Debug  使用{:?}  用于调试目的的文本格式化
    - fmt::Display 使用{}   用更优雅 用户友好的风格来显示文本

    这里只使用了Display 标准库提供了这些类型的实现 对于用户自定义类型需要更多的处理步骤

    实现了fmt::Display 就会自动实现ToString trait 这允许我们将类型转换为String
    */

    // ## Activities
    {
        // 打印pi值  指定特殊精度
        let pi = 3.1415926;
        println!("Pi is roughly {0:.3}", pi) ; // 写死精度
        println!("Pi is roughly {0:.1$}", pi, 3) ;  // 变量可控 1$ 指参数位是1下标的那个即：3 第0个是pi变量

        println!("Pi is {number:.prec$}",  prec = 3, number = pi);
    }
}

// --------------------

pub fn DebugDemo(){
    // 通过声明来自动实现fmt::Debug trait特征
    // 注意： Display 特征只能手动实现 不能声明式实现

    // 标准库的很多类都实现了Debug特征

    #[derive(Debug)]
    struct Structure(i32) ;

    #[derive(Debug)]
    struct Deep(Structure) ;

    println!("{:?} month in a year " , 12 );


    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // 可打印的结构体
    println!("Now {:?} will be printable ! ", Structure(3) ) ;

    // 声明式的实现 一个问题是不可以控制长相
    println!("Now {:?} will be print !", Deep(Structure(3)));

    // ## pretty print
    {
        #[derive(Debug)]
        struct Person<'a>{
            name: &'a str ,
            age: u8 ,
        }

        let name = "peter" ;
        let age = 27;
        let peter = Person{
          name ,
            age,
        };

        // pretty print
        println!("{:#?}" , peter);

    }

}

// 可以手动实现fmt::Display 用于显示

pub fn Display() {

    struct structure(i32) ; // tuple struct 元组结构体
    // 使用{} marker 结构体必须要实现fmt::Display特征
    impl fmt::Display for structure{
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),Error> {
          // 此方法很类似println!
            write!(f, " {} ", self.0)
        }
    }

    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax{
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
           write!(f, "({}, {})", self.0 , self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64 ,
        y: f64 ,
    }

    impl fmt::Display for Point2D{
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
           write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14) ;
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} and the small range is {small}",
        big = big_range ,
        small= small_range) ;

    let point = Point2D{
        x: 3.3,
        y: 7.2,
    };

    println!("compare points;");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);

    // ## Activity
    {
        #[derive(Debug)]
        struct Complex{
            real: f64,
            imag: f64,
        }

        impl fmt::Display for Complex{
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
               write!(f, "{} + {}i", self.real, self.imag)
            }
        }


        let cmplx = Complex{
          real: 3.3 ,
            imag: 7.2,
        };

        println!("Compare complex: ");
        println!("Display: {}", cmplx) ;
        println!("Debug: {:?}", cmplx) ;
    }
}

pub fn DisplayList(){
    struct List(Vec<i32>) ;
    impl fmt::Display for List{
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
             let vec = &self.0 ;


            write!(f, "[")?;

            // Iterate over `v` in `vec` while enumerating the iteration
            // count in `count`.
            for (count, v) in vec.iter().enumerate() {
                // For every element except the first, add a comma.
                // Use the ? operator, or try!, to return on errors.
                if count != 0 { write!(f, ", ")?; }
//                write!(f, "{}", v)?;
                write!(f, "{cnt}:{val}",cnt=count,val=v)?;
            }

            // Close the opened bracket and return a fmt::Result value.
            write!(f, "]")


        }
    }


    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

pub fn Formatting(){
   let foo =  3735928559i64;
   println!( "{}", format!("{}", foo) )  ;
   println!( "{}", format!("0x{:X}", foo) )  ;
   println!( "{}", format!("0o{:o}", foo) )  ;


    struct City {
        name: &'static str,
        // Latitude
        lat: f32,
        // Longitude
        lon: f32,
    }

    impl Display for City {
        // `f` is a buffer, and this method must write the formatted string into it
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            // `write!` is like `format!`, but it will write the formatted string
            // into a buffer (the first argument)
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                   self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl fmt::Display for Color{
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
          // RGB (128, 255, 90) 0x80FF5A
            write!(f , "RGB ({0}, {1}, {2}) 0x{0:X}{1:X}{2:X}" , self.red,self.green, self.blue)
        }
    }

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", *color);
        println!("{}", *color);
    }

}