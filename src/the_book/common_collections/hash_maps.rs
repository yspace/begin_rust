use std::collections::HashMap;

pub fn action_hash_map(){
    create() ;
}

fn create(){
    let mut scores = HashMap::new() ;

    scores.insert( String::from("blue"), 10);
    scores.insert(String::from("yellow"), 5) ;

    println!("score is {:#?}", scores) ;
    // ## 从两个向量来构造
    let   teams = vec![
    String::from("Blue"),
    String::from("Yello"),
    ];
    let initial_scores = vec![10, 50];

    // 使用下划线 rust会为我们推断出元素和键的类型的
    let mut scores : HashMap<_,_> = teams.into_iter()
        .zip(initial_scores.into_iter())
        .collect() ;
    println!("score is {:#?}", scores) ;
}

fn ownership(){
    let field_name = String::from("Favorite color");
    let field_value = String::from("blue");

    let mut map = HashMap::new() ;
    map.insert(field_name, field_value) ;
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
//    println!("{} {}", field_name, field_value);
}