use std::{
    process::Command,
    io,
    thread,
    time::Duration
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercícios() {
    println!("Descrição do exercício 002:");
    println!(
        " Um programa que lê a entrada do teclado\ne mostra no terminal o seu tipo primitivo,\ne outras as informação possíveis sobre o\nque foi digitado.

Exemplo:

* Seu Tipo Primitivo
* Se possui espaços
* Se é apenas um número
* Se é alfabético
* Se é alfanumérico
* Se está em maiúscula
* Se está em minúscula
* Se está capitalizada"
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        println!("{}", cabeçalho_do_programa);
    
        descrição_do_exercícios();
    
        println!();
    
        let resposta_e_input = obter_a_entrada_do_teclado(&cabeçalho_do_programa);

        if resposta_e_input == "n" {
            break;
        }
    }

    println!("{}", cabeçalho_do_programa);

    println!("\nVoltando para o menu de exercício...");

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn obter_a_entrada_do_teclado(cabeçalho_do_programa: &String) -> String {
    println!(
        "Digite algo, pode conter números ou não:"
    );

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            loop {
                clean_terminal_linux();

                println!("{}", cabeçalho_do_programa);

                println!("\nFoi digitado:\n'{}'", input.trim());

                println!("\nGostaria de digitar outra frase (S/N)? ");

                let mut input_02 = String::new();

                match io::stdin().read_line(&mut input_02) {
                    Ok(_) => {
                        if input_02.trim().to_lowercase() == "s" || input_02.trim().to_lowercase() == "n" {
                            clean_terminal_linux();
                            
                            return input_02.trim().to_lowercase();
                        }
                    }
                    Err(error) => println!("Error: {}", error),
                }
            }
        }
        Err(error) => println!("Error: {}", error),
    }

    input
}