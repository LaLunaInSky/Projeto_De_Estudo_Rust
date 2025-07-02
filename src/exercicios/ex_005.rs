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
    println!("Descrição do exercício 005:");
    println!(
        " Um programa que lê duas notas de um\naluno(a), calcula e mostra a média das\nnotas."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    let mut notas: Vec<f32> = vec![];

    for nota in 1..3 {
        notas.push(obter_input_de_nota(nota, &cabeçalho_do_programa));
    }

    clean_terminal_linux();

    println!("{}", cabeçalho_do_programa);
    
    descrição_do_exercício();

    println!("\nAnalisando as notas {:.2} e {:.2}...\n", &notas[0], &notas[1]);

    thread::sleep(Duration::from_millis(2000));

    println!("A média é {:.2}!", ((&notas[0] + &notas[1]) / 2.0));

    thread::sleep(Duration::from_millis(3000));

    println!("\nVoltando para o menu de exercícios...\n");

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn obter_input_de_nota(index_da_nota: u8, cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Digite {}ª Nota: ", index_da_nota);

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {

                match input.trim().parse::<f32>() {
                    Ok(number) => {

                        if number > 0.0 && number <= 10.0 {
                            let number = format!("{:.2}", number);

                            match number.parse::<f32>() {
                                Ok(number) => {
                                    clean_terminal_linux();

                                    println!("{}", cabeçalho_do_programa);

                                    descrição_do_exercício();

                                    println!("\nA Nota {} foi adicionada com sucesso!\n", number);

                                    return number;
                                }
                                Err(_) => println!("Erro!"),
                            }
                        } else {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!("\nErro! Apenas é aceito notas de 0.0 à 10.0!\n");
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Apenas é aceito números!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}