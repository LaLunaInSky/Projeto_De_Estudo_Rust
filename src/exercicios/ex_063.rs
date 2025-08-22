use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::descrição_de_exercício,
    exercicio_informacoes::Exercício_Informações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício
};

use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
};

// ex_069!
pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = Exercício_Informações::new(
        &cabeçalho_do_programa,
        descrição_de_exercício(
            String::from("063"),
            String::from("Um programa que lê a idade e o gênero de\nvárias pessoas. A cada pessoa cadastrada,\no programa deverá perguntar se o usuário\nquer ou não adicionar mais uma pessoa.\nNo final mostra:

1º - Quantas pessoas tem mais de 18 anos.
2º - Quantos homens foram cadastrados.
3º - Quantas mulheres tem menos 20 anos.")      
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    limpar_terminal();
}