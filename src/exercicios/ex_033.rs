use std::{
    io,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 033:");
    println!(
        " Um programa para aprovar o empréstimo\nbancário para a compra de uma casa.\n Pergunte o valor da casa, o salário do\ncomprador e em quantos anos ele vai pagar.
 A prestação mensal não pode exceder 30%\ndo salário ou então o empréstimo será\nnegado."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    /* Começo do Exercício */
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */


    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando o menu de exercícios...\n"
    // );

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}