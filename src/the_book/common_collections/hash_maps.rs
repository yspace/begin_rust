use std::collections::HashMap;

pub fn action_hash_map(){
    create() ;
    access_values();
    iter() ;
    update() ;
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

fn access_values(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(v)=> {
          println!("{}:{}", team_name,  v);
        },
        None =>{
            println!("has no value for {}", team_name) ;
        },
    }
}

fn iter(){
    let mut scores = HashMap::new() ;

    scores.insert(String::from("Blue"), 10) ;
    scores.insert(String::from("Yellow"), 50) ;

    for (key, value) in &scores {
        println!("{}: {}", key, value) ;
    }
}

fn update(){

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 替换旧值

    println!("{:?}", scores);

    // ## Only Inserting a Value If the Key Has No Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("UC_ENTRY: {:?}", scores);

    // ## Updating a  Value based on the old Value
    let text = "hello world wonderful world" ;
    let mut map = HashMap::new() ;

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1 ;
    }

    println!("{:?}", map) ;
}