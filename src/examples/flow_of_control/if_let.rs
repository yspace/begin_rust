
pub fn act_main(){
    awkward_match() ;
    usecase_matching_enums::act_main() ;

    usecase_matching_enums::challenge::main() ;
}

fn awkward_match(){
    let optional = Some(7) ;

    match optional {
        Some(i) =>{
            println!("this is a really long string and `{}`", i);
        },
        _ =>{},
    }
}

mod usecase_matching_enums{
    pub fn act_main(){
        let number = Some(7);
        let letter: Option<i32> = None ;
        let emoticon: Option<i32> = None ;

        // `if let` 构造读作：" 如果`let` 解构`number`到`Some(i)` ,执行块结构（`{}`） "
        if let Some(i) = number{
            println!("Matched {:?}!", i) ;
        }

        // 如果需要指定失败情况 可以使用else
        if let Some(i) = letter{
            println!("Matched {:?}!", i);
        }else {
            println!("Didn't match a number. Let's go with a letter!");
        }

        // 提供一个额外的失败条件
        let i_like_letters = false ;

        if let Some(i) = emoticon {
            println!("Matched {:?}!", i) ;
            // 解构失败。计算`else if `条件 去看看可选的失败分支是否被采用

        }else if i_like_letters {
            println!("Didn't match a number. let's go with a letter!");
        }else{
            // 条件计算为假 此为默认分支
            println!("I don't like letters. let's go with an emotion :)!") ;
        }
    }

    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }
    pub fn act_main2(){
        // Create example variables
        let a = Foo::Bar;
        let b = Foo::Baz;
        let c = Foo::Qux(100);

        // Variable a matches Foo::Bar
        if let Foo::Bar = a {
            println!("a is foobar");
        }

        // Variable b does not match Foo::Bar
        // So this will print nothing
        if let Foo::Bar = b {
            println!("b is foobar");
        }

        // Variable c matches Foo::Qux which has a value
        // Similar to Some() in the previous example
        if let Foo::Qux(value) = c {
            println!("c is {}", value);
        }

        // Binding also works with `if let`
        // 绑定
        if let Foo::Qux(value @ 100) = c {
            println!("c is one hundred");
        }
    }

    pub mod challenge{
        // This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
        enum Foo {Bar}

       pub   fn main() {
            let a = Foo::Bar;

            // Variable a matches Foo::Bar
//            if Foo::Bar == a {
            if let Foo::Bar = a {
                // ^-- this causes a compile-time error. Use `if let` instead.
                println!("a is foobar");
            }
        }

    }
}