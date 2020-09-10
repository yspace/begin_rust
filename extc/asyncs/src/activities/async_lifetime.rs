
use futures::executor ;
use std::future::Future;

async fn foo(x: &u8) -> u8 {
    *x
}
//
//fn bad() -> impl Future<Output = u8>{
//    let x = 5 ;
//    foo(&x)
//}

fn good() -> impl Future<Output = u8>{
    async{
        let x = 5 ;
        foo(&x).await
    }
}