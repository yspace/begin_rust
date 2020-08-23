

pub trait Draw {
    fn draw(&self) ;
}

// Definition of the Screen struct with a components field
// holding a vector of trait objects that implement the Draw trait
pub struct Screen {
    // trait objects allow for multiple concrete types to fill in for the trait object at runtime.
    pub components: Vec<Box<dyn Draw>> ,
}

impl Screen{
    pub fn run(&self) {
        for component in self.components.iter(){
            component.draw() ;
        }
    }
}

mod alt{
    use super::* ;
    // A generic type parameter can only be substituted with one concrete type at a time
    pub struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
        where
            T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("draw button here !")
    }
}

pub mod some_lib {
    use super::* ;
//    use gui::Draw;

    pub struct SelectBox {
     pub   width: u32,
     pub   height: u32,
     pub   options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            // code to actually draw a select box
            println!("draw the select-box here!") ;
        }
    }
}

use self::some_lib::SelectBox;
//use gui::{Button, Screen};
//use self::{Button, Screen} ;
pub fn action_main(){


    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // 扩充知识：试下any
    let sb =   Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
        ],
    }) ;
    print_any_draw(&sb) ;

    let mut a: &Any;
    a = &sb;
    println!("{:?}", a.type_id());
    assert!(a.is::<Box<SelectBox>>());

}

use std::any::{Any, TypeId};
// see: https://blog.csdn.net/K_Ohaha/article/details/105421601
// see: https://bbs.csdn.net/topics/392031097?locationNum=6&fps=1
// see: https://rustcc.gitbooks.io/rustprimer/content/trait/trait-object.html
fn print_any_draw(any: &Any) {
    // 向下转型
    if let Some(v) = any.downcast_ref::<Box<SelectBox>>() {
        println!("it is a select-box type");
    } else if let Some(v) = any.downcast_ref::<&str>() {
        println!("str {:?}", v);
    } else {
        println!("else");
    }
}