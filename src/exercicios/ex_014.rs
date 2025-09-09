use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::buscar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("014")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo de Exercício */
        let número_real_digitado = obter_um_número_real(
            &exercício_informações
        );

        analisar_o_número(
            &número_real_digitado
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

fn analisar_o_número(
    número_real: &f32
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o número real...\n"
    );

    sleep(Duration::from_millis(1500));

    let número_inteiro = *número_real as u32;

    println!(
        "O número inteiro é {:.0}.\n",
        número_inteiro 
    );

    sleep(Duration::from_millis(1100));
}

fn obter_um_número_real(
    exercício_informações: &ExercícioInformações
) -> f32 {
    loop {
        println!(
            "Digite um número real:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(número) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Número {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite um valor válido!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}