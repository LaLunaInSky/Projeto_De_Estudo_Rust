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
    println!("Descrição do exercício 040:");
    println!(
        " Desenvolva uma lógica que lê o peso e a\naltura de uma pessoa, calcula o seu IMC e\nmostra seu status corporal, de acordo com\na tabela abaixo:

- Abaixo de  18.5: Abaixo do peso
- Entre 18.5 e 25: Peso ideal
- Entre 25 e 30: Sobrepeso
- Entre 30 e 40: Obesidade
- Acima de 40: Obesidade mórbida"
    );
}

#[derive(Debug)]
struct Pessoa {
    peso: f32,
    altura: f32,
    imc: f32,
    status_corporal: String, 
}

impl Pessoa {
    fn new(peso: f32, altura: f32) -> Self {
        let imc = peso / (altura * altura);

        let status_corporal: String = if imc > 40.0 {
            String::from("Obesidade Mórbida")
        } else if imc > 30.0 {
            String::from("Obesidade")
        } else if imc > 25.0 {
            String::from("Sobrepeso")
        } else if imc > 18.5 {
            String::from("Peso Ideal")
        } else {
            String::from("Abaixo do peso")
        };
        
        Self {
            peso,
            altura,
            imc,
            status_corporal
        }
    }
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */


    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios,,,\n"
    // );

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}

fn arrendondar_um_número_real(número: f32, quantidade_depois_do_ponto: u8) -> f32 {
    let mut número_formatado = String::new();
    
    match quantidade_depois_do_ponto {
        1 => {
            número_formatado = format!(
                "{:.1}",
                número
            );
        }
        2 => {
            número_formatado = format!(
                "{:.2}",
                número
            );
        }
        _ => println!("Erro!"),
    }

    match número_formatado.parse::<f32>() {
        Ok(número_final) => número_final,
        Err(_) => 0.0,
    }
}