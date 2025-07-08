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
    println!("Descrição do exercício 016:");
    println!(
        " Um programa que lê um ângulo qualquer e\nmostra no terminal o valor do seno,\ncosseno e tangente desse ângulo."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let angulo_inforamado = obter_o_angulo(&cabeçalho_do_programa);

    thread::sleep(Duration::from_millis(1500));

    println!("Analisando o ângulo...\n");

    thread::sleep(Duration::from_millis(2500));

    calcular_o_seno_o_cosseno_e_a_tangente_do_angulo(&angulo_inforamado);

    /* Fim do Exercício */
    // thread::sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // thread::sleep(Duration::from_millis(3000));    

    // clean_terminal_linux();
}

fn calcular_o_seno_o_cosseno_e_a_tangente_do_angulo(angulo: &f32) {
    let seno = angulo * (std::f32::consts::PI / 180.0) ;
    let cosseno = angulo;
    let tangente = angulo;
    
    println!(
        "O seno......: {}",
        seno
    );

    thread::sleep(Duration::from_millis(800));

    println!(
        "O cosseno...: {}",
        cosseno
    );

    thread::sleep(Duration::from_millis(800));

    println!(
        "A tangente..: {}",
        tangente
    );

    thread::sleep(Duration::from_millis(800));
}

fn obter_o_angulo(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Digite um ângulo:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(angulo) => {
                        if angulo <= 90 {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!(
                                "\nÂngulo de {}° graus,\nAdicionado com sucesso!\n",
                                angulo
                            );

                            return angulo as f32;
                        } else {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!("\nErro! Digite um ângulo que vai até 90°!\n");
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