fn main() {
    let number = 3;
    if number < 5 {
        println!("condition true");
    } else {
        println!("condition false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 2 == 0 {
        println!("divisible by 2");
    } else {
        println!("not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 6 } else { 2 };
    println!("The value of number is: {}", number);
}
