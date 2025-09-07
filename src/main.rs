use rand::Rng;
use std::io::{self, Write};

fn main() -> io::Result<()> {

    let numero_aleatorio = rand::rng().random_range(1..=100);

    loop {
        print!("Digite um número inteiro entre 1 e 100: ");
        io::stdout().flush()?;

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada)?;
        if let Ok(numero) = entrada.trim().parse::<i32>() {
            if numero == numero_aleatorio {
                println!("Você acertou!");
                return Ok(());
            } else if numero > numero_aleatorio {
                println!("Você errou! Tente um número menor!");
            } else {
                println!("Você errou! Tente um número maior!");
            }
        } else {
            println!("Erro! Isso não é um número inteiro!");
        }
    }
}