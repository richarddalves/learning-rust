# usize e isize
Criado em: 04/05/2025 

## Resumo
Os tipos primitivos `usize` e `isize` têm tamanhos que variam dependendo da arquitetura do processador em que o código é compilado e executado.

## Detalhamento
- Em arquiteturas **32 bits**:
    - `usize` possui 32 bits.
    - `isize` possui 32 bits.
- Em arquiteturas **64 bits**:
    - `usize` possui 64 bits.
    - `isize` possui 64 bits.

## Exemplo
```rust
let days: usize = 55; // Em um computador de 32 bits, isso será um u32. Em um computador de 64 bits, será um u64.
let years: isize = -15_000; // A mesma coisa se aplica, mas com i32 e i64.
```