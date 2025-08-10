use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() -> String {
    format!(
        "Descrição do exercício 055:
 Crie um programa que leia dois números\ninteiros e mostre um menu como o do\nexemplo abaixo:
    [ 1 ] Somar
    [ 2 ] Multiplicar
    [ 3 ] Qual é o Maior
    [ 4 ] Informar Novos Números
    [ 5 ] Fechar o programa
Seu programa deverá realizar a operação\nsolicitada em cada caso.
"
    )
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    println!(
        "{}\n{}",
        cabeçalho_do_programa,
        descrição_do_exercício()
    );

    /* Corpo do Exercício */


    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}