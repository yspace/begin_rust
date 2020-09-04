// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10 ;


fn is_big(n: i32)-> bool{
    n > THRESHOLD
}
pub fn action_main(){
    let n = 16 ;

    // 在main线程中访问常量
     println!("This is {}", LANGUAGE);
     println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n){"big"}else{"small"});

    // Error! Cannot modify a `const`.
//    THRESHOLD = 5;
    // FIXME ^ Comment out this line

}