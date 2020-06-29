struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool ,
}

fn build_user(email: String, username: String) -> User{
    // 有默认值
    User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user2(email: String, username: String) -> User {
    //  field init shorthand syntax
    // 字段初始化短语法 不必重复同名字段啦
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn action_def(){
    // declare
    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1 ,
    };

    println!("user {} email is {}", user1.username, user1.email);

    // 成员可写的结构体声明
    let mut user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1 ,
    };

    user1.email = String::from("anotheremail@example.com");

    //
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // 从user1 创建user2
        // The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    };
}

// ### Tuple structs

pub fn action_tuple_struct(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

pub fn action_unit_like(){
    // they behave similarly to (), the unit type. Unit-like structs can be useful in situations
    // in which you need to implement a trait on some type but don’t have any data that
    // you want to store in the type itself.
}

// It’s possible for structs to store references to data owned by something else,
// but to do so requires the use of lifetimes
// Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
