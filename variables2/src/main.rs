fn main() {
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("X:{}, Y:{}, Z:{}", x, y, z);

    // Acess by index
    let five_hundred = tup.0;
    let six_p_four = tup.1;
    let one = tup.2;

    println!("X:{}, Y:{}, Z:{}", five_hundred, six_p_four, one);

    // Array Types
    // Simple array
    let a = [1, 2, 3, 4, 5];
    // Array passing the types
    let a2: [i32; 5] = [1, 2, 3, 4, 5];

    // Array of repeated elements  3,3,3,3,3
    let a3 = [3; 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first = months[0];
    let second = months[1];
    println!("a: {:?}, a2: {:?}, a3: {:?}", a, a2, a3);
    println!("months: {:?}", months);
    println!("first: {}, second: {}", first, second);
}
