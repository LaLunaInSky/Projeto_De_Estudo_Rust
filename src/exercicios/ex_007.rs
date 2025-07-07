use std::{
    io,
    process::Command,
    thread,
    time::Duration
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 007:");
    println!(
        " Um programa que lê um número inteiro e\nmostra no terminal a sua tabuada."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    let número_digitado = obter_o_número_inteiro(&cabeçalho_do_programa);

    println!("\nNúmero {} adicionado com sucesso!\n", número_digitado);

    obter_a_tabuado_do_número_inteiro_informado(&número_digitado);

    thread::sleep(Duration::from_millis(3000));

    println!("\nVoltando para o menu de exercícios...\n");

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn obter_a_tabuado_do_número_inteiro_informado(número_inteiro: &u32) {
    println!("A Tabuada do {} é...\n", número_inteiro);

    thread::sleep(Duration::from_millis(2000));

    for número in 1..11 {
        if número != 10 {
            println!(
                "{} x 0{} = {}",
                número_inteiro,
                número,
                (número_inteiro * número)  
            );
        } else {
            println!(
                "{} x {} = {}",
                número_inteiro,
                número,
                (número_inteiro * número)
            );
        }

        thread::sleep(Duration::from_millis(500));
    }
}

fn obter_o_número_inteiro(cabeçalho_do_programa: &String) -> u32 {
    loop{
        println!("Digite um número inteiro:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(number) => {
                        if number > 0 {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            return number;
                        } else {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!("\nErro! Digite um número maior que zero!\n");
                        }

                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite um número válido!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}