
pub fn  action_main(){
    // 因为已经手动标注了，编译器知道了`elem`类型是u8
    let elem = 5u8 ;

    // 创建一个空的向量(一种增长型数组)
    let mut vec = Vec::new() ;
    // 此时编译器还不知道vec的确切类型
    // 只知道它是某个东西的向量

    // 把`elem`插入向量
    vec.push(elem) ;
    // 哦！此时编译器就知道了 vec是类型u8的向量拉
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec) ;
}