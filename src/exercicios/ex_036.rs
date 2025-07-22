use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

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
    ano_de_nascimento: u32,
    idade: u8,
}

impl Pessoa {
    pub fn new_ano_de_nascimento(mut self, ano_digitado: u32) {
        self.ano_de_nascimento = ano_digitado;
    }

    fn calcular_idade(mut self) {
        
    }
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    /* Começo do Exercício */
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */


    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}