# Variáveis e Mutabilidade
Data: 27/04/2025
Alterado por último em: 27/04/2025

## Resumo
Para declarar variáveis em rust usamos as palavras-chave `let` ou `const`. Por padrão, as variáveis em Rust são imutáveis. Isso significa que, por padrão, não é possível alterar o valor, nem o tipo depois que foram criadas. 

É possível alterar os valores das variáveis criadas com let se, e somente se, for usado o termo `mut` na sua declaração juntamente com `let`, ficando `let mut nome_da_variavel`.

As variáveis `let` só podem ser criadas em escopo local e não precisam ter seu tipo declarado obrigatóriamente. O rust tenta fazer a inferência se não tiver o tipo declarado.

As variáveis `const` só podem receber valores *hard coded* e por convenção, seu nome deve seguir o formato SNAKE_UPPER_CASE. Além disso, seu tipo deve ser declarado obrigatoriamente.

## Exemplo
``` rust
CONST PI: f64 = 3.14159;

fn main() {
	let nome = "Richard";
	println!("Meu nome é {nome} e o valor de pi é: {PI}");
}
```