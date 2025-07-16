use std:: {
    io,
    thread::sleep,
    time::Duration,
    process::Command
};

use chrono::Utc;

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 029:");
    println!(
        " Um programa que lê um ano qualquer e\nmostra no terminal se ele é BISSEXTO."
    );
}

fn obter_o_ano_atual() -> u16 {
    let utc = Utc::now().to_string();
    let mut ano_separado: Vec<char> = vec![];

    for (index, char) in utc.chars().enumerate() {
        if index <= 3 {
            ano_separado.push(char);
        }
    }

    let ano_atual = format!(
        "{}{}{}{}",
        ano_separado[0],
        ano_separado[1],
        ano_separado[2],
        ano_separado[3]
    );

    let ano_atual: u16 = ano_atual.parse().unwrap();

    return ano_atual;
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        /* Começo do Exercício */
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício - fn main */
        let ano_escolhido = obter_um_ano(&cabeçalho_do_programa);

        analisar_se_o_ano_é_bissexto(&ano_escolhido);

        let resposta_da_pergunta_sobre_continuar = perguntar_se_quer_digitar_outro_ano(&cabeçalho_do_programa);

        if resposta_da_pergunta_sobre_continuar == false {
            break;
        }
    }

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!("\nVoltando ao menu de exercícios...\n");

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn perguntar_se_quer_digitar_outro_ano(cabeçalho_do_programa: &String) -> bool {
    loop {
        println!("Quer digitar outro ano? [S/N]");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let resposta_da_pergunta = input.trim().to_lowercase();

                if resposta_da_pergunta == "s" || resposta_da_pergunta == "n" {
                    if resposta_da_pergunta == "s" {
                        clean_terminal_linux();

                        return true;
                    } else {
                        return false;
                    }
                } else {
                    clean_terminal_linux();

                    println!("{}", cabeçalho_do_programa);

                    descrição_do_exercício();

                    println!("\nErro! Digite apenas S [sim] ou N [não]!\n");
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn analisar_se_o_ano_é_bissexto(ano_digitado: &u16) {
    let mut ano_é_bissexto = String::from("não ");

    if *ano_digitado > 1900 {
        if ano_digitado % 4 == 0 {
            if ano_digitado % 100 != 0 {
                ano_é_bissexto = String::from("");
            } else {
                if ano_digitado % 400 == 0{
                    ano_é_bissexto = String::from("");
                }
            }
        }
    }

    sleep(Duration::from_millis(1000));

    println!("Analisando o ano...\n");

    sleep(Duration::from_millis(2500));

    println!("O ano {}é BISSEXTO!\n", ano_é_bissexto);
}

fn obter_um_ano(cabeçalho_do_programa: &String) -> u16 {
    loop {
        println!("[0 para o ano atual]\nDigite um ano:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u16>() {
                    Ok(ano) => {
                        let ano_atual = obter_o_ano_atual();

                        if ano >= 1900 && ano <= ano_atual {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!(
                                "\nAno de {},\nescolhido com sucesso!\n",
                                ano
                            );

                            return ano;
                        } else if ano == 0 {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!(
                                "\nAno de {},\nescolhido com sucesso!\n",
                                ano_atual
                            );
                            return ano_atual;
                        }else {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!("\nErro! Digite um ano entre 1900 até {}!\n", ano_atual);
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite um ano válido!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}