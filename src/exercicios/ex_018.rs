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
    println!("Descrição do exercício 018:");
    println!(
        " Contexto: 
 O mesmo professor do desafio anterior\n(ex_017) quer sortear a ordem da\n apresentação dos trabalhos dos alunos.

Exercício:
 Um programa que lê o nome de quatro alunos\ne mostre em ordem sorteada os nomes para a\napresentação."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let mut nome_dos_alunos: Vec<String> = vec![];

    for indice in 1..5 {
        nome_dos_alunos.push(obter_o_nome_do_aluno(indice, &cabeçalho_do_programa));
    }

    thread::sleep(Duration::from_millis(1500));

    clean_terminal_linux();

    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!(
        "\nSorteando a ordem de apresentação\ndos alunos:\n{}, {}, {} e {}\n",
        nome_dos_alunos[0], 
        nome_dos_alunos[1],
        nome_dos_alunos[2], 
        nome_dos_alunos[3]
    );

    /* Fim do Exercício */
    // thread::sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // thread::sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}

fn sorteando_a_ordem_dos_nome(alunos: &Vec<String>) {
    let mut ordem_sorteado: Vec<usize> = vec![];

    for indice in 0..4 {
        loop {
            let número_sorteado: usize = random_range(0..4);

            if indice == 0 {
                ordem_sorteado.push(número_sorteado);

                break;
            } else {
                

            }
        }
    }
}

fn obter_o_nome_do_aluno(indice_do_nome: u8, cabeçalho_do_programa: &String) -> String {
    loop {
        println!(
            "Digite o nome do {}º Aluno:",
            indice_do_nome
        );

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                clean_terminal_linux();

                println!("{}", cabeçalho_do_programa);

                descrição_do_exercício();

                println!(
                    "\nO Aluno {},\nfoi adiconado com sucesso!\n",
                    input.trim()
                );

                return (input.trim()).to_string();
            }
            Err(_) => println!("Erro!"),
        }
    }
}