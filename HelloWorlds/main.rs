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
    let mut z: i32 = 10;
    z = 20;
    println!("{}", z);
    
        let a: i32 = 10;
    let b: i32 = 20;
    let c: i32 = a + b;
    println!("{} + {} = {}", a, b, c);
    for i in 0..10 {
        println!("{}", i);
    }
    let mut i: i32 = 0;
    loop {
        println!("{}", i);
        i += 1;
        if i == 10 {
            break;
        }
    }
    let mut i: i32 = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }
        lhs % rhs == 0
    }
}