fn main() {
    // Intro to Booleans
    let is_handsome = true;
    let is_silly = false;
    println!("Handsome: {is_handsome}. Silly: {is_silly}");
 
    let age: i32 = 21;
    let is_adult = age >= 18;
    println!("{is_adult}");
    println!("{} {}\n", age.is_positive(), age.is_negative());
 
    // Boolean Inversion with !
    let can_see_rated_r_movie = age >= 17;
    let cannot_see_rated_r_movie = !can_see_rated_r_movie;
    println!("I am {age} years old and. Can I not see this scary movie? {cannot_see_rated_r_movie}.\n");
 
    // Equality and Inequality Operators
    println!("{}", "Coke" == "Pepsi"); // false
    println!("{}", "Coke" != "Pepsi"); // true
    println!("{}", "Coke" == "coke"); // false
    println!("{}", "Coke" == "Coke "); // false
    println!("{}", "Coke" == "Coke");  // true
 
    println!("{}", 10 == 10); // true
    println!("{}", 10 != 10); // false
    
    println!("{}", 17.22 == 17.2200); // true
    println!("{}", 17.22 == 17.22001); // false
    println!("{}", 17.22 == 17.2200000000000001); // true
 
    println!("{}\n", 0.1 + 0.2 == 0.3); // false
 
    // And Logic with &&
    let purchased_ticket = true;
    let plane_on_time = true;
    let making_event = purchased_ticket && plane_on_time;
 
    println!("It is {} that I will arrive as expected.", making_event);
 }