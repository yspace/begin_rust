// 匹配需要全覆盖！


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // The code associated with each arm is an expression, and the resulting value of the expression
    // in the matching arm is the value that gets returned for the entire match expression.
    match coin {
        Coin::Penny =>  {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn action_match(){
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

pub fn action_match_option(){
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

pub fn action_underscore(){
    // The _ pattern will match any value. By putting it after our other arms, the _ will match all
    // the possible cases that aren’t specified before it. The () is just the unit value,
    // so nothing will happen in the _ case. As a result, we can say that we want to do nothing
    // for all the possible values that we don’t list before the _ placeholder.

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

pub fn action_unwrap(){
    let x: Option<u16> = Some(5) ;

    if x != None {
        // 拆包后 进行奇数偶数判断
        if x.unwrap() % 2 == 0 {
            println!("even") ;
        }
    }
}