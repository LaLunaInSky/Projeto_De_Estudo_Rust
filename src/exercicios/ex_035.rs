use std::{
    io::stdin, 
    thread::sleep, 
    time::Duration,
    process::Command 
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 035:");
    println!(
        " Um programa que lê dois números inteiros\ne compara-os, retornando no terminal:

- O primeiro valor maior;
- O segundo valor maior;
- Não existe valor maior, os dois são\niguais;"
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        /* Começo do Exercício */
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício - fn main */
        let mut números_digitados: Vec<u32> = vec![];

        for indice in 1..3 {
            números_digitados.push(
                obter_um_número_inteiro(&cabeçalho_do_programa, indice)
            );
        }

        analisar_os_números(&números_digitados);

        sleep(Duration::from_millis(2000));

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_novos_números(&cabeçalho_do_programa);

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

fn perguntar_se_quer_adicionar_novos_números(cabeçalho_do_programa: &String) -> bool {
    loop {
        println!("Quer adicionar outros números? [S/N]");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
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

                    println!("\nErro! Apenas é aceito S [sim] ou N [não]!\n")
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn analisar_os_números(números: &Vec<u32>) {
    let mut resposta = String::new();

    if números[0] > números[1] {
        resposta = String::from("O \"Primeiro\" número é o MAIOR!");
    } else if números[0] < números[1] {
        resposta = String::from("O \"Segundo\" número é o MAIOR!");
    } else {
        resposta = String::from("Não existe número maior!\nAmbos são iguais!");
    }

    sleep(Duration::from_millis(1000));

    println!("Analisando os números...\n");

    sleep(Duration::from_millis(2500));

    println!(
        "{:?}\n\n{}\n",
        números,
        resposta
    );
}

fn obter_um_número_inteiro(cabeçalho_do_programa: &String, indice_de_chamada: u8) -> u32 {
    loop {
        println!("Digite o {}º Número:", indice_de_chamada);

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nO número {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nErro! Digite apenas números inteiros!\n"
                        );
                    }
                 }
            }
            Err(_) => println!("Erro!"),
        }
    }    
}