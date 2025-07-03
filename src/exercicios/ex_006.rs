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
    println!("Descrição do exercício 006:");
    println!(
        " Um programa que lê um valor em metros e o\nexibe convertido em todos os tipos a\nseguir:

km <- hm <- dam <- m -> dm -> cm -> mm"
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    let número_input_em_metros = obter_o_número_em_float(&cabeçalho_do_programa);

    converter_o_valor_de_metros(&número_input_em_metros);

    thread::sleep(Duration::from_millis(3000));

    println!("\nVoltando para o menu de exercícios...\n");

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn converter_o_valor_de_metros(valor_em_metros: &f32) {
    println!("\nO valor {}m foi adicionado com sucesso!\n\nConvertendo o valor...\n", valor_em_metros);

    thread::sleep(Duration::from_millis(2000));

    println!(
        "km...: {}", 
        (valor_em_metros / 1000.0)
    );

    thread::sleep(Duration::from_millis(500));

    println!(
        "hm...: {}", 
        (valor_em_metros / 100.0)
    );

    thread::sleep(Duration::from_millis(500));

    println!(
        "dam..: {}", 
        (valor_em_metros / 10.0)
    );

    thread::sleep(Duration::from_millis(500));

    println!(
        "m....: {}", 
        valor_em_metros
    );

    thread::sleep(Duration::from_millis(500));

    println!(
        "dm...: {}", 
        (valor_em_metros * 10.0)
    );

    thread::sleep(Duration::from_millis(500));

    println!(
        "cm...: {}", 
        (valor_em_metros * 100.0)
    );

    thread::sleep(Duration::from_millis(500));

    println!(
        "mm...: {}", 
        (valor_em_metros * 1000.0)
    );
}

fn obter_o_número_em_float(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Digite um valor em metros:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(number) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        return number;
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