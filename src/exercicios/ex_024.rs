use std::{
    io,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 024:");
    println!(
        " Um programa que lê o nome completo de\numa pessoa, mostrando em seguida o\nprimeiro e o último nome separadamente.

Exemplo: 
\"Ana Maria de Souza\"
- primeiro.: Ana
- último...: Souza"
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let nome_completo_digitado = obter_nome_completo_da_pessoa(&cabeçalho_do_programa);

    analisar_o_nome_informado(&nome_completo_digitado);

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn analisar_o_nome_informado(nome_informado: &String) {
    let mut primeiro_nome = String::from("-");
    let mut último_nome = String::from("-");

    let nome_separado: Vec<&str> = nome_informado.split(" ").collect();

    for (index, nome) in nome_separado.iter().enumerate() {
        if index == 0 {
            primeiro_nome = nome.to_string();
        }
        
        último_nome = nome.to_string();
    }

    sleep(Duration::from_millis(1200));

    println!("Analisando o nome...\n");

    sleep(Duration::from_millis(2500));

    println!(
        "Primeiro Nome: {}",
        primeiro_nome
    );

    sleep(Duration::from_millis(900));

    println!(
        "Último Nome..: {}",
        último_nome
    );

    sleep(Duration::from_millis(1500));

}

fn obter_nome_completo_da_pessoa(cabeçalho_do_programa: &String) -> String {
    loop {
        println!("Digite o nome completo de uma pessoa:");

        let mut input = String::new();
    
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                clean_terminal_linux();

                println!("{}", cabeçalho_do_programa);

                descrição_do_exercício();

                println!(
                    "\nO nome {},\nfoi adicionado com sucesso!\n",
                    input.trim()
                );

                return (input.trim()).to_string();
            }
            Err(_) => println!("Erro!"),
        }
    }
}