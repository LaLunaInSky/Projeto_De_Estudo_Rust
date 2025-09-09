use std::{
    io::stdin
};

use crate::recursos::{
    descricao_de_exercicio::buscar_descrição_do_exercício, exercicio_informacoes::ExercícioInformações, limpar_terminal::limpar_terminal,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */    
    let exercício_informações = ExercícioInformações::new(
        cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("003")  
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
    rodar_final_do_exercício();
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