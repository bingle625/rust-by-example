// debug -> 개발자용 진단 출력은 바로 출력하기는 불가능하고
// Debug 트레이트를 구현해야 사용이 가능하다.
// 내 타입은 기본으론 안 되고, **#[derive(Debug)]**로 자동 구현하거나 직접 구현해야 함.

// 	{} = fmt::Display
// 사용자에게 보여줄 “예쁜 출력”. 대부분의 표준 타입은 이미 구현돼 있지만, 커스텀 타입은 직접 구현해야 합니다(derive 불가).

// pretty print: {:#?}
// Debug 출력에 줄바꿈과 들여쓰기까지 넣어 가독성을 높입니다.

use std::fmt;

struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

struct Printable(i32);


// 1) 사람이 보기 좋은 출력
impl fmt::Display for Printable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 원하는 모양으로 제어 가능
        write!(f, "{}", self.0)
    }
}


fn main(){
    println!("hello");

    let s1 = UnPrintable(1);

    let s2 = DebugPrintable(3);

    // println!("{:?}", s1); 
    // `UnPrintable` cannot be formatted using `{:?}` because it doesn't implement `Debug`

    println!("{:?}", s2);
    // DebugPrintable(3)

    // 위치 기반 출력
    // "Christian" "Slater" is the "actor's" name.
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's"); 

    // 예쁜 디버그 출력
    /*
    Person {
    name: "Peter",
    age: 27,
    }
    */
    let peter = Person { name: "Peter", age: 27 };
    println!("{:#?}", peter);

    let s = Printable(125);
    println!("{}", s);
    
}