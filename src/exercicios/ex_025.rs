use std::{
    io,
    thread::sleep,
    time::Duration,
    process::Command
};

use rand::random_range;

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 025:");
    println!(
        " Um programa que faça o computador \"pensar\"\nem um número inteiro entre 0 e 5 e peça\npara o usuário tentar descobrir qual foi o\nnúmero escolhido pelo computador.\n O programa deverá escrever na tela se o\nusuário venceu ou perdeu."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercíco - fn main */
    loop {
        let número_sorteado = obter_um_número_inteiro_random();

        let palpite = obter_o_palpite(&cabeçalho_do_programa);

        analisar_o_palpite(&palpite, &número_sorteado);

        sleep(Duration::from_millis(1500));

        let resposta_do_jogar_de_novo = jogar_de_novo(&cabeçalho_do_programa);

        if resposta_do_jogar_de_novo == false {
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

fn jogar_de_novo(cabeçalho_do_programa: &String) -> bool {
    loop {
        println!("Quer tentar outra vez? [S/N]");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let resposta = input.trim().to_lowercase();

                if resposta == "s" || resposta == "n" {
                    if resposta == "s" {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!();

                        return true;
                    } else {
                        return false
                    }
                } else {
                    clean_terminal_linux();

                    println!("{}", cabeçalho_do_programa);

                    descrição_do_exercício();

                    println!("\nErro! Digite S (Sim) ou N (Não)!\n");
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn analisar_o_palpite(palpite: &u8, número_sorteado: &u8) {
    println!("Analisando...\n");

    sleep(Duration::from_millis(2000));
    
    println!("Computado.: {}", número_sorteado);
    println!("Usuário...: {}", palpite);

    println!(
        "Você {}!\n",
        if número_sorteado == palpite {"ACERTOU"} else {"ERROU"}
    );
}

fn obter_o_palpite(cabeçalho_do_programa: &String) -> u8 {
    loop {
        println!("Adivinhe o número [0 à 5]:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(número) => {
                        if número >= 0 && número <= 5 {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!(
                                "\nPalpite: {}\n",
                                número
                            );

                            return número; 
                        } else {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!("\nErro! Digite um número entre 0 e 5!\n");
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite apenas número!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn obter_um_número_inteiro_random() -> u8 {
    let número_sorteado: u8 = random_range(0..6);

    println!("Escolhi um número, tente adivinhar qual é!\n");

    return número_sorteado;
}