use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

use chrono::Utc;

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 036:");
    println!(
        " Um programa que lê o ano de nascimento de\num jovem e informa, de acordo com a sua\nidade, se ele ainda vai se alistar\nao serviço militar, se é a hora de se\nalistar ou se já passou do tempo do\nalistamento.
O programa também deverá mostrar o tempo\nque falta ou que passou do prazo."
    );
}

struct Pessoa {
    _ano_de_nascimento: u32,
    idade: u32,
}

impl Pessoa {
    fn new(ano_de_nascimento: u32, ano_atual: &u32) -> Self {
        Self {
            _ano_de_nascimento: ano_de_nascimento,
            idade: *ano_atual - ano_de_nascimento,
        }
    }

}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    /* Começo do Exercício */
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let ano_atual = obter_o_ano_atual();

    let dado_da_pessoa_01 = Pessoa::new(
        obter_o_ano_de_nascimento(
            &cabeçalho_do_programa,
            &ano_atual
        ),
        &ano_atual
    );

    println!("\n{}", dado_da_pessoa_01.idade);

    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}

fn obter_o_ano_de_nascimento(cabeçalho_do_programa: &String, ano_atual: &u32) -> u32 {
    loop {
        println!("Digite o ano de nascimento:");
        
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(ano_de_nascimento) => {
                        if ano_de_nascimento > (*ano_atual - 100) {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!(
                                "\nO ano de nacimento {ano_de_nascimento},\nfoi adicionado com sucesso!"
                            );

                            return ano_de_nascimento;

                        } else {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!(
                                "\nErro! Digite um ano de nascimento maior\nque {}\n",
                                *ano_atual - 100
                            );
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nErro! Digite apenas números!\n"
                        );
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn obter_o_ano_atual() -> u32 {
    let utc = Utc::now().to_string();
    let mut ano_atual_char: Vec<char> = vec![];
    
    for (index, char) in utc.chars().enumerate() {
        if index <= 3 {
            ano_atual_char.push(char);
        }
    }
    
    let ano_atual = format!(
        "{}{}{}{}", 
        ano_atual_char[0],
        ano_atual_char[1],
        ano_atual_char[2],
        ano_atual_char[3]
    );

    let ano_atual: u32 = ano_atual.parse().unwrap();

    return ano_atual;
}