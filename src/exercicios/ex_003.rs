use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::{clean_terminal_linux, recursos::{
    descricao_de_exercicio::descrição_de_exercício, exercicio_informacoes::Exercício_Informações, limpar_terminal::limpar_terminal
}};

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */    
    let exercício_informações = Exercício_Informações::new(
        cabeçalho_do_programa,
        descrição_de_exercício(
            String::from("003"), 
            String::from("Um programa que lê um número inteiro e\nmostra na tela o seusucessor e seu\nantecessor.")   
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let número_digitado = obter_um_número_inteiro(
            &exercício_informações
        );

        antecessor_e_sucessor_do_número_inteiro(&número_digitado);
    
        let resposta_sobre_continuar = perguntar_se_quer_rodar_novamente(
            &exercício_informações
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando para o menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    limpar_terminal();
}

fn perguntar_se_quer_rodar_novamente(
    exercício_informações: &Exercício_Informações
) -> bool {
    loop {
        println!(
            "Quer iniciar novamente o exercício? [S/N]"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let resposta_da_pergunta = input.trim().to_lowercase();

                let resposta_da_pergunta = resposta_da_pergunta.as_str();

                match resposta_da_pergunta {
                    "s" => {
                        clean_terminal_linux();
                        
                        return true;
                    }
                    "n" => return false,
                    _ => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Apenas é aceito S [sim] ou N [não]!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn antecessor_e_sucessor_do_número_inteiro(
    número_inteiro: &u32
) {
    println!(
        "O Sucessor é.....: {}\nO Antescessor é..: {}\n",
        (número_inteiro - 1),
        (número_inteiro + 1)
    );
}

fn obter_um_número_inteiro(
    exercício_informações: &Exercício_Informações
) -> u32 {
    loop {
        println!(
            "Digite um número inteiro: "
        );
    
        let mut input = String::new();
    
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        if número == 0 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Digite um número válido!\n"
                            );
                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();
                            
                            println!(
                                "O Número {},\nfoi adicionado com sucesso!\n",
                                número
                            );
                            
                            return número;
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite um número válido!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}