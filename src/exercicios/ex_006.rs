use std::{
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 006:");
    println!(
        " Um programa que lê um valor em metros e o\nexibe convertido em todos os tipos a\nseguir:

km <- hm <- dam <- m -> dm -> cm -> mm"
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();
}