use std::{
    process::Command,
    io
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 005:");
    println!(
        " Um programa que lê duas notas de um\naluno(a), calcula e mostra a média das\nnotas."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    // let mut notas: Vec<f32> = vec![];

    for nota in 1..3 {
        obter_input_de_nota(nota);
    }
}

fn obter_input_de_nota(index_da_nota: u8) {
    loop {
        println!("Digite {}ª Nota: ", index_da_nota);

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("Input: {}", input);
            }
            Err(_) => println!("Erro!"),
        }
    }
}