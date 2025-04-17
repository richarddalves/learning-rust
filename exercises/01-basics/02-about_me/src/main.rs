/*
Create a new `about_me` project with the `cargo new` command.

Using the `println!` macro, output 3 sentences about yourself.
Feel free to invoke the macro multiple times.

From the Terminal, compile the `main.rs` file inside the `src`
folder with the Rust compiler, then manually run the executable.

From the Terminal, compile the project with the Cargo tool, then
manually run the executable.

From the Terminal, compile and run the project with a single
Cargo command.

Check your program for errors with `cargo check`.

Add a comment at the top of your source code explaining how to
compile the program for new Rust developers.

Add some spaces and line breaks to the code so that it is formatted
in an ugly manner. From the Terminal, style the code with the
`cargo fmt` command.

Replace the `println!` macro with `print!`. What happens?
*/

// To compile the program you just need to run "cargo build" at the terminal inside the project folder or rustc to compile manually just the file you want to.
fn main() {
    println!("I am studying Computer Science at INTELI");
    println!("I am based in São Paulo");
    println!("I am currently learning Rust");
}
