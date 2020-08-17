struct CustomSmartPointer{
    data: String ,
}

impl Drop for CustomSmartPointer{
    // Drop trait is included in the prelude

    //
    fn drop(&mut self){
        //  The drop function in Rust is one particular destructor.
        println!("Dropping CustomSmartPoint with data `{}`! ",
        self.data);
    }
}

pub fn action_drop_trait(){
    let c = CustomSmartPointer{
      data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    //
    std_mem_drop() ;
}

fn dorp_manually(){
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
//    c.drop(); // NOTE 此处是不允许手动调用的！
//    println!("CustomSmartPointer dropped before the end of main.");
}

/// std_mem_drop 标准库drop可用来手动调用类型所实现的drop方法！
fn std_mem_drop(){
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}