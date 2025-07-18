use std::{
    io,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 032:");
    println!(
        " Um programa que lê o comprimento de três\nretas e retorna se elas podem ou não\nformar um triângulo."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        /* Começo do Exercício */
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício - fn main */
        let mut segmentos_de_um_suposto_triângulo: Vec<u32> = vec![];

        for indice in 1..4 {
            segmentos_de_um_suposto_triângulo.push(
                obter_um_segmento_de_um_suposto_triângulo(
                    &cabeçalho_do_programa, 
                    &indice
                )
            );
        }

        analisar_se_os_segmentos_formam_um_triângulo(&segmentos_de_um_suposto_triângulo);

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_outros_segmentos(&cabeçalho_do_programa);

        if resposta_sobre_continuar == false {
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

fn perguntar_se_quer_adicionar_outros_segmentos(cabeçalho_do_programa: &String) -> bool {
    loop {
        println!("\nQuer adicionar outros segmentos? [S/N]");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
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

                    println!("\nErro! Apenas é aceito S [sim] ou N [não]!\n")
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn analisar_se_os_segmentos_formam_um_triângulo(segmentos: &Vec<u32>) {
    let mut resposta_da_analise = String::from("NÃO ");

    if segmentos[0] + segmentos[1] > segmentos [2] && segmentos[0] + segmentos[2] > segmentos[1] && segmentos[1] + segmentos[2] > segmentos[0] {
        resposta_da_analise = String::from("");
    }

    sleep(Duration::from_millis(1000));

    println!("Analisando os segmentos..\n");

    sleep(Duration::from_millis(2500));

    println!(
        "Os segmentos {:?},\n{}podem formar um triângulo!",
        segmentos, resposta_da_analise
    );

    sleep(Duration::from_millis(1500));
}

fn obter_um_segmento_de_um_suposto_triângulo(cabeçalho_do_programa: &String, indice_da_chamada: &u8) -> u32 {
    loop {
        println!("Digite o {indice_da_chamada}º Segmento:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(segmento) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nO segmento {},\nfoi adicionado com sucesso!\n",
                            segmento
                        );

                        return segmento;
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