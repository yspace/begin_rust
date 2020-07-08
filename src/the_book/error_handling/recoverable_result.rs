use std::fs::File;
use std::io::ErrorKind;

pub fn action_result(){
//    basic_match() ;
//    matching_on_different_errors() ;

    shortcuts4panic_unwrap() ;
}

fn basic_match(){
    let f = File::open("hello.txt");

    // In the case where File::open succeeds, the value in the variable f will be an instance of Ok
    // that contains a file handle. In the case where it fails, the value in f will be an instance of Err that contains more information about the kind of error that happened.
    let f = match f {
        Ok(file) => file ,
        Err(err) => panic!("Problem opening the file: {:?}", err),
    };
    // Note that, like the Option enum, the Result enum and its variants have been
    // brought into scope by the prelude, so we don’t need to specify Result:: before the Ok and
    // Err variants in the match arms.
}

fn matching_on_different_errors(){

    let f = File::open("hello.txt");
    // primitive
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn concise_handle(){
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn shortcuts4panic_unwrap(){
    //  If the Result value is the Ok variant, unwrap will return the value inside the Ok.
    // If the Result is the Err variant, unwrap will call the panic! macro for us

    let f = File::open("hello.txt").unwrap();

    // Using expect instead of unwrap and providing good error messages can convey your intent
    // and make tracking down the source of a panic easier.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}