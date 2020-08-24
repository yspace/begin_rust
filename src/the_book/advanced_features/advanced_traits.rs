
pub fn action_main(){
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );


    fully_qualified_syntax() ;
    associated_functions() ;

    using_super_traits() ;
    newtype_pattern() ;
}


fn specifying_placeholder_types(){

}

mod examples{
    pub trait Iterator {
        // The definition of the Iterator trait that has an associated type Item
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    // 泛型版
    pub trait Iterator_<T> {
        fn next(&mut self) -> Option<T>;
    }
}

use std::ops::Add;


#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Implementing the Add trait to overload the + operator for Point instances
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


mod tmp1{

    // a trait with one method and an associated type
    trait Add<RHS=Self> {
        // NOTE RHS=Self: this syntax is called default type parameters

        type Output;

        fn add(self, rhs: RHS) -> Self::Output;
    }
}

struct Millimeters(u32);
struct Meters(u32);
// 指定不同的默认类型 +操作符右手边的类型默认是实现该trait的类型  但可以指定不同类型 这样不同类型也可以相加了！
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Two traits are defined to have a fly method and are implemented on the Human type, and a fly method is implemented on Human directly
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn fully_qualified_syntax(){
    let person = Human;
    person.fly();  // 调用自己的方法


    // Specifying which trait’s fly method we want to call
    let person = Human;
    // Specifying the trait name before the method name clarifies to Rust which implementation of fly we want to call.
    Pilot::fly(&person);
    Wizard::fly(&person);
    // We could also write Human::fly(&person), which is equivalent to the person.fly()
    //  but this is a bit longer to write if we don’t need to disambiguate.
    person.fly();
}

// A trait with an associated function and a type with an associated function of the same name that also implements the trait
fn associated_functions(){
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());

    // Using fully qualified syntax to specify that we want to call the baby_name function from the Animal trait
    // as implemented on Dog
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // by saying that we want to treat the Dog type as an Animal for this function call

    // 语法：<Type as Trait>::function(receiver_if_method, next_arg, ...);
}


fn using_super_traits(){
    use std::fmt;
    // trait 依赖 或者说后者是前者他爹
//  Implementing the OutlinePrint trait that requires the functionality from Display
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}

//    use std::fmt;

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // 试下效果
   let p = Point{
        x: 3,
        y: 4 ,
    };
    p.outline_print() ;
}

// orphan rule that states we’re allowed to implement a trait on a type as long as either the trait or the type are local to our crate.
// 孤儿原则指出我们可以在类上实现trait 只要trait或者类任意一个 出自我们自己的crate即可

fn newtype_pattern(){
    use std::fmt;
    use std::ops::Deref;

    // Creating a Wrapper type around Vec<String> to implement Display
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // 实现解引用
    impl Deref for Wrapper{
        type Target = Vec<String>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    // 试下效果
    let inner = &*w ; // 解引用 并借用
    for item in inner{
        println!("from inner type vec!  {}", item) ;
    }

    // “新类型模式”缺点： 不能当内部类型对待 如果需要实现内部类型的所有方法 有个替代方法：实现Deref trait 解引用！
    // If we wanted the new type to have every method the inner type has, implementing the Deref trait
    //  on the Wrapper to return the inner type would be a solution.
    //
    // If we don’t want the Wrapper type to have all the methods of the inner type—for example,
    // to restrict the Wrapper type’s behavior—we would have to implement just the methods we do want manually.
}
