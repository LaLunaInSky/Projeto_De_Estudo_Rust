use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() -> String {
    format!(
        "Descrição do exercício 055:
 Crie um programa que leia dois números\ninteiros e mostre um menu como o do\nexemplo abaixo:
    [ 1 ] Somar
    [ 2 ] Multiplicar
    [ 3 ] Qual é o Maior
    [ 4 ] Informar Novos Números
    [ 5 ] Fechar o programa
Seu programa deverá realizar a operação\nsolicitada em cada caso.
"
    )
}

struct Números {
    números: Vec<u32>,
    soma: u32,
    multiplicação: u32,
    número_maior: u32,
}

impl Números {
    fn new(
        números: Vec<u32>
    ) -> Self {
        fn somar_números(
            números: &Vec<u32>
        ) -> u32 {
            let mut soma: u32 = 0;

            for número in números {
                soma += *número
            };

            return soma;
        }

        fn multiplicar_números(
            números: &Vec<u32>
        ) -> u32 {
            let mut mulplicação: u32 = 0;

            for (index, número) in números.iter().enumerate() {
                if index == 0 {
                    mulplicação += número;
                } else {
                    mulplicação *= número;
                }
            }

            return mulplicação;
        }

        fn obter_o_número_maior(
            números: &Vec<u32>
        ) -> u32 {
            let mut o_número_maior: u32 = 0;

            for (index, número) in números.iter().enumerate() {
                if index == 0 {
                    o_número_maior = *número;
                } else {
                    if *número > o_número_maior {
                        o_número_maior = *número;
                    }
                }
            }

            return o_número_maior;
        }

        let soma: u32 = somar_números(
            &números
        );

        let multiplicação: u32 = multiplicar_números(
            &números
        );

        let número_maior: u32 = obter_o_número_maior(
            &números
        );

        Self {
            números,
            soma,
            multiplicação,
            número_maior
        }
    }
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    loop {
        println!(
            "{}\n{}",
            cabeçalho_do_programa,
            descrição_do_exercício()
        );

        /* Corpo do Exercício */
        let mut números: Vec<u32> = vec![];

        for quantidade in 1..3 {
            números.push(
                obter_número_inteiro(
                    &cabeçalho_do_programa, 
                    quantidade
                )
            );
        }

        let números = Números::new(
            números
        );

        mostrar_menu_de_opções(
            &números
        );

        let respota_sobre_novos_números = obter_a_opção_escolhida(
            &cabeçalho_do_programa,
            &números
        );

        if !respota_sobre_novos_números {
            break;
        }
    }

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn obter_a_opção_escolhida(
    cabeçalho_do_programa: &String,
    números: &Números
) -> bool {
    loop {
        println!(
            "Qual opção você quer?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(opção) => {
                        match opção {
                            1..6 => {
                                if opção == 5 {
                                    return false;
                                } else if opção == 4 {
                                    clean_terminal_linux();

                                    return true;
                                } else if opção == 3 {
                                    clean_terminal_linux();

                                    println!(
                                        "{}\n{}",
                                        cabeçalho_do_programa,
                                        descrição_do_exercício()
                                    );

                                    println!(
                                        "O número maior é o {}.\n",
                                        números.número_maior
                                    );

                                    mostrar_menu_de_opções(
                                        &números
                                    );
                                } else if opção == 2 {
                                    clean_terminal_linux();

                                    println!(
                                        "{}\n{}",
                                        cabeçalho_do_programa,
                                        descrição_do_exercício()
                                    );

                                    println!(
                                        "Os números {} x {} = {}.\n",
                                        números.números[0],
                                        números.números[1],
                                        números.multiplicação
                                    );

                                    mostrar_menu_de_opções(
                                        &números
                                    );
                                } else {
                                    clean_terminal_linux();

                                    println!(
                                        "{}\n{}",
                                        cabeçalho_do_programa,
                                        descrição_do_exercício()
                                    );

                                    println!(
                                        "Os números {} + {} = {}.\n",
                                        números.números[0],
                                        números.números[1],
                                        números.soma
                                    );

                                    mostrar_menu_de_opções(
                                        &números
                                    );
                                }
                            }
                            _ => {
                                clean_terminal_linux();

                                println!(
                                    "{}\n{}",
                                    cabeçalho_do_programa,
                                    descrição_do_exercício()
                                );

                                mostrar_menu_de_opções(
                                    &números
                                );

                                println!(
                                    "Erro! Apenas opções de 1 à 5!\n"
                                );
                            }
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        mostrar_menu_de_opções(
                            &números
                        );

                        println!(
                            "Erro! Digite apenas números!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn mostrar_menu_de_opções(
    números: &Números
) {
    println!(
"Números digitados: {:?}

 [ 1 ] Somar
 [ 2 ] Multiplicar
 [ 3 ] O número maior
 [ 4 ] Adicionar novos números
 [ 5 ] Fechar o programa
",
        números.números
    )
}

fn obter_número_inteiro(
    cabeçalho_do_programa: &String,
    index_da_chamada: u8
) -> u32 {
    loop {
        println!(
            "Digite o {}º Número:",
            index_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        println!(
                            "O número {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        println!(
                            "Erro! Digite apenas números!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}