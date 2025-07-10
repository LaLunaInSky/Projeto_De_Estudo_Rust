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
    println!("Descrição do exercício 020:");
    println!(
        " Um programa que lê um número de 0 a 9999\ne mostra no terminal cada um dos dígitos\nseparados.

Exemplo: 
 1834
 unidade.: 4
 dezena..: 3
 centena.: 8
 milhar..: 1"
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let número_digitado: String = obter_o_número_inteiro(&cabeçalho_do_programa);

    separar_o_número(&número_digitado);

    /* Fim do Exercício */
    thread::sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn separar_o_número(número: &String) {
    let mut número_separado: Vec<char> = vec![];

    for char in número.chars() {
        número_separado.push(char);
    }

    let mut unidade = String::from("-");
    let mut dezena = String::from("-");
    let mut centena = String::from("-");
    let mut milhar = String::from("-");

    if número.len() == 4 {
        unidade = número_separado[3].to_string();
        dezena = número_separado[2].to_string();
        centena = número_separado[1].to_string();
        milhar = número_separado[0].to_string();
    } else if número.len() == 3 {
        unidade = número_separado[2].to_string();
        dezena = número_separado[1].to_string();
        centena = número_separado[0].to_string();
    } else if número.len() == 2 {
        unidade = número_separado[1].to_string();
        dezena = número_separado[0].to_string();
    } else if número.len() == 1 {
        unidade = número_separado[0].to_string();
    }

    thread::sleep(Duration::from_millis(2000));

    println!("Analisando o número...\n");

    thread::sleep(Duration::from_millis(2500));

    println!("Unidade.: {}", unidade);

    thread::sleep(Duration::from_millis(1000));

    println!("Dezena..: {}", dezena);

    thread::sleep(Duration::from_millis(1000));

    println!("Centena.: {}", centena);

    thread::sleep(Duration::from_millis(1000));

    println!("Milhar..: {}", milhar);

    thread::sleep(Duration::from_millis(3000));
}

fn obter_o_número_inteiro(cabeçalho_do_programa: &String) -> String {
    loop {
        println!("Digite um número inteiro [0 à 9999]:");

        let mut input = String::new();
    
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u16>() {
                    Ok(número) => {
                        if número <= 9999 {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!("\nNúmero {},\nadicionado com sucesso!\n", número);

                            return format!("{}", número);
                        } else {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!("\nErro! Digite um número que vá até 9999!\n");
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();
                    
                        println!("\nErro! Digite um valor válido!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}