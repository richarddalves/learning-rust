fn main() {
    /* strings literals */ 
    println!("Hello, world!");

    // raw strings
    println!(r"Julieta said\nI love you, Romeo!"); // Julieta said\nI love you, Romeo!
    let file_path = r"/home/richard/Desktop/my_project/src/main.rs";


    /* dinamic strings */
    let mut user_entry = String::new();

    println!("Type something:");
    io::stdin()
        .read_line(&mut user_entry)
        .expect("Error reading entry");

    println!("You typed: {}", user_entry.trim());
}