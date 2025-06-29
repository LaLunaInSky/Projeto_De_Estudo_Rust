use std::{
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercícios() {
    println!("Descrição do exercício 003:");
    println!(
        " Um programa que lê um número inteiro e mostra na tela o seu sucessor e seu antecessor."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercícios();

    println!();
}