use std::{
    io::stdin,
    // thread::sleep,
    // time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::buscar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

// 71!
pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("065")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        mostrar_slogan();

        let valor_a_sacar = obter_um_número_inteiro(
            &exercício_informações
        );

        println!(
            "{}",
            valor_a_sacar
        );

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

fn mostrar_slogan() {
    println!(
        "{:^42}\n",
        " CAIXA ELETRÔNICO "
    );
}

fn obter_um_número_inteiro(
    exercício_informações: &ExercícioInformações
) -> u32 {
    loop {
        println!(
            "Quanto quer sacar?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        mostrar_slogan();

                        println!(
                            "O valor de R${},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        mostrar_slogan();

                        println!(
                            "Erro! Digite apenas números inteiros.\n"
                        );
                    }
                }
            }
            Err(_) => ()
        }
    }
}