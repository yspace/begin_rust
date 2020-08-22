

pub fn action_main(){
    conditional_if_let_expr() ;
    conditional_loops() ;
    for_loops();
    let_statements() ;
    function_parameters() ;
}

//  Mixing if let, else if, else if let, and else
fn conditional_if_let_expr(){
    let favorite_color: Option<&str> = None ;
    let is_tuesday = false ;
    let age: Result<u8, _ > = "34".parse() ;

    if let Some(color) = favorite_color{
        println!("Using your favorite color , {}, as background!",color) ;
    }else if is_tuesday{
        println!("Tuesday is green day");
    }else if let Ok(age) = age {
        if age > 30{
            println!("Using purple as the background color");
        }else{
            println!("Using orange as the background color");
        }
    }else{
        println!("Using blue as the background color");
    }
}

fn  conditional_loops(){
    // Using a while let loop to print values for as long as stack.pop() returns Some

    let mut  stack = Vec::new() ;

    stack.push(1) ;
    stack.push(2) ;
    stack.push(3) ;

    while let Some(top) = stack.pop(){
        println!("{}", top) ;
    }
}

// Using a pattern in a for loop to destructure a tuple
fn for_loops(){
    // in for x in y the x is the pattern.

    let v = vec!['a','b','c'];

    // use the enumerate method to adapt an iterator to produce a value and that value’s index in the iterator, placed into a tu
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index) ;
    }
}

fn let_statements(){
    // let PATTERN = EXPRESSION;

    // Because the name x is the whole pattern, this pattern effectively means
    // “bind everything to the variable x, whatever the value is.”
    let x = 5; // bind what matches here to the variable x

    // uses a pattern with let to destructure a tuple
    let (x, y, z) = (1,2,3);
    println!("Using a pattern to destructure a tuple and create three variables at once\
    \
    \n x={}, y={}, z={}", x,y,z) ;
    //  ignore one or more of the values in the tuple, we could use _ or ..

    let(n , _,_) = (1,2,3) ;
    println!("n is {}", n) ;
    let (m, ..) = (1,2,3) ;
    println!("m is {}", m) ;

}

fn function_parameters(){
    // Function parameters can also be

    // A function signature uses patterns in the parameters
    // x part is a pattern
    fn foo(x: i32) {
        // code goes here
    }

    // A function with parameters that destructure a tuple
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    // We can also use patterns in closure parameter lists in the same way as in function parameter lists,
    // because closures are similar to functions
}