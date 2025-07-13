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
    println!("Descrição do exercício 027:");
    println!(
        " Um programa que lê um número inteiro e\nmostra na tela se ele é PAR ou ÍMPAR."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício - fn main */
        let número_informado = obter_um_número(&cabeçalho_do_programa);

        verificar_se_é_par_ou_ímpar(&número_informado);
    
        let resposta_sobre_continuar = perguntar_se_quer_rodar_novamento_o_exercício(&cabeçalho_do_programa);

        if resposta_sobre_continuar == false {
            break;
        }
    }
    
    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando o menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn perguntar_se_quer_rodar_novamento_o_exercício(cabeçalho_do_programa: &String) -> bool {
    loop {
        println!("Quer recomeçar o exercício? [S/N]");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let resultado = input.trim().to_lowercase();

                if resultado == "s" || resultado == "n" {
                    if resultado == "s" {
                        clean_terminal_linux();

                        return true;
                    } else {
                        return false;
                    }
                } else {
                    clean_terminal_linux();

                    println!("{}", cabeçalho_do_programa);

                    descrição_do_exercício();

                    println!("\nErro! Digite S (Sim) ou N (Não) apenas!\n")
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn verificar_se_é_par_ou_ímpar(número: &u32) {
    let mut resultado = String::from("ÍMPAR");

    if número % 2 == 0 {
        resultado = String::from("PAR");
    }

    sleep(Duration::from_millis(1000));
    
    println!("Analisando...\n");

    sleep(Duration::from_millis(2000));

    println!(
        "O número é {}!\n",
        resultado
    );

    sleep(Duration::from_millis(1000));
}

fn obter_um_número(cabeçalho_do_programa: &String) -> u32 {
    loop {
        println!("Digite um número inteiro:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nO número {},\nadicionado com sucesso!\n",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite um número!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}