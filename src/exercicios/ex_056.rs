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

mod fatorial;

use fatorial::Fatorial;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("056"),
            String::from("Um programa que lê um número inteiro e\nmostre o seu fatorial.

 Exemplo:
5! = 5 x 4 x 3 x 2 x 1 = 120")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let fatorial = Fatorial::new(
            obter_o_número_inteiro(
                &exercício_informações
            )
        );

        analisar_o_número(
            &fatorial
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
    fatorial: &Fatorial
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o fatorial...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "{}\n",
        fatorial.get_fatorial()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_número_inteiro(
    exercício_informações: &ExercícioInformações
) -> u32 {
    loop {
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
                        if número >= 1 {
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
                            "Erro! Digite apenas números!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}