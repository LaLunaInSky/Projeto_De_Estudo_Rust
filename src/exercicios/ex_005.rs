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

mod notas;

use notas::Notas;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = Exercício_Informações::new(
        &cabeçalho_do_programa,
        descrição_de_exercício(
            String::from("005"),
            String::from("Um programa que lê duas notas de um\naluno(a), calcula e mostra a média das\nnotas.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut notas: Vec<f32> = vec![];

        for nota in 1..3 {
            notas.push(
                obter_input_de_nota(
                    nota, 
                    &exercício_informações
                )
            );
        }

        let notas = Notas::new(notas);

        analisar_notas(
            &notas
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

fn analisar_notas(
    notas: &Notas
) {
    println!(
        "Analisando as notas {:.1} e {:.1}...\n", 
        notas.notas[0],
        notas.notas[1]
    );

    sleep(Duration::from_millis(2000));

    println!(
        "A média é {:.1}!\n",
        notas.média_final
    );
}

fn obter_input_de_nota(
    index_da_nota: u8, 
    exercício_informações: &Exercício_Informações
) -> f32 {
    loop {
        println!(
            "Digite {}ª Nota: ", 
            index_da_nota
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(número) => {

                        if número > 0.0 
                            && 
                           número <= 10.0 
                        {
                            let número_formatado = format!("{:.1}", número);

                            match número_formatado.parse::<f32>() {
                                Ok(número_final) => {
                                    limpar_terminal();

                                    exercício_informações.mostrar_informações();

                                    println!(
                                        "A Nota {:.1},\nfoi adicionada com sucesso!\n",
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
                                "Erro! Apenas é aceito 0.0 à 10.0!\n"
                            );
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite apenas números!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}