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

mod parede;

use parede::Parede;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = Exercício_Informações::new(
        &cabeçalho_do_programa,
        descrição_de_exercício(
            String::from("009"),
            String::from("Um programa que lê a largura e a altura\n de uma parade em metros, calcula a sua\nárea e a quantidade de tinta necessária\npara pintá-la, sabendo que cada litro de\ntinta pinta uma área de 2m².")
        )
    );

    loop{
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let parede = Parede::new(
            obter_o_tamanho_de_uma_parede_em_metros(
                "largura", 
                &exercício_informações
            ),
            obter_o_tamanho_de_uma_parede_em_metros(
                "altura", 
                &exercício_informações
            )
        );
        
        analisar_a_parede(
            &parede
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

fn analisar_a_parede(
    parede: &Parede
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Calculando a área da parade com:\nAltura: {:.1}m\nLargura: {:.1}m\n",
        parede.altura,
        parede.largura
    );
    
    sleep(Duration::from_millis(1500));
    
    println!(
        "Sua área é de {}m²\n",
        parede.área
    );

    sleep(Duration::from_millis(1500));

    calcular_quantidade_de_tinta_nescessária(
        &parede
    );
}

fn calcular_quantidade_de_tinta_nescessária(
    parede: &Parede
) {
    let um_litro_de_tinta_pinta_x_area = 2.0;

    let quantidade_de_litros_de_tinta_que_precisará = parede.área / um_litro_de_tinta_pinta_x_area;

    println!(
        "Logo precisará de {} litros de tinta!\n",
        quantidade_de_litros_de_tinta_que_precisará
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_tamanho_de_uma_parede_em_metros(
    comprimento_desejado: &str, 
    exercício_informações: &Exercício_Informações
) -> f32 {
    loop {
        println!(
            "Qual o {} da parede? ( em metros )",
            comprimento_desejado.to_uppercase()
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(number) => {
                        if number > 0.0 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            let número_formatado = format!(
                                "{:.1}", number
                            );

                            match número_formatado.parse::<f32>() {
                                Ok(número_final) => {
                                    println!(
                                        "A {} de {}m,\nfoi adicionada com sucesso!\n", 
                                        comprimento_desejado.to_uppercase(),
                                        número_final
                                    );
            
                                    return número_final;
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
            Err(_) => (),
        }
    }
}