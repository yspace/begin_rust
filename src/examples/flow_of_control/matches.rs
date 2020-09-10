
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
    destructuring::pointers_ref::act_main() ;
    destructuring::structs::act_main() ;

    guards::act_main() ;

    bingding::act_at_sigil();
    bingding::act_destruct_enum();
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

    pub mod pointers_ref{
        pub fn act_main(){
            // Assign a reference of type `i32`. The `&` signifies there
            // is a reference being assigned.
            let reference = &4;

            match reference {
                // If `reference` is pattern matched against `&val`, it results
                // in a comparison like:
                // `&i32`
                // `&val`
                // ^ We see that if the matching `&`s are dropped, then the `i32`
                // should be assigned to `val`.
                &val => println!("Got a value via destructuring: {:?}", val),
            }

            // To avoid the `&`, you dereference before matching.
            match *reference {
                val => println!("Got a value via dereferencing: {:?}", val),
            }

            // What if you don't start with a reference? `reference` was a `&`
            // because the right side was already a reference. This is not
            // a reference because the right side is not one.
            let _not_a_reference = 3;

            // Rust provides `ref` for exactly this purpose. It modifies the
            // assignment so that a reference is created for the element; this
            // reference is assigned.
            let ref _is_a_reference = 3;

            // Accordingly, by defining 2 values without references, references
            // can be retrieved via `ref` and `ref mut`.
            let value = 5;
            let mut mut_value = 6;

            // Use `ref` keyword to create a reference.
            match value {
                ref r => println!("Got a reference to a value: {:?}", r),
            }

            // Use `ref mut` similarly.
            match mut_value {
                ref mut m => {
                    // Got a reference. Gotta dereference it before we can
                    // add anything to it.
                    *m += 10;
                    println!("We added 10. `mut_value`: {:?}", m);
                },
            }
        }
    }

    pub mod structs{
        pub fn act_main(){
            struct Foo{
                x: (u32, u32),
                y: u32 ,
            }


            // Try changing the values in the struct to see what happens
            let foo = Foo { x: (1, 2), y: 3 };

            match foo {
                Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

                // you can destructure structs and rename the variables,
                // the order is not important
                Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

                // and you can also ignore some variables:
                Foo { y, .. } => println!("y = {}, we don't care about x", y),
                // this will give an error: pattern does not mention field `x`
                //Foo { y } => println!("y = {}", y),
            }

        }
    }
}

mod guards{
    pub  fn act_main() {
        let pair = (2, -2);
        // TODO ^ Try different values for `pair`

        println!("Tell me about {:?}", pair);
        match pair {
            (x, y) if x == y => println!("These are twins"),
            // The ^ `if condition` part is a guard
            (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
            (x, _) if x % 2 == 1 => println!("The first one is odd"),
            _ => println!("No correlation..."),
        }
    }
}

mod bingding{
    // A function `age` which returns a `u32`.
    fn age() -> u32 {
        15
    }

    pub fn act_at_sigil() {
        println!("Tell me what type of person you are");

        match age() {
            0             => println!("I haven't celebrated my first birthday yet"),
            // Could `match` 1 ..= 12 directly but then what age
            // would the child be? Instead, bind to `n` for the
            // sequence of 1 ..= 12. Now the age can be reported.
            n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
            n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
            // Nothing bound. Return the result.
            n             => println!("I'm an old person of age {:?}", n),
        }
    }

    fn some_number() -> Option<u32> {
        Some(42)
    }

    pub fn act_destruct_enum() {
        match some_number() {
            // Got `Some` variant, match if its value, bound to `n`,
            // is equal to 42.
            Some(n @ 42) => println!("The Answer: {}!", n),
            // Match any other number.
            Some(n)      => println!("Not interesting... {}", n),
            // Match anything else (`None` variant).
            _            => (),
        }
    }

}