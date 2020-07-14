// ---------------------------------
//  导出构造(mod or fn etc...):
pub mod generic_data_types ;
pub mod traits ;


// ---------------------------------



// finds the largest number in a list
fn find_largest_number(){
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list{
        if number > largest {
            largest = number ;
        }
    }

    println!("The largest number is {}", largest) ;
}
// 重复处理两个不同的数字列表
// 使用了相同的逻辑
fn dup_find_largest_(){
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
// 第一步消除重复
// 参数化待处理的列表 作为参数传递
fn largest(list: &[i32]) -> i32 {
    let mut  largest = list[0] ;

    for &item in list{
        if item > largest {
            largest = item ;
        }
    }

    largest
}

pub fn find_largest_in_list(){
    let number_list = vec![34, 50, 25, 100, 65] ;

    let result = largest(&number_list) ;
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}