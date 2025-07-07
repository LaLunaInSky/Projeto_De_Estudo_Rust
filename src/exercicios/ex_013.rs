use std::{
    io,
    thread,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 013:");
    println!(
        " Um programa que pergunta a quantidade de\nKm percorridos por um carro alugado e a\nquantidade de dias pelos quais ele foi\nalugado. 
 O programa irá calcular o preço à ser\npago, sabendo que o carro custa R$60.00\npor dia e R$0.15 por Km rodado."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let distância_percorrida = obter_a_quantidade_de_kms_percorridos(&cabeçalho_do_programa);

    let quantidade_de_dias = obter_a_quantidade_de_dias_permanecidos(cabeçalho_do_programa);

    thread::sleep(Duration::from_millis(1500));

    calcular_o_valor_a_ser_pago(&distância_percorrida, &quantidade_de_dias);

    /* Fim do Exercício */
    thread::sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux(); 
}

fn calcular_o_valor_a_ser_pago(distânica: &f32, dias: &f32) {
    println!("Calculando o preço final a ser pago...\n");

    thread::sleep(Duration::from_millis(2500));

    let total_a_pagar: f32 = (60.0 * dias) + (0.15 * *distânica);

    println!(
        "O total fica em R${:.2}.",
        total_a_pagar
    );
}

fn obter_a_quantidade_de_dias_permanecidos(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Quantos dias você ficou com o carro?");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(dias) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nTotal de {} dias,\nAdicionado com sucesso!\n",
                            dias
                        );

                        let dias_string = format!("{}", dias);

                        match dias_string.parse::<f32>() {
                            Ok(distância_final) => return distância_final,
                            Err(_) => println!("Erro!"),
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

fn obter_a_quantidade_de_kms_percorridos(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Quantos quilômetros(Km) você percorreu\ncom o carro?");
    
        let mut input = String::new();
    
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(distância) => {
                        if distância > 0.0 {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            let distância_formatada = format!(
                                "{:.2}", distância
                            );

                            match distância_formatada.parse::<f32>() {
                                Ok(distância_final) => {
                                    println!(
                                        "\nDistância de {:.2}Km\nAdicionada com sucesso!\n",
                                        distância_final
                                    );

                                    return distância_final
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