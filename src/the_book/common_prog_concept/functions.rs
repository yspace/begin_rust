
pub fn action_basic(){
    println!("Hello, world!");

    another_function();

    // 带参数
    another_func2(5);
    another_func3(1,2) ;

    statement_expression();

    let x = five() ;
    println!("the x is {}", x) ;

    let x = plus_one(5) ;
    println!("the value of x is {}", x) ;
}

fn another_function(){
    println!("Another function");
}

fn another_func2(x: i32) {
    // people tend to use the words parameter and argument interchangeably for either the variables
    // in a function’s definition or the concrete values passed in when you call a function
    println!("The value of  x is {}", x) ;
}

// 多参数
fn another_func3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn statement_expression(){
    // 注意 隐含的() 元组返回值 大括号里面的最后一个如果不是一个有效的表达式 那么就隐含返回一个()元组！

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resulting value.

    let x = 5;

    //  Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression, you turn it into a statement,
    // which will then not return a value.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five()-> i32{
    5
}

fn plus_one(x: i32)-> i32{
    x + 1
}