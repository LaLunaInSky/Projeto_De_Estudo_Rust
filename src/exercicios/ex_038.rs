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

struct Atleta {
    ano_de_nascimento: u32,
    idade: u32
}

impl Atleta {
    fn new(
        ano_de_nascimento: u32, 
        ano_atual: u32
    ) -> Self {
        Self { 
            ano_de_nascimento,
            idade: ano_atual - ano_de_nascimento
        }
    }
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    /* Começo do Exercício */
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let ano_atual = obter_o_ano_atual();

    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}

fn obter_o_ano_de_nascimento_do_atleta(cabeçalho_do_programa: &String, ano_atual: &u32) -> u32 {
    loop {
        println!("Digite o ano de nascimento do atleta:");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(ano) => {
                        if (ano_atual - 100) >  {

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