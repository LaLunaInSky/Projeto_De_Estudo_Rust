use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::descrição_de_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício
};

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        descrição_de_exercício(
            String::from("013"),
            String::from("Um programa que pergunta a quantidade de\nKm percorridos por um carro alugado e a\nquantidade de dias pelos quais ele foi\nalugado. 
 O programa irá calcular o preço à ser\npago, sabendo que o carro custa R$60.00\npor dia e R$0.15 por Km rodado.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let distância_percorrida = obter_a_quantidade_de_kms_percorridos(
            &exercício_informações
        );

        let quantidade_de_dias = obter_a_quantidade_de_dias_permanecidos(
            &exercício_informações
        ) as f32;

        calcular_o_valor_a_ser_pago(
            &distância_percorrida, 
            &quantidade_de_dias
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
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    limpar_terminal(); 
}

fn calcular_o_valor_a_ser_pago(
    distânica: &f32,
    dias: &f32
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Calculando o preço final a ser pago...\n"
    );

    sleep(Duration::from_millis(1500));

    let total_a_pagar: f32 = (60.0 * dias) + (0.15 * *distânica);

    println!(
        "O total fica em R${:.2}.\n",
        total_a_pagar
    );

    sleep(Duration::from_millis(1100));
}

fn obter_a_quantidade_de_dias_permanecidos(
    exercício_informaçoes: &ExercícioInformações
) -> u8 {
    loop {
        println!(
            "Quantos dias você ficou com o carro?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(dias) => {
                        limpar_terminal();

                        exercício_informaçoes.mostrar_informações();

                        println!(
                            "Total de {} dias,\nAdicionado com sucesso!\n",
                            dias
                        );

                        return dias;
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informaçoes.mostrar_informações();

                        println!(
                            "Erro! Digite um número inteiro!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn obter_a_quantidade_de_kms_percorridos(
    exercício_informações: &ExercícioInformações
) -> f32 {
    loop {
        println!(
            "Quantos quilômetros(Km) você percorreu\ncom o carro?"
        );
    
        let mut input = String::new();
    
        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(distância) => {
                        if distância > 0.0 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            let distância_formatada = format!(
                                "{:.2}",
                                distância
                            );

                            match distância_formatada.parse::<f32>() {
                                Ok(distância_final) => {
                                    println!(
                                        "Distância de {:.2}Km\nfoi adicionada com sucesso!\n",
                                        distância_final
                                    );

                                    return distância_final
                                }
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
                            "Erro! Digite um número real!\n"
                        );
                    }
                }
            }
            Err(_) => ()
        }
    }
}