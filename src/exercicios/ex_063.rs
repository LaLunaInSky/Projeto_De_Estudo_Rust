use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::criar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

// ex_069!
pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
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
    rodar_final_do_exercício();
}