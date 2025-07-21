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
    /* Começo do Exercício */
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */


    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}

fn obter_um_número_inteiro(cabeçalho_do_programa: &String, indice_de_chamada: u8) -> u32 {
    loop {
        println!("Digite o {}º Número:", indice_de_chamada);

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                println!("Input: {}", input);
            }
            Err(_) => println!("Erro!"),
        }
    }    
}