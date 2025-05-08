fn main() {
    let addition = 5 + 4;
    let subtraction = 10 - 6;
    let multiplication = 3 * 4;
    println!("Addition: {addition}, subtraction: {subtraction}, multiplication: {multiplication}");
 
    let floor_division = 12 / 7;
    println!("{floor_division}");
    
    let decimal_division = 12.0 / 7.0;
    println!("{decimal_division}");
 
    let remainder = 12 % 7;
    println!("{remainder}");

    // Augmented Assignment Operator
    let mut year = 2025;

    year += 1;
    println!("The new year is {year}");

    year -= 5;
    println!("The new year is {year}");

    year *= 2;
    println!("The new year is {year}");
   
    year /= 4;
    println!("The new year is {year}");
   
    year %= 4;
    println!("The new year is {year}");
 }