fn main() {
    println!("Hello, world!");
    my_func();
    my_func2(47);
    my_func3(32, "ºC");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn my_func() {
    println!("my func");
}

fn my_func2(x: i32) {
    println!("The value of x: {}", x);
}
fn my_func3(x: i32, scale: &str) {
    println!("The temperature is: {} {}", x, scale);
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
