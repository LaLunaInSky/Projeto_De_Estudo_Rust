use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::criar_descrição_do_exercício,
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
        criar_descrição_do_exercício(
            String::from("026"),
            String::from("Um programa que lê a velocidade de\num carro, se o mesmo ultrapassar 80Km/h,\nmostre uma mensagem dizendo que ele foi\nmultado.
 A multa vai custar R$7,00 por cada\nquilometro acima do limite.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let velocidade_do_carro = obter_a_velocidade_do_carro(
            &exercício_informações
        );

        verificar_se_a_velocidade_passou_do_limite(
            &velocidade_do_carro
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

fn calcular_o_valor_da_multa(
    velocidade_do_carro: &f32
) {
    let valor_da_multa = (velocidade_do_carro - 80.0) * 7.0;

    sleep(Duration::from_millis(1000));

    println!(
        "Calculando o valor da multa...\n"
    );
        
    sleep(Duration::from_millis(1500));

    println!(
        "O valor da multa é de R${:.2}\n",
        valor_da_multa
    );
}

fn verificar_se_a_velocidade_passou_do_limite(
    velocidade_do_carro: &f32
) {
    sleep(Duration::from_millis(1000));

    if *velocidade_do_carro > 80.0 {
        println!(
            "O Carro foi multado!\n"
        );

        calcular_o_valor_da_multa(&velocidade_do_carro);

    } else {
        println!(
            "Não se esqueca do sinto de segurança!\n"
        );
    }

    sleep(Duration::from_millis(1100));
}

fn obter_a_velocidade_do_carro(
    exercício_informações: &ExercícioInformações
) -> f32 {
    loop {
        println!(
            "Digite a velocidade do carro:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(velocidade) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        let velocidade_formatada = format!(
                            "{:.1}",
                            velocidade
                        );

                        match velocidade_formatada.parse::<f32>() {
                            Ok(velocidade_final) => {
                                println!(
                                    "Velocidade de {:.1}km/h,\nfoi adiconada com sucesso!\n",
                                    velocidade_final
                                );

                                return velocidade_final;
                            }
                            Err(_) => (),
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite um número válido!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}