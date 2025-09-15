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

mod numeros_por_extensos;

use numeros_por_extensos::NúmerosPorExtenso;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("066")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let números_por_extensos = NúmerosPorExtenso::new();

        let número_inteiro_até_vinte = obter_um_número_inteiro_até_vinte(
            &exercício_informações
        );

        analisar_o_número_inteiro_até_vinte(
            número_inteiro_até_vinte, 
            &números_por_extensos
        );

        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercíco */
    rodar_final_do_exercício();
}

fn analisar_o_número_inteiro_até_vinte(
    número_inteiro_até_vinte: u8,
    números_por_extensos: &NúmerosPorExtenso
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o número...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "O número {} por extenso é \"{}\".\n",
        número_inteiro_até_vinte,
        números_por_extensos.get_número_por_extenso(
            número_inteiro_até_vinte
        ).to_uppercase()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_um_número_inteiro_até_vinte(
    exercício_informações: &ExercícioInformações
) -> u8 {
    loop {
        println!(
            "Digite um número inteiro até 20:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(número) => {
                        if número <= 20 {
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
                                "Erro! Apenas é aceito até 20!\n"
                            );
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite apenas número inteiro!\n"
                        );
                    }
                }
            }
            Err(_) => ()
        }
    }
}