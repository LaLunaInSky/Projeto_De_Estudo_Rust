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

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("047")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut números_inteiros_digitados: Vec<u32> = vec![];

        for quantidade in 1..7 {
            números_inteiros_digitados.push(
                obter_um_número_inteiro(
                    &exercício_informações, 
                    quantidade
                )
            );
        }

        analisar_os_números(
            &números_inteiros_digitados
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

fn analisar_os_números(
    números: &Vec<u32>
) {
    let mut números_pares: Vec<u32> = vec![];
    let mut números_ímpares: Vec<u32> = vec![];
    let mut soma_dos_números_pares: u32 = 0;

    for número in números {
        if número % 2 == 0 {
            números_pares.push(*número);

            soma_dos_números_pares += número;
        } else {
            números_ímpares.push(*número);
        }
    }

    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os números...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "Os números pares: {:?} \nOs números ímpares: {:?}\n",
        números_pares, números_ímpares
    );

    sleep(Duration::from_millis(800));

    println!(
        "A soma dos números pares é: {}.\n",
        soma_dos_números_pares
    );

    sleep(Duration::from_millis(1100));
}

fn obter_um_número_inteiro(
    exercício_informações: &ExercícioInformações,
    index_da_chamada: u8
) -> u32 {
    loop {
        println!(
            "Digite o {}º Número:",
            index_da_chamada
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
                            "O número {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite apenas números.\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}