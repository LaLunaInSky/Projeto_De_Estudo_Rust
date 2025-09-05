use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::criar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

mod ano_atual;
mod pessoa;
mod pessoas;

use ano_atual::AnoAtual;
use pessoa::Pessoa;
use pessoas::Pessoas;


pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("051"),
            String::from("Um programa que lê o ano de nascimento\nde sete pessoas. No final, mostra quantas\npessoas ainda não atingiram a maioridade\ne quantas já são maiores.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut pessoas = Pessoas::new();

        for count in 1..8 {
            pessoas.adicionar_uma_nova_pessoa(
                Pessoa::new(
                    obter_o_ano_de_nascimento(
                        &exercício_informações, 
                        count
                    )
                )
            );
        }

        analisar_as_pessoas(
            &pessoas
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

fn analisar_as_pessoas(
    pessoas: &Pessoas
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando as pessoas...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "As idades digitadas foram:\n{:?}\n\nAo total são,\n{} maiores de idade e\n{} menores de idade.\n",
        pessoas.get_todas_as_idades(),
        pessoas.get_quantidade_de_maiores_de_idade(),
        pessoas.get_quantidade_de_menores_de_idade()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_ano_de_nascimento(
    exercício_informações: &ExercícioInformações,
    index_da_chamada: u8
) -> u32 {
    loop {
        println!(
            "Digite o ano de nascimento da {}ª Pessoa:",
            index_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let ano_atual = AnoAtual::new();
                let ano_atual = ano_atual.get_ano() as u32;

                match input.trim().parse::<u32>() {
                    Ok(ano) => {
                        if ano >= (ano_atual - 100) && ano <= ano_atual  {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "O ano de nascimento {},\nfoi adicionado com sucesso!\n",
                                ano
                            );

                            return ano;
                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Apenas anos maiores que {}\ne menores que {}!\n",
                                (ano_atual - 101),
                                (ano_atual)
                            );
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