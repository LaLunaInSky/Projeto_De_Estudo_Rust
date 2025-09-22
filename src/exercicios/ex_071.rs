use std::{
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    descricao_de_exercicio::buscar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

mod palavras_e_suas_vogais;

use palavras_e_suas_vogais::PalavrasESuasVogais;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("071")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut palavras_e_vogais = PalavrasESuasVogais::new();

        let palavras_sugeridas = [
            String::from("aprender"), String::from("programar"), String::from("linguagem"), String::from("python"), String::from("curso"), String::from("gratis"), String::from("estudar"), String::from("praticar"), String::from("trabalhar"), String::from("mercado"), String::from("programador"), String::from("futuro")
        ];

        for palavra_sugerida in palavras_sugeridas {
            palavras_e_vogais.adicionar_uma_nova_palavra(
                palavra_sugerida
            );
        }

        analisar_as_palavras(
            &palavras_e_vogais
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

fn analisar_as_palavras(
    palavras_e_vogais: &PalavrasESuasVogais
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando as palavras...\n"
    );

    sleep(Duration::from_millis(1500));

    for (
        index, 
        palavra
    ) in palavras_e_vogais.get_lista_de_palavras()
                          .iter().enumerate() {
        println!(
            "{} - {}",
            palavra,
            palavras_e_vogais.get_lista_de_vogais_de_cada_palavra()[
                index
            ]
        )
    }

    println!();

    sleep(Duration::from_millis(1100));
}