use std::{
    io,
    thread,
    time::Duration,
    process::Command
};

use rand::random_range; 

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 017:");
    println!(
        "Contexto:
 Um professor quer sortear um dos seus\nquatro alunos para apagar o quadro.

Exercício:
 Um programa que ajude ele, lendo o nome\ndos quatro alunos e retornando o nome do\nescolhido."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let mut nome_do_alunos: Vec<String> = vec![]; 

    for número_do_indice in 1..5 {
        nome_do_alunos.push(obter_o_nome_de_um_aluno(número_do_indice, &cabeçalho_do_programa));
    }

    thread::sleep(Duration::from_millis(1500));

    clean_terminal_linux();

    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!(
        "\nSorteando o nome entre os alunos:\n{}, {}, {} e {}\n",
        nome_do_alunos[0], nome_do_alunos[1], nome_do_alunos[2], nome_do_alunos[3]
    );

    sortear_um_nome(&nome_do_alunos);

    /* Fim do Exercício */
    thread::sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn sortear_um_nome(alunos: &Vec<String>) {
    let número_sorteado: usize = random_range(0..4);

    thread::sleep(Duration::from_millis(4000));

    println!(
        "O Aluno sorteado é {}.",
        alunos[número_sorteado]
    );

    thread::sleep(Duration::from_millis(2500));
}

fn obter_o_nome_de_um_aluno(indice_dos_nome: u8, cabeçalho_do_programa: &String) -> String {
    loop {
        println!(
            "Digite o nome do {}º Aluno:",
            indice_dos_nome
        );

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                clean_terminal_linux();

                println!("{}", cabeçalho_do_programa);

                descrição_do_exercício();

                println!(
                    "\nO Aluno {},\nfoi adicionado com sucesso!\n",
                    input.trim()
                );

                return (input.trim()).to_string();
            }
            Err(_) => {
                println!("Erro!");
            }
        }
    }
}