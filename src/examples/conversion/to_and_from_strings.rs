use std::fmt::{self, Result,Formatter} ;

struct Circle{
    radius: i32
}

impl fmt::Display for Circle{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

pub fn act_main(){
    let circle = Circle{radius: 6} ;
    println!("{}", circle.to_string()) ;

    //
    act_convert_string() ;
}

pub fn act_convert_string(){
    //  arrange for type inference
    let parsed: i32 = "5".parse().unwrap() ;
    // using the 'turbofish' syntax
    let turbo_parsed = "10".parse::<i32>().unwrap() ;

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let my_t = "45".parse::<MyT>().unwrap() ;
    println!("my t is {:?}", my_t) ;
}

#[derive(Debug)]
struct MyT {
    age: i32 ,
}

impl core::str::FromStr for MyT{
    type Err = ();

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let result  = s.parse() ;
        match result {
            Ok(age) => Ok(MyT{age}),
            Err(_) => Err(()),
        }
        // Ok(MyT{age:10})
    }
}