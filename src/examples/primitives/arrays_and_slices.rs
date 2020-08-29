use std::mem;

// this function borrows a slice
fn analize_slice(slice: &[i32]){
    println!("first element of the slice: {}",slice[0]);
    println!("the slice has {} elements",slice.len());
}

pub fn action_main(){
    // 固定大小的数组
    let xs: [i32; 5] = [1,2,3,4,5];

    // 所有的元素可被初始化为相同的值
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}",xs[1]) ;

    // 'len' 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在栈上分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可自动被借用为切片
    println!("borrow the whole array as a slice");
    analize_slice(&xs);

    // 切片可以指向数组的一个片段
    // 其形式： [starting_index .. ending_index]
    // starting_index 是在切片中的第一个位置
    // ending_index 是多于切片中的最后一个位置 NOTE: 该值的合法性取值范围是?
    println!("borrow a section of the array as a slice ") ;
    analize_slice(&ys[1 .. 4]);

    // 索引越界 导致编译错误
    // println!("{}", xs[5]);

}