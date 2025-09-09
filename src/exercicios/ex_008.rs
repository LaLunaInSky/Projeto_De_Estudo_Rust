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
            String::from("008")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let quantia_na_carteira = obter_a_quantia_de_dinheiro_na_carteira(
            &exercício_informações
        );

        converter_valor_em_real_para_dolar(
            &quantia_na_carteira
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

fn converter_valor_em_real_para_dolar(
    valor_em_real: &f32
) {
    let cotação_do_dolar = 3.27;

    let valor_em_dolar = valor_em_real/ cotação_do_dolar;

    sleep(Duration::from_millis(1200));

    println!(
        "Na cotação atual você poderá comprar\nU${:.2}.\n",
        valor_em_dolar
    );

    sleep(Duration::from_millis(1500));
}

fn obter_a_quantia_de_dinheiro_na_carteira(
    exercício_informações: &ExercícioInformações
) -> f32 {
    loop{
        println!(
            "Quanto de dinheiro você tem na carteira?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(número) => {
                        if número > 0.0 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            let número_formatado = format!(
                                "{:.2}",  número
                            );

                            match número_formatado.parse::<f32>() {
                                Ok(número_final) => {
                                    println!(
                                        "O quantia de R${:.2},\nfoi adicionada com sucesso!\n",
                                        número_final
                                    );

                                    return número_final;
                                },
                                Err(_) => (),
                            }

                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Digite um valor maior que zero!\n"
                            );
                        }
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