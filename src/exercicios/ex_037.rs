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

mod aluno;

use aluno::Aluno;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("037")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut notas_do_aluno = Aluno::new();

        for indice in 1..3 {
            notas_do_aluno.adicionar_uma_nota(
                obter_a_nota(
                    &exercício_informações, 
                    indice
                )
            )
        }

        analisar_as_notas(
            &mut notas_do_aluno
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

fn analisar_as_notas(
    notas_do_aluno: &mut Aluno
) {
    notas_do_aluno.calcular_a_média();

    let notas = notas_do_aluno.get_notas();
    let média = notas_do_aluno.get_média();
    let situação_final = notas_do_aluno.get_situação_final();

    sleep(Duration::from_millis(1000));

    println!(
        "Calculando a média das notas...\n"
    );

    for (index, nota) in notas.iter().enumerate() {
        if index == ((notas.len() - 1) as usize) {
            print!{
                "{:.1}\n\n",
                nota
            };
        } else {
            print!{
                "{:.1}, ",
                nota
            };
        }
        
    }

    sleep(Duration::from_millis(1500));

    println!(
        "A média do aluno é de {:.1}!\nO aluno está {}!\n",
        média,
        if média > 5.0 || média < 7.0 {
            format!(
                "em {}",
                situação_final
            )
        } else {
            format!(
                "{}",
                situação_final
            )
        }
    );

    sleep(Duration::from_millis(1100));
}

fn obter_a_nota(
    exercício_informações: &ExercícioInformações,
    indice_da_chamada: u8
) -> f32 {
    loop {
        println!(
            "Digite a {}ª Nota:",
            indice_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(nota) => {
                        match nota {
                            0.0..=10.0 => {
                                let nota_formatada = format!(
                                    "{:.1}",
                                    nota
                                );

                                match nota_formatada.parse::<f32>() {
                                    Ok(nota_final) => {
                                        limpar_terminal();

                                        exercício_informações.mostrar_informações();

                                        println!(
                                            "A nota {:.1},\nfoi adicionada com sucesso!\n",
                                            nota_final
                                        );

                                        return nota_final
                                    }
                                    Err(_) => (),
                                }
                            }
                            _ => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "Erro! Apenas é aceito notas de 0.0 à 10.0!\n"
                                );
                            }
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