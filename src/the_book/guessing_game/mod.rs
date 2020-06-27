////  相当于nodejs中的index文件
//
//// 声明子模块
//mod destruct;
//
//pub mod funcs;
//
////  导出
//pub use self::destruct::*;


use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn Run() {

    let secret_number = rand::thread_rng().gen_range(1, 101);
 loop{
     println!("Guess the number!");



     println!("The secret number is: {}", secret_number);

     println!("Please input your guess.");

     let mut guess = String::new();

     io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");

     println!("You guessed: {}", guess);
//     let guess: u32 = guess.trim().parse().expect("Please type a number!");
     let guess: u32 = match   guess.trim().parse() {
         Ok(num) => num ,
         Err(_) => continue ,
     };

     match guess.cmp(&secret_number) {
         Ordering::Less => println!("Too small!"),
         Ordering::Greater => println!("Too big!"),
//         Ordering::Equal => println!("You win!"),
         Ordering::Equal => {
             println!("You win!");
             break ;
         },
     }
 }
}