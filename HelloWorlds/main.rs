fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
    fn foo(x: i32) -> i32 { x*2 }
    let y: i32 = 5;
    let f: fn(i32) -> i32 = foo;
    println!("f(y) = {}", f(y));
    println!("f(10) = {}", f(10));
    println!("f(20) = {}", f(20)); 
}