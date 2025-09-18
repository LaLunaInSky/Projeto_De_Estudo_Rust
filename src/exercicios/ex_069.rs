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

mod numeros_digitados;

use numeros_digitados::NúmeroDigitados;

// 75!
pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("069")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut números_digitados = NúmeroDigitados::new();

        for quantidade in 0..4 {
            números_digitados.adicionar_um_número_a_lista(
                obter_um_número_inteiro(
                    &exercício_informações, 
                    quantidade + 1
                ),
                quantidade
            );
        }

        analisar_os_números(
            &números_digitados
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

fn obter_um_número_inteiro(
    exercício_informações: &ExercícioInformações,
    index_da_chamada: usize
) -> u32 {
    loop {
        println!(
            "Qual o {}º Número:",
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
                            "Erro! Digite apenas números!\n"
                        );
                    }
                }
            }
            Err(_) => ()
        }
    }
}

fn analisar_os_números(
    números_digitados: &NúmeroDigitados
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os números..."
    );

    sleep(Duration::from_millis(1500));

    println!(
        "
Os números digitados foram:
{:?}

O 9 parece {} vez{};
O número 3 aparece {};
Os números pares digitados são:
{:?}.
",
        números_digitados.get_lista_de_números_digitados(),
        números_digitados.get_quantidade_de_vezes_que_o_nove_apareceu(),
        if números_digitados.get_quantidade_de_vezes_que_o_nove_apareceu() != 1 {
            "es"
        } else {
            ""
        },
        números_digitados.get_posição_que_o_valor_3_apareceu_pela_primeira_vez(),
        números_digitados.get_lista_de_números_pares_digitados()

    );

    sleep(Duration::from_millis(1100));
}