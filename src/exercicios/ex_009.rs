use std::{
    process::Command,
    thread,
    time::Duration
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 009:");
    println!(
        " Um programa que lê a largura e a altura\n de uma parade em metros, calcula a sua\nárea e a quantidade de tinta necessária\npara pintá-la, sabendo que cada litro de\ntinta pinta uma área de 2m²."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();
    
    println!();

    /* Corpo do programa main */


    /* Fim do Exercício */
    // thread::sleep(Duration::from_millis(3000));

    // println!("\nVoltando para o menu de exercícios...\n");

    // thread::sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}