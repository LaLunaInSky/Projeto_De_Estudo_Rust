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
            String::from("007"),
            String::from("Um programa que lê um número inteiro e\nmostra no terminal a sua tabuada.")
        )
    );
    
    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let número_digitado = obter_o_número_inteiro(
            &exercício_informações
        );

        obter_a_tabuado_do_número_inteiro_informado(
            &número_digitado
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

fn obter_a_tabuado_do_número_inteiro_informado(
    número_inteiro: &u32
) {
    sleep(Duration::from_millis(1000));

    println!("A Tabuada do {} é...\n", número_inteiro);

    sleep(Duration::from_millis(1500));

    for número in 1..11 {
        println!(
            "{} x {:>2} = {}",
            número_inteiro,
            número,
            (número_inteiro * número)
        );
    }

    println!();

    sleep(Duration::from_millis(1100));
}

fn obter_o_número_inteiro(
    exercício_informações: &ExercícioInformações
) -> u32 {
    loop{
        println!(
            "Digite um número inteiro:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        if número > 0 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "O número {},\nfoi adicionado com sucesso!\n",
                                número
                            );

                            return número;
                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Digite um número maior que zero!\n"
                            );
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