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
    println!("Descrição do exercício 014:");
    println!(
        " Um programa que lê um número Real e\nmostra na tela a sua porção inteira.

ex: 6.127 -> 6"
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo de Exercício - fn main */
    let número_real_digitado = obter_um_número_real(&cabeçalho_do_programa);

    thread::sleep(Duration::from_millis(2000));

    analisar_o_número_inteiro(&número_real_digitado);

    /* Fim do Exercício */
    // thread::sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // thread::sleep(Duration::from_millis(3000));

    // clean_terminal_linux();  
}

fn analisar_o_número_inteiro(número_real: &f32) {
    println!("Analisando o número real...\n");

    thread::sleep(Duration::from_millis(2000));

    println!(
        "O número inteiro é {:.0}.",
        número_real
    );
}

fn obter_um_número_real(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Digite um número real:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(número) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nNúmero {} adicionado com sucesso!\n", número);

                        return número;
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