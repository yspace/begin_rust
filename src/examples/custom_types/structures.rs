
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8 ,
}

// 一个unit结构
struct Unit;

// 元组结构  带名称的元组|被命名的元组类型|有名称的元组类型
struct Pair(i32, f32);

// 有两个字段的结构
#[derive(Debug,Copy, Clone)]
struct Point{
    x: f32,
    y: f32,
}
// 结构可以被作为另一个结构的字段重用
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle{
    // 一个矩形可以通过在空间中的左上角和右下角的点来指定
    top_left: Point,
    bottom_right: Point,
}

pub fn  action_main(){
    // 用字段初始化短形式来创建结构
    let name = "Peter";
    let age = 27 ;
    let peter = Person{
      name,
        age,
    };

    // 打印调试 结构体
    println!("{:?}", peter);

    // 初始化一个点
    let point: Point = Point{
        x: 10.3,
        y: 0.4,
    };

    // 访问点的字段
    println!("point coordinates: ({}, {})", point.x, point.y) ;

    // 通过使用结构更新语法 用我们另一个结构的字段来构造一个新点
    let bottom_right = Point{
        x: 5.2,
        ..point  // 不允许一成不变的复制另一个结构体实例，意思就是说至少重新设定一个字段的值才能引用其他实例的值。
    };

    // 右下角的y坐标同第一个点 因为我们就用的那个点的y来构造的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 使用`let`绑定来解构点
    let Point{x: top_edge, y: left_edge} = point ;

    let _rectangle = Rectangle{
        // 结构实例化也是一个表达式
        top_left: Point{x: left_edge, y: top_edge},
        bottom_right: bottom_right ,
    };

    // 实例化一个unit结构
    let _unit = Unit ;

    // 实例化一个元组结构
    let pair = Pair(1,0.1) ;

    // 访问元组结构的字段
    println!("pair contians {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构
    let Pair(integer, decimal ) = pair ;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // 练习
    activity::act1() ;
    activity::act2() ;
}

mod activity{
    use super::* ;

    impl Rectangle{
        pub fn rect_area(&self)-> f32{
            let Rectangle{
                top_left: Point{x: x1,y: y1},
                bottom_right: Point{x: x2, y: y2}
            } = self ;

            (x2 - x1) * (y2 - y1)
        }
    }

    pub fn act1(){
        let r = Rectangle{
          top_left:Point{x: 0.0, y:0.0},
          bottom_right:Point{x: 4.0, y:4.0},
        };
        println!("area of a rectangle: {}", r.rect_area());
    }

    impl Rectangle{
        // 根据给定的点已经长宽来构造一个正方形
        pub fn square(point: &Point, width_and_height: f32 )-> Self{
            Rectangle{
                top_left: point.clone() , // NOTE Point{..point} not allowed, must modify one field of the Point,
                bottom_right: Point{
                  x: point.x+width_and_height,
                    y: point.y + width_and_height ,
                },
            }
        }


    }
    pub fn act2(){
        let p = Point{
          x: 0.0,
            y: 0.0
        };
        let r = Rectangle::square(&p,4.0) ;
        println!("{:?}", r) ;

    }
}