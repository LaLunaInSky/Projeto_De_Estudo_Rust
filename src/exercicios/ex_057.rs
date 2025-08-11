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
        "Descrição do exercício 057:
 Melhore o EX_048, perguntando para o\nusuário se ele quer mostrar mais alguns\ntermos. O programa encerra quando ele\ndisser que quer mostrar 0 termos.
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