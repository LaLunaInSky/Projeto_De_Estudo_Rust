use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!(
        "Descrição do exercício 045:"
    );
    println!(
        " Um programa que calcula a soma entre todos\nos números ímpares que são múltiplos de\ntrês e que se encontram no intervalo de 1\naté 500."
    );
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    println!(
        "{}", cabeçalho_do_programa
    );

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício */


    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}