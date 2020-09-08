
pub fn act_main(){
//    for n in 1 .. 101 {
    for n in 1 ..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 ==0 {
            println!("fizz");
        }else if n % 5 == 0{
            println!("buzz");
        }else{
            println!("{}", n);
        }
    }

    //
    act_for_and_iterators();
}

pub fn act_for_and_iterators(){
    let names = vec![
    "Bob",
        "Frank",
        "Ferris",
    ];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}",name),
        }
    }
    // 此处还可重用

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // 此处names 不可用了 已经move了
    // println!("{:?}", names) ;

    // 下面的允许在迭代时修改元素
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}