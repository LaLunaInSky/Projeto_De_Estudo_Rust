use std::{
    io,
    process::Command,
    thread,
    time::Duration
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercícios() {
    println!("Descrição do exercício 001:");
    println!(
        "Um programa que lê dois números inteiro e\nmostra a soma entre os mesmos."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercícios();

    println!();

    let mut números_digitados: Vec<i32> = vec![];

    for indice in 1..3 {
        números_digitados.push(obter_a_entrada_de_um_número_inteiro(indice, &cabeçalho_do_programa));
    }

    números_digitados.push(
        soma_de_dois_números_inteiros(números_digitados[0], números_digitados[1])
    );

    println!(
        "A soma dos números {} + {} é igual a {}!",
        números_digitados[0], números_digitados[1], números_digitados[2]
    );

    thread::sleep(Duration::from_millis(2000));

    println!("\nVoltando para o menu de exercício...");

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn soma_de_dois_números_inteiros(primeiro_número: i32, segundo_número: i32) -> i32 {
    primeiro_número + segundo_número
}

fn obter_a_entrada_de_um_número_inteiro(
    indice_da_chamada_do_input: i32,
    cabeçalho_do_programa: &String
) -> i32 {
    loop {
        println!(
            "Digite o {indice_da_chamada_do_input}º número inteiro: "
        );

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<i32>() {
                    Ok(number) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercícios();

                        println!("\nNúmero Digitado com Sucesso!\n");
                        return number;
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercícios();

                        println!("\nErro! Digite novamente um número válido!\n")
                    }
                }
            },
            Err(_) => println!("\nErro! Digite novamente um número válido!\n"),
        }

        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soma_de_dois_números() {
        let primeiro_número = 5;
        let segundo_número = 5;

        let resultado_da_soma = soma_de_dois_números_inteiros(primeiro_número, segundo_número);

        assert_eq!(resultado_da_soma, 10);
    }
}