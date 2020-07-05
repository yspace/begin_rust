

pub fn action_vector(){
    // 下面两种方式是等效的
    let v:  Vec<i32> = Vec::new() ;
    let v = vec![1, 2, 3]; // 因为给了初始值 所以可以推断其元素类型

    // 修改
    let mut v = Vec::new() ; // 因为下面的操作 类型也是可以推断出来的
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    println!("vec is {:?}", v) ;

    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2] ;
    println!("the third element is {}", third) ;

    match v.get(2) {
        Some(third) => println!("the third element is {}", third) ,
        None => println!("there is no third element"),
    }

    let v = vec![100, 32, 57];
    for i in &v{
        println!("{}", i) ;
    }
    // 修改元素
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 解引用
    }

    // ## Using an Enum to Store Multiple Types
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        for cell in row{
            println!("{:?}", cell) ;
        }
}