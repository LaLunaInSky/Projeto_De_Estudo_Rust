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
    println!("Descrição do exercício 003:");
    println!(
        " Um programa que lê um número inteiro e mostra na tela o seu sucessor e seu antecessor."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercícios();

    println!();

    let número_digitado = obter_um_número_inteiro(&cabeçalho_do_programa);

    antecessor_e_sucessor_do_número_inteiro(&número_digitado);

    thread::sleep(Duration::from_millis(3000));

    println!("Voltando para o menu de exercício...");

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn antecessor_e_sucessor_do_número_inteiro(número_inteiro: &u32) {
    println!(
        "O Sucessor é.....: {}\nO Antescessor é..: {}\n",
        (número_inteiro - 1),
        (número_inteiro + 1)
    );
}

fn obter_um_número_inteiro(cabeçalho_do_programa: &String) -> u32 {
    loop {
        println!("Digite um número inteiro: ");
    
        let mut input = String::new();
    
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(number) => {
                        if number == 0 {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercícios();

                            println!("\nErro! Digite um número válido!\n");
                        } else {
                            clean_terminal_linux();
                            
                            println!("{}", cabeçalho_do_programa);
                            
                            descrição_do_exercícios();
                            
                            println!("\nNúmero {} foi adicionado com sucesso!\n", number);
                            
                            return number;
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercícios();

                        println!("\nErro! Digite um número válido!\n");
                    }
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}