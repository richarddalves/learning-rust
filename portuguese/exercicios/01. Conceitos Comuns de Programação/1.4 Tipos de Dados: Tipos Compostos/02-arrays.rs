fn main() {
    let arr = [1, 2, 3, 4, 5];
    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];

    let arr2: [i32; 5] = [1, 2, 3, 4, 5];

    let arr3 = [3; 5]; // [3, 3, 3, 3, 3]
    let arr4 = [3, 5]; // [3, 5]

    println!("arr3: {:?}", arr3);
    println!("arr4: {:?}", arr4);

    // indexa começando pelo elemento 0
    println!("O segundo elemento do array \"meses\" é {}", meses[1]);
}