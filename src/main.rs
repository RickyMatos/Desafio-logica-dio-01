// Desafio de projeto 01
// Dev: Rychards Matos
// Data: 24/08/2024
// Linguagem: Rust

use std::io;

fn main() {
    //Variáveis
    println!("Digite o nome do Heroi:");
    let mut nome_heroi = String::new();
    io::stdin()
        .read_line(&mut nome_heroi)
        .expect("Falhar ao ler a linha");
    let mut input = String::new();
    println!("Digite o xp do Heroi:");
    io::stdin()
        .read_line(&mut input)
        .expect("Falhar ao ler a linha");
    let xp: u16 = input.trim().parse().expect("Falha ao converte string");

    //Operadores
    let class_range: [(&str, u16); 8] = [
        ("Ferro", 0),         // XP ate 1000
        ("Bronze", 1001),     // XP ate 2000
        ("Prata", 2001),      // XP ate 5000
        ("Ouro", 5001),       // XP ate 7000
        ("Platina", 7001),    // XP ate 8000
        ("Ascendente", 8001), // XP ate 9000
        ("Imortal", 9001),    // XP ate 10000
        ("Radiante", 10001),  // XP ate 60000
    ];

    //Estrutura de decisão
    let mut classe = "Desconhecida";
    for (class_name, min_xp) in class_range.iter() {
        if xp >= *min_xp {
            classe = class_name;
        } else {
            break;
        }
    }

    //Saida
    println!(
        "O herói {} tem {} XP e está na classe {}.",
        nome_heroi, xp, classe
    );
}
