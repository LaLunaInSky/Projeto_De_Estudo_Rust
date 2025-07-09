use std::{
    io,
    thread,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear");
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 018:");
    println!(
        " Contexto: 
 O mesmo professor do desafio anterior\n(ex_017) quer sortear a ordem da\n apresentação dos trabalhos dos alunos.

Exercício:
 Um programa que lê o nome de quatro alunos\ne mostre em ordem sorteada os nomes para a\napresentação."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */


    /* Fim do Exercício */
    // thread::sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // thread::sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}