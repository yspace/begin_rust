use std::convert::TryFrom;
use std::convert::TryInto;

// Unlike From/Into, the TryFrom/TryInto traits are used for fallible conversions,
// and as such, return Results.
// 不同于From/Into TryFrom/TryInto traits 是用于可失败转型的 因此返回Results

#[derive(Debug ,PartialEq)]
struct EvenNumber(i32) ;

impl TryFrom<i32> for EvenNumber{
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 ==0 {
            Ok(EvenNumber(value))
        }else{
            Err(())
        }
    }
}

pub fn act_main(){
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber,()> = 8i32.try_into() ;
    assert_eq!(result , Ok(EvenNumber(8))) ;
    let result: Result<EvenNumber, ()> = 5i32.try_into() ;
    assert_eq!(result, Err(())) ;
}