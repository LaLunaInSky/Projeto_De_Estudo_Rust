use std::{
    io,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 008:");
    println!(
        " Um programa que lê quanto dinheiro uma\npessoa tem na carteira e mostra quantos\nDólares ela pode comprar.

Considere US$1,00 = R$3,27"
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    obter_a_quantia_de_dinheiro_na_carteira();
}

fn obter_a_quantia_de_dinheiro_na_carteira() {
    println!("Quanto de dinheiro você tem na carteira?");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<f32>() {
                Ok(number) => {
                    if number > 0.0 {
                        println!("Number: R${:.2}", number);
                    } else {
                        println!("Erro! Valor inválido!");
                    }
                }
                Err(_) => println!("Erro!"),
            }
        }
        Err(_) => println!("Erro!"),
    } 
}