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
            String::from("021")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let nome_da_cidade = obter_o_nome_de_uma_cidade(
            &exercício_informações
        );

        analisar_o_nome_da_cidade(
            &nome_da_cidade
        );

        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim de Exercício */
    rodar_final_do_exercício();
}

fn analisar_o_nome_da_cidade(
    nome_da_cidade: &String
) {
    let nome_da_cidade = nome_da_cidade.to_lowercase();

    let nome_da_cidade_separado: Vec<&str> = nome_da_cidade.split_whitespace().collect();

    let mut resposta_da_analise = String::from("não ");

    sleep(Duration::from_millis(1000));

    println!(
        "Analisando a cidade...\n"
    );

    sleep(Duration::from_millis(1500));


    if nome_da_cidade_separado[0].contains("santo") {
        resposta_da_analise = String::from("")
    }

    println!(
        "A cidade {},\n{}começa com \"SANTO\"!\n",
        nome_da_cidade.to_uppercase(),
        resposta_da_analise
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_nome_de_uma_cidade(
    exercício_informações: &ExercícioInformações
) -> String {
    loop {
        println!(
            "Digite o nome de um cidade:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                limpar_terminal();

                exercício_informações.mostrar_informações();

                let nome_da_cidade = input.trim().to_lowercase();

                println!(
                    "A cidade {},\nfoi adicionada com sucesso!\n",
                    nome_da_cidade
                );

                return nome_da_cidade;
            }
            Err(_) => (),
        }
    }
}