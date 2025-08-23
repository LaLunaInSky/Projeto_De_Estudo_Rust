use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::descrição_de_exercício,
    exercicio_informacoes::Exercício_Informações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício
};

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = Exercício_Informações::new(
        &cabeçalho_do_programa,
        descrição_de_exercício(
            String::from("008"), 
            String::from("Um programa que lê quanto dinheiro uma\npessoa tem na carteira e mostra quantos\nDólares ela pode comprar.

Considere US$1,00 = R$3,27")
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
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando para o menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    limpar_terminal();
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
    exercício_informações: &Exercício_Informações
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