use std::{
    io,
    process::Command,
    thread,
    time::Duration
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 010:");
    println!(
        " Um programa que lê o preço de um produto\ne mostra seu novo preço com 5% de desconto."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do exercício main*/
    let preço_do_produto = obter_o_preco_de_um_produto(&cabeçalho_do_programa);

    calcular_o_desconto_do_produto(&preço_do_produto);

    /* Fim do Programa */
    thread::sleep(Duration::from_millis(3000));

    println!("\nVoltando ao menu de exercícios...\n");

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn calcular_o_desconto_do_produto(valor_do_produto: &f32) {
    let valor_com_o_desconto = valor_do_produto - (valor_do_produto * (5.0 / 100.0));

    println!("Calculando o desconto de 5%...\n");

    thread::sleep(Duration::from_millis(2000));

    println!(
        "O produto fica R${:.2}",
        valor_com_o_desconto
    );
}

fn obter_o_preco_de_um_produto(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Digite o preço do produto:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(number) => {
                        if number > 0.0 {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            let número_formatado = format!(
                                "{:.2}", number
                            );

                            match número_formatado.parse::<f32>() {
                                Ok(número) => {
                                    println!(
                                        "\nPreço de R${:.2} adicionado com sucesso!\n",
                                        número
                                    );

                                    return número;
                                }
                                Err(_) => println!("Erro!"),
                            }
                        } else {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!("\nErro! Digite um valor maior que zero!\n");
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite um valor válido!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}