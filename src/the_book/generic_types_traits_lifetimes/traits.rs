
pub trait Summary{
    fn summarize(&self)-> String ;
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        format!("{}, by {} ({})",
        self.headline,
        self.author,
        self.location,
        )
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarize(&self) -> String {
      format!(
        "{}: {}",
          self.username,
          self.content,
      )
    }
}

pub  fn action_define_implements(){

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

pub fn action_default_impl(){
    // Note that it isn’t possible to call the default implementation from an overriding implementation of that same method.
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    struct NewsArticle{
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {}

    let na = NewsArticle{
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    } ;

    println!("{}", na.summarize());

}

pub trait Summary_v2 {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn action_default_impl2(){
    struct Tweet{
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary_v2 for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let t = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("tweet summarize: {}", t.summarize());
}

pub fn action_traits_as_param(){
    let t = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    let na = NewsArticle{
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    } ;
    notify(&t);
    notify(&na);
}
pub fn notify(item: &impl Summary){
    println!("Breaking news !{}", item.summarize());
}

//## Trait Bound Syntax
// The impl Trait syntax is convenient and makes for more concise code in simple cases. The trait bound syntax can express more complexity in other cases.
pub fn notify_<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// 多个trait约束
//pub fn notify2(item: &(impl Summary + Display)) {
//
//}
//pub fn notify_v2<T: Summary + Display>(item: &T) {
//
//}
// where约束
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//
//}
//fn some_function_<T, U>(t: &T, u: &U) -> i32
//    where T: Display + Clone,
//          U: Clone + Debug
//{
//
//}

//##Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn returns_summarizable2(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        // NOTE 由于impl 的编译器实现机制 此处不可用使用不同于NewArticle的类型了 参考：
        // https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
//        Tweet {
//            username: String::from("horse_ebooks"),
//            content: String::from(
//                "of course, as you probably already know, people",
//            ),
//            reply: false,
//            retweet: false,
//        }

        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn action_largest(){
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

}

pub fn action_conditionally_implement_methods(){
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

pub fn action_blanket_implementations(){
    // impl<T: Display> ToString for T {
    //    // --snip--
    //}
    // Because the standard library has this blanket implementation, we can call the to_string method
    // defined by the ToString trait on any type that implements the Display trait.
    let s = 3.to_string();
    println!("{}", s) ;
}