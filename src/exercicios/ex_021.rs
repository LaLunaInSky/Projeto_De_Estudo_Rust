use std::{
    io,
    thread,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 021:");
    println!(
        " Um programa que lê o nome de uma cidade e\nretorna se ela começa ou não com o nome\n\"SANTO\"."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    obter_o_nome_de_uma_cidade(&cabeçalho_do_programa);

    /* Fim de Exercício */
    // thread::sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // thread::sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}

fn obter_o_nome_de_uma_cidade(cabeçalho_do_programa: &String) {
    loop {
        println!("Digite o nome de um cidade:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("Input: {}", input);
            }
            Err(_) => println!("Erro!"),
        }
    }
}