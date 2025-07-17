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
    println!("Descrição do exercício 030:");
    println!(
        " Um programa que lê três números, e mostra\nqual é o MAIOR e qual é o MENOR entre\neles."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        /* Começo do Exercício */
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício - fn main */
        let mut números_digitados: Vec<u32> = vec![];

        for index in 1..4 {
            números_digitados.push(obter_um_número_inteiro(&index, &cabeçalho_do_programa));
        }

        analisar_qual_é_o_maior_e_o_menor_número_na_lista(&números_digitados);

        sleep(Duration::from_millis(1500));

        let resposta_da_pergunta_sobre_continuar = perguntar_se_quer_rodar_novamente_o_exercício(&cabeçalho_do_programa);

        if resposta_da_pergunta_sobre_continuar == false {
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

fn perguntar_se_quer_rodar_novamente_o_exercício(cabeçalho_do_programa: &String) -> bool {
    loop {
        println!("\nQuer adicionar novos número? [S/N]");

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

                    println!("\nErro! Digite apenas S [sim] ou N [não]!\n");
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn analisar_qual_é_o_maior_e_o_menor_número_na_lista(números_inteiros: &Vec<u32>) {
    let mut número_maior: u32 = 0;
    let mut número_menor: u32 = 0;

    for (index, número) in números_inteiros.iter().enumerate() {
        if index == 0 {
            número_menor = *número;
            número_maior = *número;
        } else {
            if número < &número_menor {
                número_menor = *número;
            }

            if número > &número_maior {
                número_maior = *número;
            }
        }
    }

    sleep(Duration::from_millis(1000));

    println!("Analisando os números...");

    sleep(Duration::from_millis(1000));
    
    println!("{:?}\n", números_inteiros);

    sleep(Duration::from_millis(3500));

    println!("Maior.: {}", número_maior);

    sleep(Duration::from_millis(1000));

    println!("Menor.: {}", número_menor);
}

fn obter_um_número_inteiro(index: &u8, cabeçalho_do_programa: &String) -> u32 {
    loop {
        println!("Digite o {}º Número:", index);

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nNúmero {},\nadicionado com sucesso!\n",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite apenas número!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}