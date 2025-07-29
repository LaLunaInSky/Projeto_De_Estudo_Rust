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
        "Descrição do exercício 044:"
    );
    println!(
        " Um programa que mostra no terminal todos\nos números pares que estão no intervalo\nentre 1 e 50."
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
    let números_pares: Vec<u8> = obter_o_números_pares_até_x();

    println!(
        "Os números pares são:"
    );

    mostrar_resultado(
        &números_pares
    );

    /* Fim do Exercício */
    sleep(Duration::from_millis(6000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn mostrar_resultado(
    números: &Vec<u8>
) {
    for (index, número) in números.iter().enumerate() {
        if index % 11 == 0 && index > 0 {
            print!("{número}, \n");
        } else if index == (números.len() - 1) {
            println!("{número}.")
        } else {
            print!("{número}, ");
        }
    }
}

fn obter_o_números_pares_até_x() -> Vec<u8> {
    let mut números_pares: Vec<u8> = vec![];

    for número in 1..51 {
        if número % 2 == 0 {
            números_pares.push(número);
        }
    }

    return números_pares;
}