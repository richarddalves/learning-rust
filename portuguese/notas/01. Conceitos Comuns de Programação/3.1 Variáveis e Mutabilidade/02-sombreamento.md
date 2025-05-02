# Sombreamento
Data: 27/04/2025 <br>
Alterado por último em: 27/04/2025

## Resumo
Em Rust é possível declarar uma variável com o mesmo nome que outra variável já existente.
Por exemplo, é valido fazer:
``` rust
fn main() {
	let nome = "Richard";
	let nome = "Alves";
	println!("Meu nome é {nome}");
}
```
Ao fazer isso, será criada uma nova variável, mas com o mesmo nome da antiga. A antiga não será excluída, apenas não será mais acessível. Por isso, esse termo se chama *sombreamento*.

Mesmo que uma variável seja mutável (usando `let mut`), a mutabilidade permite apenas mudar o valor, mas o tipo da variável permanece sempre o mesmo até o fim de sua vida. Isso significa que não seria possível alterar o valor de uma variável que armazenava uma string para um número. Com sombreamento, isso passa a ser possível. Como estamos declarando novamente do zero uma variável, podemos atribuir o valor e tipo que quisermos. Isso passa a ser bastante útil quando precisamos trocar tanto o valor como o tipo da variável, sem precisar ficar criando várias variáveis com nomes diferentes como `espacos_str`, `espacos_int`, etc... 

## Exemplo
``` rust
fn main() {
	let espacos = "    ";
	let espacos = espacos.len();
}
```