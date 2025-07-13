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
    println!("Descrição do exercício 026:");
    println!(
        " Um programa que lê a velocidade de\num carro, se o mesmo ultrapassar 80Km/h,\nmostre uma mensagem dizendo que ele foi\nmultado.
A multa vai custar R$7,00 por cada\nquilometro acima do limite."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
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