use std::{
    io::stdin
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::criar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    final_do_exercicio::rodar_final_do_exercício
};

mod arredondador_de_numeros_reais;
mod produto;

use arredondador_de_numeros_reais::arrendondar_um_número_real;
use produto::Produto;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("041"),
            String::from("Um programa que calcula o valor a ser\npago por um produto, considerando o seu\npreço normal e condição de pagamento:
        
- À vista dinheiro/cheque: 10% desconto
- À vista no cartão: 5% desconto
- Em até 2x no cartão: preço normal
- 3x ou mais no cartão: 20% de juros")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut produto = Produto::new(
            obter_o_valor_do_produto(
                &exercício_informações
            )
        );

        let resposta_sobre_continuar = obter_a_opção_digitada(
            &exercício_informações,
            &mut produto
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn obter_a_quantidade_de_parcelas(
    exercício_informações: &ExercícioInformações
) -> u8 {
    loop {
        println!("\n[12x parcelas é o máximo!]\nQuantas parcelas você quer?");

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(quantidade) => {
                        if quantidade >= 3 && quantidade <= 12 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "A quantidade de {} parcelas,\nfoi adicionada com sucesso!\n",
                                quantidade
                            );

                            return quantidade;
                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Aceito apenas parcelas de 3 à 12!\n"
                            );
                        }
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

fn obter_a_opção_digitada(
    exercício_informações: &ExercícioInformações,
    produto: &mut Produto
) -> bool {
    menu_de_opções();

    loop {
        println!(
            "Qual opção você escolhe?"
        );

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(opção) => {
                        match opção {
                            1 => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "Produto de R${:.2}...\nNo pix fica R${:.2}",
                                    produto.get_preço_do_produto(),
                                    produto.get_preço_no_pix()
                                );
                                
                                menu_de_opções();
                            }
                            2 => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "Produto de R${:.2}...\nNo débito fica R${:.2}",
                                    produto.get_preço_do_produto(),
                                    produto.get_preço_no_débito()
                                );
                                
                                menu_de_opções();
                            }
                            3 => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "Produto de R${:.2}...\n\nNo crédito em 2x fica com\nparcelas de R${:.2}",
                                    produto.get_preço_do_produto(),
                                    produto.get_preço_no_crédito_2_vezes()
                                );
                                
                                menu_de_opções();
                            }
                            4 => {
                                let quantidade_de_parcelas = obter_a_quantidade_de_parcelas(
                                    &exercício_informações
                                );

                                produto.calcular_preço_no_credito_x_vezes(
                                    &produto.get_preço_do_produto(),
                                    quantidade_de_parcelas
                                );

                                let preço_no_crédito_x_vezes = produto.get_preço_no_crédito_x_vezes();

                                println!(
                                    "Produto de R${:.2}...\nNo crédito em {}x fica R${:.2}\ncom parcelas de R${:.2}",
                                    produto.get_preço_do_produto(),
                                    quantidade_de_parcelas,
                                    preço_no_crédito_x_vezes[0],
                                    preço_no_crédito_x_vezes[1]
                                );
                                
                                menu_de_opções();
                            }
                            5 => {
                                limpar_terminal();

                                return true;
                            }
                            6 => {
                                return false;
                            }
                            _ => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "Produto de R${:.2}...\n",
                                    produto.get_preço_do_produto()
                                );
                                
                                menu_de_opções();

                                println!(
                                    "Erro! Digite apenas 1 à 6!\n"
                                );
                            }
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "O valor de R${:.2}...\n",
                            produto.get_preço_do_produto()
                        );
                        
                        menu_de_opções();

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

fn obter_o_valor_do_produto(
    exercício_informações: &ExercícioInformações
) -> f32 {
    loop {
        println!(
            "Digite o valor do produto:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(valor) => {
                        let valor = arrendondar_um_número_real(
                            valor,
                            2
                        );

                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "O valor de R${:.2},\nfoi adicionado com sucesso!",
                            valor
                        );

                        return valor;
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

fn menu_de_opções() {
    println!(
        "
 [ 1 ] Pix: 10% de desconto
 [ 2 ] Débito: 5% de desconto
 [ 3 ] 2x Crédito: SEM desconto
 [ 4 ] 3x ou mais Crédito: 20% de Juros
 [ 5 ] Adicionar novo valor
 [ 6 ] Fechar o exercício
"
    );
}