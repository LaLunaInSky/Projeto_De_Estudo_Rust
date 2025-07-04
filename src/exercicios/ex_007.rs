use std::{
    io,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 007:");
    println!(
        " Um programa que lê um número inteiro e\nmostra no terminal a sua tabuada."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    obter_o_número_inteiro();
}

fn obter_o_número_inteiro() {
    println!("Digite um número inteiro:");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("Input: {}", input),
        Err(_) => println!("Erro!"),
    }
}