use std::{
    process::Command,
    io,
    thread,
    time::Duration
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 004:");
    println!(
        " Um programa que lê um número inteiro e\nmostra o seu dobro, triplo e a raiz\nquadrada."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    let número_do_input = obter_input_de_um_número_inteiro(&cabeçalho_do_programa);

    thread::sleep(Duration::from_millis(2000));

    analisador_deo_número_inteiro(&número_do_input);

    thread::sleep(Duration::from_millis(3000));

    println!("\nVoltando para o menu de exercícios...\n");

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn obter_input_de_um_número_inteiro(cabeçalho_do_programa: &String) -> u32 {
    loop {
        println!("Digite um número inteiro:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(number) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!();

                        println!("Analisando o número digitado {}...", number);

                        return number;
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!();

                        println!("Erro! Entrada digitada não é válida!\n");
                    }
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}

fn analisador_deo_número_inteiro(número_inteiro: &u32) {
    println!();

    println!(
        "O Dobro é..........: {}",
        (número_inteiro * 2)
    );

    println!(
        "O triplo é.........: {}",
        (número_inteiro * 3)
    );

    println!(
        "A Raiz Quadrada é..: {}",
        número_inteiro.isqrt()
    );
}