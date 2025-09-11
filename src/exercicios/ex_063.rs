use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::buscar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

mod pessoa;
mod genero;
mod pessoas;

use pessoa::Pessoa;
use genero::Gênero;
use pessoas::Pessoas;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("063")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut pessoas = Pessoas::new();

        loop {
            let contagem = pessoas.get_lista_de_pessoas().len() + 1;

            pessoas.adicionar_uma_nova_pessoa(
                Pessoa::new(
                    obter_a_idade_de_uma_pessoa(
                        &exercício_informações, 
                        contagem as u8
                    ),
                    obter_o_gênero_de_uma_pessoa(
                        &exercício_informações, 
                        contagem as u8
                    )
                )
            );

            let resposta_sobre_adicionar_mais_uma_pessoa = perguntar_se_quer_adicionar_mais_uma_pessoa(
                &exercício_informações
            );

            if !resposta_sobre_adicionar_mais_uma_pessoa {
                break;
            }
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
        "\nAnalisando as pessoas..."
    );

    sleep(Duration::from_millis(1500));

    println!(
        "
Total de maiores de idade: {},
Total de homens digitados: {},
Total de mulheres com menos de 20: {}.
",
        pessoas.get_quantidade_de_pessoas_maiores_de_18_anos(),
        pessoas.get_quantidade_de_homens_cadastrados(),
        pessoas.get_quantidade_de_mulheres_com_menos_de_20_anos()
    );

    sleep(Duration::from_millis(1100));
}

fn perguntar_se_quer_adicionar_mais_uma_pessoa(
    exercício_informações: &ExercícioInformações
) -> bool {
    loop {
        println!(
            "Quer adicionar mais uma pessoa? [S/N]"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let resposta_da_pergunta = input.trim().to_lowercase();

                let resposta_da_pergunta = resposta_da_pergunta.as_str();

                match resposta_da_pergunta {
                    "s" => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        return true;
                    }
                    "n" => return false,
                    _ => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Apenas é aceito S [sim] ou N [não]!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn obter_o_gênero_de_uma_pessoa(
    exercício_informações: &ExercícioInformações,
    indice_da_chamada: u8
) -> Gênero {
    loop {
        println!(
            "Qual o Gênero da {}ª Pessoa [F/M]?",
            indice_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let resposta_da_pergunta = input.trim().to_lowercase();

                let resposta_da_pergunta = resposta_da_pergunta.as_str();

                match resposta_da_pergunta {
                    "f" | "m" => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        if resposta_da_pergunta == "f" {
                            println!(
                                "O gênero Feminino,\nfoi adicionado com sucesso!\n"
                            );

                            return Gênero::FEMININO;
                        } else {
                            println!(
                                "O gênero Masculino,\nfoi adicionado com sucesso!\n",
                            );

                            return Gênero::MASCULINO;
                        }
                    }
                    _ => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Apenas é aceito F ou M!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn obter_a_idade_de_uma_pessoa(
    exercício_informações: &ExercícioInformações,
    indice_da_chamada: u8
) -> u8 {
    loop {
        println!(
            "Qual a idade da {}ª Pessoa?",
            indice_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(idade) => {
                        match idade {
                            0..100 => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "A idade de {} anos,\nfoi adicionada com sucesso!\n",
                                    idade
                                );

                                return idade;
                            }
                            _ => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "Erro! Digite apenas idade até 100 anos!\n"
                                );
                            }
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