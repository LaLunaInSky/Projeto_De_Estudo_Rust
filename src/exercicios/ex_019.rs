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
- Quantas letras o nome todo possui (sem\nconsiderar espaços).
- Quantas letras tem o primeiro nome."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let nome_digitado = obter_o_nome_completo(&cabeçalho_do_programa);

    thread::sleep(Duration::from_millis(2000));

    analisar_o_nome(&nome_digitado);

    /* Fim do Exercício */
    thread::sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn analisar_o_nome(nome: &String) {
    let nome_separado: Vec<&str> = nome.split_whitespace().collect();

    let mut total_de_letras_do_nome_todo: u8 = 0;
    let mut total_de_letras_do_primeiro_nome: u8 = 0;

    for (index, palavra) in nome_separado.iter().enumerate() {
        if index == 0 {
            total_de_letras_do_primeiro_nome += palavra.len() as u8;
        }

        total_de_letras_do_nome_todo += palavra.len() as u8;
    }

    println!(
        " O seu nome em minúsculo é:\n{}\n",
        nome.to_lowercase()
    );

    thread::sleep(Duration::from_millis(2000));

    println!(
        " O seu nome em maiúscula é:\n{}\n",
        nome.to_uppercase()
    );

    thread::sleep(Duration::from_millis(2000));

    println!(
        " O seu nome tem {} letras, sem contar os\nespaços!\n",
        total_de_letras_do_nome_todo
    );

    thread::sleep(Duration::from_millis(2000));

    println!(
        " O seu primeiro nome tem {} letras.",
        total_de_letras_do_primeiro_nome
    );

    thread::sleep(Duration::from_millis(2000));
}

fn obter_o_nome_completo(cabeçalho_do_programa: &String) -> String {
    loop {
        println!("Digite o seu nome completo:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                clean_terminal_linux();

                println!("{}", cabeçalho_do_programa);

                descrição_do_exercício();

                println!("\nOlá {}!\n", input.trim());

                return (input.trim()).to_string();
            }
            Err(_) => println!("Erro!"),
        }
    }
}