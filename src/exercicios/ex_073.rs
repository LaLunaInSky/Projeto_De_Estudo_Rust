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

mod numeros_em_ordem_crescente;

use numeros_em_ordem_crescente::NúmeroEmOrdemCrescente;

// 80!
pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("073")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut números_digitados = NúmeroEmOrdemCrescente::new();

        for quantidade in 1..6 {
            números_digitados.adicionar_um_novo_número(
                obter_um_número_inteiro(
                    &exercício_informações,
                    quantidade
                )
            );
        }

        analisar_os_números_digitados(
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

fn analisar_os_números_digitados(
    números_digitados: &NúmeroEmOrdemCrescente
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os números digitados...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "Os número em ordem crescente:\n{:?}\n",
        números_digitados.get_lista_de_números_em_ordem_crescente()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_um_número_inteiro(
    exercício_informações: &ExercícioInformações,
    index_da_chamada: usize
) -> u32 {
    loop {
        println!(
            "Digite o {}º Número: ",
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
                            "Erro! Digite apenas números\n"
                        );
                    }
                }
            }   
            Err(_) => ()
        }
    }
}