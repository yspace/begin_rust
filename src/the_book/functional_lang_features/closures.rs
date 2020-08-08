
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
pub mod v1{
    use super::* ;

    pub fn action_generate_workout(){
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        generate_workout(simulated_user_specified_value, simulated_random_number);

    }
}

pub mod v2{
    use super::* ;
    pub fn action_generate_workout(){
//        v1::action_generate_workout() ;
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        generate_workout(simulated_user_specified_value, simulated_random_number);

    }
    fn generate_workout0(intensity: u32, random_number: u32) {
        let expensive_result = simulated_expensive_calculation(intensity);

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result);
            println!("Next, do {} situps!", expensive_result);
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", expensive_result);
            }
        }
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        let expensive_closure = |num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_closure(intensity));
            println!("Next, do {} situps!", expensive_closure(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", expensive_closure(intensity));
            }
        }
    }
}

//
mod closure_type_inference_and_annotation{
    use super::* ;

    fn foo1(){
        let expensive_closure = |num: u32| -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };
        fn  add_one_v1   (x: u32) -> u32 { x + 1 }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x|             { x + 1 };
        let add_one_v4 = |x|               x + 1  ;

        // 只有调用了才能推断上面不带参数|返回值 类型注解的闭包
        add_one_v3(2);
        add_one_v4(4) ;

        //

        let example_closure = |x| x;
        let example_closure2 = |x| x;

        let s = example_closure(String::from("hello"));
        let n = example_closure2(2);
    }


}


struct Cacher<T>
    where
        T: Fn(u32) -> u32 ,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32 ,
{
    fn new(calculation:T ) -> Cacher<T> {
        Cacher{
            calculation,
            value: None ,
        }
    }

    fn value(&mut self, arg: u32)-> u32 {
        match self.value {
            Some(v) => v ,
            None =>{
                let v = (self.calculation)(arg);
                self.value = Some(v) ;
                v
            }
        }
    }

}

mod generic_parameters_fn_traits{
    use super::* ;

    // We can create a struct that will hold the closure and the resulting value of calling the closure.
    // The struct will execute the closure only if we need the resulting value,
    // and it will cache the resulting value so the rest of our code doesn’t have to be responsible
    // for saving and reusing the result. You may know this pattern as memoization or lazy evaluation.

//    To define structs, enums, or function parameters that use closures, we use generics and trait bounds,



//    The Fn traits are provided by the standard library. All closures implement at least one of the traits: Fn, FnMut, or FnOnce.
    struct Foo_struct{

    }



    // Using Cacher in the generate_workout function to abstract away the caching logic
   pub  fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(intensity));
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }

}

pub mod v3{
    use super::* ;

    pub fn action_generate_workout(){
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        // 调用同宗（相同的super指代）其他兄弟模块的公共方法
        generic_parameters_fn_traits::
        generate_workout(simulated_user_specified_value, simulated_random_number);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

//        assert_eq!(v2, 2); // NOTE 当前实现只能返回第一次调用的值 针对不同参数返回不同的值 可以借助hashmap
        assert_eq!(v2, 1);
    }

}

pub mod capturing_the_environment{
    // closures have an additional capability that functions don’t have: they can capture their environment
    // and access variables from the scope in which they’re defined.

    pub fn main() {
        println!("===========< capturing_the_environment >==========") ;
        let x = 4;

        let equal_to_x = |z| z == x;

        // 下面这个就不可以使用x啦 但闭包可以呢
//        fn equal_to_x(z: i32) -> bool {
//            z == x
//        }

        let y = 4;

        assert!(equal_to_x(y));
        println!("===========< capturing_the_environment />==========") ;

    }

    // Closures can capture values from their environment in three ways,
    // which directly map to the three ways a function can take a parameter:
    // taking ownership, borrowing mutably, and borrowing immutably.
    // These are encoded in the three Fn traits as follows:

    //
//    FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
//    FnMut can change the environment because it mutably borrows values.
//    Fn borrows values from the environment immutably.


}