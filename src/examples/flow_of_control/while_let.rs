
fn increments_i(){
    let mut optional = Some(0) ;

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9");
                    optional = None ;
                }else{
                    println!("`i` is `{:?}`. Try again." , i );
                    optional = Some(i+1);
                }
            },
            _ => {break;}

        }
    }
}

pub fn nicer_way(){
    let mut optional = Some(0) ;

     // This reads: "while `let` destructures `optional` into
     // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
}