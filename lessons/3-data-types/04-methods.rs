fn main() {
    let value: i32 = -15;
    println!("{}", value.abs());

    let empty_space = "           my content    ";
    println!("{}", empty_space.trim());

    println!("{}", value.pow(0));
    println!("{}", value.pow(1));
    println!("{}", value.pow(2));
}