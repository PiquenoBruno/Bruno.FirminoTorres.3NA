mod utils;

use std::io;

fn main() {
    let mut input = String::new();
    println!("Digite uma palavra:");
    io::stdin().read_line(&mut input).expect("Falha ao ler a linha");

   
    let input = input.trim();

    
    let reversed = utils::inverter(&input);
    println!("Palavra invertida: {}", reversed);
}
