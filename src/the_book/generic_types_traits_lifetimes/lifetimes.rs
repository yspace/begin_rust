
pub fn action_lifetimes(){
    nested_scopes() ;
    lifetimes_annotation() ;
    lifetime_annotation_struct() ;
}

fn nested_scopes(){
    {
        let r ;
     //   {
            let x = 5 ;
            r = &x ;
     //   }
        println!("r :{}", r)
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetimes_annotation(){
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    {
        let string1 = String::from("long string is long");
        let result;
//        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
//        }
        println!("The longest string is {}", result);
    }
}

fn lifetime_annotation_struct(){
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// ## Lifetime Elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Lifetimes on function or method parameters are called input lifetimes,
// and lifetimes on return values are called output lifetimes
// ## lifetime elision rules
// - The first rule is that each parameter that is a reference gets its own lifetime parameter
// - The second rule is if there is exactly one input lifetime parameter,
// that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
// - The third rule is if there are multiple input lifetime parameters, but one of them is &self
// or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

// ## Lifetime Annotations in Method Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn  static_lifetime(){
    // reference  can live for the entire duration of the program
    let s: &'static str = "I have a static lifetime.";
    // The text of this string is stored directly in the program’s binary
    println!("{}", s) ;
}


use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}