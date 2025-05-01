// tuples and arrays
fn main() {
    // Tuple Type
    let mut tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");

    let mut n1 = tup.0;
    let n2 = tup.1;
    let n3 = tup.2;
    tup.1 = 7.7;
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // Array Type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let all3 = [3; 5]; // let a = [3, 3, 3, 3, 3];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let first_month = months[0];
    let last_month = months[months.len() - 1];
}