use std::{
    process::Command,
    thread,
    time::Duration
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 010:");
    println!(
        " Um programa que lê o preço de um produto\ne mostra seu novo preço com 5% de desconto."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do exercício main*/


    /* Fim do Programa */
    // thread::sleep(Duration::from_millis(3000));

    // println!("\nVoltando ao menu de exercícios...\n");

    // thread::sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}