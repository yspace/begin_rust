

pub fn action_main(){
    matching_literals() ;
    matching_named_variables() ;
    match_multiple_patterns() ;
    match_range_of_values() ;

    destructuring_structs();
    destructuring_enums();
    destructuring_nested_structs_and_enums() ;
    ignore_values_in_a_pattern();
    ignoring_parts_of_a_value();
    ignoring_multiple_parts_of_a_tuple() ;

    unused_variables() ;
    ignoring_remaining_parts_of_a_value();
    match_guard( );
    at_bindings() ;
}

fn matching_literals(){
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_variables(){
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn match_multiple_patterns(){
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_range_of_values(){
    // Ranges are only allowed with numeric values or char values,
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    //
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// Destructuring to Break Apart Values
struct Point {
    x: i32,
    y: i32,
}
fn destructuring_structs(){

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Destructuring struct fields using struct field shorthand
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    //
    let p = Point { x: 0, y: 7 };
    // Destructuring and matching literal values in one pattern
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructuring_enums(){
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}

fn destructuring_nested_structs_and_enums(){
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

fn destructuring_structs_and_tuples(){
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

fn ignore_values_in_a_pattern(){
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);;
}

fn ignoring_parts_of_a_value(){
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

fn ignoring_multiple_parts_of_a_tuple(){
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

fn unused_variables(){
    let _x = 5;
//    let y = 10;


    let s = Some(String::from("Hello!"));

//    if let Some(_s) = s { // NOTE the _s still bind to value
    if let Some(_) = s {  // no move happens
        println!("found a string");
    }

    println!("{:?}", s);
}

fn ignoring_remaining_parts_of_a_value(){
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        // Ignoring all fields of a Point except for x by using ..
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // Matching only the first and last values in a tuple and ignoring all other values
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

fn match_guard(){
    let num = Some(4);

    //  Adding a match guard to a pattern
    match num {
        // A match guard is an additional if condition specified after the pattern in a match arm that must also match
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    //
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    // Combining multiple patterns with a match guard
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn at_bindings(){
    // Using @ to bind to a value in a pattern while also testing it

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    // Using @ lets us test a value and save it in a variable within one pattern.
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}