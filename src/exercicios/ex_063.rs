use crate::recursos::limpar_terminal::limpar_terminal;

use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
};

// ex_069!
fn descrição_do_exercício() -> String {
    format!(
        "Descrição do exercício 063:
 Um programa que lê a idade e o gênero de\nvárias pessoas. A cada pessoa cadastrada,\no programa deverá perguntar se o usuário\nquer ou não adicionar mais uma pessoa.\nNo final mostra:

1º - Quantas pessoas tem mais de 18 anos.
2º - Quantos homens foram cadastrados.
3º - Quantas mulheres tem menos 20 anos.
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
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    limpar_terminal();
}