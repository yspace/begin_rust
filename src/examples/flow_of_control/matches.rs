
// rust match 类似c中的 switch

pub fn act_main(){
    let number  = 13 ;
    println!("Tell me about {}", number);

    match number {
        // 匹配单个值
        1 => println!("One"),
        // 匹配多值
        2|3|5|7|11 => println!("this is a prime"),
        // 匹配一个包含区间（开区间)
        13 ..= 19 =>println!("A teen"),
        // 处理其他情形
        _ => println!("Ain't special"),
    }

    let boolean = true ;
    // match 也是一个表达式
    let binary = match boolean {
        // match的 臂 必须覆盖所有可能的值
        false => 0 ,
        true =>1,

    };
    println!("{} -> {}", boolean, binary) ;

    destructuring::tuples::act_main() ;
    destructuring::enums::act_main() ;
}

mod destructuring{
    pub mod tuples{
        pub fn act_main(){
            let pair = (0, -2);

            println!("Tell me about {:?}", pair);

            match pair {
                (0,y) => println!("First is `0` and y is `{:?}`", y),
                (x,0) => println!("`x` is `{:?}` and last is `0`", x),
                _ => println!("It does't matter what they are"),
                // `_`意味着不绑定值到任何变量
            }
        }
    }

    pub mod enums{
        // `allow` required to silence warnings because only
// one variant is used.
        #[allow(dead_code)]
        enum Color {
            // These 3 are specified solely by their name.
            Red,
            Blue,
            Green,
            // These likewise tie `u32` tuples to different names: color models.
            RGB(u32, u32, u32),
            HSV(u32, u32, u32),
            HSL(u32, u32, u32),
            CMY(u32, u32, u32),
            CMYK(u32, u32, u32, u32),
        }

        pub fn act_main(){
            let color = Color::RGB(122, 17, 40);
            // TODO ^ Try different variants for `color`

            println!("What color is it?");
            // An `enum` can be destructured using a `match`.
            match color {
                Color::Red   => println!("The color is Red!"),
                Color::Blue  => println!("The color is Blue!"),
                Color::Green => println!("The color is Green!"),
                Color::RGB(r, g, b) =>
                    println!("Red: {}, green: {}, and blue: {}!", r, g, b),
                Color::HSV(h, s, v) =>
                    println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
                Color::HSL(h, s, l) =>
                    println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
                Color::CMY(c, m, y) =>
                    println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
                Color::CMYK(c, m, y, k) =>
                    println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                             c, m, y, k),
                // Don't need another arm because all variants have been examined
            }
        }
    }
}