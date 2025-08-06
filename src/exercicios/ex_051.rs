use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

use chrono::Utc;

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() -> String {
    format!(
        "Descrição do exercício 051:
 Um programa que lê o ano de nascimento\nde sete pessoas. No final, mostra quantas\npessoas ainda não atingiram a maioridade\ne quantas já são maiores.       
"
    )
}

fn obter_o_ano_atual() -> u32 {
    let utc = Utc::now().to_string();
    let utc: Vec<char> = utc.chars().collect();

    let ano_atual = format!(
        "{}{}{}{}",
        utc[0], utc[1],
        utc[2], utc[3]
    );
    let ano_atual: u32 = ano_atual.parse().unwrap();

    return ano_atual;
}

#[derive(Debug)]
struct Pessoa {
    ano_de_nascimento: u32,
    idade: u8,
    é_maior_de_idade: bool,
}

impl Pessoa {
    fn new(
        ano_de_nascimento: u32
    ) -> Self {
        let ano_atual = obter_o_ano_atual();
        let idade = (ano_atual - ano_de_nascimento) as u8;
        let é_maior_de_idade = if idade >= 18 {
            true
        } else {
            false
        };
        
        Self {
            ano_de_nascimento,
            idade,
            é_maior_de_idade
        }
    }
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    loop {
        println!(
            "{}\n{}",
            cabeçalho_do_programa,
            descrição_do_exercício()
        );

        /* Corpo do Exercício */
        let mut pessoas: Vec<Pessoa> = vec![];

        for count in 1..8 {
            pessoas.push(
                Pessoa::new(
                    obter_o_ano_de_nascimento(
                        &cabeçalho_do_programa, count)
                )
            );
        }

        analisar_as_pessoas(
            &pessoas
        );
        
        let resposta_sobre_continuar = perguntar_se_quer_adicionar_novos_anos(
            &cabeçalho_do_programa
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn analisar_as_pessoas(
    pessoas: &Vec<Pessoa>
) {
    let mut quantidade_de_maiores_de_idade = 0;
    let mut quantidade_de_menores_de_idade = 0;
    let mut idades: Vec<u8> = vec![];

    for pessoa in pessoas {
        if pessoa.é_maior_de_idade {
            quantidade_de_maiores_de_idade += 1;
        } else {
            quantidade_de_menores_de_idade += 1;
        }

        idades.push(pessoa.idade);
    }

    sleep(Duration::from_millis(1000));

    println!(
        "Analisando as pessoas...\n"
    );

    sleep(Duration::from_millis(2500));

    println!(
        "As idades digitadas foram:\n{:?}\n\nAo total são,\n{} maiores de idade e\n{} menores de idade.\n",
        idades,
        quantidade_de_maiores_de_idade,
        quantidade_de_menores_de_idade
    );

    sleep(Duration::from_millis(1500));
}

fn obter_o_ano_de_nascimento(
    cabeçalho_do_programa: &String,
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
                let ano_atual = obter_o_ano_atual();

                match input.trim().parse::<u32>() {
                    Ok(ano) => {
                        if ano >= (ano_atual - 100) && ano <= ano_atual  {
                            clean_terminal_linux();

                            println!(
                                "{}\n{}",
                                cabeçalho_do_programa,
                                descrição_do_exercício()
                            );

                            println!(
                                "O ano de nascimento {},\nfoi adicionado com sucesso!\n",
                                ano
                            );

                            return ano;
                        } else {
                            clean_terminal_linux();

                            println!(
                                "{}\n{}",
                                cabeçalho_do_programa,
                                descrição_do_exercício()
                            );

                            println!(
                                "Erro! Apenas anos maiores que {}\ne menores que {}!\n",
                                (ano_atual - 101),
                                (ano_atual)
                            );
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

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

fn perguntar_se_quer_adicionar_novos_anos(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Quer adicionar novos anos? [S/N]"
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
                        clean_terminal_linux();

                        return true;
                    }
                    "n" => return false,
                    _ => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

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