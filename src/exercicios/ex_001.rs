use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::limpar_terminal::limpar_terminal;

fn descrição_do_exercícios() -> String {
    format!(
        "Descrição do exercício 001:
 Um programa que lê dois números inteiro e\nmostra a soma entre os mesmos.
"
    )
}

struct Números {
    números: Vec<u32>,
    soma_dos_números: u32
}

impl Números {
    fn new(
        números: Vec<u32>
    ) -> Self {
        let mut soma_dos_números: u32 = 0;

        for número in &números {
            soma_dos_números += número;
        }

        Self {
            números,
            soma_dos_números
        }
    }
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    println!(
        "{}\n{}",
        cabeçalho_do_programa,
        descrição_do_exercícios()
    );

    /* Corpo do Exercício */
    let mut números_digitados: Vec<u32> = vec![];

    for indice in 1..3 {
        números_digitados.push(
            obter_a_entrada_de_um_número_inteiro(
                indice, 
                &cabeçalho_do_programa
            )
        );
    }

    let números = Números::new(
        números_digitados
    );

    println!(
        "A soma dos números {} + {} é igual a {}!",
        números.números[0],
        números.números[1],
        números.soma_dos_números
    );

    /* Fim do Exercício */
    sleep(Duration::from_millis(2000));

    println!(
        "\nVoltando para o menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    limpar_terminal();
}

fn obter_a_entrada_de_um_número_inteiro(
    indice_da_chamada_do_input: u8,
    cabeçalho_do_programa: &String
) -> u32 {
    loop {
        println!(
            "Digite o {indice_da_chamada_do_input}º número inteiro: "
        );

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        limpar_terminal();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercícios()
                        );

                        println!(
                            "O número {},\nfoi adicionado com sucesso!\n",
                            número
                        );
                        
                        return número;
                    }
                    Err(_) => {
                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercícios()
                        );

                        println!(
                            "Erro! Digite novamente um número válido!\n"
                        )
                    }
                }
            },
            Err(_) => (),
        }

        
    }
}