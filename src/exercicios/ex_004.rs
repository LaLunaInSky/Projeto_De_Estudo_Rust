use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
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
            String::from("004"),
            String::from("Um programa que lê um número inteiro e\nmostra o seu dobro, triplo e a raiz\nquadrada.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let número_do_input = obter_um_número_inteiro(
            &exercício_informações
        );

        analisar_o_número_inteiro(
            número_do_input
        );

        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando para o menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    limpar_terminal();
}

fn obter_um_número_inteiro(
    exercício_informações: &Exercício_Informações
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
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "O número {},\nfoi adicionado com sucesso!",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Entrada digitada não é válida!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn analisar_o_número_inteiro(
    número_inteiro: u32
) {
    sleep(Duration::from_millis(1000));

    println!(
        "\nO Dobro é..........: {}",
        (número_inteiro * 2)
    );

    sleep(Duration::from_millis(1000));

    println!(
        "O triplo é.........: {}",
        (número_inteiro * 3)
    );

    sleep(Duration::from_millis(1000));

    println!(
        "A Raiz Quadrada é..: {}\n",
        número_inteiro.isqrt()
    );
}