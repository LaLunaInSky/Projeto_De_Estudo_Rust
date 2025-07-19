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
    println!("Descrição do exercício 033:");
    println!(
        " Um programa para aprovar o empréstimo\nbancário para a compra de uma casa.\n Pergunte o valor da casa, o salário do\ncomprador e em quantos anos ele vai pagar.
 A prestação mensal não pode exceder 30%\ndo salário ou então o empréstimo será\nnegado."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        /* Começo do Exercício */
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício - fn main */
        let valor_da_casa = obter_valor_da_casa(&cabeçalho_do_programa);

        let salário_do_comprador = obter_o_salário_do_comprador(&cabeçalho_do_programa);

        let quantidade_de_anos_para_pagar_a_casa = obter_a_quantidade_de_anos_para_pagar_a_casa(&cabeçalho_do_programa);

        analisar_se_é_possivel_o_empréstimo(&valor_da_casa, &salário_do_comprador, &quantidade_de_anos_para_pagar_a_casa);

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_novos_valores(&cabeçalho_do_programa);

        if resposta_sobre_continuar == false {
            break;
        }
    }

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando o menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn perguntar_se_quer_adicionar_novos_valores(cabeçalho_do_programa: &String) -> bool {
    loop {
        println!("\nGostaria de adicionar novos valores?\n[S/N]");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                let resposta_da_pergunta = input.trim().to_lowercase();

                if resposta_da_pergunta == "s" || resposta_da_pergunta == "n" {
                    if resposta_da_pergunta == "s" {
                        clean_terminal_linux();

                        return true;
                    } else {
                        return false;
                    }
                } else {
                    clean_terminal_linux();

                    println!("{}", cabeçalho_do_programa);

                    descrição_do_exercício();

                    println!("\nErro! Apenas é aceito S [sim] ou N [não]!\n");
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn analisar_se_é_possivel_o_empréstimo(valor_da_casa: &f32, salário_do_comprador: &f32, quantidade_de_anos_para_pagar: &u8) {
    let quantidade_de_anos_para_pagar = *quantidade_de_anos_para_pagar as f32;

    let trinta_porcento_do_salário: f32 = *salário_do_comprador * (30.0 / 100.0);

    let prestação_mensal_da_casa: f32 = *valor_da_casa / (quantidade_de_anos_para_pagar * 12.0);

    let mut aprovação_do_empréstimo = false;

    if prestação_mensal_da_casa < trinta_porcento_do_salário {
        aprovação_do_empréstimo = true;
    }

    sleep(Duration::from_millis(1000));

    println!("Analisando a liberação do Empréstimo...\n");

    sleep(Duration::from_millis(2500));

    println!(
        "O empréstimo está {}!",
        if aprovação_do_empréstimo == true {"LIBERADO"} else {"NEGADO"}
    );

    sleep(Duration::from_millis(1000));
}

fn obter_a_quantidade_de_anos_para_pagar_a_casa(cabeçalho_do_programa: &String) -> u8 {
    loop {
        println!("Em quantos anos vai pagar a casa?");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(anos) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nA quantidade de {} anos,\nfoi adicionada com sucesso!\n",
                            anos
                        );

                        return anos;
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
            Err(_) => println!("Erro!"),
        }
    }
}

fn obter_o_salário_do_comprador(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Qual o salário do comprador?");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(salário) => {
                        let salário_formatado = format!(
                            "{:.2}",
                            salário
                        );

                        match salário_formatado.parse::<f32>() {
                            Ok(salário_final) => {
                                clean_terminal_linux();

                                println!("{}", cabeçalho_do_programa);

                                descrição_do_exercício();

                                println!(
                                    "\nO salário de R${:.2},\nfoi adicionado com sucesso!\n",
                                    salário_final
                                );

                                return salário_final;
                            }
                            Err(_) => println!("Erro!"),
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite apenas números!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn obter_valor_da_casa(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Qual o valor da casa?");

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
                                    "\nO valor da casa de R${:.2},\nfoi adicionado com sucesso!\n",
                                    valor_final
                                );

                                return valor_final
                            }
                            Err(_) => println!("Erro!"),
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite apenas números!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}