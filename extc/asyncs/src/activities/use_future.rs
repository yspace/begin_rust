
use std::future::Future;

async fn foo() -> u8 { 5 }

fn bar() -> impl Future<Output = u8> {

    async{
        let x : u8 = foo().await ;
        x +  5
    }
}

//fn my_func() -> impl Future<Output = u8> {
//    let closure = async |x: u8| {
//       x
//    } ;
//    closure(5)
//}