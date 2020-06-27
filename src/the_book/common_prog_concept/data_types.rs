
pub fn action_data_types(){
    // ## 转型
    {
        let guess : u32 = "42".parse().expect("Not a number"); // parse 的类型依赖声明
        let guess2 : f64 = "42".parse().expect("Not a number"); // parse 的类型依赖声明

        println!("guess is :{}",guess);
        println!("guess is :{}",guess2);
    }
    // ##
    {
        let x  = 2.0; // 默认f64
        let y: f32 = 3.0 ; // 手动指定类型
        println!("x is {} , y is {}",x, y) ;
    }

    // ## ops
    {
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;

        // remainder
        let remainder = 43 % 5;

        println!("{} {} {} {} {}" ,sum, difference, product, quotient, remainder)
    }

    // ## bool  一字节
    {
        let t = true ;

        let f: bool = false ;

        if t {
            println!("it is true");
        }
        if f {
            println!("this won't print unless if it is true");
        }else{
            println!("it is false " ) ;
        }

    }

    // ## char
    {
        // Rust’s char type is four bytes in size and represents a Unicode Scalar Value
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';

        println!("{} {} {} ", c , z , heart_eyed_cat) ;
    }

    {

        let _tup: (i32, f64, u8) = (500, 6.4, 1);
        let tup = (500,6.1, 1) ;
        // we can use pattern matching to destructure a tuple value, like this:
        let (x, y, z) = tup ;
        println!("the value of y is {}", y) ;

        // . 运算
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;
    }
    // ## 数组 定长 同类型
    {
        // vector 是不定长的
        let a = [1,2,3,4,5];
        println!("a is : {:?}", a) ;

        let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];

        println!("12 months : {:?}", months) ;

        let a: [i32; 5] = [1, 2, 3, 4, 5];
        println!("the a is : {:?}", a) ;

        // 带初始值的数组
        let a = [3; 5];
        println!(" the a is {:?}", a) ;

        // 访问数组元素
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];
        println!("first : {} , second {}", first, second) ;
    }
}