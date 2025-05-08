fn main() {
    // Intro to Booleans
    let is_handsome = true;
    let is_silly = false;
    println!("Handsome: {is_handsome}. Silly: {is_silly}");
 
    let age: i32 = 21;
    let is_adult = age >= 18;
    println!("{is_adult}");
    println!("{} {}", age.is_positive(), age.is_negative());
 
    // Boolean Inversion with !
    let can_see_rated_r_movie = age >= 17;
    let cannot_see_rated_r_movie = !can_see_rated_r_movie;
 
    println!("I am {age} years old and. Can I not see this scary movie? {cannot_see_rated_r_movie}.");
 }