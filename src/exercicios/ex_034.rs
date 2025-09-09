use std:: {
    io::stdin
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::buscar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    final_do_exercicio::rodar_final_do_exercício
};

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("034")
        )
    );
    
    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let número_inteiro_digitado = obter_um_número_inteiro(
            &exercício_informações
        );

        mostrar_o_menu_de_opções();

        let resposta_sobre_continuar = obter_a_opção_digitada(
            &exercício_informações,
            &número_inteiro_digitado
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn obter_a_opção_digitada(
    exercício_informações: &ExercícioInformações, 
    número_digitado: &u32
) -> bool {
    loop {
        let mut _converção_na_base = String::new();

        println!(
            "\nQual opção você escolhe?"
        );
    
        let mut input = String::new();
        
        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(opção_do_menu) => {
                        if opção_do_menu < 1 || opção_do_menu > 6 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();
                            
                            println!(
                                "Você digitou o número {}\n",
                                número_digitado
                            );

                            mostrar_o_menu_de_opções();

                            println!("\nErro! Apenas é aceito o número 1 à 5!");
                        } else {
                            if opção_do_menu == 5 {
                                return false;

                            } else if opção_do_menu == 4 {
                                limpar_terminal();

                                return  true;

                            } else if opção_do_menu == 3 {
                                _converção_na_base = format!(
                                    "{:x}",
                                    número_digitado
                                );

                                limpar_terminal();

                                exercício_informações.mostrar_informações();
                                
                                println!(
                                    "O número {} em HEXA é {}\n", 
                                    número_digitado,
                                    _converção_na_base
                                );

                                mostrar_o_menu_de_opções();

                            } else if opção_do_menu == 2 {
                                _converção_na_base = format!(
                                    "{:o}",
                                    número_digitado
                                );

                                limpar_terminal();

                                exercício_informações.mostrar_informações();
                                
                                println!(
                                    "O número {} em OCTAL é {}\n", 
                                    número_digitado,
                                    _converção_na_base
                                );

                                mostrar_o_menu_de_opções();
                            } else {
                                _converção_na_base = format!(
                                    "{:b}",
                                    número_digitado
                                );

                                limpar_terminal();

                                exercício_informações.mostrar_informações();
                                
                                println!(
                                    "O número {} em BINÁRIO é {}\n", 
                                    número_digitado,
                                    _converção_na_base
                                );

                                mostrar_o_menu_de_opções();
                            }
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();
                        
                        println!(
                            "Você digitou o número {}\n", 
                            número_digitado
                        );

                        mostrar_o_menu_de_opções();
    
                        println!(
                            "\nErro! Digite apenas números!"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn mostrar_o_menu_de_opções() {
    println!(
        "[ 1 ] Base Binária
[ 2 ] Base Octal
[ 3 ] Base Hexadecimal
[ 4 ] Adicionar novo número
[ 5 ] Sair do Exercício"
    );
}

fn obter_um_número_inteiro(
    exercício_informações: &ExercícioInformações
) -> u32 {
    loop {
        println!(
            "Digite um número inteiro:"
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
                            "Erro! Digite apenas números inteiros!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}