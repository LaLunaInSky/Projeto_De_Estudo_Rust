use std::{
    io,
    thread,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 015:");
    println!(
        " Um programa que lê o comprimento do\ncateto oposto e do cateto adjacente de um\ntriângulo retângulo, e depois calcula o\ncomprimento da hipotenusa."
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