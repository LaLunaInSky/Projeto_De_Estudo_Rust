use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    descricao_de_exercicio::descrição_de_exercício, exercicio_informacoes::ExercícioInformações, limpar_terminal::limpar_terminal,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício
};

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */    
    let exercício_informações = ExercícioInformações::new(
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
    
        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(
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
    exercício_informações: &ExercícioInformações
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