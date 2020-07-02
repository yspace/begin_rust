
enum IpAddrKind {
    V4 ,
    V6 ,
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}


pub fn action_enum(){
    let four = IpAddrKind::V4 ;
    let six  = IpAddrKind::V6 ;

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr2::V4(String::from("127.0.0.1"));

    let loopback = IpAddr2::V6(String::from("::1"));

    //
    let home = IpAddr3::V4(127, 0, 0, 1);

    let loopback = IpAddr3::V6(String::from("::1"));
}

// store different amounts and types of values
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

//
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


// The Option<T> enum is so useful that it’s even included in the prelude;
// you don’t need to bring it into scope explicitly. In addition, so are its variants:
// you can use Some and None directly without the Option:: prefix.
// The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.
pub fn action_option(){
    let some_number = Some(5);
    let some_string = Some("a string");

    // If we use None rather than Some, we need to tell Rust what type of Option<T> we have,
    // because the compiler can’t infer the type that the Some variant will hold by looking only at a None value.
    let absent_number: Option<i32> = None ;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

   // let sum = x + y; //
    // In other words, you have to convert an Option<T> to a T before you can perform T operations with it.

    
}