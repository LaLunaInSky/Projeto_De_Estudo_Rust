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
    println!("Descrição do exercício 043:");
    println!(
        " Um programa que mostra no terminal uma\ncontagem regressiva para o estoura de\nfogos de artifícios, indo de 10 até 0,\ncom uma pausa de 1 segundo entre eles."
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