use std::{
    io::stdin
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::criar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    final_do_exercicio::rodar_final_do_exercício
};

mod numeros;

use numeros::Números;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("055"),
            String::from("Crie um programa que leia dois números\ninteiros e mostre um menu como o do\nexemplo abaixo:
    [ 1 ] Somar
    [ 2 ] Multiplicar
    [ 3 ] Qual é o Maior
    [ 4 ] Informar Novos Números
    [ 5 ] Fechar o programa
Seu programa deverá realizar a operação\nsolicitada em cada caso.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut números: [u32; 2] = [0, 0];

        for quantidade in 1..3 {
            números[
                (quantidade - 1)
                as usize
            ] = obter_número_inteiro(
                    &exercício_informações, 
                    quantidade
                );
        }

        let números = Números::new(
            números
        );

        mostrar_menu_de_opções(
            &números
        );

        let respota_sobre_novos_números = obter_a_opção_escolhida(
            &exercício_informações,
            &números
        );

        if !respota_sobre_novos_números {
            break;
        }
    }

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn obter_a_opção_escolhida(
    exercício_informações: &ExercícioInformações,
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
                                    limpar_terminal();

                                    return true;
                                } else if opção == 3 {
                                    limpar_terminal();

                                    exercício_informações.mostrar_informações();

                                    println!(
                                        "O número maior é o {}.\n",
                                        números.get_número_maior()
                                    );

                                    mostrar_menu_de_opções(
                                        &números
                                    );
                                } else if opção == 2 {
                                    limpar_terminal();

                                    exercício_informações.mostrar_informações();

                                    let lista_de_números = números.get_números();

                                    println!(
                                        "Os números {} x {} = {}.\n",
                                        lista_de_números[0],
                                        lista_de_números[1],
                                        números.get_multiplicação()
                                    );

                                    mostrar_menu_de_opções(
                                        &números
                                    );
                                } else {
                                    limpar_terminal();

                                    exercício_informações.mostrar_informações();

                                    let lista_de_números = números.get_números();

                                    println!(
                                        "Os números {} + {} = {}.\n",
                                        lista_de_números[0],
                                        lista_de_números[1],
                                        números.get_soma()
                                    );

                                    mostrar_menu_de_opções(
                                        &números
                                    );
                                }
                            }
                            _ => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

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
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

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
        números.get_números()
    )
}

fn obter_número_inteiro(
    exercício_informações: &ExercícioInformações,
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
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "O número {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

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