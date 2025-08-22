use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    descricao_de_exercicio::descrição_de_exercício, exercicio_informacoes::{self, Exercício_Informações}, limpar_terminal::limpar_terminal, perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício
};

mod numeros;

use numeros::Números;

// fn descrição_do_exercícios() -> String {
//     format!(
//         "Descrição do exercício 001:
//  Um programa que lê dois números inteiro e\nmostra a soma entre os mesmos.
// "
//     )
// }

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = Exercício_Informações::new(
        &cabeçalho_do_programa,
        descrição_de_exercício(
            String::from("001"),
            String::from("Um programa que lê dois números inteiro e\nmostra a soma entre os mesmos.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut números_digitados: Vec<u32> = vec![];

        for indice in 1..3 {
            números_digitados.push(
                obter_a_entrada_de_um_número_inteiro(
                    &exercício_informações,
                    indice
                )
            );
        }

        let números = Números::new(
            números_digitados
        );

        println!(
            "A soma dos números {} + {} é igual a {}!\n",
            números.números[0],
            números.números[1],
            números.soma_dos_números
        );

        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    sleep(Duration::from_millis(2000));

    println!(
        "\nVoltando para o menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    limpar_terminal();
}

fn obter_a_entrada_de_um_número_inteiro(
    exercício_informacoes: &Exercício_Informações,
    indice_da_chamada_do_input: u8
) -> u32 {
    loop {
        println!(
            "Digite o {indice_da_chamada_do_input}º número inteiro: "
        );

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        limpar_terminal();

                        exercício_informacoes.mostrar_informações();

                        println!(
                            "O número {},\nfoi adicionado com sucesso!\n",
                            número
                        );
                        
                        return número;
                    }
                    Err(_) => {
                       limpar_terminal();

                       exercício_informacoes.mostrar_informações();

                        println!(
                            "Erro! Digite novamente um número válido!\n"
                        )
                    }
                }
            },
            Err(_) => (),
        }

        
    }
}