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
    let nome_da_cidade = obter_o_nome_de_uma_cidade(&cabeçalho_do_programa);

    analisar_o_nome_da_cidade(&nome_da_cidade);

    /* Fim de Exercício */
    thread::sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn analisar_o_nome_da_cidade(nome_da_cidade: &String) {
    thread::sleep(Duration::from_millis(2000));

    println!("Analisando a cidade...\n");

    thread::sleep(Duration::from_millis(2500));

    let nome_da_cidade = nome_da_cidade.to_lowercase();

    let nome_da_cidade_separado: Vec<&str> = nome_da_cidade.split(" ").collect();

    let mut resposta_da_analise = String::from("não ");

    if nome_da_cidade_separado[0].contains("santo") {
        resposta_da_analise = String::from("")
    }

    println!(
        "A cidade {},\n{}começa com \"SANTO\"!",
        nome_da_cidade.to_uppercase(),
        resposta_da_analise
    );

    thread::sleep(Duration::from_millis(3000));
}

fn obter_o_nome_de_uma_cidade(cabeçalho_do_programa: &String) -> String {
    loop {
        println!("Digite o nome de um cidade:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                clean_terminal_linux();

                println!("{}", cabeçalho_do_programa);

                descrição_do_exercício();

                println!(
                    "\nA cidade {},\nfoi adicionada com sucesso!\n",
                    input.trim()
                );

                return (input.trim()).to_string();
            }
            Err(_) => println!("Erro!"),
        }
    }
}