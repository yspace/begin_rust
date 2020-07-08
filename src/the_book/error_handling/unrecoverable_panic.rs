
pub fn action_panic(){
//    panic!("crash and burn") ;

    // NOTE win跟*nix环境下 执行带环境变量的命令会不相同:  set RUST_BACKTRACE=1 && cargo run book-panic
    let  v = vec![1,2,3] ;
    v[99];

}