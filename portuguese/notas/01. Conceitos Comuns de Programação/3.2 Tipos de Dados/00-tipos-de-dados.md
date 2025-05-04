# Tipos de Dados
- Todo valor em Rust tem um **tipo de dado**.
- Rust é uma linguagem tipada estaticamente, o que significa que o compilador precisa saber os tipos de todas as variáveis em tempo de compilação.
- O compilador pode *inferir* os tipos das variáveis se baseando nos seus valores iniciais. 

## Tipos Escalares
- Um tipo escalar é um tipo que guarda um único valor
- Rust tem 4 tipos escalares: inteiros, números de ponto flutuante, booleanos e caracteres.
- Um número do tipo inteiro é um número sem parte decimal e um número de ponto flutuante é um número decimal.

### Inteiros com e sem sinal Signed and Unsigned Integers
- Inteiros com sinal suportam valores positivos e negativos. Eles começam com `i` (de *integer*). Aqui, um bit é reservado para o sinal do número.
- Inteiros sem sinal apenas suportam 0 e valores positivos. Eles começam com `u` (de *unsigned*).

#### Bits
- O número depois de `i` ou `u` *(ex.: `i8`, `i32`, `u16`, etc...)* se refere à quantidade de bits da memória do computador que o número inteiro precisa.
- Um **bit** é a menor unidade possível a ser usada na memória. Ele representa um valor de 0 ou 1.
- Equivalências
    - 8 bits equivalem a 1 byte
    - 1024 bytes equivalem a 1 *kibibyte* (KiB)
    - 1024 kilobytes equivalem a 1 *mebibyte* (MiB)
    - 1024 megabytes equivalem a 1 *gibibyte* (GiB)
- Um `i32` precisa de 32 bits (4 bytes) de memória. Um `f64` ocupa 64 bits (8 bytes).

