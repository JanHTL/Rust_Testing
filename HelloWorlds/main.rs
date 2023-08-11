fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
    let a: i32 = 10;
    let b: i32 = 20;
    let c: i32 = a + b;
    println!("{} + {} = {}", a, b, c);
    println!("for loop");
    for i in 0..10 {
        println!("{}", i);
    }
    println!("normal loop");
    let mut i: i32 = 0;
    loop {
        println!("{}", i);
        i += 1;
        if i == 10 {
            break;
        }
    }
    println!("while loop");
    let mut i: i32 = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }
    println!("if else");
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }
        lhs % rhs == 0
    }
    println!("{} is divisible by {}? {}", 9, 3, is_divisible_by(9, 3));
    println!("{} is divisible by {}? {}", 9, 2, is_divisible_by(9, 2));
    let n = 5;
    fn fizzbuzz(n: u32) {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    fizzbuzz(n);
    println!("arrays");
    let a = [0, 1, 2, 3, 4];
    println!("a has {} elements", a.len());
    println!("a[0] = {}", a[0]);
    for i in 0..a.len() {
        println!("a[{}] = {}", i, a[i]);
    }
    println!("slices");
    let a = [0, 1, 2, 3, 4];
    let complete = &a[..];
    let middle = &a[1..4];
    println!("complete = {:?}", complete);
    println!("middle = {:?}", middle);
    println!("tuples");
    let x = (1, "hello");
    println!("x = {:?}", x);
    let y: (i32, &str) = (1, "hello");
    println!("y = {:?}", y);
    let (x, y, z) = (1, 2, 3);
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("functions");
    fn foo(x: i32) -> i32 { x*2 }
    let x: i32 = 5;
    let f: fn(i32) -> i32 = foo;
    println!("f(x) = {}", f(x));
    println!("f(10) = {}", f(10));
    println!("f(20) = {}", f(20));
    println!("methods");
    struct Point {
        x: f64,
        y: f64,
    }
    impl Point {
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }
    struct Rectangle {
        p1: Point,
        p2: Point,
    }
    impl Rectangle {
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            ((x1 - x2) * (y1 - y2)).abs()
        }
        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;
            self.p1.y += y;
            self.p2.y += y;
        }
    }
    let mut rect = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("Rectangle: p1 = {:?}, p2 = {:?}", rect.p1, rect.p2);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    rect.translate(1.0, 0.0);
    println!("Rectangle: p1 = {:?}, p2 = {:?}", rect.p1, rect.p2);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("enums");
    enum Person {
        Engineer,
        Scientist,
        Height(i32),
        Weight(i32),
        Info { name: String, height: i32 }
    }
    fn inspect(p: Person) {
        match p {
            Person::Engineer => println!("Is an engineer!"),
            Person::Scientist => println!("Is a scientist!"),
            Person::Height(i) => println!("Has a height of {}.", i),
            Person::Weight(i) => println!("Has a weight of {}.", i),
            Person::Info { name, height } => {
                println!("{} is {} tall!", name, height);
            },
        }
    }

}