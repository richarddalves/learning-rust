fn main() {
    let month_days = 1..31;
    println!("{month_days:?}");

    let month_days = 1..=31;
    println!("{month_days:?}");

    for day in month_days {
        println!("{day}");
    }

    let letters = 'b'..'z';
    for letter in letters {
        println!("{letter}");
    }
}