
// Patterns come in two forms
// - Patterns that will match for any possible value passed are irrefutable
// - Patterns that can fail to match for some possible value are refutable

pub fn action_main(){
    refutable_pattern_with_let();
}

fn  refutable_pattern_with_let(){
     // let some_option_value =  None ;
     let some_option_value = Some(2) ;

     if let Some(x) = some_option_value{
        println!("{}", x);
    }

    // Attempting to use an irrefutable pattern with if let
    if let x = 5 {
        println!("{}", x);
    };
}