# Strings e Raw Strings
Criado em: 04/05/2025

## Resumo
Uma *string* é apenas uma coleção de caracteres em sequência. Em rust, precisa ser criada necessariamente com o seu conteúdo entre aspas duplas (`"string aqui"`).
Existem muitos tipos de strings em Rust, cada uma com um propósito diferente.
## Detalhamento
### 1. Strings Literais
*Strings literais* são strings que o compilador já conhece na hora da compilação. Isso acontece porque nós escrevemos a string diretamente no código fonte.

``` rust
println!("Hello, world!"); // Aqui, "Hello, world!" é uma string literal.
```

### 2. Strings Dinâmicas
Também podemos, por exemplo, criar um programa que pede para o usuário inserir um texto. Nesse caso, o texto não é conhecido previamente na hora de compilar o código, e, inclusive, pode ser diferente várias vezes, já que o usuário pode digitar sempre um texto diferente.

``` rust
use std::io;

fn main() {
    let mut entrada_usuario = String::new();

    println!("Digite algo:");
    io::stdin()
        .read_line(&mut entrada_usuario)
        .expect("Falha ao ler entrada");

    println!("Você digitou: {}", entrada_usuario.trim());
}

```

### Raw Strings
Também podemos incluir caracteres especiais (como `\n`, `\t`, `\\`) nas strings. Normalmente, esses caracteres são interpretados (por exemplo, `\n` vira uma nova linha).  
Caso não queiramos essa interpretação automática, usamos _raw strings_, que ignoram completamente os caracteres de escape, mostrando exatamente o que foi digitado.

```rust
println!(r"Julieta disse\nEu te amo, Romeu!"); // Julieta disse\nEu te amo, Romeu!
let file_path = r"/home/richard/Desktop/meu_projeto/src/main.rs";
```