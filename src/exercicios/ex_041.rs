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
    println!("Descrição do exercício 041:");
    println!(
        " Um programa que calcula o valor a ser\npago por um produto, considerando o seu\npreço normal e condição de pagamento:
        
- À vista dinheiro/cheque: 10% desconto
- À vista no cartão: 5% desconto
- Em até 2x no cartão: preço normal
- 3x ou mais no cartão: 20% de juros"
    );
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    loop {
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício */
        let valor_do_produto = obter_o_valor_do_produto(
            &cabeçalho_do_programa
        );

        let resposta_sobre_continuar = obter_a_opção_digitada(
            &cabeçalho_do_programa, 
            &valor_do_produto
        );

        if !resposta_sobre_continuar {
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

struct Produto {
    valor: f32
}

impl Produto {
    fn pix(&self) -> String {
        let valor_com_10_de_desconto = self.valor - (self.valor * (10.0 / 100.0));

        format!(
            "No Pix é de R${:2}!",
            valor_com_10_de_desconto
        )
    }

    fn debito(&self) -> String {
        let valor_com_5_de_desconto = self.valor - (self.valor * (5.0 / 100.0));

        format!(
            "No débito é de R${:.2}!",
            valor_com_5_de_desconto
        )
    }

    fn credito_2x(&self) -> String {
        let valor_de_cada_parcela = self.valor / 2.0;

        format!(
            "No crédito 2x fica R${:.2},\nde R${:.2}!",
            valor_de_cada_parcela, self.valor
        )
    }
}

fn obter_a_opção_digitada(
    cabeçalho_do_programa: &String,
    valor_do_produto: &f32
) -> bool {
    menu_de_opções();
    let produto = Produto {
        valor: *valor_do_produto
    };

    loop {
        println!("Qual opção você escolhe?");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(opção) => {
                        match opção {
                            1 => {
                                clean_terminal_linux();

                                println!("{}", cabeçalho_do_programa);

                                descrição_do_exercício();

                                println!(
                                    "\nProduto de R${:.2}...",
                                    valor_do_produto
                                );

                                println!(
                                    "{}\n",
                                    produto.pix()
                                );
                                
                                menu_de_opções();
                            }
                            2 => {
                                clean_terminal_linux();

                                println!("{}", cabeçalho_do_programa);

                                descrição_do_exercício();

                                println!(
                                    "\nProduto de R${:.2}...",
                                    valor_do_produto
                                );

                                println!(
                                    "{}\n",
                                    produto.debito()
                                );
                                
                                menu_de_opções();
                            }
                            3 => {
                                clean_terminal_linux();

                                println!("{}", cabeçalho_do_programa);

                                descrição_do_exercício();

                                println!(
                                    "\nProduto de R${:.2}...",
                                    valor_do_produto
                                );

                                println!(
                                    "{}\n",
                                    produto.credito_2x()
                                );
                                
                                menu_de_opções();
                            }
                            4 => {
                                clean_terminal_linux();

                                println!("{}", cabeçalho_do_programa);

                                descrição_do_exercício();

                                println!("\nOpção 4\n");
                                
                                menu_de_opções();
                            }
                            5 => {
                                clean_terminal_linux();

                                return true;
                            }
                            6 => {
                                return false;
                            }
                            _ => {
                                clean_terminal_linux();

                                println!("{}", cabeçalho_do_programa);

                                descrição_do_exercício();

                                println!(
                                    "\nProduto de R${:.2}...\n",
                                    valor_do_produto
                                );
                                
                                menu_de_opções();

                                println!(
                                    "Erro! Digite apenas 1 à 6!\n"
                                );
                            }
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nO valor de R${:.2}\n",
                            valor_do_produto
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
    cabeçalho_do_programa: &String
) -> f32 {
    loop {
        println!("Digite o valor do produto:");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(valor) => {
                        let valor_formatado = format!(
                            "{:.2}",
                            valor
                        );

                        match valor_formatado.parse::<f32>() {
                            Ok(valor_final) => {
                                clean_terminal_linux();

                                println!("{}", cabeçalho_do_programa);

                                descrição_do_exercício();

                                println!(
                                    "\nO valor de R${:.2},\nfoi adicionado com sucesso!\n",
                                    valor_final
                                );

                                return valor_final;
                            }
                            Err(_) => (),
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nErro! Digite apenas números!\n"
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
        " [ 1 ] Pix: 10% de desconto
 [ 2 ] Débito: 5% de desconto
 [ 3 ] 2x Crédito: SEM desconto
 [ 4 ] 3x ou mais Crédito: 20% de Juros
 [ 5 ] Adicionar novo valor
 [ 6 ] Fechar o exercício
"
    );
}