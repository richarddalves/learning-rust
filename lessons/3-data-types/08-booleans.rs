fn main() {
    let is_handsome = true;
    let is_silly = false;
    println!("Handsome: {is_handsome}. Silly: {is_silly}");
 
    let age: i32 = 21;
    let is_adult = age >= 18;
    println!("{is_adult}");
    println!("{} {}", age.is_positive(), age.is_negative());
 }