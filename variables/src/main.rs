fn main(){
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let num1: i8 = 127;
    let num2: i8 = -128;
    println!("num1: {}, num2: {}", num1, num2);

    let num3 = 90_234;
    let num4 = 0b1111_0101;
    let num5 = b'A';
    println!("num3: {}, num4: {}, num5: {}", num3, num4, num5);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x: {}, y: {}", x, y);


    // addition
    let sum = 5 + 10;
    println!("Sum: {}", sum);
    // subtraction
    let diff = 95.5 - 4.3;
    println!("Diff: {}", diff);

    // multiplication
    let prod = 4 * 30;
    println!("Prod: {}", prod);

    // division
    let quotient = 56.7 / 32.2;
    println!("Quotient: {}", quotient);

    let floored = 2 / 3; // Results in 0
    println!("Floored: {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("Remainder: {}", remainder);


    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("C:{}, Z: {}, Emote: {}", c, z, heart_eyed_cat)

}