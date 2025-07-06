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
    println!("Descrição do exercício 012:");
    println!(
        " Um programa que converte uma temperatura\nem °C para °F."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do exercício main */


    /* Fim do Exercício */
    // thread::sleep(Duration::from_millis(3000));

    // println!("\nVoltando ao menu de exercícios...\n");

    // thread::sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}