fn main(){
    // format! -> text를 String으로 작성해준다.
    // print!: format! 과 같지만, 콘솔상으로 출력한다.
    
    let a = "name1";
    let b = "name2";
    let s = format!("test: hello, {}", a);
    println!("{}", s);
    println!("{0} 과 {1}은 {0}의 집에서 모였다.", a, b);
    println!("{x} 과 {y}은 {x}의 집에서 모였다.", x=a, y=b);

    println!("{:>5}", 42);  // "   42" (오른쪽 정렬)
    println!("{:<5}", 42);  // "42   " (왼쪽 정렬)
    println!("{:^5}", 42);  // " 42  " (가운데 정렬)

    println!("{:x}", 255);  // "ff" (16진수)
    println!("{:b}", 10);   // "1010" (2진수)
    println!("{:08}", 42);  // "00000042" (0 패딩, 8자리)
    
    let v = vec![1, 2, 3];
    println!("{:?}", v); // "[1, 2, 3]"

    /*
    Rust에서 **매크로(macro)**는 코드를 생성하는 코드예요.
일반 함수랑 달리, 호출되는 시점이 아니라 컴파일 시점에 코드를 확장해서 넣어줍니다.
	•	함수: 입력 → 출력 (런타임 실행)
	•	매크로: 입력(토큰들) → 새로운 코드 생성 (컴파일 타임 실행)

예를 들어 println!("hello {}", name); 은 사실 함수 호출이 아니라, 컴파일러가 println! 매크로를 보고 내부적으로 std::io::_print(...) 같은 코드로 변환해주는 거예요.

이 덕분에:
	•	인자의 개수가 가변적일 수 있고 (함수는 고정 파라미터 필요),
	•	복잡한 코드를 반복 작성하지 않고 패턴화할 수 있습니다.

Rust 표준 라이브러리에 있는 println!, format!, vec!, dbg! 등이 전부 매크로예요.
    */
let number: f64 = 1.0;
let width: usize = 5;
println!("{number:>width$}");
// "    1"
}