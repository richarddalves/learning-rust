fn main() {
    let first_initial = 'R';
    let emoji: char = '😀';
 
    println!("{} {}", first_initial.is_alphabetic(), emoji.is_alphabetic());
    println!("{} {}", first_initial.is_uppercase(), emoji.is_uppercase());
    println!("{} {}", first_initial.is_lowercase(), emoji.is_lowercase());
 }