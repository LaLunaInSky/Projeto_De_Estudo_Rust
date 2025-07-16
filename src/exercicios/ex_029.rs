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
    /* Começo do Exercício */
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let ano_escolhido = obter_um_ano(&cabeçalho_do_programa);

    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!("{}", cabeçalho_do_programa);

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
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