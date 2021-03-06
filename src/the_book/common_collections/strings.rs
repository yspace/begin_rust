
pub fn action_string(){
    create() ;
    update() ;
    index() ;
    slicing() ;
    iter() ;
}

fn create(){
    let mut s = String::new() ;

    let data = "initial data" ;
    let s2 = data.to_string() ; // 很多实现了Display的类都有此方法

    // the method also works on a literal directly:
//    let s = "initial contents".to_string();

//    let s = String::from("initial contents");

//    let hello = String::from("السلام عليكم");
//    let hello = String::from("Dobrý den");
//    let hello = String::from("Hello");
//    let hello = String::from("שָׁלוֹם");
//    let hello = String::from("नमस्ते");
//    let hello = String::from("こんにちは");
//    let hello = String::from("안녕하세요");
//    let hello = String::from("你好");
//    let hello = String::from("Olá");
//    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let hi =     hello + " 世界" ;
    println!("{}", hi) ;

}

fn update(){
    let mut s = String::from("foo") ;
    s.push_str("bar") ;
    println!("{}", s) ;

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // 单字符
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    // 连接
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3) ;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {}", s) ;

    // format 宏 不会拿走字符串的所有权的 + 第一个字符串所有权进行了转移哦
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}

fn index(){
//    let s1 = String::from("hello");
//    let h = s1[0];
}
fn slicing(){
    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("{}", s) ;
}

fn iter(){
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}