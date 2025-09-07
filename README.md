# Advinhar Número em Rust

Este é um programa simples em Rust que gera um número aleatório e pede para o usuário tentar adivinhar.

## Como executar

1. Instale o Rust (caso não tenha):  
   https://www.rust-lang.org/tools/install

2. Clone o repositório:
    ```bash
    git clone git@github.com:dcoelhosantos/Advinhar-numero-rust.git
    cd Advinhar-numero-rust
    ```
   
3. Execute o programa:
    ```bash
    cargo run
    ```
4. Digite um número entre 1 e 100. O programa vai dizer se seu número é
  - O número certo (igual ao gerado aleatoriamente pelo programa);
  - Menor que o certo;
  - Maior que o certo.
    
5. Continue tentando até advinhar o número.
  

O código principal está em src/main.rs.
