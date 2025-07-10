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
    println!("Descrição do exercício 019:");
    println!(
        " Um programa que lê o nome completo de\numa pessoa e mostra:
        
- O nome com todas as letras maiúsculas e\nminúsculas.
- Quantas letras ao todo possui (sem\nconsiderar espaços).
- Quantas letras tem o primeiro nome."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    obter_o_nome_completo(&cabeçalho_do_programa);

    /* Fim do Exercício */
    // thread::sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // thread::sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}

fn obter_o_nome_completo(cabeçalho_do_programa: &String) {
    loop {
        println!("Digite o seu nome completo:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("Input: {}", input);
            }
            Err(_) => println!("Erro!"),
        }
    }
}