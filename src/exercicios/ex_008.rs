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
    println!("Descrição do exercício 008:");
    println!(
        " Um programa que lê quanto dinheiro uma\npessoa tem na carteira e mostra quantos\nDólares ela pode comprar.

Considere US$1,00 = R$3,27"
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    let quantia_na_carteira = obter_a_quantia_de_dinheiro_na_carteira(&cabeçalho_do_programa);

    println!(
        "\nO valor de R${:.2} adicionado com\nsucesso.\n",
        quantia_na_carteira
    );

    converter_valor_em_real_para_dolar(&quantia_na_carteira);

    thread::sleep(Duration::from_millis(3000));

    println!("\nVoltando para o menu de exercícios...\n");

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn converter_valor_em_real_para_dolar(valor_em_real: &f32) {
    let cotação_do_dolar = 3.27;

    let valor_em_dolar = (valor_em_real/ cotação_do_dolar);

    thread::sleep(Duration::from_millis(1500));

    println!(
        "Na cotação atual você poderá comprar\nU${:.2}.",
        valor_em_dolar
    );
}

fn obter_a_quantia_de_dinheiro_na_carteira(cabeçalho_do_programa: &String) -> f32 {
    loop{
        println!("Quanto de dinheiro você tem na carteira?");

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
                                "{:.2}",  number
                            );

                            match número_formatado.parse::<f32>() {
                                Ok(número) => return número,
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