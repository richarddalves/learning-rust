fn main() {
   // The Array Type
   let numbers: [i32; 6] = [4, 8, 15, 16, 23, 26];
   let apples = ["Granny Smith", "McIntosh", "Red Delicious"];
   println!("Length: {}", apples.len());

   let currency_rates: [f64; 0] = [];

   // Reading and Writing Array Elements
   let mut seasons = ["Spring", "Summer", "Fall", "Winter"];

   let _first = seasons[0];
   let _last = seasons[seasons.len() - 1];

   println!("{}", seasons[2]);
   seasons[2] = "Autumn";
   println!("{}", seasons[2]);
}