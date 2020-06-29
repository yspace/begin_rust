

pub fn action_rectangles(){
    v0() ;
    tuple_version() ;
    struct_version() ;

    derived_traits();
}
fn v0(){
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

fn tuple_version(){
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixiels",
        area(rect1)
    );

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}

fn struct_version(){
    // Adding more meaning

    struct Rectangle{
        width: u32,
        height: u32,
            }

    // area which calculate the area of the specify rectangle
    // rect: immutable borrow of a struct Rectangle instance
    fn area(rect: &Rectangle) ->u32 {
        rect.width * rect.height
    }

    let rect1 = Rectangle{
      width: 30,
        height: 50,
    };

    println!(
      "The area of the rectangle is {} square pixles",
        area(&rect1)
    );
}

fn derived_traits(){
    //  Adding Useful Functionality with Derived Traits

    //  Adding the annotation to derive the Debug trait and printing the Rectangle instance using debug formatting
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle{
        width: 30 ,
        height: 50,
    };

    println!("rect1 is {:?}", rect1) ;
    // 更可读形式：
    println!("rect1 is {:#?}", rect1) ;

    // Rust has provided a number of traits for us to use with the derive annotation
    // that can add useful behavior to our custom types.
}