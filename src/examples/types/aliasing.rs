// 类型别名 类型必须是UpperCamelCase 大写驼峰方式 不然编译器会报警告的 ，
//  此规则的例外就是原生类型  usize f64 等

// 纳秒
type NanoSecond = u64 ;
type Inch = u64 ;

// 使用属性来静默警告
#[allow(non_camel_case_types)]
type u64_t = u64;
// TODO ^ Try removing the attribute

pub fn action_main(){

    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t ;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
    // 别名不是新类型 不提供额外的类型安全保障
    // The main use of aliases is to reduce boilerplate; for example the IoResult<T> type is an alias
    // for the Result<T, IoError> type.
}