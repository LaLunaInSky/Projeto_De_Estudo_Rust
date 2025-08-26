use std::{
    io::stdin
};

use crate::recursos::{
    descricao_de_exercicio::criar_descrição_do_exercício, exercicio_informacoes::ExercícioInformações, limpar_terminal::limpar_terminal, perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

mod numeros;

use numeros::Números;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
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
    rodar_final_do_exercício();
}

fn obter_a_entrada_de_um_número_inteiro(
    exercício_informacoes: &ExercícioInformações,
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