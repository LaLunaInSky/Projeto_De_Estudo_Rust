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

fn descrição_do_exercício() {
    println!("Descrição do exercício 038:");
    println!(
        " A Confederação Nacional de Natação precisa\nde um programa que leia o ano de nascimento\nde um atleta e mostra sua categoria, de\nacordo com a idade:
        
- Até 9 anos: MIRIM
- Até 14 anos: INFANTIL
- Até 19 anos: JUNIOR
- Até 25 anos: SÊNIOR
- Acima: MASTER"
    );
}

#[derive(Debug)]
struct Atleta {
    ano_de_nascimento: u32,
    idade: u32,
    categoria: String,
}

impl Atleta {
    fn new(
        ano_de_nascimento: u32, 
        ano_atual: &u32
    ) -> Self {
        let idade = ano_atual - ano_de_nascimento;

        let categoria = if idade > 25 {
            String::from("MASTER")
        } else if idade > 19 {
            String::from("SÊNIOR")
        } else if idade > 14 {
            String::from("JUNIOR")
        } else if idade > 9 {
            String::from("INFANTIL")
        } else {
            String::from("MIRIM")
        };

        Self { 
            ano_de_nascimento,
            idade,
            categoria
        }
    }
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        /* Começo do Exercício */
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício - fn main */
        let ano_atual = obter_o_ano_atual();

        let dados_do_atleta = Atleta::new(
            obter_o_ano_de_nascimento_do_atleta(
                &cabeçalho_do_programa, 
                &ano_atual
            ),
            &ano_atual
        );

        analisar_a_categoria_do_atleta(&dados_do_atleta);

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_um_novo_ano(&cabeçalho_do_programa);

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

fn analisar_a_categoria_do_atleta(dados_do_atleta: &Atleta) {
    sleep(Duration::from_millis(1000));

    println!("Analisando a categoria do Atleta...\n");

    sleep(Duration::from_millis(3000));

    println!(
        "O atleta com {} anos,\nestá na categoria: {}!\n",
        dados_do_atleta.idade,
        dados_do_atleta.categoria
    );

    sleep(Duration::from_millis(1500));
}

fn perguntar_se_quer_adicionar_um_novo_ano(cabeçalho_do_programa: &String) -> bool {
    loop {
        println!("Quer adicionar um novo ano? [S/N]");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
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

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Apenas é aceito S [sim] ou N [não]!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn obter_o_ano_de_nascimento_do_atleta(cabeçalho_do_programa: &String, ano_atual: &u32) -> u32 {
    loop {
        println!("Digite o ano de nascimento do atleta:");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(ano) => {
                        let ano_limite = ano_atual - 100;

                        if ano >= ano_limite  {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!(
                                "\nO ano de {ano},\nfoi adicionado com sucesso!\n"
                            );

                            return ano;
                        } else {
                            clean_terminal_linux();
                            
                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!(
                                "\nErro! Apenas é aceito ano superior que {ano_limite}!\n"
                            );
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nErro! Digite apenas número!\n"
                        );
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
   }
}

fn obter_o_ano_atual() -> u32 {
    let utc = Utc::now().to_string();

    let mut data_formata_com_os_4_primeiros_indices: Vec<char> = vec![];

    for (index, char) in utc.chars().enumerate() {
        if index <= 3 {
            data_formata_com_os_4_primeiros_indices.push(char);
        }
    }

    let ano_atual = format!(
        "{}{}{}{}",
        data_formata_com_os_4_primeiros_indices[0],
        data_formata_com_os_4_primeiros_indices[1],
        data_formata_com_os_4_primeiros_indices[2],
        data_formata_com_os_4_primeiros_indices[3]
    );

    let ano_atual: u32 = ano_atual.parse().unwrap();

    ano_atual
}