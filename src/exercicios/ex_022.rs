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
    println!("Descrição do exercíco 022:");
    println!(
        " Um programa que lê o nome de uma pessoa e\nretorna se ela tem \"SILVA\" no nome."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let nome_digitado = obter_o_nome_da_pessoa(&cabeçalho_do_programa);

    analisar_o_nome_digitado(&nome_digitado);

    /* Fim do Exercício */
    thread::sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn analisar_o_nome_digitado(nome_digitado: &String) {
    let nome_digitado = nome_digitado.to_lowercase();

    let mut resultado_da_analise = String::from("não ");

    if nome_digitado.contains("silva") {
        resultado_da_analise = String::from("");
    }

    thread::sleep(Duration::from_millis(1500));

    println!("Analisando do nome digitado...\n");

    thread::sleep(Duration::from_millis(3500));

    println!(
        "No nome {},\n{}existe \"SILVA\" no nome!",
        nome_digitado.to_uppercase(),
        resultado_da_analise
    );

    thread::sleep(Duration::from_millis(2500));
}

fn obter_o_nome_da_pessoa(cabeçalho_do_programa: &String) -> String {
    loop {
        println!("Digite o um nome com sobrenome:");

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