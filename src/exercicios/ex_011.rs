use std::{
    io,
    process::Command,
    thread,
    time::Duration
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 011:");
    println!(
        " Um programa que lê o salário de um funcionário e mostra o seu novo salário com 15% de aumento."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do exercício main */
    obter_o_salario();

    /* Fim do Exercício */
    // thread::sleep(Duration::from_millis(3000));

    // println!("\nVoltando ao menu de exercícios...\n");

    // thread::sleep(Duration::from_millis(3000));  

    // clean_terminal_linux();  
}

fn obter_o_salario() {
    loop {
        println!("Digite o salário do funcionário:");
    
        let mut input = String::new();
    
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(number) => println!("Number: {}", number),
                    Err(_) => println!("Erro! Digite um valor válido!"),
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}