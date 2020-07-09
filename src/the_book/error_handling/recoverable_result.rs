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

fn propagating_errors(){
    use std::io::{self , Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // 调用函数
    read_username_from_file() ;
}

fn shortcut4propagating_errors(){
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }


    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
}

use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn question_mark(){
    // 返回值需要是Result|Option 哦！ or another type that implements std::ops::Try
//    let f = File::open("hello.txt")?;

    use std::error::Error;
    use std::fs::File;

    // 这也是另一个合法的main函数形式呢
    //The Box<dyn Error> type is called a trait object, 

    // For now, you can read Box<dyn Error> to mean “any kind of error.”
    // Using ? in a main function with this return type is allowed.
    fn _main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
    }
}