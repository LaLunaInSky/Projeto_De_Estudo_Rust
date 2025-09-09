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

mod pesos;

use pesos::Pesos;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("052")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut pesos = Pesos::new();

        for contagem in 1..6 {
            pesos.adicionar_novo_peso(
                obter_o_peso(
                    &exercício_informações,
                    contagem
                )
            );
        }

        analisar_os_pesos(
            &pesos
        );

        let resposta_sobre_continuar =  perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn analisar_os_pesos(
    pesos: &Pesos 
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os pesos...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "Os pesos:\n{:?}\n\nO menor peso é: {:.2}Kg\nO maior peso é: {:.2}Kg\n",
        pesos.get_pesos(),
        pesos.get_menor_peso(),
        pesos.get_maior_peso()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_peso(
    exercício_informações: &ExercícioInformações,
    indice_da_chamada: u8
) -> f32 {
    loop {
        println!(
            "Digite o {indice_da_chamada}º peso:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(peso) => {
                        let formatação_do_peso = format!(
                            "{:.2}",
                            peso
                        );

                        match formatação_do_peso.parse::<f32>() {
                            Ok(peso_final) => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "O peso de {:.2},\nfoi adicionado com sucesso!\n",
                                    peso_final
                                );

                                return peso_final;
                            }
                            Err(_) => (),
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