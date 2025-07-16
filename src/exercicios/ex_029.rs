use std:: {
    io,
    thread::sleep,
    time::Duration,
    process::Command
};

use chrono::{DateTime, Utc};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 029:");
    println!(
        " Um programa que lê um ano qualquer e\nmostra no terminal se ele é BISSEXTO."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    /* Começo do Exercício */
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    obter_um_ano(&cabeçalho_do_programa);

    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!("{}", cabeçalho_do_programa);

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}

fn obter_um_ano(cabeçalho_do_programa: &String) {
    loop {
        println!("[0 para o ano atual]\nDigite um ano:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u16>() {
                    Ok(ano) => {
                        let utc: DateTime<Utc> = 

                        println!("\nAno: {}\n", ano);
                        println!("DateTime: {}", )
                    }
                    Err(_) => {
                        println!("\nErro! Digite um ano válido!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}