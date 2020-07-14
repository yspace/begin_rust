

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn action_largest(){
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

}

// largest 是在某些类型T上的泛化函数
// - list 是类型T的切片
//
// - return 返回类型T的值
//fn largest<T: PartialOrd>(list: &[T]) -> T {
//    let mut largest = list[0];
//
//    for &item in list {
////    for item in list {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}
//
//pub fn action_call_largest(){
//    let number_list = vec![34, 50, 25, 100, 65];
//
//    let result = largest(&number_list);
//    println!("The largest number is {}", result);
//
//    let char_list = vec!['y', 'm', 'a', 'q'];
//
//    let result = largest(&char_list);
//    println!("The largest char is {}", result);
//}

struct Point<T>{
    x: T,
    y: T,
}

pub fn action_use_generic_struct(){
    let p = Point{
      x: 10,
        y: 20,
    };
}

struct Point2<T, U> {
    x: T,
    y: U,
}

pub fn action_use_generic_struct2(){
    let p = Point2{
        x: 10,
        y: 20.3,
    };

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
}


// ### In Enum Definitions
#[derive(Debug)]
enum MyOption<T>{
    Some(T),
    None,
}

enum MyResult<T, E>{
    Ok(T),
    Err(E),
}

pub fn action_use_enum(){
    #[derive(Debug)]
    struct Person<T>{
        address: MyOption<T>,
    }

    let a = MyOption::Some(1);
    let p = Person{
        address: a,
    };

    println!("{:?}", p) ;
}

//### In Method Definition
pub fn action_in_method_def(){
    struct Point<T> {
        x: T,
        y: T,
    }

    // 实现在泛型结构体类型上的泛型方法
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    // 只对指定类型的 泛型结构体 实现某些方法
    // An impl block that only applies to a struct with a particular concrete type for the generic type parameter T
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
}

pub fn action_mixup(){
    struct Point<T, U>{
        x: T,
        y: U,
    }

    impl <T, U> Point<T, U> {
        fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W>{
            Point{
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}