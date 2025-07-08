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
    println!("Descrição do exercício 015:");
    println!(
        " Um programa que lê o comprimento do\ncateto oposto e do cateto adjacente de um\ntriângulo retângulo, e depois calcula o\ncomprimento da hipotenusa."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let mut tamanho_dos_catetos: Vec<u32> = vec![];

    tamanho_dos_catetos.push(obter_o_tamanho_do_cateto("oposto".to_string(), &cabeçalho_do_programa));

    tamanho_dos_catetos.push(obter_o_tamanho_do_cateto("adjacente".to_string(), &cabeçalho_do_programa));

    calcular_a_hipotenusa(&tamanho_dos_catetos[0], &tamanho_dos_catetos[1]);

    /* Fim do Exercício */
    thread::sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn calcular_a_hipotenusa(cateto_oposto: &u32, cateto_adjacente: &u32) {
    thread::sleep(Duration::from_millis(1500));

    println!("Calculando a hipotenusa...\n");

    thread::sleep(Duration::from_millis(3000));

    let hipotenusa: f64 = (((*cateto_adjacente * *cateto_adjacente) + (*cateto_oposto * *cateto_oposto)) as f64).sqrt();

    println!(
        "O triângulo retângulo com os catetos:\nOposto: {}\nAdjacente: {}\nSua Hipotenusa é de {}.",
        cateto_oposto,
        cateto_adjacente,
        hipotenusa
    );

    thread::sleep(Duration::from_millis(2000));
}

fn obter_o_tamanho_do_cateto(nome_do_cateto: String, cabeçalho_do_programa: &String) -> u32 {
    loop {
        println!("Digite o tamanho do cateto {}:", nome_do_cateto);

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(cateto) => {
                        if cateto > 0 {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!(
                                "\nO cateto {} de {},\nAdicionado com sucesso!\n",
                                nome_do_cateto,
                                cateto
                            );

                            return cateto;
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