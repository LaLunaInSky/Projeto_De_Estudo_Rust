use std::{
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
}